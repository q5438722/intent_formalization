# Test Summary: `lemma_seq_fold_left_sum_len_int_positive`

## Specification Under Test

```rust
pub proof fn lemma_seq_fold_left_sum_len_int_positive<A, B>(s: Seq<A>, low: nat, f: spec_fn(A) -> Seq<B>)
  ensures
    s.fold_left(low as int, |acc: int, x: A| acc + f(x).len()) >= 0,
```

- **No preconditions** (`requires` absent) — any inputs are valid.
- **Postcondition**: The fold-left accumulation (starting from `low`, adding `f(x).len()` per element) is non-negative.

## Key Observation

The ensures clause says `>= 0`, but mathematically the result is always `>= low` (since `low: nat >= 0` and each `f(x).len() >= 0`). The specification is **weaker than the true invariant** — it loses precision by only guaranteeing non-negativity rather than the tighter bound `>= low`.

---

## Results

### Boundary Tests — 3/3 FAILED ✅

| Test | Property Asserted | Result | Reason |
|------|------------------|--------|--------|
| `test_boundary_empty_seq_strict_positive` | `fold([], 0, f) > 0` | FAIL ✅ | fold of empty seq at 0 is exactly 0 |
| `test_boundary_nonempty_all_zero_lengths` | `fold([1,2,3], 0, f_empty) > 0` | FAIL ✅ | f maps everything to empty seq, so fold = 0 |
| `test_boundary_empty_seq_below_low` | `fold([], 1, f) < 1` | FAIL ✅ | fold of empty seq at 1 is exactly 1 |

### Behavioral Mutation Tests — 3/3 FAILED ✅

| Test | Mutation | Result | Reason |
|------|----------|--------|--------|
| `test_mutation_strict_positive` | `>= 0` → `> 0` | FAIL ✅ | Counterexample: empty seq + low=0 gives 0 |
| `test_mutation_above_low_plus_one` | `>= 0` → `>= low + 1` | FAIL ✅ | Counterexample: empty seq gives exactly low |
| `test_mutation_exact_zero` | `>= 0` → `== 0` | FAIL ✅ | Counterexample: low > 0 gives result > 0 |

### Logical Tests — 3/3 FAILED ✅

| Test | Property | Result | Reason |
|------|----------|--------|--------|
| `test_logical_result_at_least_low` | `fold >= low` | FAIL ✅ | True in practice, but spec only says `>= 0` — spec weakness |
| `test_logical_monotone_in_low` | `low1 < low2 ⇒ fold(low1) < fold(low2)` | FAIL ✅ | True in practice, but not derivable from `>= 0` |
| `test_logical_upper_bound` | `fold <= low + len * 100` | FAIL ✅ | No upper bound in spec; `>= 0` says nothing about upper limits |

---

## Conclusion

**All 9 adversarial tests failed verification as expected**, confirming:

1. **Boundary adequacy**: The spec correctly rejects false claims at edge cases (empty sequences, zero accumulator).
2. **Behavioral precision**: Mutations to the postcondition (strict inequality, exact equality, shifted bound) are properly rejected.
3. **Logical containment**: The spec does not entail stronger properties like `>= low`, monotonicity in `low`, or upper bounds.

**Spec weakness identified**: The ensures clause `>= 0` is weaker than the true property `>= low`. The logical test `test_logical_result_at_least_low` confirms this — Verus cannot derive `fold >= low` from the spec alone, even though it is mathematically true. This means the specification loses information about the relationship between the initial accumulator and the result.
