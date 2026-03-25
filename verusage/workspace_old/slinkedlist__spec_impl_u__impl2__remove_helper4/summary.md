# Specification Testing Summary: `remove_helper4`

## File Under Test

`slinkedlist__spec_impl_u__impl2__remove_helper4.rs` — Defines `StaticLinkedList<T, N>`, a doubly-linked list over a fixed-size array. The main function `remove_helper4` removes a middle element (neither head nor tail) when the free list is empty. Also includes proof lemmas `seq_push_lemma` and `seq_remove_lemma` for sequence reasoning.

---

## Correctness Results (all should PASS ✅)

**Result: 18 verified, 0 errors**

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_push_fresh_contained` | `seq_push_lemma`: pushed element is contained | PASS | ✅ PASS |
| 2 | `test_push_preserves_existing` | `seq_push_lemma`: existing elements remain after push | PASS | ✅ PASS |
| 3 | `test_push_non_member_stays_absent` | `seq_push_lemma`: non-member doesn't appear after unrelated push | PASS | ✅ PASS |
| 4 | `test_push_chain_all_contained` | `seq_push_lemma`: chained pushes — all elements contained | PASS | ✅ PASS |
| 5 | `test_remove_index_before` | `seq_remove_lemma`: index before removal unchanged | PASS | ✅ PASS |
| 6 | `test_remove_index_after` | `seq_remove_lemma`: index after removal shifts down | PASS | ✅ PASS |
| 7 | `test_remove_element_gone` | `seq_remove_lemma`: removed element no longer contained | PASS | ✅ PASS |
| 8 | `test_remove_preserves_other` | `seq_remove_lemma`: non-removed element preserved | PASS | ✅ PASS |
| 9 | `test_remove_value_equivalence` | `seq_remove_lemma`: subrange removal == `remove_value` | PASS | ✅ PASS |
| 10 | `test_rh4_wf_maintained` | `remove_helper4`: well-formedness preserved (N=5) | PASS | ✅ PASS |
| 11 | `test_rh4_return_value` | `remove_helper4`: returns correct value `v@` | PASS | ✅ PASS |
| 12 | `test_rh4_seq_updated` | `remove_helper4`: spec sequence = old.remove_value(v@) | PASS | ✅ PASS |
| 13 | `test_rh4_unique_preserved` | `remove_helper4`: uniqueness maintained | PASS | ✅ PASS |
| 14 | `test_rh4_node_refs_preserved` | `remove_helper4`: node refs preserved for remaining elements | PASS | ✅ PASS |
| 15 | `test_rh4_n10` | `remove_helper4`: all postconditions hold with N=10 | PASS | ✅ PASS |
| 16 | `test_rh4_len_decreases` | `remove_helper4`: length decreases by 1 | PASS | ✅ PASS |
| 17 | `test_rh4_all_postconditions` | `remove_helper4`: all postconditions verified together | PASS | ✅ PASS |

---

## Completeness Results (all should FAIL ❌)

### Round 1: Precondition Violations (7 errors, all tests fail ✅)

| # | Test Name | What it Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_missing_wf` | Drop `wf()` precondition | FAIL | ❌ FAIL |
| 2 | `test_missing_contains` | Drop `@.contains(v@)` precondition | FAIL | ❌ FAIL |
| 3 | `test_missing_node_ref` | Drop `get_node_ref(v@) == remove_index` | FAIL | ❌ FAIL |
| 4 | `test_wrong_ghost_value` | Pass non-contained ghost value `w` instead of `v` | FAIL | ❌ FAIL |
| 5 | `test_missing_free_list_empty` | Drop `free_list_len == 0` | FAIL | ❌ FAIL |
| 6 | `test_missing_not_tail` | Drop `value_list_tail != remove_index` | FAIL | ❌ FAIL |
| 7 | `test_missing_not_head` | Drop `value_list_head != remove_index` | FAIL | ❌ FAIL |

