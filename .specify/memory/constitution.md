<!--
Sync Impact Report
- Version change: N/A (template) → 1.0.0
- Modified principles:
	- Template Principle 1 → I. UX Is the Product
	- Template Principle 2 → II. Test-First Delivery (Red-Green-Refactor)
	- Template Principle 3 → III. Minimal, Observable Automation
	- Template Principle 4 → IV. Simplicity and Maintainability
	- Template Principle 5 → V. Safe Change and Review Discipline
- Added sections:
	- Product & Experience Standards
	- Engineering Workflow & Quality Gates
- Removed sections:
	- None
- Templates requiring updates:
	- ✅ updated: .specify/templates/plan-template.md
	- ✅ updated: .specify/templates/spec-template.md
	- ✅ updated: .specify/templates/tasks-template.md
	- ⚠ pending: .specify/templates/commands/*.md (directory not present)
- Follow-up TODOs:
	- None.
-->

# Nos Autoclicker Constitution

## Core Principles

### I. UX Is the Product
Every feature MUST prioritize user experience quality as a first-class outcome, not a
post-implementation polish pass. Form and function MUST be treated as one decision:
interactions MUST feel intentional, clear, and consistent. Feature proposals and reviews
MUST include explicit UX acceptance criteria and reject changes that degrade usability.
Rationale: product value is delivered through user perception and task success, not
through implementation volume.

### II. Test-First Delivery (Red-Green-Refactor)
All implementation work MUST follow strict TDD. For each behavior change: write tests
first, run and record failure (Red), implement the minimum code to pass (Green), then
refactor while preserving passing tests (Refactor). No feature code may be merged without
evidence that the target tests failed before implementation and pass after.
Rationale: this minimizes regressions, constrains scope creep, and keeps behavior aligned
with requirements.

### III. Minimal, Observable Automation
Automation behavior MUST be deterministic, debuggable, and measurable. New workflows
MUST emit actionable logs/events for key state transitions, retries, and failures.
Features MUST define success/failure signals that can be validated in development and CI.
Rationale: automation without observability is fragile and expensive to support.

### IV. Simplicity and Maintainability
Solutions MUST default to the simplest design that satisfies current requirements.
Unjustified abstraction, speculative architecture, and duplicate logic MUST be avoided.
Each change MUST keep code paths understandable to a new maintainer within one review
session.
Rationale: lower complexity accelerates delivery and reduces long-term defects.

### V. Safe Change and Review Discipline
Every change MUST map to a requirement, include targeted tests, and pass constitutional
checks in planning artifacts. Pull requests MUST document scope, risks, and rollback
strategy when behavior changes are user-visible or operationally significant.
Rationale: explicit traceability and review rigor prevent accidental product regressions.

## Product & Experience Standards

- Every spec MUST include measurable UX outcomes (task success, clarity, or speed).
- Acceptance criteria MUST describe both functional correctness and interaction quality.
- UI/UX decisions MUST prefer consistency with established project behavior over novelty.
- Usability regressions discovered during review or testing MUST block merge until resolved.

## Engineering Workflow & Quality Gates

1. Define behavior in spec and tasks with explicit UX and test expectations.
2. Implement tests first and capture initial failure before production code changes.
3. Implement minimum viable change to satisfy tests and user-story acceptance.
4. Refactor for readability and maintainability with tests continuously passing.
5. Run CI checks (tests, lint, and relevant build steps) before merge.
6. Document operational impact for any automation behavior change.

## Governance

- This constitution supersedes ad hoc team habits for planning, implementation, and
	review decisions.
- Amendments MUST be proposed through a pull request that includes rationale, impact,
	migration actions (if needed), and updates to dependent templates and guidance files.
- Versioning policy for this constitution follows semantic versioning:
	- MAJOR: removals or redefinitions that break prior governance expectations.
	- MINOR: new principle or materially expanded mandatory guidance.
	- PATCH: wording clarifications and non-semantic refinements.
- Compliance review is mandatory in every plan and pull request; reviewers MUST block
	merge when constitutional gates are unmet.

**Version**: 1.0.0 | **Ratified**: 2026-02-15 | **Last Amended**: 2026-02-15
