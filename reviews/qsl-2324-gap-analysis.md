---
id: SR-616
title: "QSL-2324 Filament gap-analysis review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/filament-mltl; QSL-2324; Cargo.toml, Cargo.lock, src/, six changed test files; frozen diff 386dfcf...33abda2"
review_set: subset
---

## Summary

Frozen head 33abda2b76db0c97d2f66ba7ea08877fb5a8e2bf; base 386dfcfb840b2b23be5a68ddc4d1acb0007e5f89. Ticket QSL-2324; PR filament-mltl#31. No spec/plan changes: spec-review and sub-analyses do not apply. No applicable AssuranceProfile exists. The requested generic upstream reader is consumed directly, validator errors propagate through E: From<OwnerReadError>, and no parser, validator, RefCell side channel, placeholder error or changed refusal precedence was added. The local typed contract/identity refusals are real Filament-owned conditions.

Bounded changed-surface trace, reverse-gap and stub inspection found no additional introduced assurance defect. Repository baseline trace gaps and the bounded audit scope prevent a repository assurance PASS.

## Verdict

**CONDITIONAL** — Bounded changed-surface trace, reverse-gap and stub inspection found no additional introduced assurance defect. Repository baseline trace gaps and the bounded audit scope prevent a repository assurance PASS.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Reconciliation: quire matrix 0.36.2 (engine 0.50.2), exit 0; quire coverage, exit 0. Static statuses: 29 tagged, 11 untagged, 2 method-without-symbol. The 11 unchanged untagged rows are FR-001-AC-8, FR-001-CON-1/2, FR-002-CON-1, FR-003-AC-3/4/5, FR-005-AC-5, NFR-001-AC-2, NFR-001-M-1/2. Criteria and trace tags are unchanged by this diff, so these are baseline context, not newly introduced findings. Coverage reports 50 untracked symbols (TC identifiers), Rust census 57 candidates, 57 tagged, 57 bound; diagnostics include absent archetypes, a pre-existing FR-005-AC-5 tag on a non-binding helper, uncatalogued Compile test, and catch-all-universal. Exact JSON and stderr are retained beside these artifacts. No run-evidence store was read. No full repository assurance PASS is asserted.

Plan completion: not assessed

Bounded reverse-gap/stub inspection inventoried five affected behavior groups: request derivation/read, result evaluation/non-evaluating read, mapping read, compatibility dispatch, and owner error conversion/catalog. All have FR-001/FR-002/FR-003 ownership; zero newly unowned groups and zero source/test stubs found in the changed surface. Existing tests exercise actual owner/evaluator code and assert typed outcomes. A whole-tree reverse-gap audit and optional semantic analysis were not performed in this bounded dispatch.

## Evidence

Retained final-tests.log records 6 passed, 0 failed, 0 ignored (19 filtered): one outside-consumer compile witness, two future/past and substitution cases, one three-reader unknown-contract case, and two non-evaluating/typed-refusal reads. commands.txt and clippy-changed-targets.log record lib plus six changed test-target Clippy exit 0; fmt-final.log is empty with commands.txt recording the 12-file rustfmt exit 0. No extra Cargo was run by reviewer. No workspace/whole-crate tests, full gates, cargo-deny, hosted CI or Miri are claimed. CI configuration is unchanged. Cargo.lock resolves tl-mltl 0.4.1 at the reviewed merge and exactly one tl-syntax 0.4.0 SourceId; the existing filament-observation branch-main edge is unchanged. The outside-consumer fixture deliberately uses this selected cohort; it does not establish compatibility with FSL's different observation revision. FSL adoption remains a separate consuming change and QSL-2324 stays open.

## Scope

