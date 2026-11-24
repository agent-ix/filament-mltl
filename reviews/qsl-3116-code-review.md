---
id: SR-620
title: "QSL-3116 filament-mltl pin code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/filament-mltl; QSL-3116; Cargo.toml, Cargo.lock; PR #38 head 3000aa75be0a602731dc2b03882c90ec90acc068"
review_set: subset
relationships:
  - { target: "ix://agent-ix/filament-mltl/MRS-001", type: references }
---

## Summary

Reviewed PR #38 against origin/main. The change only advances the exact `filament-observation` git revision in `Cargo.toml` and its `Cargo.lock` source; the target observation revision pins semantic-value at 689f259e, and the resolved graph contains one semantic-value identity. No defect is introduced by the diff.

## Verdict

**PASS** — The pin and lockfile resolve consistently, the locked build and selected test pass, and the diff does not alter gates or production code.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Inspection

Diff scope: `Cargo.toml` and `Cargo.lock` only. `Cargo.toml` selects observation revision `a36e5ed2e1bde35a27461fa38084d3e140e09fcd`. The lock records that exact observation revision and semantic-value revision `689f259eab592b62cb362f5e649dba16adf874f7`. `cargo tree --locked -i filament-semantic-value` resolves exactly one semantic-value node at 689f259e, through observation a36e5.

`cargo fmt --all -- --check`, `cargo check --locked --all-targets --all-features -j 4`, `cargo clippy --locked --all-targets --all-features -j 4 -- -D warnings`, and the filtered TC-088 test passed. The filtered test ran 1 test with 5 filtered. `git diff --check` passed. `cargo deny check` fails for disallowed git sources `filament-exact` and `filament-semantic-value`; `deny.toml` is unchanged, and both sources were already present on origin/main (semantic-value at 4274b4e and filament-exact at 08a5a17). This is a pre-existing deny policy conflict, not a regression from this pin.

No applicable AssuranceProfile exists. No CI or gate configuration changed. No separate repository gap-analysis was needed for this manifest/lock-only diff; baseline matrix state is documented separately by the gap analysis review.

## Coverage

Rust lane reviewed as part of code-review. Full diff scope examined: `Cargo.toml`, `Cargo.lock`. No AC implementation or test changed. Independent repository gap-analysis: SR-621.

## Scope

```yaml
scope:
- id: Cargo.toml
  path: Cargo.toml
  role: examined
  excerpt: 'filament-observation = { version = "=0.1.1", git = "https://github.com/agent-ix/filament-observation", rev = "a36e5ed2e1bde35a27461fa38084d3e140e09fcd" }'
- id: Cargo.lock
  path: Cargo.lock
  role: examined
  excerpt: 'filament-observation 0.1.1 resolves from a36e5ed2e1bde35a27461fa38084d3e140e09fcd and filament-semantic-value 0.1.5 resolves from 689f259eab592b62cb362f5e649dba16adf874f7.'
- id: MRS-001
  path: spec/spec.md
  role: context_only
  excerpt: 'filament-mltl consumes observation semantics as a dependency and owns its bridge contracts and mapping logic.'
```
