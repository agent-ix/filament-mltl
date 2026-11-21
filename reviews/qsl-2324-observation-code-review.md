---
id: SR-617
title: "QSL-2324 observation follow-up code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-mltl; QSL-2324; Cargo.toml, Cargo.lock, tests/request_input_type_identity.rs, tests/support/mod.rs; frozen a216de11..6db12637"
review_set: subset
---

## Summary

Frozen head 6db12637b28c9522e29474e597d9739bd6825d0c; base a216de11d9b81bb7ed5b884534f70a2c2b00443c. Ticket QSL-2324; PR filament-mltl#32. Four changed files: source and outside-consumer manifests, Cargo.lock, and shared test support. Production source, specs, plans, schemas, CI and gates are unchanged. No applicable AssuranceProfile exists. Spec-review and its sub-analyses therefore do not apply.

The explicit architecture authorization requires the bridge to select exactly FSL's observation revision at pin-time: FSL f5977125981288ea1bd36fb67f71780182055aac selected observation 1e356f99358aaf0f9bbc42dd3451c44cc61a4d06, version 0.1.1. Both manifests now select that revision. This bounded ruling supersedes the older branch-main wording in FR-001-AC-10 for this cohort alignment; no duplicate type identity or local owner implementation is introduced. Future observation/FSL adoption belongs to its separately coordinated change, not this review.

## Verdict

**PASS** — No introduced defect found in the authorized four-file observation alignment.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Inspection

The completeness fixture calls the existing clock-family helper with Closed and the existing positions. EventPosition and FixedSample therefore retain lower=0, inclusive upper=positions-1, carrier end=positions, and watermark=positions; TimestampedEvent uses the corresponding pre-existing nanosecond mapping. The selected upstream completeness API requires this explicit TemporalBoundary; its derive path validates the boundary and fact population. Closed supplies the helper's range/watermark selection, not an inference that completeness derives closure: the completeness FactStatus and all four progress/closure views remain selected independently. No new owner assertion is synthesized by bridge production code. The changed Option ingestion field preserves an explicit value rather than replacing it with absence.

Cargo.lock has one observation 0.1.1 source and its owner-declared exact/semantic-value/quire-canonical dependencies; the obsolete fsl-foundation edge and its unused foldhash/hashbrown graph disappear. No unrelated direct owner pin changes, copied implementation, gate weakening, ignored test, source/test stub, conversion shim, new API or new unowned behavior was found in the four-file diff. Search found only the updated completeness constructor and ingestion literal in the tracked Rust tree.

## Coverage

Code-review with Rust lane ran. No separate Rust SR is created. The selected AC/test bindings below were checked against the actual assertions. The independent gap-analysis is SR-618.

## Evidence

Retained commands.txt and tests.log record five selected tests passed, zero failed or ignored, twelve filtered. The outside-consumer test compiles its own crate against the same owner SourceIds. The other witnesses cover future/past evaluation, supported clocks and semantic profiles, all 192 independent owner-axis/completeness/availability combinations, and typed read refusals. They use actual owner admission, evaluator and strict reader paths. clippy-final.log plus commands.txt record scoped lib and three selected test-target Clippy exit 0; fmt-final.log plus commands.txt record formatting exit 0. Initial clippy.log's ingestion-time type error is retained and resolved by Some(1), preserving the original supplied timestamp.

The reviewer ran no additional Cargo. Full make ci, workspace/all-target/whole-crate tests, cargo-deny, Miri and hosted CI were outside the explicit bounded authorization and were not run or inferred from these logs. This is a source-edge review, not FSL consuming validation or ticket completion. QSL-2324 stays open; FSL consumption follows the reviewed merge. git diff --check passed and the frozen worktree remained clean.

## Scope