### Round 2: Overly Strong Postconditions (5 errors, all tests fail ✅)

| # | Test Name | What it Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_len_becomes_zero` | Assert `sll@.len() == 0` (too strong) | FAIL | ❌ FAIL |
| 2 | `test_seq_unchanged` | Assert `sll@ =~= old(sll)@` (sequence unchanged) | FAIL | ❌ FAIL |
| 3 | `test_len_increases` | Assert `sll.len() == old.len() + 1` (wrong direction) | FAIL | ❌ FAIL |
| 4 | `test_seq_empty` | Assert `sll@ =~= Seq::empty()` (too strong) | FAIL | ❌ FAIL |
| 5 | `test_removed_still_contained` | Assert removed value still in sequence | FAIL | ❌ FAIL |

### Round 3: Negated Postconditions (5 errors, all tests fail ✅)

| # | Test Name | What it Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_negate_wf` | Assert `!sll.wf()` | FAIL | ❌ FAIL |
| 2 | `test_negate_ret_value` | Assert `ret != v@` | FAIL | ❌ FAIL |
| 3 | `test_negate_unique` | Assert `!sll.unique()` | FAIL | ❌ FAIL |
| 4 | `test_negate_len_decrease` | Assert `sll.len() == old.len()` (no decrease) | FAIL | ❌ FAIL |
| 5 | `test_negate_remove_value` | Assert `sll@.len() > old@.len()` (sequence grows) | FAIL | ❌ FAIL |

### Round 4: Wrong Specific Values (5 errors, all tests fail ✅)

| # | Test Name | What it Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_push_wrong_value_contained` | Pushing 1 makes 2 contained | FAIL | ❌ FAIL |
| 2 | `test_empty_contains` | Empty seq contains 42 | FAIL | ❌ FAIL |
| 3 | `test_remove_wrong_index` | Wrong index mapping after removal | FAIL | ❌ FAIL |
| 4 | `test_remove_value_still_present` | Removed element still present | FAIL | ❌ FAIL |
| 5 | `test_push_unrelated_creates_member` | Pushing 2 makes 99 appear | FAIL | ❌ FAIL |

### Round 5: Cross-Function Misuse & Edge Cases (5 errors, all tests fail ✅)

| # | Test Name | What it Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_double_remove` | Call remove_helper4 twice (free_list != 0 for 2nd) | FAIL | ❌ FAIL |
| 2 | `test_removed_node_ref` | Assert node_ref preserved for removed element | FAIL | ❌ FAIL |
| 3 | `test_wrong_position_claim` | Assert removed value is at index 0 | FAIL | ❌ FAIL |
| 4 | `test_push_remove_cancel_wrong` | Claim removed element reappears after push | FAIL | ❌ FAIL |
| 5 | `test_remove_makes_empty` | Assert list becomes empty after removal | FAIL | ❌ FAIL |

---

## Overall Assessment

### Correctness: ✅ PASS
All 17 correctness tests verify successfully. The specs correctly describe the behavior of `remove_helper4`, `seq_push_lemma`, and `seq_remove_lemma`.

### Completeness: ✅ PASS
All 27 completeness tests fail as expected. The specs are tight enough to reject:
- Missing preconditions
- Overly strong claims
- Negated postconditions
- Wrong concrete values
- Cross-function misuse

### Findings

**Redundant precondition discovered**: The original precondition `value_list_len != 1` on `remove_helper4` is redundant when combined with `wf()` and `free_list_len == 0`, since `wf()` implies `free_list_len + value_list_len == N` and `N > 2`, so `value_list_len == N > 2 > 1`. This was discovered when an initial completeness test that dropped only `value_list_len != 1` still passed verification. The test was replaced with `test_wrong_ghost_value` (passing a non-contained value).

### Spec Quality
The specifications are both correct and complete. They accurately capture the contract of removing a middle element from a static linked list with an empty free list, and they are precise enough to reject all tested forms of misuse.
