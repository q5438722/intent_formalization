# vstd missing-spec generation experiment results

## 1. Experiment question

For every vstd exec definition without a local postcondition:

1. ask a model to generate a useful `ensures`, or explicitly skip;
2. run the generated candidate through the existing determinism checker;
3. return Verus/type, determinism, and anti-vacuity feedback;
4. allow one repair round;
5. score both raw determinism reward and guarded reward.

Syntactic scope:

```text
44 source-level exec definitions without a local postcondition
```

Effective scope after trait inheritance:

```text
18 definitions without a local or inherited postcondition
```

The 44 targets contain:

| Semantic group | Count |
|---|---:|
| Contract already inherited from trait | 26 |
| Compiler/runtime plumbing | 10 |
| Linear resources, hidden state, prophecy, or mode-only API | 8 |

The only newly visible public no-post method introduced by `verus_!` alias
normalization is `vstd::cell::invcell::InvCell::set`.

## 2. Reward definition

### Raw determinism reward

```text
1 if the candidate typechecks and R0 = unsat
0 otherwise
```

### Guarded reward

Raw reward is accepted only when the candidate:

- has a modeled observable output;
- adds information beyond `requires`;
- does not rely on a false domain;
- is not a duplicate trait contract;
- is semantically callable;
- is not compiler-erased or an unmodeled runtime effect.

## 3. Aggregate results

### Hand-written diagnostic controls

| Candidate | Raw reward | Guarded reward |
|---|---:|---:|
| `InvCell::set ensures self.inv(val)` | 1 | 0 |
| `unreached ensures false` | 1 | 0 |
| `runtime_assert ensures b` | 1 | 0 |
| `IsThread::clone ensures result@ == self@` (two cfg bodies) | 0 | 0 |
| **Total** | **3** | **0** |

These controls demonstrate that raw determinism rewards:

- a postcondition copied from `requires`;
- a vacuous false-precondition function;
- a unit-returning function with no modeled state.

### LLM runs

| Metric | `gpt-5-mini` | `gpt-5.6-sol` |
|---|---:|---:|
| Initial `add_spec` | 29 | 5 |
| Initial `skip` | 15 | 39 |
| Final `add_spec` | 4 | 5 |
| Final `skip` | 40 | 39 |
| Raw reward | 0 | 0 |
| Guarded reward | 0 | 0 |
| LLM errors | 0 | 0 |

No generated candidate from either model passed guarded reward.

## 4. Representative examples

### Example A — existing `InvCell::replace` spec is genuinely underconstrained

This example comes from the existing-spec completeness experiment rather than
the no-post generation batch.

Target:

```text
deprecated vstd::cell::InvCell::replace
vstd/cell.rs:359
```

Existing contract:

```rust
pub fn replace(&self, val: T) -> (old_val: T)
    requires
        self.inv(val),
    ensures
        self.inv(old_val),
```

`inv(v)` only means that `v` belongs to the cell's allowed-value predicate. It
does not identify the exact value currently stored in the cell.

For example, construct a cell whose invariant allows both `0` and `1`:

```rust
InvCell::new(0, Ghost(|v| v == 0 || v == 1))
```

The implementation of `replace(1)` deterministically returns the actual old
value `0`. The specification, however, permits:

```text
old_val = 0
old_val = 1
```

because both satisfy `self.inv(old_val)`.

The generated determinism obligation is:

```rust
self.inv(r1) && self.inv(r2) ==> r1 == r2
```

which is false for a non-functional invariant predicate.

Audit result:

```text
genuine semantic underconstraint
```

This case cannot be repaired by changing equal-fn. A stronger postcondition
would require new public ghost vocabulary for the exact current value, for
example:

```rust
ensures
    old_val == old(self).current_value(),
```

but `InvCell` intentionally does not expose such an accessor. The alternative
is to document the possible-value abstraction or change the API so it does not
return an exact old value.

### Example B — hidden-state invariant preservation

Target:

```text
cell::invcell::InvCell::set
```

`gpt-5-mini` round 0:

```rust
ensures
    self.predicate() == old(self.predicate()),
```

Checker result:

```text
verus_error
```

`old(...)` is only meaningful for mutable-reference state; `predicate()` here
returns a value and the call uses shared interior mutability.

Round 1:

```rust
ensures
    self.wf(),
```

Checker result:

```text
verus_error: wf is private/not visible from the standalone harness
```

Even a type-correct invariant-preservation clause would not identify the hidden
stored value. The function returns unit, so guarded reward remains zero.

