---
id: SR-619
title: "Code review — QSL-2158 workscope owner pins"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-mltl; Cargo.toml, Cargo.lock, tests/request_input_type_identity.rs, tests/support/mod.rs; QSL-2158"
review_set: subset
---

## Summary

Reviewed PR filament-mltl#35 at frozen candidate 6688f820ad2dca3327d81e4e2f936e4b0b4a1fa7 against base c21d0d0a41664adb57fe5a1065b9816f81f6a37d. Ticket: QSL-2158. The four-file diff advances accepted owner dependency revisions and supplies the new upstream admission field in an existing test fixture. No defects were found in this bounded review.

## Verdict

**PASS** — source and generated consumer observation selections agree, the lockfile matches the selected owners, and the empty relationship-extents field preserves the existing fixture's no-relationship meaning. This verdict covers the diff and the focused qualification below, rather than asserting that every repository gate or obligation was rechecked.

## Methods and boundaries

Code-review ran with its Rust-review lane in this single artifact. Read repository AGENTS.md, CLAUDE.md, CONTRIBUTING.md, Cargo.toml, Makefile, clippy.toml, deny.toml, rustfmt.toml, the code-review, rust-review and rust-style instructions, and relevant implementation/testing references. No additional repository Rust skill was found. No applicable AssuranceProfile exists under spec/.

Gap-analysis is not applicable to this pin/test-fixture-only change under the explicit dispatch scope: no production source or production obligation is changed. Spec-review and its sub-analyses are not applicable: no spec or plan changed. No full matrix sweep, requirement creation, external model call, subagent, or local Cargo invocation occurred. No source boundary, public API, enum dispatch, wire representation, unsafe code, concurrency, resource policy or CI/gate configuration changes in this diff required additional safety, Tokio or boundary analysis. Broader fmt, Clippy, cargo-deny, docs and full-suite gates were not run under the dispatch's explicit restriction; their success is not claimed.

## Review reasoning

The manifest keeps filament-observation at version =0.1.1 and advances its existing git selection to the accepted owner revision. Its actual Cargo.toml at that revision requests filament-exact a58474cc105a86f3307a11def3fc001e2459679f and filament-semantic-value 31d807ccb8ffd5a88beb0b1a570b80acb099f5e9. The latter's actual manifest selects that same exact revision. Cargo.lock contains those sources once each; dependency lists, package versions and all unrelated TL pins remain unchanged. Existing observation features, including the QSL-2145 behavior in the accepted owner, are not edited here. There is no added eval/IR dependency, vendored implementation, duplicated domain definition, lint waiver, test deletion or narrowed CI lane.

The existing generated consumer manifest now names the same observation source as the root manifest. Its real generated Rust code takes owner authority views and passes them directly into ObservationInputs and RequestInput; future/past inputs use tl-mltl and formula/proposition types use tl-syntax. A stale observation source can create incompatible Rust package/type identities, causing its nested offline Cargo compilation to fail. This is a compile-time type-identity oracle. It does not execute runtime evaluation. The existing fixture's revision literal is aligned in place under the explicit same-PR owner-alignment scope; no new pin mechanism was introduced.

Upstream AdmissionRequest at the selected observation revision defines relationship_extents as explicit completeness states for listed relationship declarations, with absent declarations unlisted. In tests/support/mod.rs, relationships and required_relationships were already empty, the producer has no supplied relationships, and max_relationships/max_required_relationships remain zero. The added empty vector introduces no completeness assertion or relationship premise. Upstream validation over the empty extents and required-premise sets succeeds vacuously; real admission and authority view derivation still execute. This fixes the stale struct literal without weakening its existing data or assertions.

The selected TC-084 test derives/admit-reads the request using real owner views, evaluates and strict-reads the result, asserts truth/usage/identity, maps the result, and exercises both future and past lanes. The test's FR-001-AC-1/FR-003-AC-1 tags are relevant to those assertions. This focused check supports compatibility for the exercised cases; it is not exhaustive evidence for every negative case in those criteria.

## Qualification evidence

Evidence below was reported by the dispatching lead from sequential execution on Titan at exactly frozen candidate 6688f820ad2dca3327d81e4e2f936e4b0b4a1fa7; the reviewer inspected the code independently and did not rerun Cargo locally.

