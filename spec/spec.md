---
id: MRS-001
title: filament-mltl v0.1 master requirements
type: MasterRequirements
relationships:
  - target: ix://agent-ix/tl-mltl/MRS-001
    type: depends_on
  - target: ix://agent-ix/filament-observation/MRS-001
    type: depends_on
---

# Master Requirements Specification

## Purpose

filament-mltl bridges `tl-mltl`'s TL-owned temporal evaluation types to
`filament-observation`'s owner-assertion views. It is the one deliberate
exception to `tl-mltl`'s independence from the Filament ecosystem: an
architect ruling (Linear epic
[TL-175](https://linear.app/agent-ix/issue/TL-175)) requires every TL-*
crate to stay independent of Filament, with only a dedicated integration crate
permitted to bridge them. `filament-mltl` exists so `tl-mltl` itself never has to
depend on, or know about, `filament-observation`.

This specification owns the wire contracts and mapping logic being ported out
of `tl-mltl` under this same epic (TL-178): the temporal-assessment
request/result contract pair, the QObs C00 compatibility dispatch shim, and
the Contract-IR result mapping. It does not re-derive temporal evaluation
semantics, observation-authority semantics, or formula/trace syntax — those
stay owned by `tl-mltl`, `filament-observation`, and `tl-syntax` respectively,
and `filament-mltl` consumes them as dependencies.

### Why filament-mltl is Filament-owned and tl-mltl is not

`tl-mltl` is fully independent of the Filament ecosystem (TL-180) — an ordinary
Rust crate that happens to be usable by Filament, not a component of it.
`filament-mltl` takes the opposite posture on purpose: it directly incorporates
`filament-observation` (a Filament-governed observation boundary) and exists
specifically to hand Filament-shaped evidence — Contract-IR mappings,
owner-assertion-qualified temporal results — back into the ecosystem. It is
not an ordinary consumer of Filament types; it is the load-bearing seam between a
Filament-independent evaluator and Filament's own observation authority. That seam
role makes `filament-mltl` itself a Filament-owned program repository, not merely a
consumer of one. The two repositories are not inconsistent with each other —
they are the two halves of the same boundary, each stating its own posture
correctly.

## Scope

### In Scope

- Owner-strict `tl-mltl.temporal-assessment-request/v1` derivation and reading
  against independently supplied `filament_observation::authority::{clock,
  progress, closure, completeness, availability}` owner views, plus
  `tl-syntax` formula/proposition-map documents and a `tl-mltl` trace or
  origin-complete history.
- Owner-strict `tl-mltl.temporal-assessment-result/v1` evaluation and reading,
  delegating the actual future/past truth evaluation to `tl-mltl`'s existing
  evaluators and preserving every owner assertion, settlement basis, decision
  support and correction relation the request carried.
- QObs C00 compatibility dispatch (the `dispatch` module, renamed from
  `tl-mltl`'s `wire::observation`): delegates the supported temporal handoff
  to request derivation with no duplicated evaluation or observation-authority
  logic, and returns an explicit typed unsupported outcome — naming the exact
  foreign contract — for the QObs-owned
  repair-plan and closed-population-query contracts, without accepting,
  inspecting, or mutating either artifact.
- Deriving a TL-owned `tl-mltl.contract-ir-result-map/v1` value-or-typed-non-value
  outcome from a validated temporal result (`mapping::contract_ir`), re-derived
  entirely from that result with no caller-supplied fields, label parser,
  callback, or trust flag.
- The strict reading, canonical-bytes, content-identity, and bounded-resource
  discipline `tl-mltl`'s existing owner boundary established, applied to the
  four ported contracts.
- Lossless preservation of the native subject and correspondence identities a
  request carries, across the request, the result and the Contract-IR mapping,
  with a typed non-value for any correspondence class this crate cannot
  represent, and strict refusal of an unknown contract label on the three
  contracts a consumer admits by label.
- A closed, reviewed census of the native-correspondence classes the bridge
  carries losslessly, with one canonical fixture and one independently derived
  expected outcome per applicable class, and replay of each one.

### Out of Scope

- Temporal evaluation semantics themselves — future/past truth evaluation,
  horizon analysis, decision-scope semantics — remain owned by `tl-mltl` and
  are consumed, not reimplemented.
- Observation admission, replay, revisioned authority derivation, repair
  coordination, and closed-population query evaluation remain owned by
  `filament-observation` and are consumed, not reimplemented or projected.
- Formula and proposition-map parsing and strict syntax validation remain
  owned by `tl-syntax`.
- Contract-IR vocabulary, parsers, evaluators, Boolean coercion helpers,
  callbacks, or trust flags — the mapping accepts no caller-supplied result
  fields and owns no Contract-IR wire type of its own.
- The FSL grammar, checked-predicate semantics, signal-catalog
  derivation, clock/history correspondence derivation, and the native/TL
  agreement decision. Those are `filament-contract-ir`'s under FR-025 and FR-026,
  and `filament-spec-language`'s for the native source itself. `filament-mltl` is
  their supplier, not their consumer, and takes no dependency on either.
- `tl-mltl`'s other owner contracts (`trace`, `command`, and the legacy
  aliases) and CLI surface, which are not part of the TL-178 port and remain
  in `tl-mltl` only.

## System Overview

`filament-mltl` sits directly between `tl-mltl` and `filament-observation`: it
depends on both crates (plus `tl-syntax`) rather than either depending on it
or on each other. A caller independently strict-reads the
`filament_observation::authority` owner views and the `tl-syntax`/`tl-mltl`
formula, proposition-map, trace, or history documents it needs, and supplies
all of them to `filament-mltl`. `filament-mltl` binds them into one immutable
canonical temporal-assessment request, evaluates it through `tl-mltl`'s
existing evaluators, emits one immutable canonical result, and can derive a
separate TL-owned Contract-IR mapping view from that result. Every emitted
document is bounded and canonical, and a strict reader either re-derives it
(request, mapping) or, for a result, binds the recorded outcome to its request
without re-running the evaluator; no branch accepts a caller-asserted fact it did not itself validate
against the supplied owner views.

## Requirements Architecture

FR-001 owns temporal-assessment request derivation, strict reading, and
result evaluation. FR-002 owns the QObs C00 compatibility dispatch boundary
and depends on FR-001 for its one supported branch. FR-003 owns the
Contract-IR result mapping and depends on FR-001 for the validated result it
maps. FR-004 owns native-correspondence identity preservation, the typed
refusal of an unrepresentable class, and strict contract-label admission
across all three documents. FR-005 owns the closed correspondence-class census and its
replay. NFR-001 constrains this crate's governance and qualification boundary.

### Why the native-correspondence dimension is here

`filament-contract-ir` FR-026 owns the native/TL correspondence, constructs the
TL artifacts for a profile — including the request — and joins a
`TlMappedResultView`. Under
[ADR-001](./decisions/ADR-001-tl-crates-stay-filament-independent-filament-mltl-bridges.md),
TL-181 repoints exactly those imports to `filament_mltl::*`. So
`filament-contract-ir` is this crate's consumer, and the unspecified half of that
boundary was the *counterparty* obligation: that the documents FR-026
constructs and reads carry the correspondence losslessly and refuse legibly.
That obligation belongs to the crate that owns those documents.

It was previously specified in `tl-mltl`'s M4 corpus campaign as a `tl-mltl`
lane with `producer_repository: agent-ix/filament-contract-ir` — wrong repository
under ADR-001, and wrong direction besides. FR-004 and FR-005 are its
correctly-scoped replacement; see
[ADR-002](./decisions/ADR-002-filament-mltl-owns-the-native-correspondence-dimension.md).

## References

- [Linear epic TL-175](https://linear.app/agent-ix/issue/TL-175) — TL-*
  crates stay independent of the Filament ecosystem; `filament-mltl` is the one
  dedicated bridge.
- `tl-mltl`'s
  [FR-018](https://github.com/agent-ix/tl-mltl/blob/main/spec/requirements/FR-018-publish-temporal-owner-wire.md)
  owns the generic TL-owned assertion boundary this crate adapts; the QObs C00
  consumption path formerly in `tl-mltl` FR-019 is this crate's FR-002.
- `filament-contract-ir`
  [FR-025](https://github.com/agent-ix/filament-contract-ir/blob/main/spec/contract/FR-025-native-predicate-tl-projection.md)
  and
  [FR-026](https://github.com/agent-ix/filament-contract-ir/blob/main/spec/contract/FR-026-native-temporal-tl-correspondence.md)
  own the native predicate projection and the native/TL temporal
  correspondence this crate supplies the TL side of.
- `tl-mltl` ADR-002 drops the same dimension from its M4 corpus campaign, the
  other half of this relocation.
