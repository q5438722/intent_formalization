# Adversarial Proof Test Summary

**Target**: `slinkedlist__spec_impl_u__impl2__remove_helper2.rs`  
**Function under test**: `StaticLinkedList::remove_helper2`

---

## Results Overview

| Test Category          | Total | Failed (expected) | Passed (unexpected) |
|------------------------|-------|--------------------|---------------------|
| Boundary Tests         | 5     | 5 ✅               | 0                   |
| Behavioral Mutation    | 4     | 4 ✅               | 0                   |
| Logical Tests          | 4     | 4 ✅               | 0                   |

**All 13 adversarial tests failed verification as expected**, confirming the specification correctly rejects invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Boundary Tests (boundary_tests.rs)

Each test violates a single precondition of `remove_helper2`:

| # | Test | Violated Precondition | Result |
|---|------|-----------------------|--------|
| 1 | `test_boundary_no_wf` | Missing `wf()` | ❌ Failed: `old(self).wf()` |
| 2 | `test_boundary_value_absent` | `!contains(v@)` | ❌ Failed: `old(self)@.contains(v@)` |
| 3 | `test_boundary_single_element` | `value_list_len == 1` | ❌ Failed: `value_list_len != 1` |
| 4 | `test_boundary_head_mismatch` | `value_list_head != remove_index` | ❌ Failed: `free_list_len == 0 && value_list_head == remove_index` |
| 5 | `test_boundary_free_list_nonempty` | `free_list_len > 0` | ❌ Failed: `free_list_len == 0 && value_list_head == remove_index` |

## Behavioral Mutation Tests (behavioral_mutation_tests.rs)

Each test asserts a mutated (incorrect) postcondition:

| # | Test | Mutated Property | Result |
|---|------|------------------|--------|
| 1 | `test_mutation_wrong_length_decrease` | `len - 2` instead of `len - 1` | ❌ Failed postcondition |
| 2 | `test_mutation_wrong_return_value` | `ret != v@` instead of `ret == v@` | ❌ Failed postcondition |
| 3 | `test_mutation_value_still_present` | `contains(ret)` after removal | ❌ Failed assertion |
| 4 | `test_mutation_length_unchanged` | `len == old_len` | ❌ Failed postcondition |

## Logical Tests (logical_tests.rs)

Each test asserts a property NOT explicitly in the postconditions:

| # | Test | Unguaranteed Property | Result |
|---|------|-----------------------|--------|
| 1 | `test_logical_empty_after_remove` | List becomes empty | ❌ Failed assertion |
| 2 | `test_logical_arr_seq_unchanged` | Internal array unchanged | ❌ Failed postcondition |
| 3 | `test_logical_seq_len_equals_n` | Sequence length == N | ❌ Failed postcondition |
| 4 | `test_logical_head_unchanged` | `value_list_head` unchanged | ❌ Failed postcondition |

---

## Notable Finding: `closed spec wf()` Transparency

During initial testing, a logical test asserting `sll.free_list_len == 1` **PASSED** verification. This property is not in the explicit postconditions but is derivable because:

1. **Precondition**: `old(sll).free_list_len == 0`
2. **Postcondition**: `self.wf()` (closed spec containing `free_list_len + value_list_len == N`)
3. **Postcondition**: `self.len() == old(self).len() - 1`

Within the same module, `closed spec fn wf()` is transparent enough for Verus to unfold and derive internal invariant relationships. This means the spec **permits reasoning about internal implementation details** (e.g., `free_list_len`) through the opaque `wf()` predicate, which may or may not be intentional.

---

## Conclusion

The specification of `remove_helper2` is **well-constrained**:
- All 5 preconditions are independently necessary (boundary tests)
- All 4 key postcondition properties are precise (mutation tests)
- The spec does not imply unwarranted external properties (logical tests)

The only concern is the transparency of `closed spec wf()` within the same module, which allows derivation of internal state properties (`free_list_len`) from the postconditions.