```yaml
scope:
- id: FR-001-AC-1
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: Given valid independently-strict-read formula/proposition-map/trace-or-history/clock
    and all owner assertion views, `derive` produces one canonical request document
    with a domain-separated SHA-256 identity over its own canonical bytes, and `read`
    strict-reads it back to a constructor-private view only when every supplied owner
    input matches exactly.
- id: FR-001-AC-2
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: A future request accepts only formula-v1/non-past-operator graphs under
    a semantic profile consistent with the trace's closure, and a past request accepts
    only formula-v2/non-future-node graphs under an origin-complete history with an
    anchor at or before its through-position; every other combination is refused before
    evaluation.
- id: FR-001-AC-3
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: The clock, the four progress/closure axes, and the completeness/availability
    assertions are echoed verbatim from the supplied `filament_observation` owner
    views with no derivation between them, and a request whose clock/scope/authority
    identities disagree across those views is refused; the formula and proposition-map
    `ArtifactReference`s are independently derived from the supplied documents' own
    content identity and canonical bytes, and the evaluator reference is a distinct
    `EvaluatorReference` synthesized in-crate, neither read from a supplied view.
- id: FR-001-AC-4
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: '`evaluate` yields exactly one execution and one truth state per admitted
    request, sets `final_result` only for a completed Boolean, selects the settlement
    basis required by lane and closure, and downgrades an over-limit decision-support
    population to `ResourceIncomplete`/`Unavailable` with an empty support list instead
    of a partial one.'
- id: FR-001-AC-10
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: A consumer can construct `RequestInput` and `ObservationInputs` directly
    from the public `tl-mltl`, `tl-syntax`, and `filament-observation` branch-main
    cohort types; each field has one compile-time type identity with its owner crate,
    with no conversion or duplicate rev-pinned identity.
- id: FR-001-AC-13
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: '`report::read` of a produced result is non-evaluating: under every owner
    ceiling (input bytes, JSON depth, string bytes, visited fields, formula nodes,
    positions, propositions, support, evaluation steps and recursion depth), measuring
    bytes, depth, string bytes and visited fields from the read bytes and requiring
    the recorded semantic usage to agree with the request, it decodes the typed wire,
    requires canonical bytes, content identity and internal state consistency, and
    binds the document to the supplied request and relation (request identity and
    digest, echoed request fields, lineage revisio'
- id: FR-003-AC-1
  path: spec/requirements/FR-003-derive-contract-ir-result-mapping.md
  role: examined
  excerpt: '`map` derives `Value { .. }` only from a completed, final, Satisfied/Violated
    result with `Complete` completeness and `Available` availability; every other
    execution/completeness/availability/truth combination maps to the typed `NonValue`
    reason the source state requires.'
- id: Cargo.toml
  path: Cargo.toml
  role: examined
  excerpt: filament-observation = { version = "=0.1.1", git = "https://github.com/agent-ix/filament-observation",
    rev = "1e356f99358aaf0f9bbc42dd3451c44cc61a4d06" }
- id: Cargo.lock
  path: Cargo.lock
  role: examined
  excerpt: 'name = "filament-observation"

    version = "0.1.1"

    source = "git+https://github.com/agent-ix/filament-observation?rev=1e356f99358aaf0f9bbc42dd3451c44cc61a4d06#1e356f99358aaf0f9bbc42dd3451c44cc61a4d06"'
- id: tests/request_input_type_identity.rs
  path: tests/request_input_type_identity.rs
  role: examined
  excerpt: filament-observation = {{ version = "=0.1.1", git = "https://github.com/agent-ix/filament-observation",
    rev = "1e356f99358aaf0f9bbc42dd3451c44cc61a4d06" }}
- id: tests/support/mod.rs
  path: tests/support/mod.rs
  role: examined
  excerpt: '            fixture_clock.boundary(OpenClosed::Closed, positions),'
- id: tests/support/mod.rs::ingestion_time_nanos
  path: tests/support/mod.rs
  role: examined
  excerpt: '            ingestion_time_nanos: Some(1),'
bindings:
- test_id: outside_consumer_passes_main_cohort_types_into_request_input
  ac_id: FR-001-AC-10
  trace: correct
- test_id: tc_084_future_and_past_requests_evaluate_and_strict_read
  ac_id: FR-001-AC-1
  trace: correct
- test_id: tc_084_future_and_past_requests_evaluate_and_strict_read
  ac_id: FR-003-AC-1
  trace: correct
- test_id: tc_084_all_profiles_past_operators_and_supported_clocks_use_one_owner_path
  ac_id: FR-001-AC-2
  trace: correct
- test_id: tc_084_all_profiles_past_operators_and_supported_clocks_use_one_owner_path
  ac_id: FR-001-AC-4
  trace: correct
- test_id: tc_084_owner_axes_remain_independent_and_non_values_are_total
  ac_id: FR-001-AC-3
  trace: correct
- test_id: tc_084_owner_axes_remain_independent_and_non_values_are_total
  ac_id: FR-003-AC-1
  trace: correct
- test_id: read_refuses_with_typed_codes
  ac_id: FR-001-AC-13
  trace: correct
```