```yaml
scope:
  - id: FR-001-AC-1
    path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
    role: examined
    excerpt: Given valid independently-strict-read
      formula/proposition-map/trace-or-history/clock and all owner assertion
      views, `derive` produces one canonical request document with a
      domain-separated SHA-256 identity over its own canonical bytes, and `read`
      strict-reads it back to a constructor-private view only when every
      supplied owner input matches exactly.
  - id: FR-001-AC-4
    path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
    role: examined
    excerpt: "`evaluate` yields exactly one execution and one truth state per
      admitted request, sets `final_result` only for a completed Boolean,
      selects the settlement basis required by lane and closure, and downgrades
      an over-limit decision-support population to
      `ResourceIncomplete`/`Unavailable` with an empty support list instead of a
      partial one."
  - id: FR-001-AC-5
    path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
    role: examined
    excerpt: A correction is accepted only when the predecessor's subject,
      correspondence, formula, proposition map, clock, evaluator, anchor, and
      lane exactly match; it receives a strictly greater revision and its own
      new identity, and the predecessor's bytes are never mutated.
  - id: FR-001-AC-7
    path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
    role: examined
    excerpt: A structurally valid request or result document whose bytes do not
      equal the canonical re-serialization of their own decoded value is refused
      with `OwnerReadErrorCode::NonCanonical`, even when every field value is
      otherwise valid.
  - id: FR-001-AC-10
    path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
    role: examined
    excerpt: A consumer can construct `RequestInput` and `ObservationInputs`
      directly from the public `tl-mltl`, `tl-syntax`, and
      `filament-observation` branch-main cohort types; each field has one
      compile-time type identity with its owner crate, with no conversion or
      duplicate rev-pinned identity.
  - id: FR-001-AC-13
    path: spec/requirements/FR-001-admit-temporal-assessment-requests-and-results.md
    role: examined
    excerpt: "`report::read` of a produced result is non-evaluating: under every
      owner ceiling (input bytes, JSON depth, string bytes, visited fields,
      formula nodes, positions, propositions, support, evaluation steps and
      recursion depth), measuring bytes, depth, string bytes and visited fields
      from the read bytes and requiring the recorded semantic usage to agree
      with the request, it decodes the typed wire, requires canonical bytes,
      content identity and internal state consistency, and binds the document to
      the supplied request and relation (request identity and digest, echoed
      request fields, lineage revisio"
  - id: FR-003-AC-1
    path: spec/requirements/FR-003-derive-contract-ir-result-mapping.md
    role: examined
    excerpt: "`map` derives `Value { .. }` only from a completed, final,
      Satisfied/Violated result with `Complete` completeness and `Available`
      availability; every other execution/completeness/availability/truth
      combination maps to the typed `NonValue` reason the source state
      requires."
  - id: FR-003-AC-2
    path: spec/requirements/FR-003-derive-contract-ir-result-mapping.md
    role: examined
    excerpt: "`MappingSelection::for_result` binds only to the exact supplied
      result's own identity and byte digest; `map` and `read` refuse with
      `ExpectedMismatch` when a supplied selection does not match that exact
      result."
  - id: NFR-001-AC-1
    path: spec/requirements/NFR-001-governance-boundary.md
    role: examined
    excerpt: Unknown schema/contract identities are rejected by every strict reader.
  - id: Cargo.lock
    path: Cargo.lock
    role: examined
    excerpt: >-
      # This file is automatically @generated by Cargo.

      # It is not intended for manual editing.

      version = 4


      [[package]]

      name = "allocator-api2"

      version = "0.2.21"

      source = "registry+https://github.com/rust-lang/crates.io-index"

      checksum =
      "683d7910e743518b0e34f1186f92494becacb047c7b6bf616c96772180fef923"


      [[package]]

      name = "arrayvec"

      version = "0.7.8"

      source = "registry+https://github.com/rust-lang/crates.io-index"

      checksum =
      "d3fb67a6e08acf24fdeccbac2cb6ac4305825bd1f117462e0e6f2f193345ad56"


      [[package]]

      name = "autocfg"

      version = "1.5.1"

      source = "registry+https://github.com/rust-lang/crates.io-
  - id: Cargo.toml
    path: Cargo.toml
    role: examined
    excerpt: >-
      [package]

      name = "filament-mltl"

      version = "0.1.0"

      edition = "2021"

      rust-version = "1.98"

      description = "Bridges tl-mltl's TL-owned evaluation types to
      filament-observation's owner-assertion views. The one deliberate exception
      to tl-mltl's independence from the Filament ecosystem."

      authors = ["Peter Krenesky <kreneskyp@gmail.com>"]

      license = "AGPL-3.0-or-later"

      repository = "https://github.com/agent-ix/filament-mltl"

      publish = false


      [workspace]


      [workspace.lints.clippy]

      undocumented_unsafe_blocks = "deny"


      [lints]

      workspace = true


      [lib]

      name = "filament_mltl"

      path = "src/lib.rs"


      [dependenci
  - id: src/contract_ir.rs
    path: src/contract_ir.rs
    role: examined
    excerpt: >-
      //! Owner-selected normalized Contract-IR result handoff.

      //!

      //! Ported from `tl-mltl`'s `mapping::contract_ir` (TL-178/TL-175). This

      //! module deliberately owns no Contract-IR vocabulary, parser, evaluator,

      //! Boolean coercion, callback, or trust flag. It derives a closed
      TL-owned

      //! map only from a constructor-private [`ValidatedTemporalResult`]. See

      //! FR-003.


      use filament_observation::authority::{availability, completeness};

      use serde::{Deserialize, Serialize};


      use crate::owner_error::{OwnerReadError, OwnerReadErrorCode};

      use tl_mltl::wire::common::{identity, is_sha256, produce, raw
  - id: src/dispatch.rs
    path: src/dispatch.rs
    role: examined
    excerpt: >-
      //! QObs C00 compatibility dispatch for the temporal owner boundary.

      //!

      //! Ported from `tl-mltl`'s `wire::observation` (TL-178/TL-175), renamed

      //! because this crate is the compat-dispatch shim itself, not the

      //! observation owner. Temporal handoff delegates to the strict bounded

      //! request adapter. QObs-owned repair and population-query semantics
      remain

      //! outside this crate and are represented by explicit compatibility

      //! dispositions rather than projections. See FR-002.


      use crate::owner_error::OwnerReadError;

      use filament_observation::authority::{query, repair};

      use tl_mltl::wire::O
  - id: src/lib.rs
    path: src/lib.rs
    role: examined
    excerpt: >-
      //! Bridges tl-mltl's TL-owned evaluation types to filament-observation's

      //! owner-assertion views. The one deliberate exception to tl-mltl's

      //! independence from the Filament ecosystem.

      //!

      //! This crate ports `tl-mltl`'s `wire::request`, `wire::observation` (as

      //! [`dispatch`]), `wire::report`, and `mapping::contract_ir` modules

      //! (TL-178/TL-175). See `spec/spec.md` (MRS-001) and FR-001..FR-003 for
      the

      //! governing requirements.


      #![warn(missing_docs)]


      pub mod census;

      pub mod contract_ir;

      pub mod dispatch;

      pub mod owner_error;

      pub mod report;

      pub mod request;


      pub use owner_error::{O
  - id: src/owner_error.rs
    path: src/owner_error.rs
    role: examined
    excerpt: >-
      //! Owner-boundary refusal types for the filament-mltl wire readers.

      //!

      //! tl-mltl 0.4 removed `ContractMismatch` and `IdentityMismatch` from its

      //! `OwnerReadErrorCode`, but FR-001-AC-13 requires them as distinct codes
      on

      //! the filament-mltl readers. [`OwnerReadErrorCode`] therefore carries
      every

      //! tl-mltl code plus those two, with identical wire labels, and

      //! [`OwnerReadError`] converts losslessly from tl-mltl's error.


      use tl_mltl::wire::{OwnerReadError as TlError, OwnerReadErrorCode as
      TlCode, OwnerUsage};


      /// Stable machine-readable refusal codes of the filament-mltl owner reade
  - id: src/report.rs
    path: src/report.rs
    role: examined
    excerpt: >
      //! Canonical `filament-mltl.temporal-assessment-result/v1` owner
      boundary.

      //!

      //! Ported from `tl-mltl`'s `wire::report` (TL-178/TL-175). Behavior is

      //! unchanged; evaluation itself is delegated entirely to `tl-mltl`'s

      //! existing future/past evaluators. See FR-001.


      use std::collections::{BTreeMap, BTreeSet};


      use filament_observation::authority::completeness;

      use serde::{Deserialize, Serialize};


      use crate::owner_error::{OwnerReadError, OwnerReadErrorCode};

      use tl_mltl::wire::common::{identity, is_sha256, produce, raw_sha256};

      use tl_mltl::wire::{OwnerLimits, OwnerUsage};

      use tl_mltl::{
  - id: src/request.rs
    path: src/request.rs
    role: examined
    excerpt: >-
      //! Canonical `filament-mltl.temporal-assessment-request/v1` owner
      boundary.

      //!

      //! Ported from `tl-mltl`'s `wire::request` (TL-178/TL-175). See FR-001.


      use filament_observation::authority::{
          availability, clock, closure, completeness, progress, OpenClosed,
      };

      use serde::{Deserialize, Serialize};

      use tl_syntax::{
          FormulaDocument, FormulaSchemaVersion, PropositionMapDocument, SemanticProfile,
          StrictDocumentReadError, SyntaxArtifactLimits, PAST_OPERATORS_V1,
      };


      use crate::owner_error::{OwnerReadError, OwnerReadErrorCode};

      use tl_mltl::wire::common::{identity, is_sha256, produce, r
  - id: tests/request_input_type_identity.rs
    path: tests/request_input_type_identity.rs
    role: examined
    excerpt: >-
      //! Outside-consumer contract for the public request input (FR-001-AC-10).

      //!

      //! A generated crate declares the git-revision tl-syntax, tl-mltl and

      //! filament-observation dependencies itself and hands their types to

      //! `filament_mltl::request`. It only compiles when those dependencies
      resolve to

      //! the same packages filament-mltl uses.


      use std::fs;

      use std::path::{Path, PathBuf};

      use std::process::Command;


      use ix_trace_rs::trace;


      const CONSUMER_MAIN: &str = r#"use
      filament_mltl::request::{ObservationInputs, RequestInput, TemporalInput};

      use filament_observation::authority::{availabili
  - id: tests/tc_084_full_predecessor.rs
    path: tests/tc_084_full_predecessor.rs
    role: examined
    excerpt: >-
      //! TC-084: complete predecessor custody and non-evaluating exact
      validation.


      mod support;


      use filament_mltl::report::{
          self, ResultRelationInput, ResultRelationKind, SuppliedPredecessorErrorCode,
          ValidatedTemporalResult,
      };

      use filament_mltl::request::ValidatedTemporalRequest;

      use filament_mltl::OwnerReadErrorCode;

      use support::{
          admit_request, future_formula, future_request_input, observations, owner_views,
          proposition_map, trace_document,
      };

      use tl_mltl::wire::{trace, OwnerLimits};

      use tl_syntax::SemanticProfile;


      fn admitted_request() -> ValidatedTemporalRequest {
          let
  - id: tests/tc_084_temporal_owner_wire.rs
    path: tests/tc_084_temporal_owner_wire.rs
    role: examined
    excerpt: >-
      //! TC-084/TC-085: the exact QObs admission boundary end to end (FR-001,

      //! FR-002, FR-003). Ported from `tl-mltl`'s
      `tests/tc_084_temporal_owner_wire.rs`

      //! (TL-178/TL-175); `tl-mltl`'s own FR-018/FR-019 described this same
      wire

      //! boundary before the port.

      //!

      //! `filament_observation::AdmissionRequest.producer` is typed

      //! `filament_observation::AdmittedStaticBundle` — a real, currently
      load-bearing

      //! member of QObs's public admission API. QObs previously sourced this
      type

      //! from `filament-core-data`'s `agent-ix-baseline-producer` crate;

      //! `filament-core-data` deliberately delete
  - id: tests/tc_086_native_correspondence.rs
    path: tests/tc_086_native_correspondence.rs
    role: examined
    excerpt: >-
      //! TC-086: FR-004, native-correspondence identity preservation.

      //!

      //! Exercises `subject_identity`/`correspondence_identity` admission,

      //! content-identity preimage membership, verbatim propagation into the

      //! result and Contract-IR mapping, correction-chain identity stability,

      //! read-time and evaluation-time refusal, unknown-contract-label refusal,

      //! and the no-`filament-contract-ir`-dependency boundary
      (FR-004-AC-1..7).

      //! Reuses `tc_084_temporal_owner_wire.rs`'s owner-admission fixtures via

      //! `tests/support`.


      mod support;


      use filament_mltl::contract_ir::{self, MappedOutcome, M
  - id: tests/tc_087_native_correspondence_census.rs
    path: tests/tc_087_native_correspondence_census.rs
    role: examined
    excerpt: >-
      //! TC-087: FR-005, the closed native-correspondence class census and its

      //! replay.

      //!

      //! Each `tc_087_*` fixture function below is named exactly by one

      //! `census::ApplicableClass::fixture_id` in `src/census.rs`'s `REGISTRY`.

      //! Every fixture: (1) builds one canonical request/result/mapping through

      //! this crate's real `derive`/`evaluate`/`read`/`map` paths, (2) compares

      //! the *re-derived* outcome against an outcome literal authored by hand
      in

      //! this file (never produced by calling this crate's own paths first --

      //! FR-005-AC-4's "non-independent" rule), and (3) proves a corrupted
  - id: tests/tc_088_nonevaluating_read.rs
    path: tests/tc_088_nonevaluating_read.rs
    role: examined
    excerpt: >-
      //! TC-088: strict read of a produced result is non-evaluating.


      mod support;


      use filament_mltl::report::{self, ResultRelationInput, TemporalTruth,
      ValidatedTemporalResult};

      use filament_mltl::request::ValidatedTemporalRequest;

      use filament_mltl::{OwnerReadError, OwnerReadErrorCode};

      use ix_trace_rs::trace;

      use sha2::{Digest, Sha256};

      use support::{
          admit_request, future_formula, future_request_input, observations, owner_views,
          proposition_map, trace_document,
      };

      use tl_mltl::wire::{trace, OwnerLimits};

      use tl_syntax::SemanticProfile;


      fn admitted_request() -> ValidatedTemporalRequest
  - id: owner_error::OwnerReadErrorCode::as_str
    path: src/owner_error.rs
    role: examined
    excerpt: |-
      Self::InvalidUtf8 => "TL-OWNER-INVALID-UTF8",
                  Self::InvalidJson => "TL-OWNER-INVALID-JSON",
                  Self::NonCanonical => "TL-OWNER-NONCANONICAL",
                  Self::ResourceIncomplete => "TL-OWNER-RESOURCE-INCOMPLETE",
                  Self::ExpectedMismatch => "TL-OWNER-EXPECTED-MISMATCH",
                  Self::InvalidCombination => "TL-OWNER-INVALID-COMBINATION",
                  Self::Encoding => "TL-OWNER-ENCODING",
  - id: owner_error::OwnerReadErrorCode
    path: src/owner_error.rs
    role: examined
    excerpt: |-
      impl OwnerReadErrorCode {
          /// Stable wire label.
          #[must_use]
          pub const fn as_str(self) -> &'static str {
              match self {
                  Self::InvalidUtf8 => "TL-OWNER-INVALID-UTF8",
                  Self::InvalidJson => "TL-OWNER-INVALID-JSON",
                  Self::NonCanonical => "TL-OWNER-NONCANONICAL",
                  Self::ResourceIncomplete => "TL-OWNER-RESOURCE-INCOMPLETE",
                  Self::ExpectedMismatch => "TL-OWNER-EXPECTED-MISMATCH",
                  Self::InvalidCombination => "TL-OWNER-INVALID-COMBINATION",
                  Self::Encoding => "TL-OWNER-ENCODING",
                  Se
bindings:
  - test_id: outside_consumer_passes_main_cohort_types_into_request_input
    ac_id: FR-001-AC-10
    trace: correct
  - test_id: tc_084_future_and_past_requests_evaluate_and_strict_read
    ac_id: FR-001-AC-1
    trace: correct
  - test_id: tc_084_request_result_map_and_lineage_reject_substitution
    ac_id: FR-001-AC-5
    trace: correct
  - test_id: tc_086_unknown_contract_label_is_refused
    ac_id: NFR-001-AC-1
    trace: correct
  - test_id: read_refuses_with_typed_codes
    ac_id: FR-001-AC-13
    trace: correct
  - test_id: read_returns_recorded_truth_without_running_an_evaluator
    ac_id: FR-001-AC-13
    trace: correct
```
