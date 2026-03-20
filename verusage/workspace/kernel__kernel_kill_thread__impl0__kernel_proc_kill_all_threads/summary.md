# Verus Specification Test Summary

## File Under Test

`kernel__kernel_kill_thread__impl0__kernel_proc_kill_all_threads.rs`

This file defines the `Kernel` struct and two methods:

1. **`kernel_kill_thread`** (`#[verifier::external_body]`): Kills a single thread. Trusted spec — removes the thread from `thread_dom`, decrements the owning process's `owned_threads` count, and preserves `wf`, `proc_dom`, `container_dom`, and tree structures.

2. **`kernel_proc_kill_all_threads`** (verified body): Kills all threads belonging to a process by iterating and calling `kernel_kill_thread` for each. Ensures `owned_threads.len() == 0` afterwards while preserving `wf`, `proc_dom`, `container_dom`, and tree structures.

---

## Correctness Results (correctness_tests.rs)

**Verus output**: `58 verified, 0 errors` ✅

| # | Test Name | Description | Expected | Actual |
|---|-----------|-------------|----------|--------|
| 1 | `test_kill_thread_ensures_wf` | kill_thread preserves wf | PASS | ✅ PASS |
| 2 | `test_kill_thread_removes_thread` | kill_thread removes thread from thread_dom | PASS | ✅ PASS |
| 3 | `test_kill_thread_preserves_proc_dom` | kill_thread leaves proc_dom unchanged | PASS | ✅ PASS |
| 4 | `test_kill_thread_preserves_container_dom` | kill_thread leaves container_dom unchanged | PASS | ✅ PASS |
| 5 | `test_kill_thread_decrements_owned_threads_len` | kill_thread decrements owning proc's thread count by 1 | PASS | ✅ PASS |
| 6 | `test_kill_thread_process_tree_unchanged` | kill_thread preserves process tree | PASS | ✅ PASS |
| 7 | `test_kill_thread_containers_tree_unchanged` | kill_thread preserves container tree | PASS | ✅ PASS |
| 8 | `test_kill_all_ensures_wf` | kill_all preserves wf | PASS | ✅ PASS |
| 9 | `test_kill_all_proc_in_dom` | kill_all keeps proc in proc_dom | PASS | ✅ PASS |
| 10 | `test_kill_all_empties_threads` | kill_all results in owned_threads.len() == 0 | PASS | ✅ PASS |
| 11 | `test_kill_all_preserves_proc_dom` | kill_all leaves proc_dom unchanged | PASS | ✅ PASS |
| 12 | `test_kill_all_preserves_container_dom` | kill_all leaves container_dom unchanged | PASS | ✅ PASS |
| 13 | `test_kill_all_process_tree_unchanged` | kill_all preserves process tree | PASS | ✅ PASS |
| 14 | `test_kill_all_containers_tree_unchanged` | kill_all preserves container tree | PASS | ✅ PASS |
| 15 | `test_kill_all_combined` | All kill_all postconditions combined | PASS | ✅ PASS |
| 16 | `test_kill_all_other_procs_preserved` | Other procs remain in domain after kill_all | PASS | ✅ PASS |

---

## Completeness Results

### Round 1: Precondition Violations (completeness_round1.rs)

**Verus output**: `40 verified, 5 errors` ✅ (all test functions fail)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_kill_thread_no_wf` | Call kill_thread without wf precondition | FAIL | ✅ FAIL |
| 2 | `test_fail_kill_thread_not_in_dom` | Call kill_thread on thread not in domain | FAIL | ✅ FAIL |
| 3 | `test_fail_kill_all_no_wf` | Call kill_all without wf precondition | FAIL | ✅ FAIL |
| 4 | `test_fail_kill_all_not_in_dom` | Call kill_all on proc not in domain | FAIL | ✅ FAIL |
| 5 | `test_fail_kill_thread_no_preconditions` | Call kill_thread with no preconditions at all | FAIL | ✅ FAIL |

### Round 2: Overly Strong Postconditions (completeness_round2.rs)

**Verus output**: `40 verified, 5 errors` ✅ (all test functions fail)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_kill_thread_dom_empty` | thread_dom becomes empty after killing one thread | FAIL | ✅ FAIL |
| 2 | `test_fail_kill_all_removes_proc` | proc removed from domain after kill_all | FAIL | ✅ FAIL |
| 3 | `test_fail_kill_all_proc_dom_shrinks` | proc_dom shrinks (loses target proc) after kill_all | FAIL | ✅ FAIL |
| 4 | `test_fail_kill_all_container_dom_empty` | container_dom becomes empty after kill_all | FAIL | ✅ FAIL |
| 5 | `test_fail_kill_all_threads_len_1` | owned_threads.len() == 1 after kill_all | FAIL | ✅ FAIL |

