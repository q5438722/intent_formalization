# Summary: Verus Specification Tests for `remove_helper7`

## File Under Test

`slinkedlist__spec_impl_u__impl2__remove_helper7.rs` — Implements `remove_helper7` for `StaticLinkedList<T, N>`, which removes a middle node (neither head nor tail) from a doubly-linked list backed by a fixed-size array. The removed node is appended to the free list.

**Preconditions**: list is well-formed (`wf()`), value exists, index matches, list has >1 element, free list is non-empty, node is not head or tail.

**Postconditions**: list remains well-formed, length decreases by 1, uniqueness preserved, view equals old view minus removed value, return value equals ghost value, node references preserved for remaining elements.

---

## Correctness Results

All tests **PASS** (verification successful).

| Test | Description | Expected | Actual |
|------|-------------|----------|--------|
| `test_wf_preserved` | Assert `wf()` after removal | PASS | ✅ PASS |
| `test_unique_preserved` | Assert `unique()` after removal | PASS | ✅ PASS |
| `test_ret_equals_v` | Assert `ret == v@` | PASS | ✅ PASS |
| `test_view_remove` | Assert `sll@ =~= old_view.remove_value(ret)` | PASS | ✅ PASS |
| `test_len_decreases` | Assert length decreases by exactly 1 | PASS | ✅ PASS |
| `test_node_refs_preserved` | Assert node refs unchanged for remaining elements | PASS | ✅ PASS |
| `test_all_postconditions` | Assert all postconditions together | PASS | ✅ PASS |
| `test_min_n` | Same test with N=3 (minimum valid) | PASS | ✅ PASS |
| `test_large_n` | Same test with N=1000 | PASS | ✅ PASS |
| `test_generic_type` | Same test with `i32` instead of `u64` | PASS | ✅ PASS |

**Verus output**: `11 verified, 0 errors`

---

## Completeness Results

### Round 1: Precondition Violations

All tests **FAIL** as expected (verification errors).

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_missing_wf` | Omit `wf()` precondition | FAIL | ✅ FAIL |
| `test_missing_contains` | Omit `contains(v@)` precondition | FAIL | ✅ FAIL |
| `test_missing_index_match` | Omit `get_node_ref(v@) == remove_index` | FAIL | ✅ FAIL |
| `test_value_list_len_eq_1` | Set `value_list_len == 1` (violates `!= 1`) | FAIL | ✅ FAIL |
| `test_free_list_len_eq_0` | Set `free_list_len == 0` (violates `!= 0`) | FAIL | ✅ FAIL |
| `test_is_tail` | Set `value_list_tail == remove_index` | FAIL | ✅ FAIL |
| `test_is_head` | Set `value_list_head == remove_index` | FAIL | ✅ FAIL |

**Verus output**: `1 verified, 7 errors`

### Round 2: Overly Strong Postconditions

All tests **FAIL** as expected.

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_len_decreases_by_2` | Assert length decreases by 2 | FAIL | ✅ FAIL |
| `test_len_unchanged` | Assert length stays the same | FAIL | ✅ FAIL |
| `test_len_zero` | Assert length becomes 0 | FAIL | ✅ FAIL |
| `test_view_unchanged` | Assert view is unchanged | FAIL | ✅ FAIL |
| `test_view_push_instead_of_remove` | Assert view = old.push(ret) | FAIL | ✅ FAIL |
| `test_len_increases` | Assert length increases by 1 | FAIL | ✅ FAIL |

**Verus output**: `1 verified, 6 errors`

### Round 3: Negated/Contradicted Postconditions

All tests **FAIL** as expected.

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_not_wf` | Assert `!wf()` after removal | FAIL | ✅ FAIL |
| `test_not_unique` | Assert `!unique()` after removal | FAIL | ✅ FAIL |
| `test_ret_not_v` | Assert `ret != v@` | FAIL | ✅ FAIL |
| `test_view_not_remove` | Assert view ≠ old.remove_value(ret) | FAIL | ✅ FAIL |
| `test_removed_still_contains` | Assert removed value still in list | FAIL | ✅ FAIL |
| `test_len_increased` | Assert length increased | FAIL | ✅ FAIL |

**Verus output**: `1 verified, 6 errors`

### Round 4: Wrong Specific Values

All tests **FAIL** as expected.

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_ret_always_zero` | Assert ret == 0 (with v@ != 0) | FAIL | ✅ FAIL |
| `test_ret_always_42` | Assert ret == 42 (with v@ != 42) | FAIL | ✅ FAIL |
| `test_len_always_5` | Assert len always 5 after removal | FAIL | ✅ FAIL |
| `test_view_len_zero` | Assert view length is 0 | FAIL | ✅ FAIL |
| `test_removed_node_ref_neg1` | Assert node ref for removed value is -1 | FAIL | ✅ FAIL |
| `test_remove_index_always_zero` | Assert remove_index == 0 (with != 0) | FAIL | ✅ FAIL |

**Verus output**: `1 verified, 6 errors`

### Round 5: Cross-Function Misuse & Edge Cases

All tests **FAIL** as expected.

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_double_remove` | Call remove_helper7 twice with same value | FAIL | ✅ FAIL |
| `test_head_unchanged_after_remove` | Assert free_list_len unchanged | FAIL | ✅ FAIL |
| `test_list_empty_after_remove` | Assert list is empty after removal | FAIL | ✅ FAIL |
| `test_value_list_len_unchanged` | Assert value_list_len unchanged | FAIL | ✅ FAIL |
| `test_false_after_remove` | Assert false after valid removal | FAIL | ✅ FAIL |
| `test_wrong_len_relationship` | Assert value_list_len increased | FAIL | ✅ FAIL |

**Verus output**: `1 verified, 6 errors`

---

## Overall Assessment

### Correctness: ✅ PASS
All 10 correctness tests verify successfully. The postconditions of `remove_helper7` are correct — the function body satisfies the specification for all valid inputs.

### Completeness: ✅ PASS
All 31 completeness tests fail as expected. The specification is tight enough to reject:
- Missing preconditions (7/7 detected)
- Overly strong postconditions (6/6 detected)
- Negated postconditions (6/6 detected)
- Wrong specific values (6/6 detected)
- Cross-function misuse and edge cases (6/6 detected)

### Spec Gaps Found: None
No spec gaps were identified. The `remove_helper7` specification is both correct and complete for the tested properties.
