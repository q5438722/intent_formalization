# vstd no-post specification generation and determinism reward

## Goal

Attempt to add postconditions to every vstd exec definition that has no local
postcondition, using the existing determinism checker as a reward signal.

Matching snapshot:

```text
Verus 0.2026.05.17.e479cce
vstd /home/chentianyu/nanvix/toolchain/verus/vstd
```

Syntactic inventory:

```text
44 exec definitions without a local postcondition
```

Effective-contract inventory:

```text
26 already inherit a trait postcondition
18 genuinely have no local or inherited postcondition
```

## Semantic audit before generation

| Group | Count | Result |
|---|---:|---|
| Effective contract inherited from trait | 26 | Not actually missing a spec |
| Compiler/runtime plumbing | 10 | No meaningful postcondition |
| Linear resource, hidden state, prophecy, or mode-only API | 8 | No useful deterministic postcondition |
| **Total** | **44** | **0 useful upstream candidates** |

Among the targets newly discovered by `verus_!` alias normalization, the only
public method without an effective postcondition is:

```text
vstd::cell::invcell::InvCell::set
```

### Trait-inherited definitions

The 26 definitions under `contrib::exec_spec` implement:

- `ToRef::get_ref`;
- `ToOwned::get_owned`;
- `DeepViewClone::deep_clone`;
- `ExecSpecEq::exec_eq`.

Their trait declarations already provide the effective `ensures`. Copying those
clauses into every impl would be redundant and should receive zero reward.

### Compiler/runtime definitions

The invariant-opening encoding functions are removed during VIR conversion.
Pervasive helpers cover:

- false-precondition unreachable code;
- stdout;
- runtime assertion;
- panic divergence;
- unmodeled `Debug` formatting.

None exposes a useful verification-level post-state.

### Resource/hidden-state definitions

The remaining functions use:

- consumed permissions (`deallocate`, `free`, lock release);
- hidden interior state (`InvCell::set`);
- intentionally unconstrained prophecy (`Prophecy::new`);
- a tracked type that cannot call exec-mode `Clone` (`IsThread::clone`).

The effect is represented by ownership/mode or is intentionally hidden, not by
a missing output relation.

## Diagnostic candidates

Five candidates were deliberately checked as negative controls:

| Target | Candidate |
|---|---|
| `cell::invcell::InvCell::set` | `ensures self.inv(val)` |
| `pervasive::unreached` | `ensures false` |
| `pervasive::runtime_assert` | `ensures b` |
| `thread::IsThread::clone` cfg body 1 | `ensures result@ == self@` |
| `thread::IsThread::clone` cfg body 2 | `ensures result@ == self@` |

Results:

| Metric | Value |
|---|---:|
| Diagnostic candidates | 5 |
| Raw determinism reward | 3 |
| Guarded reward | 0 |

Raw positive rewards:

1. `InvCell::set`: return type is unit and the candidate merely repeats the
   existing precondition.
2. `unreached`: `requires false` makes any postcondition vacuous.
3. `runtime_assert`: return type is unit and the candidate repeats `requires b`.

The two `IsThread::clone` candidates remain solver-unknown, but the more
important issue is mode correctness: `IsThread` is tracked/proof-only while
`Clone::clone` is exec-mode, so verified code cannot meaningfully call the
method.

## Main finding

Determinism alone is an unsafe reward for missing-spec generation.

A candidate can receive `R0 = unsat` because:

- the function returns unit;
- the precondition is false;
- the candidate repeats the precondition;
- the equal-fn ignores all observable output;
- the API's real effect is represented by linear resource consumption.

None of these means the generated postcondition improves the specification.

## Recommended reward

Use determinism only after anti-vacuity gates:

```text
guarded_reward =
    typechecks
  * determinism_is_unsat
  * has_observable_output
  * postcondition_adds_information
  * precondition_is_satisfiable
  * not_already_inherited_from_trait
  * API_is_semantically_callable
```

Additional checks should reject:

- `ensures true`;
- postconditions implied directly by `requires`;
- false-precondition tasks;
- unit-only outputs with no modeled mutable state;
- candidate specs for compiler-erased functions;
- duplicate trait contracts.

## Outcome

No new postcondition from this 44-function batch should be proposed upstream.

The useful research result is negative:

> On vstd's remaining no-post definitions, raw determinism reward is dominated
> by vacuity, inherited contracts, hidden state, and linear-resource semantics.

Future spec-generation experiments should instead target functions with:

- a non-unit observable output or modeled mutable state;
- no inherited trait contract;
- a satisfiable domain;
- public semantic projections sufficient to express the intended result.

Structured artifacts:

- `missing-spec-reward-2026-07-20/manifest.json`
- `missing-spec-reward-2026-07-20/summary.json`
- `missing-spec-reward-2026-07-20/SUMMARY.md`
