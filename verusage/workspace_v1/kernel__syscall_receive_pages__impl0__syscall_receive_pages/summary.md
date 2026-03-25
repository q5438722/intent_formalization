# Adversarial Proof Test Summary: `syscall_receive_pages`

## Target
`kernel__syscall_receive_pages__impl0__syscall_receive_pages.rs`

## Results Overview

| Test File | Tests | All Failed (as expected) |
|---|---|---|
| `boundary_tests.rs` | 7 | ✅ 7/7 errors |
| `behavioral_mutation_tests.rs` | 6 | ✅ 6/6 errors |
| `logical_tests.rs` | 7 | ✅ 7/7 errors |

**Total: 20 tests, 20 verification failures (all expected)**

---

## Boundary Tests (7/7 FAIL ✅)

| # | Test | Failure Mode |
|---|---|---|
| 1 | `test_boundary_endpoint_idx_at_max` | Off-by-one: `MAX == 128` is not `< 128` |
| 2 | `test_boundary_thread_not_in_domain` | Thread not in domain cannot access container |
| 3 | `test_boundary_receiver_thread_blocked` | BLOCKED ≠ RUNNING (violates precondition) |
| 4 | `test_boundary_receiver_thread_scheduled` | SCHEDULED ≠ RUNNING (violates precondition) |
| 5 | `test_boundary_endpoint_idx_usize_max` | usize::MAX is not < 128 |
| 6 | `test_boundary_exact_max_endpoint_idx` | 128 is not < 128 |
| 7 | `test_boundary_zero_len_va_range_implies_positive` | len == 0 does not imply len > 0 |

**Interpretation:** The spec correctly rejects all boundary violations. Preconditions on thread state, endpoint index range, and thread domain membership are all enforced.

---

## Behavioral Mutation Tests (6/6 FAIL ✅)

| # | Test | Mutation |
|---|---|---|
| 1 | `test_mutation_no_endpoint_state_changes` | Endpoint doesn't exist → claim state changed (spec says `old =~= new`) |
| 2 | `test_mutation_no_sender_queue_unchanged` | No sender path → claim queue unchanged (spec says receiver pushed) |
| 3 | `test_mutation_sender_queue_empty_state_stays_send` | Empty sender queue → claim state stays SEND (spec says → RECEIVE) |
| 4 | `test_mutation_success_queue_unchanged` | Success path → claim queue unchanged (spec says skip(1)) |
| 5 | `test_mutation_success_receiver_address_space_empty` | Success path → claim receiver VA unmapped (spec says mapped) |
| 6 | `test_mutation_success_page_mapping_domain_changed` | Success path → claim page mapping domain grew (spec says preserved) |

**Interpretation:** The spec correctly rejects all behavioral mutations. The three failure branches (no endpoint, no sender, empty sender queue) and the success branch all enforce their specified state transitions.

---

## Logical Tests (7/7 FAIL ✅)

| # | Test | Unentailed Property |
|---|---|---|
| 1 | `test_logical_state_deterministic` | Determinism: two valid post-states must be equal |
| 2 | `test_logical_sender_thread_state_scheduled` | Sender thread state → SCHEDULED after success |
| 3 | `test_logical_success_return_code_is_else` | Return code must be `Else` on success |
| 4 | `test_logical_sender_pages_removed_after_receive` | Sender loses pages (share ≠ move) |
| 5 | `test_logical_free_pages_always_decrease` | Free pages strictly decrease in error paths |
| 6 | `test_logical_receiver_proc_changes` | Receiver process pcid changes |
| 7 | `test_logical_page_mapping_domain_grows` | Page mapping domain strictly grows |

**Interpretation:** The spec correctly rejects all unentailed logical properties. Notable findings:
- **Tests 1-3** reveal that the spec is deliberately under-specified: it does not guarantee determinism of the post-state, does not constrain the sender thread's final state, and does not specify the return error code. These are potential areas of spec weakness where unintended behaviors could be admitted.
- **Test 4** confirms the spec models page *sharing* (not moving).
- **Tests 5-7** confirm the spec does not entail stronger quantitative or structural claims.

---

## Conclusion

All 20 adversarial tests were **correctly rejected** by the specification, indicating the spec is consistent with respect to the tested properties. The logical tests (1-3) expose deliberate under-specification in the sender thread state, return code, and state determinism — these are areas where the spec could be strengthened if tighter guarantees are desired.
