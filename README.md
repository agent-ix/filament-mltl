# filament-mltl

[![Discord](https://img.shields.io/badge/Discord-Join%20us-5865F2?logo=discord&logoColor=white)](https://discord.gg/k8DVhuYBR2)
[![Filament](https://img.shields.io/badge/Filament-Specification%20Language-1F6FEB)](https://github.com/agent-ix/filament-spec-language)
[![License](https://img.shields.io/badge/license-AGPL--3.0--or--later-blue)](#license)
[![Rust](https://img.shields.io/badge/rust-1.98%2B-orange?logo=rust)](Cargo.toml)

filament-mltl evaluates a Mission-time Linear Temporal Logic (MLTL) formula on a
recorded trace or history and gives the Filament assurance record either a Boolean
value or a named reason there is none, such as pending, unavailable or incomplete.

It takes a formula, a proposition map, a trace or history, a clock and the
progress, closure, completeness and availability facts that
[filament-observation](https://github.com/agent-ix/filament-observation) reports, and
writes them as one canonical request. Each step writes a document with its own
identity. Reading a request or mapping back re-derives it from the inputs; reading a
result back binds it to its request without re-running the evaluator, so a result
cannot be paired with the wrong request.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/images/hero-dark.svg">
  <img alt="filament-mltl takes an MLTL formula, a proposition map, a trace or history, a clock and the observation facts about that evidence, and derives one canonical temporal request. The request is evaluated by tl-mltl into one result, and the result is mapped to either a Boolean value or a typed non-value reason." src="docs/images/hero-light.svg">
</picture>

## Getting started

filament-mltl is not on crates.io. Depend on it through git:

```toml
[dependencies]
filament-mltl = { git = "https://github.com/agent-ix/filament-mltl" }
```

Add `rev = "<commit>"` to pin a commit. It needs the Rust version in `rust-version` in `Cargo.toml`, or newer.

The build also fetches filament-observation from GitHub, so it needs read access to that repository.

To build from a clone:

```sh
git clone https://github.com/agent-ix/filament-mltl.git
cd filament-mltl
cargo build --release
```

## Usage

One request gives one result at one evaluation point, the `anchor`. The excerpt
below starts from `future_request`, a request that `request::derive` wrote and `request::read`
checked. It evaluates the request, reads the result back and maps it.

```rust
let result_document = report::evaluate(
    &future_request,
    report::ResultRelationInput::Original,
    OwnerLimits::default(),
)
.unwrap();
let result = report::read(
    result_document.bytes(),
    &future_request,
    report::ResultRelationInput::Original,
    OwnerLimits::default(),
)
.unwrap();
assert_eq!(result.truth(), report::TemporalTruth::Satisfied);

let selection = MappingSelection::for_result(&result);
let mapped_document = contract_ir::map(&result, &selection, OwnerLimits::default()).unwrap();
let mapped = contract_ir::read(
    mapped_document.bytes(),
    &result,
    &selection,
    OwnerLimits::default(),
)
.unwrap();
assert_eq!(mapped.outcome(), MappedOutcome::Value { value: true });
```

Every document carries an identity computed from its own canonical bytes. Every
`read` checks the document against its inputs and refuses on any mismatch (`report::read`
binds the recorded result to its request and never re-runs the evaluator), so a
result cannot be read against a different request and a mapping cannot be read
against a result it was not selected for. More examples are in
[docs/examples.md](docs/examples.md).

## Reference

| Module | Contract label | What it does |
| --- | --- | --- |
| `request` | `filament-mltl.temporal-assessment-request/v1` | Admits the inputs and writes one canonical request. `derive` writes it and `read` checks bytes against independently supplied inputs. |
| `report` | `filament-mltl.temporal-assessment-result/v1` | Evaluates a request with the tl-mltl future or past evaluator and records execution, truth, settlement, decision support, resource usage and the relation to an earlier result. |
| `contract_ir` | `filament-mltl.contract-ir-result-map/v1` | Maps a result to `Value { value }` or `NonValue { reason }` and embeds the whole source result. |
| `dispatch` | the request label | Reports which Filament observation contracts the crate consumes. |
| `census` | none | A closed registry of the native/temporal correspondence classes, each applicable, excluded with a reason, or blocked on a named dependency. |

| Result field | Values |
| --- | --- |
| `execution` | `completed`, `resource-incomplete`, `unsupported`, `failed`, `refused` |
| `truth` | `satisfied`, `violated`, `pending`, `unavailable` |
| `settlement` | `closed-scope`, `decisive-witness`, `decisive-counterexample`, `unsettled`, `unavailable` |
| `relation` | `original`, `superseding`, `invalidating` |

The non-value reasons are `pending`, `unavailable`, `incomplete`, `unsupported`,
`failed`, `refused`, `contradicted` and `resource-incomplete`. `contract_ir::map`
checks the result in a fixed order and the first row that applies decides the
outcome. Only a completed, complete, available and settled result becomes a Boolean.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/images/outcomes-dark.svg">
  <img alt="A result is checked in order. Execution that did not complete, then completeness that is not complete, then an unavailable result, then unsettled truth each map to a NonValue with its own reason. Only a settled satisfied or violated result maps to a Boolean Value." src="docs/images/outcomes-light.svg">
</picture>

The JSON Schemas for the three contracts are in [`schemas/`](schemas/). See
[docs/api-reference.md](docs/api-reference.md) for the public items.

## The TL stack

filament-mltl is the one crate that connects the family to Filament.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/images/tl-stack-dark.svg">
  <img alt="The TL crates. tl-syntax defines the formula graph, profiles and signal catalog, and tl-parse, tl-oracle, tl-mltl, tl-rewrite and filament-mltl all depend on it. tl-rewrite depends on tl-mltl. filament-mltl depends on tl-mltl and filament-observation, and is the single bridge to Filament. Dashed lines are development-only: tl-mltl and tl-rewrite test against tl-oracle, and tl-rewrite tests against tl-parse." src="docs/images/tl-stack-light.svg">
</picture>

## Filament ecosystem

filament-mltl depends on [tl-mltl](https://github.com/agent-ix/tl-mltl) and
[tl-syntax](https://github.com/agent-ix/tl-syntax) for the formula, trace and
evaluator, and on [filament-observation](https://github.com/agent-ix/filament-observation)
for the progress, closure, completeness, availability and clock assertions. Its
mapped outcomes are shaped to join the native/temporal correspondence contract in
[filament-contract-ir](https://github.com/agent-ix/filament-contract-ir).

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/images/ecosystem-dark.svg">
  <img alt="The Filament repositories. The main chain runs from the standard, filament-specification, to the compiler, filament-spec-language, then filament-contract-ir, filament-contract-codegen and filament-contract-runtime. filament-spec-language also feeds filament-protocol, and filament-contract-ir feeds filament-analyze. filament-observation feeds filament-protocol and filament-mltl. Above the chain, filament-driver runs it, filament-integration tests it and filament-verification plans verification. Below it, shared libraries used throughout: filament-eval, filament-exact, filament-semantic-value, filament-walk, quire-canonical and filament-verification-contracts." src="docs/images/ecosystem-light.svg">
</picture>

## Building and testing

```sh
make test   # cargo test
make ci     # format, clippy, tests, cargo deny, rustdoc
```

The Rust toolchain is pinned in [`rust-toolchain.toml`](rust-toolchain.toml).
Run `make ci` before opening a pull request. The build fetches filament-observation, tl-mltl and tl-syntax from
GitHub, so it needs read access to those repositories. The requirements and
architecture decisions are in [`spec/`](spec/).

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) and [CLA.md](CLA.md).

## License

Licensed under [AGPL-3.0-or-later](LICENSE). filament-mltl incorporates
filament-observation, which is also AGPL-3.0-or-later.