`gpt-5.6-sol` correctly chose `skip` in both rounds for this target.

### Example C — vacuous determinism reward

Target:

```text
pervasive::unreached
```

Diagnostic candidate:

```rust
ensures
    false,
```

Original precondition:

```rust
requires
    false,
```

Checker result:

```text
R0 = unsat
raw reward = 1
guarded reward = 0
```

The checker proves determinism only because the function has no valid input.
This is reward hacking, not a specification improvement.

### Example D — requires mirroring on a unit function

Target:

```text
pervasive::runtime_assert
```

Candidate:

```rust
ensures
    b,
```

Original contract:

```rust
requires
    b,
```

Checker result:

```text
R0 = unsat
raw reward = 1
guarded reward = 0
```

The candidate adds no information and the function returns unit.

### Example E — linear resource effect expressed as natural language

Target:

```text
invariant::spend_open_invariant_credit
```

`gpt-5-mini` initially skipped, then feedback caused it to generate:

```text
The supplied Tracked<OpenInvariantCredit> is consumed:
on return the caller no longer owns or may use that tracked credit.
```

The semantic explanation is correct, but it is not a Verus expression.

Checker result:

```text
verus_error
```

The actual effect is already represented by consuming the tracked parameter.
There is no useful postcondition to add.

### Example F — tracked/exec mode incompatibility

Target:

```text
thread::IsThread::clone
```

Generated candidates:

```rust
ensures result@ == self@
```

and:

```rust
ensures result.view() == self.view()
```

Checker result:

```text
R0 = unknown
guarded reward = 0
```

`IsThread` is a tracked proof-only value while `Clone::clone` is an exec-mode
trait method. Verified code cannot meaningfully invoke this method. The
contract is semantically plausible but operationally unusable.

### Example G — stronger model introduces a redundant weak clause

Target:

```text
contrib::exec_spec::map::deep_clone
```

`gpt-5.6-sol` initially skipped, but after feedback generated:

```rust
ensures
    res.len() <= self.len(),
```

Problems:

- the method already inherits the stronger `DeepViewClone` contract;
- `res` was not the extracted return binding;
- the private module could not be imported by the standalone harness;
- the candidate is weaker than the existing semantic view equality.

Checker result:

```text
verus_error
guarded reward = 0
```

### Example H — reference identity instead of semantic deep view

Targets:

```text
contrib::exec_spec::option::get_ref
contrib::exec_spec::set::get_ref
```

`gpt-5.6-sol` generated:

```rust
ensures result == self
```

The inherited trait contract is instead:

```rust
ensures result.deep_view() == self.deep_view()
```

The generated clause chooses raw reference/container identity rather than the
intended mathematical abstraction and duplicates an already-specified method.

## 5. Feedback behavior

### `gpt-5-mini`

```text
initial: 29 add_spec, 15 skip
final:    4 add_spec, 40 skip
```

Feedback successfully removed all 26 duplicate trait specs.

### `gpt-5.6-sol`

```text
initial:  5 add_spec, 39 skip
final:    5 add_spec, 39 skip
```

It was much more conservative initially. However, feedback caused:

```text
5 add_spec -> skip
5 skip -> add_spec
```

The newly added candidates were redundant trait specs and all failed the
checker.

## 6. Final conclusion

No generated candidate should be applied to vstd.

The experiment supports three conclusions:

1. Most syntactic no-post definitions are not genuinely missing specs.
2. Determinism alone is an unsafe generation reward.
3. Checker feedback is useful only when combined with semantic eligibility and
   anti-vacuity gates.

Recommended policy:

```text
Do not enter a feedback round for:
- already_specified_via_trait
- compiler_internal
- runtime_effect_unmodeled
- linear_resource_effect
- intentional_nondeterminism
- semantically_uncallable
```

Feedback should be reserved for semantically eligible candidates that fail
typechecking or determinism.

## 7. Detailed artifacts

- Rule-based controls:
  `missing-spec-reward-2026-07-20/`
- `gpt-5-mini`:
  `llm-missing-spec-gpt5mini-2026-07-20/`
- `gpt-5.6-sol`:
  `llm-missing-spec-gpt56sol-2026-07-20/`
- Mini report:
  `LLM-MISSING-SPEC-FEEDBACK-2026-07-20.md`
- Model comparison:
  `LLM-MISSING-SPEC-MODEL-COMPARISON-2026-07-20.md`
