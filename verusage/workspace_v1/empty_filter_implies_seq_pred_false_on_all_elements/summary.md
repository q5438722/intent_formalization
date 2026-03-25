# Adversarial Proof Test Summary

## Target
`empty_filter_implies_seq_pred_false_on_all_elements.rs`

**Specification:**
- **Requires:** `s.filter(pred).len() == 0`
- **Ensures:** `forall |e: A| s.contains(e) ==> !pred(e)`

Meaning: if filtering sequence `s` by predicate `pred` yields an empty result, then `pred` is false on every element of `s`.

---

## Results: All 9 tests FAILED verification ✅

The specification correctly rejects all adversarial queries.

### (1) Boundary Tests — 3/3 failed ✓

| Test | Failure Mode | Verus Error |
|------|-------------|-------------|
| `boundary_test_nonempty_filter` | Call with `s.filter(pred).len() > 0` | precondition not satisfied |
| `boundary_test_filter_len_one` | Call with `s.filter(pred).len() == 1` | precondition not satisfied |
| `boundary_test_all_elements_match` | Call with `s.filter(pred).len() == s.len()`, `s.len() > 0` | precondition not satisfied |

**Conclusion:** The precondition `s.filter(pred).len() == 0` properly rejects all non-zero filter lengths, including the off-by-one boundary (== 1) and the maximal case (filter == sequence).

### (2) Behavioral Mutation Tests — 3/3 failed ✓

| Test | Mutation | Verus Error |
|------|----------|-------------|
| `mutation_test_flip_negation` | `!pred(e)` → `pred(e)` in universal | assertion failed |
| `mutation_test_exists_pred_true` | `forall !pred` → `exists pred` | assertion failed |
| `mutation_test_first_element_pred_true` | Assert `pred(s[0])` on first element | assertion failed |

**Conclusion:** The postcondition is tight enough to reject flipped, existential, and point-wise mutations. Incorrect input-output relationships are not entailed.

### (3) Logical Tests — 3/3 failed ✓

| Test | Unintended Property | Verus Error |
|------|---------------------|-------------|
| `logical_test_empty_filter_implies_empty_seq` | `s.len() == 0` (empty filter ⇒ empty seq) | assertion failed |
| `logical_test_cross_sequence_transfer` | `!pred` on different sequence `s2` | assertion failed |
| `logical_test_global_pred_false` | `forall |e: int| !pred(e)` (global falsity) | assertion failed |

**Conclusion:** The specification does not entail unintended reasoning:
- It does not confuse an empty filter result with an empty sequence.
- Its conclusion is scoped to the argument sequence only, not transferable.
- `!pred` is bounded to elements of `s`, not universally quantified over all values.

---

## Overall Assessment

**The specification is consistent.** All 9 adversarial queries across boundary, behavioral, and logical dimensions were correctly rejected by the Verus verifier. The precondition is precise (rejects invalid inputs), the postcondition is tight (rejects mutated behaviors), and no unintended logical inferences are entailable from the spec.
