# Adversarial Test Summary: `tla_forall_always_equality_variant`

## Target
`source-projects/anvil-library/verified/temporal_logic/tla_forall_always_equality_variant.rs`

The function `tla_forall_always_equality_variant` proves that if `a_to_always(a)` is semantically equivalent to `always(a_to_p(a))` for all `a` (bi-directional entailment), then `tla_forall(a_to_always) == always(tla_forall(a_to_p))`.

---

## Results: All 9 tests FAILED verification ✅

All adversarial tests were correctly rejected by the specification.

### Boundary Tests (3/3 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `boundary_test_true_vs_false` | `a_to_always=true`, `a_to_p=false` — no entailment in either direction | precondition not satisfied ✅ |
| `boundary_test_one_direction_only` | `a_to_always=false`, `a_to_p=true` — only forward entailment holds | precondition not satisfied ✅ |
| `boundary_test_missing_always_wrapper` | `a_to_always=a_to_p` (no `always` wrapper) — `p(a)` doesn't entail `always(p(a))` | precondition not satisfied ✅ |

### Behavioral Mutation Tests (3/3 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `mutation_test_negated_conclusion` | Assert `tla_forall(…) != always(…)` after lemma proves equality | assertion failed ✅ |
| `mutation_test_equality_then_neq` | Assert `p != q` after `temp_pred_equality` proves `p == q` | assertion failed ✅ |
| `mutation_test_dropped_always` | Assert mutated relation `tla_forall(…) == tla_forall(p)` (drop `always` on RHS) | assertion failed ✅ |

### Logical Tests (3/3 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `logical_test_equality_without_lemma` | Assert TLA forall-always equality without calling any lemma | assertion failed ✅ |
| `logical_test_always_idempotent` | Assert `always(always(p)) == always(p)` — not provable without dedicated axiom | assertion failed ✅ |
| `logical_test_extensional_equality_without_axiom` | Assert extensional equality of semantically equivalent but syntactically different `TempPred`s without `temp_pred_equality` | assertion failed ✅ |

---

## Findings

1. **The specification correctly rejects invalid inputs.** All boundary violations (missing entailment, one-way entailment, missing `always` wrapper) are caught by the `requires` clause.

2. **The specification correctly rejects incorrect behaviors.** Negated conclusions, dropped operators, and mutated relations are all rejected.

3. **The specification does not allow unintended logical reasoning.** The TLA equality is not provable without the axiom; `always` idempotence is not implicitly assumed; and extensional equality of `TempPred`s requires explicit use of `temp_pred_equality`.

4. **Note on syntactic closure equality:** During development, a test using *syntactically identical* closure bodies (`|ex| state(0) == 0` in both `p` and `q`) was verified as equal by Verus without any axiom. This is expected Verus behavior (syntactically identical `spec_fn` expressions are structurally equal), not a specification weakness. The test was revised to use syntactically different but semantically equivalent closures, which correctly fails.

## Conclusion

The specification of `tla_forall_always_equality_variant` and its supporting axioms are **consistent** with respect to the adversarial queries tested. No unintended entailments were found.
