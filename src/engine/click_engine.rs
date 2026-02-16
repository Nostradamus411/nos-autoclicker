use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::domain::errors::EngineError;
use crate::domain::events::EventRecord;
use crate::domain::profile::ClickProfile;
use crate::domain::session::RunSession;
use crate::domain::settings::{RunState, StopReason};
use crate::engine::scheduler;

/// Commands that can be sent to the click engine.
#[derive(Debug, Clone)]
pub enum EngineCommand {
    Start(ClickProfile),
    Stop,
    PanicStop,
}

/// Shared engine state accessible from UI and engine thread.
pub struct ClickEngine {
    /// The current run session (None if no session exists).
    pub session: Arc<Mutex<Option<RunSession>>>,
    /// Event log buffer.
    pub events: Arc<Mutex<Vec<EventRecord>>>,
    /// Flag to signal the click loop to stop.
    running: Arc<AtomicBool>,
    /// Flag for panic stop.
    panic: Arc<AtomicBool>,
}

impl ClickEngine {
    pub fn new() -> Self {
        Self {
            session: Arc::new(Mutex::new(None)),
            events: Arc::new(Mutex::new(Vec::new())),
            running: Arc::new(AtomicBool::new(false)),
            panic: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Push an event to the log buffer.
    pub fn push_event(&self, event: EventRecord) {
        event.emit();
        if let Ok(mut events) = self.events.lock() {
            events.push(event);
            // Keep last 1000 events
            if events.len() > 1000 {
                let excess = events.len() - 1000;
                events.drain(0..excess);
            }
        }
    }

    /// Get the current run state.
    pub fn current_state(&self) -> RunState {
        self.session
            .lock()
            .ok()
            .and_then(|s| s.as_ref().map(|s| s.state))
            .unwrap_or(RunState::Idle)
    }

    /// Start a click session with the given profile.
    /// Spawns a background thread that performs clicks at the configured rate.
    pub fn start(
        &self,
        profile: &ClickProfile,
        click_fn: Arc<dyn Fn() -> bool + Send + Sync>,
    ) -> Result<(), EngineError> {
        // Validate the profile first
        profile.validate()?;

        // Check not already running
        if self.running.load(Ordering::SeqCst) {
            return Err(EngineError::AlreadyRunning);
        }

        // Create a new session
        let mut session = RunSession::new(
            profile.id.clone(),
            profile.cps,
            profile.scope_mode,
        );
        session
            .transition_to(RunState::Running)
            .map_err(|e| EngineError::InvalidTransition(e))?;

        *self.session.lock().unwrap() = Some(session);
        self.running.store(true, Ordering::SeqCst);
        self.panic.store(false, Ordering::SeqCst);

        self.push_event(EventRecord::run_started(&profile.id, profile.cps));

        // Clone state for the click loop thread
        let session_ref = Arc::clone(&self.session);
        let events_ref = Arc::clone(&self.events);
        let running = Arc::clone(&self.running);
        let panic = Arc::clone(&self.panic);
        let cps = profile.cps;
        let jitter = profile.jitter_percent;
        let stop_after_enabled = profile.stop_after_enabled;
        let stop_after_seconds = profile.stop_after_seconds;

        thread::spawn(move || {
            let start_time = Instant::now();

            while running.load(Ordering::SeqCst) {
                // Check panic stop
                if panic.load(Ordering::SeqCst) {
                    if let Ok(mut s) = session_ref.lock() {
                        if let Some(ref mut session) = *s {
                            let _ = session.stop(StopReason::PanicStop);
                        }
                    }
                    running.store(false, Ordering::SeqCst);
                    let event = EventRecord::panic_stop();
                    if let Ok(mut events) = events_ref.lock() {
                        events.push(event);
                    }
                    break;
                }

                // Check timed stop
                if stop_after_enabled {
                    if let Some(max_secs) = stop_after_seconds {
                        if start_time.elapsed().as_secs() >= max_secs as u64 {
                            if let Ok(mut s) = session_ref.lock() {
                                if let Some(ref mut session) = *s {
                                    let _ = session.stop(StopReason::TimedStop);
                                }
                            }
                            running.store(false, Ordering::SeqCst);
                            let event = EventRecord::timed_stop(max_secs);
                            if let Ok(mut events) = events_ref.lock() {
                                events.push(event);
                            }
                            break;
                        }
                    }
                }

                // Perform click
                let clicked = click_fn();
                if clicked {
                    if let Ok(mut s) = session_ref.lock() {
                        if let Some(ref mut session) = *s {
                            session.record_click();
                        }
                    }
                }

                // Sleep for the interval
                let duration = scheduler::tick_duration(cps, jitter);
                thread::sleep(duration);
            }
        });

        Ok(())
    }

    /// Stop the current session with a manual stop reason.
    pub fn stop(&self) -> Result<(), EngineError> {
        if !self.running.load(Ordering::SeqCst) {
            return Err(EngineError::NotRunning);
        }

        self.running.store(false, Ordering::SeqCst);

        if let Ok(mut s) = self.session.lock() {
            if let Some(ref mut session) = *s {
                session
                    .stop(StopReason::ManualStop)
                    .map_err(|e| EngineError::InvalidTransition(e))?;
            }
        }

        self.push_event(EventRecord::run_stopped("manual_stop"));
        Ok(())
    }

    /// Trigger an immediate panic stop.
    pub fn panic_stop(&self) -> Result<(), EngineError> {
        self.panic.store(true, Ordering::SeqCst);
        // If not currently running via thread, stop directly
        if !self.running.load(Ordering::SeqCst) {
            return Ok(());
        }
        // The click loop thread will pick up the panic flag
        // Wait briefly for the thread to finish
        std::thread::sleep(std::time::Duration::from_millis(50));
        Ok(())
    }

    /// Check if currently running.
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Get a snapshot of recent events.
    pub fn recent_events(&self, limit: usize) -> Vec<EventRecord> {
        self.events
            .lock()
            .ok()
            .map(|events| {
                let start = events.len().saturating_sub(limit);
                events[start..].to_vec()
            })
            .unwrap_or_default()
    }
}

impl Default for ClickEngine {
    fn default() -> Self {
        Self::new()
    }
}
