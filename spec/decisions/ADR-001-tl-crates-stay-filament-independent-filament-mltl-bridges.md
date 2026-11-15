---
id: ADR-001
title: "TL-* crates stay independent of Filament; filament-mltl is the one deliberate bridge"
type: ADR
status: accepted
owner: kreneskyp
relationships:
  - target: ix://agent-ix/filament-mltl/MRS-001
    type: relates_to
---
# ADR-001: TL-* crates stay independent of Filament; filament-mltl is the one deliberate bridge

## Status

**Accepted** — architect ruling recorded against Linear epic
[TL-175](https://linear.app/agent-ix/issue/TL-175). TL-176 scaffolded
this repository as the ruling's bridge crate; this document (TL-177) records
the decision itself. TL-178 performs the wholesale port of `tl-mltl`'s
`wire::request`, `wire::observation` (as `dispatch`), `wire::report`, and
`mapping::contract_ir` modules into this crate; TL-179 removes `tl-mltl`'s
`filament-observation` dependency and rewrites its FR-018 to a generic owner
boundary; TL-180 removes `tl-mltl`'s Filament governance citations and
retires FR-019, completing `tl-mltl`'s independence. TL-181 repoints
`filament-contract-ir`'s own imports and schema catalog from `tl-mltl` to
`filament-mltl`, completing the consumer side of this decision.

## Context

`tl-mltl` is a deterministic reference implementation of bounded
Mission-time Linear Temporal Logic evaluation over finite traces. It had
grown a direct `filament-observation` dependency to carry its
`tl-mltl.temporal-assessment-request/v1` and
`tl-mltl.temporal-assessment-result/v1` wire contracts, which bind
`filament_observation::authority::{clock,progress,closure,completeness,
availability}` owner-assertion views directly into the request/result
documents, plus a QObs C00 compatibility-dispatch shim and a Contract-IR
result mapping. That coupling meant every consumer of `tl-mltl` — including
one that only wants a standalone MLTL evaluator with no Filament involvement at
all — pulled in `filament-observation` and, through it, Filament governance that
has nothing to do with temporal evaluation.

The owner ruled that every TL-* crate (`tl-syntax`, `tl-parse`, `tl-mltl`,
`tl-rewrite`) SHALL stay independent of the Filament ecosystem, and that
only a dedicated integration crate may bridge a TL-* crate to Filament types.
`tl-mltl`'s existing wire/mapping modules are exactly that bridge, misplaced
inside the crate they should not be coupling.

## Decision

Create `filament-mltl` as the one deliberate exception: a dedicated integration
crate that depends on both `tl-mltl` and `filament-observation` (plus
`tl-syntax`) so that neither has to depend on, or know about, the other.

- `tl-mltl`'s `wire::request`, `wire::observation` (renamed `dispatch`),
  `wire::report`, and `mapping::contract_ir` modules move to `filament-mltl`
  unchanged in behavior (TL-178).
  `tl-mltl` retains only its own future/past evaluation semantics, horizon
  analysis, and the wire contracts that carry no `filament-observation` type
  (`trace`, `command`, legacy aliases).
- `tl-mltl` drops its `filament-observation` dependency (TL-179) and its
  Filament governance citations (TL-180), becoming an
  ordinary Rust crate usable by, but not part of, the Filament ecosystem.
- `filament-mltl` takes on the `filament-observation` dependency directly. It is not an ordinary consumer of Filament types;
  it is the load-bearing seam between a Filament-independent evaluator and
  Filament's own observation authority, and it is treated as a Filament-owned program repository. See
  [MRS-001](../spec.md) for the full rationale.
- `filament-mltl` is licensed `AGPL-3.0-or-later`, this org's default for
  `filament-*` program repositories — not `AGPL-3.0-only`. An
  earlier scaffold draft of this repository declared `-only` as a deliberate
  exception, reasoned from a local `filament-observation` checkout that was
  sitting on a stale branch; `filament-observation`'s actual `origin/main` has
  been `AGPL-3.0-or-later` since its 2026-09-13 relicense, so nothing in this
  crate's dependency graph forces a narrower term.

## Consequences

- `tl-mltl` becomes reusable by any consumer that wants bounded MLTL
  evaluation without accepting a Filament dependency or Filament governance —
  the ruling's stated goal.
- A consumer that specifically needs the QObs-bridged temporal-assessment
  contract, the compatibility dispatch, or the Contract-IR mapping now
  depends on `filament-mltl`, not `tl-mltl`, for that surface.
- `filament-contract-ir` is exactly that consumer: TL-181 repoints its
  `src/temporal/request.rs`, `correspondence.rs`, `join.rs`, `reader.rs`, and
  schema-catalog entries from `tl_mltl::wire::*`/`tl_mltl::mapping::contract_ir::*`
  to `filament_mltl::*`, and updates its own
  `FR-026` and `FR-028` to reflect the new boundary. This is the
  consumer-side half of this decision; `filament-contract-ir` keeps its direct
  `tl-mltl` and `filament-observation` dependencies for the lower-level joins
  that remain its own domain job, gaining no new higher-level API from
  `filament-mltl`.
- Only this repository carries the `agent-ix-baseline-producer` dev-dependency
  and the governance obligations (see
  [NFR-001](../requirements/NFR-001-governance-boundary.md)) that follow from
  being the genuinely Filament-owned half of the boundary.

## Alternatives Considered

- **Keep the wire boundary inside `tl-mltl` and let it depend on
  `filament-observation` directly (the prior arrangement).** Rejected: couples
  the reference evaluator to Filament governance and forces every non-Filament
  consumer to accept a dependency and a compatibility posture it does not
  need. This is exactly what the architect ruling prohibits.
- **Merge `filament-observation`'s authority types into `tl-mltl`, or fold
  `tl-mltl` into `filament-observation`.** Rejected: contradicts the existing
  ownership split — `tl-mltl` owns temporal evaluation semantics,
  `filament-observation` owns observation admission, replay, and authority
  derivation. A merge would duplicate governance and blur the component boundary both crates already have.
- **Leave the `filament-observation` dependency optional behind a Cargo feature
  inside `tl-mltl`, default-off.** Rejected: a feature flag is a
  compatibility layer, and this org does not add compatibility layers to
  prerelease software without an asked-for, expiring exception. It also does
  not remove the coupling from `tl-mltl`'s own dependency graph or
  governance posture when the feature is compiled in by any consumer that
  needs it — the ruling requires the dependency to not exist in `tl-mltl` at
  all, not merely to be optional.
