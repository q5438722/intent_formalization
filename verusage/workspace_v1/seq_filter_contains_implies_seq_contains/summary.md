# Adversarial Test Summary: `seq_filter_contains_implies_seq_contains`

## Target Specification
- **Axiom** (`seq_filter_is_a_subset_of_original_seq`): Elements in `s.filter(pred)` are also in `s`.
- **Lemma** (`seq_filter_contains_implies_seq_contains`): If `s.filter(pred).contains(elt)`, then `s.contains(elt)`.

## Results: 15/15 tests FAILED (as expected)

All adversarial tests were correctly rejected by the verifier, indicating the specification is consistent within the tested semantic boundary.

| # | Category | Test Name | Failure Mode | Result |
|---|----------|-----------|--------------|--------|
| 1 | Boundary | `test_boundary_empty_seq` | Empty seq → precondition violated | ✅ REJECTED |
| 2 | Boundary | `test_boundary_element_excluded_by_pred` | No element satisfies pred → precondition violated | ✅ REJECTED |
| 3 | Boundary | `test_boundary_element_not_in_seq` | Element absent from seq → precondition violated | ✅ REJECTED |
| 4 | Boundary | `test_boundary_always_false_pred` | Always-false pred → empty filter | ✅ REJECTED |
| 5 | Boundary | `test_boundary_axiom_empty_seq_contains` | Axiom on empty seq, then assert contains | ✅ REJECTED |
| 6 | Behavioral | `test_mutation_negated_postcondition` | Negate ensures: `!s.contains(elt)` | ✅ REJECTED |
| 7 | Behavioral | `test_mutation_converse_direction` | Converse: `s.contains(e) ⟹ filter.contains(e)` | ✅ REJECTED |
| 8 | Behavioral | `test_mutation_filter_preserves_length` | Assert `filter.len() == s.len()` | ✅ REJECTED |
| 9 | Behavioral | `test_mutation_wrong_element` | Knowing elt1 ∈ filter, conclude elt2 ∈ s | ✅ REJECTED |
| 10 | Behavioral | `test_mutation_cross_predicate` | filter(pred1).contains ⟹ filter(pred2).contains | ✅ REJECTED |
| 11 | Logical | `test_logical_derive_false` | Derive `false` from axiom (soundness check) | ✅ REJECTED |
| 12 | Logical | `test_logical_specific_index` | Stronger: filter containment pins `s[0] == elt` | ✅ REJECTED |
| 13 | Logical | `test_logical_uniqueness` | Assert `filter.len() <= 1` (uniqueness) | ✅ REJECTED |
| 14 | Logical | `test_logical_filter_is_identity` | Assert `s.filter(pred) =~= s` (identity) | ✅ REJECTED |
| 15 | Logical | `test_logical_pred_irrelevant_to_length` | Different preds → same filter length | ✅ REJECTED |

## Conclusion

The specification correctly:
- **Rejects invalid inputs**: Precondition `s.filter(pred).contains(elt)` guards against empty sequences, absent elements, and non-matching predicates (tests 1–5).
- **Rejects incorrect behaviors**: Negated postconditions, converse directions, wrong-element transfers, and cross-predicate reasoning are all rejected (tests 6–10).
- **Rejects unintended reasoning**: The axiom does not introduce unsoundness, and does not entail stronger properties like index specificity, uniqueness, identity, or predicate-irrelevant length (tests 11–15).

**No spec weaknesses detected** in the tested semantic space.
