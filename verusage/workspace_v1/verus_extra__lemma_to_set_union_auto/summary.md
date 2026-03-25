# Test Summary: `verus_extra__lemma_to_set_union_auto`

## Target Specification

Two lemmas establishing that converting a concatenation of sequences to a set distributes over set union:
- `lemma_to_set_distributes_over_addition(s, t)` — ensures `(s+t).to_set() == s.to_set() + t.to_set()` (external_body, trusted)
- `lemma_to_set_union_auto()` — universally quantified version with auto-trigger

No preconditions (`requires`) on either function.

---

## Results Overview

| Category | Tests | All Failed | Spec Weakness Found |
|---|---|---|---|
| Boundary | 5 | ✅ Yes (5/5) | None |
| Behavioral Mutation | 5 | ✅ Yes (5/5) | None |
| Logical | 5 | ✅ Yes (5/5) | None |

**Total: 15/15 tests correctly rejected by the verifier.**

---

## Boundary Tests (5/5 FAILED ✅)

| # | Test | Property Queried | Result |
|---|---|---|---|
| 1 | `test_boundary_1_empty_concat_is_nonempty` | `(empty + empty).to_set() !== Set::empty()` | FAILED ✅ |
| 2 | `test_boundary_2_concat_empty_right_changes_set` | `(s + empty).to_set() !== s.to_set()` | FAILED ✅ |
| 3 | `test_boundary_3_concat_empty_left_changes_set` | `(empty + t).to_set() !== t.to_set()` | FAILED ✅ |
| 4 | `test_boundary_4_self_concat_gives_empty_set` | `(s + s).to_set() =~= Set::empty()` for non-empty s | FAILED ✅ |
| 5 | `test_boundary_5_phantom_element_in_union` | `(s + t).to_set().contains(99)` where 99 ∉ s,t | FAILED ✅ |

**Interpretation**: The spec correctly handles all edge cases involving empty sequences, self-concatenation, and non-member elements. No invalid inputs are admitted.

---

## Behavioral Mutation Tests (5/5 FAILED ✅)

| # | Test | Mutation Applied | Result |
|---|---|---|---|
| 1 | `test_mutation_1_drop_second_operand` | `(s+t).to_set() =~= s.to_set()` (drop t) | FAILED ✅ |
| 2 | `test_mutation_2_drop_first_operand` | `(s+t).to_set() =~= t.to_set()` (drop s) | FAILED ✅ |
| 3 | `test_mutation_3_union_to_intersection` | Replace union with intersection | FAILED ✅ |
| 4 | `test_mutation_4_negate_ensures` | `(s+t).to_set() !== s.to_set() + t.to_set()` | FAILED ✅ |
| 5 | `test_mutation_5_result_is_empty` | `(s+t).to_set() =~= Set::empty()` for non-empty inputs | FAILED ✅ |

**Interpretation**: The spec correctly rejects all mutated behaviors — dropping operands, swapping union for intersection, negating the postcondition, and claiming empty results.

---

## Logical Tests (5/5 FAILED ✅)

| # | Test | Unintended Property Queried | Result |
|---|---|---|---|
| 1 | `test_logical_1_cardinality_additive` | `|A ∪ B| == |A| + |B|` (false for overlapping sets) | FAILED ✅ |
| 2 | `test_logical_2_difference_recovers_operand` | `(A ∪ B) \ A == B` (false when A ∩ B ≠ ∅) | FAILED ✅ |
| 3 | `test_logical_3_seq_len_eq_set_len` | `|(s+t).to_set()| == |s+t|` (false with duplicates) | FAILED ✅ |
| 4 | `test_logical_4_disjointness_guaranteed` | `s.to_set() ∩ t.to_set() == ∅` (not guaranteed) | FAILED ✅ |
| 5 | `test_logical_5_concat_strictly_enlarges` | `|(s+t).to_set()| > |s.to_set()|` (false when t ⊆ s) | FAILED ✅ |

**Interpretation**: The spec does not over-admit logical consequences. It correctly refuses to entail cardinality additivity, set-difference reversibility, length preservation, implicit disjointness, or strict enlargement — all properties that would be mathematically incorrect in general.

---

## Conclusion

The specification for `lemma_to_set_union_auto` is **consistent** with respect to all 15 adversarial queries tested. It:
- Rejects all invalid boundary reasoning (5/5)
- Rejects all mutated behavioral claims (5/5)
- Rejects all unintended logical inferences (5/5)

No specification weaknesses were detected. The postcondition `(s+t).to_set() == s.to_set() + t.to_set()` is precise: it captures exactly the set-union distributivity property without over- or under-specifying.
