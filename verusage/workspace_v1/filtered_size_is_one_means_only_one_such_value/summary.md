# Adversarial Test Summary: `filtered_size_is_one_means_only_one_such_value`

## Specification Under Test

The lemma establishes a biconditional:
```
(m.filter(f).len() == 1) ⟺ (∃v. m.contains(v) ∧ f(v)) ∧ (∀v. m.contains(v) ∧ f(v) ⟹ m.count(v) == 1 ∧ (∀u. m.contains(u) ∧ f(u) ⟹ u == v))
```

## Results: All 9 tests FAILED verification ✅

The specification correctly rejects all invalid inputs, incorrect behaviors, and unintended logical reasoning.

---

### Boundary Tests (3/3 rejected)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_empty_multiset_filter_len_one` | Empty multiset, assert filter len == 1 | ❌ Rejected |
| 2 | `test_boundary_duplicate_element_filter_len_one` | Multiset {5,5} (count=2), assert filter len == 1 | ❌ Rejected |
| 3 | `test_boundary_no_match_filter_len_one` | Multiset {3} with f=|v|v==5, assert filter len == 1 | ❌ Rejected |

**Analysis**: The spec correctly handles all edge cases — no elements, duplicates beyond count 1, and non-matching predicates all prevent the filter length from being 1.

---

### Behavioral Mutation Tests (3/3 rejected)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_mutation_wrong_count` | filter len==1, assert matching value has count ≥ 2 | ❌ Rejected |
| 2 | `test_mutation_non_unique_match` | filter len==1, assert ∃ second distinct matching value | ❌ Rejected |
| 3 | `test_mutation_flip_result` | Multiset {5} with f=|v|v==5, assert filter len ≠ 1 | ❌ Rejected |

**Analysis**: The spec correctly enforces both directions of the equivalence — it rejects wrong counts (mutation of count==1), rejects non-unique matches (mutation of uniqueness), and correctly forces filter len==1 when the RHS holds (reverse direction).

---

### Logical Tests (3/3 rejected)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_logical_stronger_multiset_len` | filter(f).len()==1 ⟹ m.len()==1 (stronger property) | ❌ Rejected |
| 2 | `test_logical_cross_predicate` | filter(f).len()==1 ⟹ filter(g).len()≤1 (cross-predicate) | ❌ Rejected |
| 3 | `test_logical_all_elements_satisfy_f` | filter(f).len()==1 ⟹ ∀v. m.contains(v) → f(v) (over-generalization) | ❌ Rejected |

**Analysis**: The spec does not entail any of these stronger/unintended properties — it does not constrain the total multiset size, does not leak information to unrelated predicates, and does not over-generalize filter membership to all elements.

---

## Conclusion

The specification for `filtered_size_is_one_means_only_one_such_value` is **consistent** with respect to all 9 adversarial queries tested. It correctly:
- Rejects invalid boundary inputs (empty, duplicates, non-matching)
- Rejects mutated behavioral outputs (wrong count, non-uniqueness, flipped result)
- Rejects unintended logical entailments (stronger properties, cross-predicate leaks, over-generalizations)

No spec weaknesses were detected.