### Round 3: Negated Postconditions (completeness_round3.rs)

**Verus output**: `40 verified, 5 errors` ✅ (all test functions fail)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_kill_all_not_wf` | Assert !wf() after kill_all | FAIL | ✅ FAIL |
| 2 | `test_fail_kill_all_proc_not_in_dom` | Assert proc NOT in domain after kill_all | FAIL | ✅ FAIL |
| 3 | `test_fail_kill_all_threads_not_empty` | Assert owned_threads.len() != 0 after kill_all | FAIL | ✅ FAIL |
| 4 | `test_fail_kill_all_process_tree_changed` | Assert process tree changed after kill_all | FAIL | ✅ FAIL |
| 5 | `test_fail_kill_thread_not_wf` | Assert !wf() after kill_thread | FAIL | ✅ FAIL |

### Round 4: Wrong Specific Values (completeness_round4.rs)

**Verus output**: `40 verified, 5 errors` ✅ (all test functions fail)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_kill_all_len_42` | Assert owned_threads.len() == 42 after kill_all | FAIL | ✅ FAIL |
| 2 | `test_fail_kill_all_len_max` | Assert owned_threads.len() == 128 after kill_all | FAIL | ✅ FAIL |
| 3 | `test_fail_kill_thread_still_in_dom` | Assert killed thread still in domain | FAIL | ✅ FAIL |
| 4 | `test_fail_kill_thread_dom_unchanged` | Assert thread_dom unchanged after kill_thread | FAIL | ✅ FAIL |
| 5 | `test_fail_kill_thread_len_increases` | Assert owned_threads.len() increases after kill_thread | FAIL | ✅ FAIL |

### Round 5: Cross-Function Misuse & Edge Cases (completeness_round5.rs)

**Verus output**: `40 verified, 5 errors` ✅ (all test functions fail)

| # | Test Name | What It Tests | Expected | Actual |
|---|-----------|---------------|----------|--------|
| 1 | `test_fail_kill_all_containers_tree_changed` | Assert container tree changed after kill_all | FAIL | ✅ FAIL |
| 2 | `test_fail_kill_all_other_proc_removed` | Assert unrelated proc removed from domain | FAIL | ✅ FAIL |
| 3 | `test_fail_kill_thread_changes_proc_dom` | Assert kill_thread removes proc from proc_dom | FAIL | ✅ FAIL |
| 4 | `test_fail_kill_thread_changes_container_dom` | Assert kill_thread removes container from container_dom | FAIL | ✅ FAIL |
| 5 | `test_fail_kill_all_contradictory` | Assert thread_dom unchanged when proc had threads and kill_all zeroed them | FAIL | ✅ FAIL |

---

## Overall Assessment

### Correctness: ✅ PASS
All 16 correctness tests verify successfully. The specs for both `kernel_kill_thread` and `kernel_proc_kill_all_threads` are correct — every postcondition is provable from the preconditions and function bodies.

### Completeness: ✅ PASS
All 25 completeness tests fail as expected. The specs reject:
- **Precondition violations**: Both `wf()` and domain membership preconditions are necessary
- **Overly strong claims**: The spec doesn't allow proving that domains shrink or empty, or that thread counts are non-zero
- **Negated postconditions**: Every postcondition is tight — its negation is rejected
- **Wrong values**: Specific incorrect values for thread counts are rejected
- **Cross-function misuse**: The spec correctly isolates effects (kill_all doesn't affect other procs, kill_thread doesn't affect proc/container domains)

### Spec Gaps Found: None
The specifications are both correct and complete for the properties tested. No unexpected passes or failures were observed.