- Exact selection tc_084_future_and_past_requests_evaluate_and_strict_read: 1 passed, 0 failed, 0 ignored.
- Exact selection outside_consumer_passes_main_cohort_types_into_request_input: 1 passed, 0 failed, 0 ignored; the nested offline Cargo check compiled the actual generated outside consumer.
- cargo check --locked --all-targets --all-features -p filament-mltl: exit 0, confirmed by the lead after the initial review dispatch.

Total focused runtime harness results: 2 selected, 2 passed, 0 failed, 0 ignored, plus the successful package check. The earlier 51d3 candidate failed to compile with E0063 due to the missing fixture field; no runtime success at that older candidate is claimed or substituted for frozen-head evidence.

## Examined scope

The data below records the changed statements and selected existing criteria examined, including clean units. The same data is carried in the held private tracker-comment draft.

```yaml
clean: true
scope:
- id: Cargo.toml:26
  path: Cargo.toml
  role: examined
  excerpt: filament-observation = { version = "=0.1.1", git = "https://github.com/agent-ix/filament-observation",
    rev = "004c7c4f934d15022d2a80b98c451d436109b44b" }
- id: Cargo.lock:70
  path: Cargo.lock
  role: examined
  excerpt: source = "git+https://github.com/agent-ix/filament-exact?rev=a58474cc105a86f3307a11def3fc001e2459679f#a58474cc105a86f3307a11def3fc001e2459679f"
- id: Cargo.lock:102
  path: Cargo.lock
  role: examined
  excerpt: source = "git+https://github.com/agent-ix/filament-observation?rev=004c7c4f934d15022d2a80b98c451d436109b44b#004c7c4f934d15022d2a80b98c451d436109b44b"
- id: Cargo.lock:116
  path: Cargo.lock
  role: examined
  excerpt: source = "git+https://github.com/agent-ix/filament-semantic-value?rev=31d807ccb8ffd5a88beb0b1a570b80acb099f5e9#31d807ccb8ffd5a88beb0b1a570b80acb099f5e9"
- id: tests/request_input_type_identity.rs:80
  path: tests/request_input_type_identity.rs
  role: examined
  excerpt: filament-observation = {{ version = "=0.1.1", git = "https://github.com/agent-ix/filament-observation",
    rev = "004c7c4f934d15022d2a80b98c451d436109b44b" }}
- id: tests/support/mod.rs:222
  path: tests/support/mod.rs
  role: examined
  excerpt: '        relationship_extents: vec![],'
- id: FR-001-AC-1
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: Given valid independently-strict-read formula/proposition-map/trace-or-history/clock and all owner assertion
    views, `derive` produces one canonical request document with a domain-separated SHA-256 identity over its own
    canonical bytes, and `read` strict-reads it back to a constructor-private view only when every supplied owner
    input matches exactly.
- id: FR-001-AC-10
  path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
  role: examined
  excerpt: A consumer can construct `RequestInput` and `ObservationInputs` directly from the public `tl-mltl`, `tl-syntax`,
    and `filament-observation` branch-main cohort types; each field has one compile-time type identity with its
    owner crate, with no conversion or duplicate rev-pinned identity.
- id: FR-003-AC-1
  path: spec/requirements/FR-003-derive-contract-ir-result-mapping.md
  role: examined
  excerpt: '`map` derives `Value { .. }` only from a completed, final, Satisfied/Violated result with `Complete`
    completeness and `Available` availability; every other execution/completeness/availability/truth combination
    maps to the typed `NonValue` reason the source state requires.'
findings: []
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
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Artifact custody and publication

The artifact is prepared in the assigned scratch directory for eventual reviews/qsl-2158-workscope-owner-pins-code-review.md custody. Private comment draft and publication manifest are held separately. No Linear/ix-board read or write, GitHub finding publication, label mutation, branch edit, commit, push, merge or relay occurred. Publication, deduplication, label state and read-back remain pending root release; they must be recorded in the separate manifest without rewriting this frozen review artifact.

## Tooling

Used the installed quoin 0.36.8 (/home/peter/.npm-global/bin/quoin), quire 0.52.0 (engine 0.62.2; /home/peter/.npm-global/bin/quire), and spec-artifacts-process@v0.36.0. Obtained the current SpecReview contract with quoin write . --types SpecReview. Explicit-path quire validation passed (exit 0). No older CLI or cached-version fallback is used.
