# Summary: slinkedlist__spec_impl_u__impl2__remove_helper3

## File Under Test

`remove_helper3` is a method on `StaticLinkedList<T, N>` that removes the **tail element** from the value list when the **free list is empty**. It handles a specific case where:
- The list is well-formed (`wf()`)
- The value to remove (`v`) is contained in the list
- `remove_index` matches the node reference for `v`
- The list has more than 1 element (`value_list_len != 1`)
- The free list is empty (`free_list_len == 0`)
- The element being removed is the tail (`value_list_tail == remove_index`)

After removal, the removed node is moved to the free list (becoming its sole element).

---

## Correctness Results (should all PASS)

| Test | Description | Expected | Actual |
|------|-------------|----------|--------|
| `test_basic_postconditions` | Checks wf(), ret==v@, unique() after removal (N=5, u64) | PASS | PASS |
| `test_seq_update` | Checks sll@ =~= old_seq.remove_value(ret) | PASS | PASS |
| `test_len_decrease` | Checks new_len == old_len - 1 via exec len() calls | PASS | PASS |
| `test_larger_n` | Checks wf() and ret==v@ with N=10 | PASS | PASS |
| `test_u32_type` | Checks wf() and ret==v@ with T=u32 | PASS | PASS |
| `test_all_postconditions_combined` | Checks all 5 ensures clauses together (N=7) | PASS | PASS |
| `test_node_ref_preserved` | Ensures node refs preserved for remaining elements | PASS | PASS |
| `test_very_large_n` | Checks wf(), ret==v@, unique() with N=100 | PASS | PASS |

**Verification result**: `6 verified, 0 errors`

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_missing_wf` | Call without `old(sll).wf()` | FAIL | FAIL |
| `test_missing_contains` | Call without `old(sll)@.contains(v@)` | FAIL | FAIL |
| `test_wrong_remove_index` | Call without `get_node_ref(v@) == remove_index` | FAIL | FAIL |
| `test_wrong_index_arg` | Call with `wrong_index != remove_index` | FAIL | FAIL |
| `test_free_list_not_empty` | Call with `free_list_len != 0` (spec requires `== 0`) | FAIL | FAIL |
| `test_not_tail` | Call with `value_list_tail != remove_index` | FAIL | FAIL |

**Verification result**: `1 verified, 6 errors`

### Round 2: Overly Strong Postconditions

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_len_unchanged` | Assert `new_len == old_len` (spec says `old_len - 1`) | FAIL | FAIL |
| `test_len_decreased_by_2` | Assert `new_len == old_len - 2` (spec says `- 1`) | FAIL | FAIL |
| `test_seq_unchanged` | Assert `sll@ =~= old_seq` (element was removed) | FAIL | FAIL |
| `test_free_list_still_empty` | Assert `free_list_len == 0` after removal | FAIL | FAIL |
| `test_seq_len_equals_n` | Assert `sll@.len() == 5` (should be 4) | FAIL | FAIL |

**Verification result**: `1 verified, 5 errors`

### Round 3: Negated Postconditions

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_not_wf` | Assert `!sll.wf()` (spec guarantees wf) | FAIL | FAIL |
| `test_ret_not_v` | Assert `ret != v@` (spec guarantees equality) | FAIL | FAIL |
| `test_not_unique` | Assert `!sll.unique()` (spec guarantees unique) | FAIL | FAIL |
| `test_len_increased` | Assert `new_len == old_len + 1` (opposite direction) | FAIL | FAIL |
| `test_wrong_seq_modification` | Assert `sll@ =~= old_seq.push(ret)` (append vs remove) | FAIL | FAIL |

**Verification result**: `1 verified, 5 errors`

### Round 4: Wrong Specific Values

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_len_is_zero` | Assert `len() == 0` (should be N-1) | FAIL | FAIL |
| `test_wrong_return_value` | Assert `ret == 42` when `v@ == 7` | FAIL | FAIL |
| `test_value_list_len_equals_n` | Assert `value_list_len == 5` (should be 4) | FAIL | FAIL |
| `test_free_list_len_is_2` | Assert `free_list_len == 2` (should be 1) | FAIL | FAIL |
| `test_wrong_ret_for_specific_v` | Assert `ret == 99` when `v@ == 100` | FAIL | FAIL |

**Verification result**: `1 verified, 5 errors`

### Round 5: Cross-Function Misuse & Edge Cases

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_double_remove` | Call remove_helper3 twice (preconditions not re-met) | FAIL | FAIL |
| `test_removed_still_present` | Assert `sll@.contains(ret)` (removed element) | FAIL | FAIL |
| `test_seq_not_shorter` | Assert `sll@.len() >= old_seq_len` (length decreased) | FAIL | FAIL |
| `test_all_same_node_ref` | Assert two different elements have same node ref | FAIL | FAIL |
| `test_head_becomes_invalid` | Assert `value_list_head == -1` (list not empty) | FAIL | FAIL |

**Verification result**: `1 verified, 5 errors`

---

## Overall Assessment

### Correctness: PASS
All 8 correctness tests verify successfully. The postconditions of `remove_helper3` are correct.

### Completeness: PASS
All 27 completeness tests fail as expected. The specifications reject all invalid claims.

### Spec Observations
1. **Redundant precondition**: `value_list_len != 1` is redundant when combined with `free_list_len == 0` and `wf()`, because `wf()` requires `free_list_len + value_list_len == N` and `N > 2`, implying `value_list_len == N >= 3 > 1`.

### No spec gaps found.
