# Test Summary: `seq_filter_is_a_subset_of_original_seq`

## Specification Under Test

```rust
pub proof fn seq_filter_is_a_subset_of_original_seq<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    ensures
        forall |e: A| s.filter(pred).contains(e) ==> #[trigger] s.contains(e),
        forall |i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger] s.filter(pred)[i]),
```

No preconditions (`requires`). The spec guarantees that every element in the filtered sequence is contained in the original sequence (subset property), stated in both element-based and index-based forms.

---

## Results

| # | Category | Test Name | Property Tested | Result |
|---|----------|-----------|-----------------|--------|
| 1 | Boundary | `test_boundary_empty_seq_nonempty_filter` | Empty seq filter has len > 0 | ✅ FAILED |
| 2 | Boundary | `test_boundary_single_element_not_matching` | Non-matching element is in filter | ✅ FAILED |
| 3 | Boundary | `test_boundary_all_filtered_out_positive_length` | Filter of non-matching elements has len > 0 | ✅ FAILED |
| 4 | Behavioral | `test_mutation_reverse_containment` | Converse: s.contains(e) ⟹ filter.contains(e) | ✅ FAILED |
| 5 | Behavioral | `test_mutation_length_preservation` | filter.len() == s.len() after selective filtering | ✅ FAILED |
| 6 | Behavioral | `test_mutation_filter_equals_original` | filter =~= s when pred rejects elements | ✅ FAILED |
| 7 | Logical | `test_logical_derive_false` | Derive `false` from the axiom (soundness) | ✅ FAILED |
| 8 | Logical | `test_logical_filter_length_exceeds_original` | filter.len() > s.len() | ✅ FAILED |
| 9 | Logical | `test_logical_index_correspondence` | filter[0] == s[0] when first element is filtered out | ✅ FAILED |

**Total: 9/9 tests correctly FAILED verification.**

---

## Analysis

The specification correctly rejects all 9 adversarial queries:

- **Boundary**: The spec does not allow deriving non-empty filter results from empty sequences or sequences where no elements match the predicate. Edge cases are properly handled.

- **Behavioral Mutations**: The spec resists mutated postconditions — it does not entail the converse of the subset relation, length preservation, or sequence equality. The one-directional implication (filter ⊆ original) is tight enough to reject these reverse/strengthened claims.

- **Logical**: The axiom is sound (cannot derive `false`), does not leak stronger length bounds (filter cannot exceed original length), and does not entail index-position correspondence between the filtered and original sequences.

## Conclusion

The specification is **consistent** with respect to all tested properties. No unintended entailments were detected. The ensures clause precisely captures the subset property without admitting incorrect reasoning.
