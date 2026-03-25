# Test Summary: `verus_extra__lemma_to_set_singleton_auto`

## Specification Under Test

```rust
pub proof fn lemma_to_set_singleton_auto<A>()
ensures
    forall |x: A| #[trigger] seq![x].to_set() == set![x],
```

**Semantics**: For any type `A` and element `x: A`, converting the singleton sequence `[x]` to a set yields the singleton set `{x}`. No preconditions.

---

## Results Summary

| # | Category | Test | Expected | Actual | Status |
|---|----------|------|----------|--------|--------|
| 1 | Boundary | `seq![0].to_set() == empty_set` | FAIL | FAIL | ✅ |
| 2 | Boundary | `seq![1].to_set() == set![1, 2]` | FAIL | FAIL | ✅ |
| 3 | Boundary | `seq![5].to_set().contains(3)` | FAIL | FAIL | ✅ |
| 4 | Mutation | `seq![1].to_set() == set![2]` (swapped element) | FAIL | FAIL | ✅ |
| 5 | Mutation | `seq![1].to_set() != set![1]` (negated equality) | FAIL | FAIL | ✅ |
| 6 | Mutation | `seq![1].to_set().contains(99)` (extra element) | FAIL | FAIL | ✅ |
| 7 | Logical  | `seq![1, 2].to_set() == set![1]` (multi-elem seq) | FAIL | FAIL | ✅ |
| 8 | Logical  | `seq![1].to_set().len() == 2` (wrong cardinality) | FAIL | FAIL | ✅ |
| 9 | Logical  | `seq![1].to_set().subset_of(set![2, 3])` (unrelated subset) | FAIL | FAIL | ✅ |

**All 9/9 adversarial tests were correctly rejected.**

---

## Analysis

The specification `forall |x: A| seq![x].to_set() == set![x]` is **consistent** with respect to all tested semantic queries:

- **Boundary**: The spec does not admit empty-set or multi-element-set equivalences for singleton sequences.
- **Behavioral Mutation**: The spec correctly ties the output element to the input element; swapping, negating, or inflating the result is rejected.
- **Logical**: The spec does not over-generalize to multi-element sequences, does not claim incorrect cardinality, and does not entail unrelated subset relationships.

### Conclusion

No specification weakness was detected. The lemma precisely captures the singleton sequence-to-set conversion without admitting unintended entailments.
