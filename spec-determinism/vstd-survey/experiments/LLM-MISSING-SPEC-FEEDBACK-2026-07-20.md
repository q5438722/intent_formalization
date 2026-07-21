# LLM vstd missing-spec generation with determinism feedback

## Setup

| Setting | Value |
|---|---|
| Model | `gpt-5-mini` |
| Targets | 44 no-post exec definitions |
| Parallel jobs | 4 |
| Feedback rounds | 1 |
| Candidate checker | existing vstd spec-determinism runner |
| Reward | raw determinism + anti-vacuity guarded reward |

Each target received:

1. its source/context;
2. a request for the strongest sound useful `ensures`, or `skip`;
3. Verus/determinism feedback when a candidate was generated;
4. anti-vacuity and semantic-disposition feedback;
5. one repair/skip opportunity.

## Results

| Metric | Initial | Final |
|---|---:|---:|
| `add_spec` decisions | 29 | 4 |
| `skip` decisions | 15 | 40 |
| Raw determinism reward | 0 | 0 |
| Guarded reward | 0 | 0 |
| LLM errors | 0 | 0 |

Decision transitions:

| Transition | Count |
|---|---:|
| `add_spec -> skip` | 26 |
| `skip -> skip` | 14 |
| `add_spec -> add_spec` | 3 |
| `skip -> add_spec` | 1 |

The 26 `add_spec -> skip` cases are exactly the `contrib::exec_spec`
implementations whose contracts were already inherited from traits. Initial
generation duplicated clauses such as:

```rust
res.deep_view() == self.deep_view()
```

After receiving `already_specified_via_trait` and checker feedback, the model
correctly removed all 26 redundant candidates.

## Final four add-spec decisions

### `cell::invcell::InvCell::set`

Final candidate:

```rust
ensures self.wf()
```

Result: `verus_error`.

`wf` is private/internal and the function has no modeled observable output.
Even a type-correct invariant-preservation clause would not establish the
hidden current value and would receive zero guarded reward.

### `invariant::spend_open_invariant_credit`

Final candidate was natural-language text:

```text
The supplied Tracked<OpenInvariantCredit> is consumed...
```

Result: `verus_error`.

The model correctly identified the semantic effect but failed to express a
Verus formula. More importantly, token consumption is already represented by
linear ownership and cannot be restated as a useful postcondition.

### `thread::IsThread::clone` — two cfg bodies

Final candidates:

```rust
result.view() == self.view()
```

and:

```rust
result@ == self@
```

Result: `R0 = unknown`.

The semantic relation is reasonable, but the API is mode-incompatible:
`IsThread` is tracked/proof-only while `Clone::clone` is exec-mode, so verified
code cannot meaningfully invoke it. Both receive zero guarded reward.

## What feedback accomplished

The feedback loop substantially improved model behavior:

- removed all 26 duplicate trait contracts;
- retained skips for compiler/runtime/linear-resource cases;
- recognized that most no-post functions have no useful postcondition.

However, feedback did not make the remaining four candidates useful:

- one referenced a private implementation invariant;
- one was not valid Verus syntax;
- two targeted an uncallable mode combination.

## Main conclusion

The LLM experiment confirms the prior semantic audit:

> None of the 44 no-post definitions currently admits a useful new
> postcondition suitable for upstream vstd.

Determinism feedback is useful for rejecting or repairing candidate specs, but
it is not sufficient by itself. The successful behavior came from combining:

- determinism result;
- type/checker status;
- trait-inheritance detection;
- observable-output detection;
- semantic-disposition labels;
- anti-vacuity feedback.

## Reward lesson

For missing-spec generation, use:

```text
guarded_reward =
    typechecks
  * determinism_is_unsat
  * has_observable_output
  * adds_information_beyond_requires
  * satisfiable_domain
  * not_inherited_from_trait
  * semantically_callable
```

A pure `R0 = unsat` reward would otherwise favor unit-returning, vacuous,
redundant, or abstraction-breaking specs.

## Artifacts

Directory:

```text
vstd-survey/experiments/llm-missing-spec-gpt5mini-2026-07-20/
```

Important files:

- `batch_summary.json`
- `SUMMARY.md`
- `targets/<target>/source_context.rs.txt`
- `targets/<target>/round_00/{prompt,response,candidate,round_result}`
- `targets/<target>/round_01/{prompt,response,candidate,round_result}`
- checker artifacts for generated candidates

Runner:

```text
vstd-survey/run_llm_missing_spec_feedback.py
```
