# Adversarial Test Results: `lemma_seq_fold_left_sum_le`

## Specification Under Test

```
requires: forall |i:int| 0 <= i < s.len() ==> f(s[i]) <= high
ensures:  s.fold_left(init, |acc, x| acc + f(x)) <= init + s.len() * high
```

## Results Summary

| Test File | Tests | All Failed? | Verdict |
|-----------|-------|-------------|---------|
| boundary_tests.rs | 3 | ✅ Yes (3/3) | Spec correctly rejects invalid inputs |
| behavioral_mutation_tests.rs | 3 | ✅ Yes (3/3) | Spec correctly rejects mutated behaviors |
| logical_tests.rs | 3 | ✅ Yes (3/3) | Spec does not entail unwarranted properties |

**Overall: 9/9 tests failed verification as intended.**

---

## Boundary Tests (precondition violations)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_f_exceeds_high` | f(s[i]) ∈ {10,20,30} > high=5 | ❌ precondition not satisfied |
| 2 | `test_boundary_high_zero_positive_f` | f(s[i]) ∈ {1,2,3} > high=0 | ❌ precondition not satisfied |
| 3 | `test_boundary_negative_high_exceeded` | f(s[i]) ∈ {1,2} > high=-10 | ❌ precondition not satisfied |

**Conclusion:** The `requires` clause properly guards against all tested invalid input patterns.

## Behavioral Mutation Tests (postcondition mutations)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_strict_inequality` | `<=` → `<` (strict) | ❌ postcondition not satisfied |
| 2 | `test_mutation_tighter_bound` | `s.len()` → `s.len()-1` (tighter) | ❌ postcondition not satisfied |
| 3 | `test_mutation_reversed_inequality` | `<=` → `>=` (reversed) | ❌ postcondition not satisfied |

**Conclusion:** The upper bound `init + s.len() * high` is tight (equality is achievable), and the direction of the inequality is correct. No mutated behavior is admitted.

## Logical Tests (unentailed properties)

| # | Test | Unwarranted Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_lower_bound` | `fold >= init` | ❌ postcondition not satisfied |
| 2 | `test_logical_non_negative` | `fold >= 0` | ❌ postcondition not satisfied |
| 3 | `test_logical_exact_equality` | `fold == init + s.len() * high` | ❌ postcondition not satisfied |

**Conclusion:** The spec does not inadvertently entail a lower bound, non-negativity, or exact equality. Since `f` can return arbitrarily negative values (only bounded above by `high`), the fold result has no guaranteed lower bound — this is correctly reflected.

---

## Overall Assessment

The specification for `lemma_seq_fold_left_sum_le` is **well-calibrated**:

1. **Input validation**: Preconditions correctly reject all invalid inputs tested.
2. **Output precision**: The upper bound is tight — no strictly tighter bound passes.
3. **Logical restraint**: The spec does not over-promise — no unwarranted lower bounds, non-negativity, or equality properties are entailed.

No specification weaknesses were detected across all three categories of adversarial testing.
