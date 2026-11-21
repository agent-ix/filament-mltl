---
id: SR-618
title: "QSL-2324 observation follow-up gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-mltl; QSL-2324; Cargo.toml, Cargo.lock, tests/request_input_type_identity.rs, tests/support/mod.rs; frozen a216de11..6db12637"
review_set: subset
---

## Summary

Frozen head 6db12637b28c9522e29474e597d9739bd6825d0c; base a216de11d9b81bb7ed5b884534f70a2c2b00443c. Ticket QSL-2324; PR filament-mltl#32. Four changed files: source and outside-consumer manifests, Cargo.lock, and shared test support. Production source, specs, plans, schemas, CI and gates are unchanged. No applicable AssuranceProfile exists. Spec-review and its sub-analyses therefore do not apply.

The explicit architecture authorization requires the bridge to select exactly FSL's observation revision at pin-time: FSL f5977125981288ea1bd36fb67f71780182055aac selected observation 1e356f99358aaf0f9bbc42dd3451c44cc61a4d06, version 0.1.1. Both manifests now select that revision. This bounded ruling supersedes the older branch-main wording in FR-001-AC-10 for this cohort alignment; no duplicate type identity or local owner implementation is introduced. Future observation/FSL adoption belongs to its separately coordinated change, not this review.

## Verdict

**CONDITIONAL** — No introduced gap found; unchanged baseline trace gaps and bounded reverse inspection prevent a repository assurance PASS.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Inspection

The completeness fixture calls the existing clock-family helper with Closed and the existing positions. EventPosition and FixedSample therefore retain lower=0, inclusive upper=positions-1, carrier end=positions, and watermark=positions; TimestampedEvent uses the corresponding pre-existing nanosecond mapping. The selected upstream completeness API requires this explicit TemporalBoundary; its derive path validates the boundary and fact population. Closed supplies the helper's range/watermark selection, not an inference that completeness derives closure: the completeness FactStatus and all four progress/closure views remain selected independently. No new owner assertion is synthesized by bridge production code. The changed Option ingestion field preserves an explicit value rather than replacing it with absence.

Cargo.lock has one observation 0.1.1 source and its owner-declared exact/semantic-value/quire-canonical dependencies; the obsolete fsl-foundation edge and its unused foldhash/hashbrown graph disappear. No unrelated direct owner pin changes, copied implementation, gate weakening, ignored test, source/test stub, conversion shim, new API or new unowned behavior was found in the four-file diff. Search found only the updated completeness constructor and ingestion literal in the tracked Rust tree.

## Coverage

Reconciliation: quoin matrix 0.29.1 and quire coverage 0.36.2 (engine 0.50.2), fresh outputs retained as matrix.json and coverage.json. Across the full spec the engine reports 42 obligations: 29 tagged, 11 untagged, 2 method-without-symbol; all 42 have no run evidence in the store. This does not invalidate the separately retained selected runtime logs and does not establish engine-backed execution evidence.

The unchanged untagged baseline is FR-001-AC-8, FR-001-CON-1/2, FR-002-CON-1, FR-003-AC-3/4/5, FR-005-AC-5, NFR-001-AC-2, NFR-001-M-1/2, already recorded in SR-616. Coverage reports 50 untracked TC identifiers, Rust census 59 candidates/tagged/bound; diagnostics include absent archetypes, a pre-existing FR-005-AC-5 tag on a non-binding helper, uncatalogued Compile test, and catch-all-universal. These remain baseline context under the explicit architecture-focused ruling; no new source fix or new ticket is invented for them. They prevent a repository assurance PASS.

Bounded reverse-gap/stub inspection covers the changed dependency selection, outside-consumer type-identity witness, ingestion literal, and completeness fixture boundary. Their ownership remains FR-001/FR-003 and MRS-001's explicit dependency boundary. Zero newly unowned behaviors, source stubs or test stubs were found in this changed surface. No production function changed. The fresh matrix covers the repository's declared criteria, while a new whole-tree reverse-gap audit and optional semantic analysis were not performed in this bounded dispatch; no full gap-audit PASS is claimed.

Plan completion: not assessed


