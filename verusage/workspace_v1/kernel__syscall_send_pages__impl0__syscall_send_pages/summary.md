# Adversarial Proof Test Summary: `syscall_send_pages`

## Target
`kernel__syscall_send_pages__impl0__syscall_send_pages.rs`

## Results Overview

| Test Category | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary Tests | 7 | 7 | 0 |
| Behavioral Mutation Tests | 6 | 6 | 0 |
| Logical Tests | 7 | 7 | 0 |
| **Total** | **20** | **20** | **0** |

All 20 tests failed verification as intended, meaning the specification correctly rejects all tested invalid inputs, incorrect behaviors, and unintended logical properties.

---

## Boundary Tests (7/7 FAIL ✓)

| # | Test | Failure Mode | Result |
|---|---|---|---|
| 1 | `test_boundary_endpoint_index_at_max` | Off-by-one: `MAX_NUM_ENDPOINT_DESCRIPTORS < MAX_NUM_ENDPOINT_DESCRIPTORS` | FAIL ✓ |
| 2 | `test_boundary_thread_not_in_domain` | Thread outside domain lacks owning_container guarantee | FAIL ✓ |
| 3 | `test_boundary_sender_thread_not_running` | BLOCKED thread != RUNNING | FAIL ✓ |
| 4 | `test_boundary_sender_thread_scheduled` | SCHEDULED thread != RUNNING | FAIL ✓ |
| 5 | `test_boundary_endpoint_index_usize_max` | `usize::MAX` exceeds valid endpoint range | FAIL ✓ |
| 6 | `test_boundary_exact_max_endpoint_idx` | Exact boundary: 128 is not < 128 | FAIL ✓ |
| 7 | `test_boundary_zero_len_va_range_implies_positive` | Zero-length va_range does not imply len > 0 | FAIL ✓ |

## Behavioral Mutation Tests (6/6 FAIL ✓)

| # | Test | Mutation | Result |
|---|---|---|---|
| 1 | `test_mutation_no_endpoint_state_changes` | Claim proc_dom changes when endpoint doesn't exist (should be no-op) | FAIL ✓ |
| 2 | `test_mutation_sender_queue_full_mapping_changes` | Claim thread_dom changes when sender queue full (should be no-op) | FAIL ✓ |
| 3 | `test_mutation_no_receiver_queue_unchanged` | Claim endpoint queue unchanged when no receiver (should push sender) | FAIL ✓ |
| 4 | `test_mutation_receiver_empty_queue_state_unchanged` | Claim queue_state stays RECEIVE when receiver queue empty (should become SEND) | FAIL ✓ |
| 5 | `test_mutation_len_mismatch_state_changes` | Claim container_dom changes when va_range lengths mismatch (should be no-op) | FAIL ✓ |
| 6 | `test_mutation_same_proc_state_changes` | Claim endpoint_dom changes when sender==receiver proc (should be no-op) | FAIL ✓ |

## Logical Tests (7/7 FAIL ✓)

| # | Test | Unintended Property Tested | Result |
|---|---|---|---|
| 1 | `test_logical_return_deterministic` | Determinism: two runs produce identical new kernel states | FAIL ✓ |
| 2 | `test_logical_receiver_thread_state_scheduled` | Receiver thread state becomes SCHEDULED after success (not specified) | FAIL ✓ |
| 3 | `test_logical_success_return_code_not_specified` | Return code is Else on success (spec doesn't constrain ret) | FAIL ✓ |
| 4 | `test_logical_sender_pages_removed_after_share` | Sender loses page mappings after share (it's share, not move) | FAIL ✓ |
| 5 | `test_logical_free_pages_always_decrease` | Free pages always strictly decrease (false in error paths) | FAIL ✓ |
| 6 | `test_logical_sender_proc_changes_on_success` | Sender process pcid changes on success (procs are preserved) | FAIL ✓ |
| 7 | `test_logical_page_mapping_domain_grows` | Physical page mapping domain grows (domain is preserved =~=) | FAIL ✓ |

---

## Conclusion

The specification for `syscall_send_pages` demonstrates strong consistency across all three test categories:

1. **Boundary correctness**: The preconditions correctly reject invalid indices, out-of-domain threads, and wrong thread states.
2. **Behavioral correctness**: The spec correctly distinguishes all error/success paths — no-ops remain no-ops, and state mutations match expected behavior.
3. **Logical soundness**: The spec does not entail unintended properties such as determinism of new state, return code constraints, or page transfer semantics beyond sharing.

**Notable spec weakness detected**: The spec does not constrain the return value (`ret`) in any branch, nor does it specify the receiver thread's state transition in the success path. These are properties the implementation provides but the specification does not export.
