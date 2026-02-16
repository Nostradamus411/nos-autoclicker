//! Unit tests for scheduler interval calculation (T014)
//! RED PHASE: These tests are written before implementation.

use nos_autoclicker::engine::scheduler::{cps_to_interval_ms, apply_jitter};

#[test]
fn test_cps_1_gives_1000ms() {
    assert_eq!(cps_to_interval_ms(1), 1000);
}

#[test]
fn test_cps_10_gives_100ms() {
    assert_eq!(cps_to_interval_ms(10), 100);
}

#[test]
fn test_cps_50_gives_20ms() {
    assert_eq!(cps_to_interval_ms(50), 20);
}

#[test]
fn test_cps_25_gives_40ms() {
    assert_eq!(cps_to_interval_ms(25), 40);
}

#[test]
fn test_cps_2_gives_500ms() {
    assert_eq!(cps_to_interval_ms(2), 500);
}

#[test]
fn test_jitter_zero_returns_exact_interval() {
    let interval = 100;
    let result = apply_jitter(interval, 0);
    assert_eq!(result, interval, "Zero jitter must return exact interval");
}

#[test]
fn test_jitter_within_bounds() {
    let interval = 100u64;
    let jitter_percent = 10u32;
    // Run many iterations to probabilistically verify bounds
    for _ in 0..1000 {
        let result = apply_jitter(interval, jitter_percent);
        let min = interval - (interval * jitter_percent as u64 / 100);
        let max = interval + (interval * jitter_percent as u64 / 100);
        assert!(
            result >= min && result <= max,
            "Jitter result {result} outside [{min}, {max}]"
        );
    }
}

#[test]
fn test_jitter_max_30_percent() {
    let interval = 100u64;
    for _ in 0..1000 {
        let result = apply_jitter(interval, 30);
        assert!(result >= 70 && result <= 130, "30% jitter on 100ms: got {result}");
    }
}

#[cfg(test)]
mod proptests {
    use proptest::prelude::*;
    use nos_autoclicker::engine::scheduler::cps_to_interval_ms;
    use nos_autoclicker::domain::settings::{CPS_MIN, CPS_MAX};

    proptest! {
        #[test]
        fn interval_is_positive_for_valid_cps(cps in CPS_MIN..=CPS_MAX) {
            let interval = cps_to_interval_ms(cps);
            prop_assert!(interval > 0, "Interval must be positive for CPS={cps}");
        }

        #[test]
        fn higher_cps_gives_shorter_or_equal_interval(
            a in CPS_MIN..=CPS_MAX,
            b in CPS_MIN..=CPS_MAX,
        ) {
            if a < b {
                prop_assert!(cps_to_interval_ms(a) >= cps_to_interval_ms(b));
            }
        }
    }
}