## Evidence

Retained commands.txt and tests.log record five selected tests passed, zero failed or ignored, twelve filtered. The outside-consumer test compiles its own crate against the same owner SourceIds. The other witnesses cover future/past evaluation, supported clocks and semantic profiles, all 192 independent owner-axis/completeness/availability combinations, and typed read refusals. They use actual owner admission, evaluator and strict reader paths. clippy-final.log plus commands.txt record scoped lib and three selected test-target Clippy exit 0; fmt-final.log plus commands.txt record formatting exit 0. Initial clippy.log's ingestion-time type error is retained and resolved by Some(1), preserving the original supplied timestamp.

The reviewer ran no additional Cargo. Full make ci, workspace/all-target/whole-crate tests, cargo-deny, Miri and hosted CI were outside the explicit bounded authorization and were not run or inferred from these logs. This is a source-edge review, not FSL consuming validation or ticket completion. QSL-2324 stays open; FSL consumption follows the reviewed merge. git diff --check passed and the frozen worktree remained clean.

## Scope

```yaml
scope:
- id: FR-002-CON-1
  path: spec/requirements/FR-002-dispatch-fobs-c00-compatibility.md
  role: examined
  excerpt: Unsupported repair-plan and closed-population-query dispatch never expose
    a Boolean, aggregate, repaired result, or partial temporal document.
- id: FR-002-AC-1
  path: spec/requirements/FR-002-dispatch-fobs-c00-compatibility.md
  role: examined
  excerpt: Given valid temporal owner views, `consume_temporal` produces bytes and
    resource usage identical to a direct FR-001 request-derivation call for the same
    inputs and limits, under both exact and one-over limits.
- id: FR-002-AC-2
  path: spec/requirements/FR-002-dispatch-fobs-c00-compatibility.md
  role: examined
  excerpt: Given the `RepairPlan` or `ClosedPopulationQuery` selector, `compatibility`
    returns the corresponding typed `Unsupported` disposition, with no value-bearing
    or artifact input/output at any point.
- id: FR-001-CON-1
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: Every owner-declared byte, depth, string, formula-node, position, proposition,
    support, and history-span ceiling is enforced before allocation or traversal completes;
    one-over refuses with a typed resource-incomplete outcome and no partial document.
- id: FR-001-CON-2
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: The mapping and result modules accept no caller-supplied truth, execution,
    or settlement value; every field is produced only from the admitted request and
    the evaluator's own outcome by `evaluate`, and `report::read` accepts a recorded
    outcome only as the document's own content-identified, state-consistent value
    bound to the admitted request, never as a caller-supplied override.
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
- id: FR-001-AC-5
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: A correction is accepted only when the predecessor's subject, correspondence,
    formula, proposition map, clock, evaluator, anchor, and lane exactly match; it
    receives a strictly greater revision and its own new identity, and the predecessor's
    bytes are never mutated.
- id: FR-001-AC-6
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: Exceeding any owner-declared resource ceiling on the request or the result
    refuses with a typed resource-incomplete outcome and no partial document is emitted;
    the same input one unit under the ceiling is admitted.
- id: FR-001-AC-7
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: A structurally valid request or result document whose bytes do not equal
    the canonical re-serialization of their own decoded value is refused with `OwnerReadErrorCode::NonCanonical`,
    even when every field value is otherwise valid.
- id: FR-001-AC-8
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: A formula referencing a `Proposition` node id absent from the supplied
    proposition map is refused with `ExpectedMismatch("propositionMap")` before evaluation
    begins; the same formula against a proposition map that includes every referenced
    id is admitted.
- id: FR-001-AC-9
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: For the past lane, a clock/history-binding pairing other than `EventPosition`/`EventPosition`
    or an exactly-agreeing `FixedSample`/`FixedSample` pair (nanosecond unit, denominator
    1 on both epoch and period, numerators equal to the clock's `epochNanos`/`periodNanos`)
    is refused with `ExpectedMismatch("clockBinding")`.
- id: FR-001-AC-10
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: A consumer can construct `RequestInput` and `ObservationInputs` directly
    from the public `tl-mltl`, `tl-syntax`, and `filament-observation` branch-main
    cohort types; each field has one compile-time type identity with its owner crate,
    with no conversion or duplicate rev-pinned identity.
- id: FR-001-AC-11
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: A validated superseding or invalidating result publicly exposes all canonical
    bytes of its admitted direct predecessor; an original exposes none. The retained
    direct predecessor remains complete and unchanged after its source value is dropped,
    without recursively retaining earlier lineage. Retention is admitted at the current
    input-byte ceiling and refuses one byte over before copying, even when the predecessor
    was admitted under a larger earlier ceiling.
- id: FR-001-AC-12
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: Non-evaluating supplied-predecessor validation accepts exactly the retained
    complete canonical bytes (or absence for an original), distinguishes missing,
    unexpected, and unequal bytes with typed errors, and rejects same-identity/different-value,
    truncated, extended, or identity-only offers without changing result bytes, usage,
    or predecessor bytes.
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
- id: FR-003-AC-2
  path: spec/requirements/FR-003-derive-contract-ir-result-mapping.md
  role: examined
  excerpt: '`MappingSelection::for_result` binds only to the exact supplied result''s
    own identity and byte digest; `map` and `read` refuse with `ExpectedMismatch`
    when a supplied selection does not match that exact result.'
- id: FR-003-AC-3
  path: spec/requirements/FR-003-derive-contract-ir-result-mapping.md
  role: examined
  excerpt: "Every mapped document embeds the complete source result wire \u2014 assessment\
    \ execution, owner references, settlement, decision-support identities, and correction\
    \ relation \u2014 so a reader losslessly joins the mapping back to its source\
    \ without a second fetch."
- id: FR-003-AC-4
  path: spec/requirements/FR-003-derive-contract-ir-result-mapping.md
  role: examined
  excerpt: The mapping's content identity is a domain-separated SHA-256 distinct from
    its source result's identity; `read` independently re-derives the full expected
    mapping, including its outcome, from the exact validated source result and refuses
    any byte that disagrees.
- id: FR-003-AC-5
  path: spec/requirements/FR-003-derive-contract-ir-result-mapping.md
  role: examined
  excerpt: '`map`/`read` accept no caller-supplied result field, Contract-IR label
    parser, callback, or trust flag; the outcome is re-derived entirely from the validated
    source result.'
- id: FR-005-AC-1
  path: spec/requirements/FR-005-census-native-correspondence-classes.md
  role: examined
  excerpt: Every entry in the reviewed class registry appears exactly once as `applicable`,
    `excluded`, or `blocked`; every class in the closed catalog appears in at least
    one entry; and deleting, duplicating, reordering, reclassifying, or adding an
    unknown entry makes the census fail while naming the complete conflicting set.
- id: FR-005-AC-2
  path: spec/requirements/FR-005-census-native-correspondence-classes.md
  role: examined
  excerpt: Every `applicable` class has exactly one canonical fixture and an independently
    derived expected outcome, every `excluded` class has a stable reason code that
    is not "not implemented", and every `blocked` class names an unresolved dependency
    and its exact admission condition and carries no outcome.
- id: FR-005-AC-3
  path: spec/requirements/FR-005-census-native-correspondence-classes.md
  role: examined
  excerpt: A class key excludes lifecycle state, so moving a class from `blocked`
    to `applicable` preserves its key and its prior population remains comparable.
- id: FR-005-AC-4
  path: spec/requirements/FR-005-census-native-correspondence-classes.md
  role: examined
  excerpt: Replay of every applicable class strict-reads its documents, re-derives
    the outcome, and agrees exactly with the recorded expectation; an expectation
    produced by calling this crate's own `derive`/`evaluate`/`read`/`map` paths is
    refused as non-independent.
- id: FR-005-AC-5
  path: spec/requirements/FR-005-census-native-correspondence-classes.md
  role: examined
  excerpt: A fixture whose bytes or contract label disagrees with what it declares
    is a typed refusal; the same fixture with agreeing identities replays.
- id: FR-005-AC-6
  path: spec/requirements/FR-005-census-native-correspondence-classes.md
  role: examined
  excerpt: "A test enumerates every file under `spec/` and `tests/` in this repository\
    \ and confirms none embeds a `filament-contract-ir` schema, vocabulary table,\
    \ or fixture body: each class's `filament-contract-ir` reference is exactly a\
    \ bounded identity record \u2014 contract labels, as text."
- id: FR-005-AC-7
  path: spec/requirements/FR-005-census-native-correspondence-classes.md
  role: examined
  excerpt: Every reported figure carries its population, exclusions, and blocked set;
    a census output that states a ratio without them, or that records an approval,
    qualification, or release decision, is refused.
- id: FR-004-AC-1
  path: spec/requirements/FR-004-preserve-native-correspondence-identity.md
  role: examined
  excerpt: A request is admitted only with a present, well-formed native subject identity
    and correspondence identity; an absent, empty, or malformed value is refused with
    a typed error and no partial document, and neither identity is ever derived, defaulted,
    or substituted from the formula, the clock, or any owner view.
- id: FR-004-AC-2
  path: spec/requirements/FR-004-preserve-native-correspondence-identity.md
  role: examined
  excerpt: 'Both identities are inside the request''s content-identity preimage: two
    requests identical except for the correspondence identity produce different request
    identities, and every result and mapping identity derived from them differs accordingly.'
- id: FR-004-AC-3
  path: spec/requirements/FR-004-preserve-native-correspondence-identity.md
  role: examined
  excerpt: The emitted result and the emitted Contract-IR mapping both carry the native
    subject and correspondence identities of the originating request byte-identically;
    a mapping or result whose embedded identities disagree with its source is refused
    with `ExpectedMismatch` rather than corrected.
- id: FR-004-AC-4
  path: spec/requirements/FR-004-preserve-native-correspondence-identity.md
  role: examined
  excerpt: A correction chain preserves both identities unchanged at every revision;
    a superseding or invalidating result that would change either identity is refused.
- id: FR-004-AC-5
  path: spec/requirements/FR-004-preserve-native-correspondence-identity.md
  role: examined
  excerpt: "Every read-time-refused correspondence class \u2014 including at least\
    \ an unsupported clock family and a disagreeing past-lane clock binding \u2014\
    \ is refused during `derive`/`read`, before any result or mapping document exists,\
    \ as a typed `OwnerReadError` naming the disagreeing field, and produces no partial\
    \ document; an over-ceiling resource case discovered at read time is likewise\
    \ refused as a typed `OwnerReadError` and never as a `NonValueKind`."
- id: FR-004-AC-6
  path: spec/requirements/FR-004-preserve-native-correspondence-identity.md
  role: examined
  excerpt: "Every evaluation-time-refused correspondence class \u2014 including at\
    \ least an incompatible semantic profile or lane and an over-ceiling resource\
    \ case discovered during evaluation \u2014 yields a typed `NonValueKind` in the\
    \ emitted result and mapping naming the exact refused dimension, and never a Boolean,\
    \ a pending truth, a defaulted value, or a silently dropped outcome."
- id: FR-004-AC-7
  path: spec/requirements/FR-004-preserve-native-correspondence-identity.md
  role: examined
  excerpt: No public operation accepts a native grammar, predicate parser or evaluator,
    catalog derivation, correspondence derivation, agreement decision, Boolean coercion,
    callback, plugin, or trust flag, and the crate manifest declares no `filament-contract-ir`
    dependency.
- id: NFR-001-AC-1
  path: spec/requirements/NFR-001-governance-boundary.md
  role: examined
  excerpt: Unknown schema/contract identities are rejected by every strict reader.
- id: NFR-001-AC-2
  path: spec/requirements/NFR-001-governance-boundary.md
  role: examined
  excerpt: Every requirement-tagged Rust test is a test Cargo actually compiles and
    runs; no compiled requirement-tagged test is ignored or configured out.
- id: NFR-001-AC-3
  path: spec/requirements/NFR-001-governance-boundary.md
  role: examined
  excerpt: No request, result, dispatch disposition, or Contract-IR mapping this crate
    emits asserts that it has been approved, that a consuming project has validated
    it, or that it constitutes a release decision.
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
