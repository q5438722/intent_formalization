# Test Summary: `remove_helper5` Specification Consistency

**Target**: `slinkedlist__spec_impl_u__impl2__remove_helper5.rs`
**Function**: `StaticLinkedList::remove_helper5(&mut self, remove_index: SLLIndex, v: Ghost<T>) -> T`

## Results: All 15 adversarial tests correctly FAILED ✅

The specification correctly rejects all invalid inputs, incorrect behaviors, and unintended reasoning.

---

### Boundary Tests (5/5 FAILED ✅)

| # | Test | Violation | Result |
|---|------|-----------|--------|
| 1 | `test_remove_missing_wf` | Missing `wf()` precondition | precondition not satisfied |
| 2 | `test_remove_value_not_in_list` | `!contains(v@)` — value absent | precondition not satisfied |
| 3 | `test_remove_single_element` | `value_list_len == 1` — singleton list | precondition not satisfied |
| 4 | `test_remove_empty_free_list` | `free_list_len == 0` — no free nodes | precondition not satisfied |
| 5 | `test_remove_not_head` | `value_list_head != remove_index` — not removing head | precondition not satisfied |

### Behavioral Mutation Tests (5/5 FAILED ✅)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_remove_length_unchanged` | `len == old_len` (should be `old_len - 1`) | postcondition not satisfied |
| 2 | `test_remove_length_decreases_by_two` | `len == old_len - 2` (should be `-1`) | postcondition not satisfied |
| 3 | `test_remove_wrong_return_value` | `ret != v@` (should be `ret == v@`) | postcondition not satisfied |
| 4 | `test_remove_seq_unchanged` | `self@ =~= old@` (should reflect removal) | postcondition not satisfied |
| 5 | `test_remove_element_still_contained` | `contains(ret)` after removal | assertion failed |

### Logical Tests (5/5 FAILED ✅)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_remove_index_is_zero` | `remove_index == 0` — index not fixed | assertion failed |
| 2 | `test_free_list_len_unchanged` | `free_list_len` unchanged — hidden invariant | assertion failed |
| 3 | `test_removed_leq_new_head` | `ret <= sll@[0]` — no ordering guarantee | assertion failed |
| 4 | `test_value_list_head_unchanged` | `value_list_head` unchanged after head removal | assertion failed |
| 5 | `test_stronger_length_bound` | `len >= 2` — could be 1 after removal | assertion failed |

---

### Notable Finding

During development, an initial logical test asserting `ret == old(sll)@[0]` **passed verification**, revealing that the spec entails this property through closed spec unfolding within the same module. This is semantically expected (the function removes the head element), confirming the spec's internal consistency on this point. The test was replaced with an ordering test (`ret <= sll@[0]`) that correctly fails.

### Conclusion

The specification for `remove_helper5` is **well-constrained**:
- All 5 preconditions are necessary and enforced
- All postconditions correctly reject mutated behaviors
- No unintended logical properties are entailed
