# Adversarial Test Execution Summary

## Target
`kernel__syscall_new_thread_with_endpoint__impl0__syscall_new_thread_with_endpoint.rs`

## Specification Under Test
- **`syscall_new_thread_with_endpoint_requirement`**: 5-condition guard (thread list not full, quota > 0, scheduler not full, free pages > 0, endpoint shareable)
- **`syscall_new_thread_with_endpoint_spec`**: If requirement fails → no change; if succeeds → new thread added to thread_dom, owned_threads, container owned_threads, endpoint owning_threads updated, quota decremented by 1, endpoint descriptors initialized

---

## Results Summary

| File | Tests | Failures (expected) | Passes (unexpected) |
|------|-------|---------------------|---------------------|
| `boundary_tests.rs` | 7 | 7 ✅ | 0 |
| `behavioral_mutation_tests.rs` | 11 | 11 ✅ | 0 |
| `logical_tests.rs` | 10 | 10 ✅ | 0 |
| **Total** | **28** | **28 ✅** | **0** |

All 28 adversarial tests were correctly **rejected** by Verus verification, indicating the specification is consistent with respect to the tested properties.

---

## Boundary Tests (7/7 failed ✅)

| # | Test | Failure Mode |
|---|------|-------------|
| 1 | `test_boundary_endpoint_index_at_max` | endpoint_index = 128 ≥ MAX (128) |
| 2 | `test_boundary_endpoint_index_overflow` | endpoint_index = usize::MAX |
| 3 | `test_boundary_zero_quota_subtract` | mem_4k = 0, subtract 1 → underflow |
| 4 | `test_boundary_quota_subtract_exceeds` | subtract 10 from mem_4k = 5 |
| 5 | `test_boundary_endpoint_index_zero_invalid` | 0 IS valid, negation fails |
| 6 | `test_boundary_thread_list_full_not_detected` | 128 threads = full, negation fails |
| 7 | `test_boundary_scheduler_full_not_detected` | 10 schedulers = full, negation fails |

## Behavioral Mutation Tests (11/11 failed ✅)

| # | Test | Mutation |
|---|------|---------|
| 1 | `test_mutation_quota_subtract_wrong_amount` | Subtracted 2 instead of 1 |
| 2 | `test_mutation_quota_mem_2m_changed` | mem_2m mutated (50→49) |
| 3 | `test_mutation_quota_pcid_changed` | pcid mutated (5→4) |
| 4 | `test_mutation_thread_dom_changes_on_failure` | Thread domain grew despite failure |
| 5 | `test_mutation_proc_dom_changes_on_success` | Proc domain grew on success |
| 6 | `test_mutation_container_dom_changes` | Container domain grew |
| 7 | `test_mutation_endpoint_dom_changes` | Endpoint domain grew |
| 8 | `test_mutation_owned_threads_wrong_push` | Pushed wrong thread ptr |
| 9 | `test_mutation_endpoint_owning_threads_wrong_idx` | Wrong descriptor index (1 vs 0) |
| 10 | `test_mutation_endpoint_descriptors_wrong_index` | Endpoint at index 1 instead of 0 |
| 11 | `test_mutation_page_mapping_changes` | Physical page mapping changed |

## Logical Tests (10/10 failed ✅)

| # | Test | Unintended Property |
|---|------|-------------------|
| 1 | `test_logical_determinism_thread_ptr` | Two different new_thread_ptrs yield same dom |
| 2 | `test_logical_stronger_quota_positive` | Quota stays positive after exact drain |
| 3 | `test_logical_thread_dom_grows_by_two` | Thread domain grew by 2 instead of 1 |
| 4 | `test_logical_endpoint_queue_state_changes` | RECEIVE =~= SEND |
| 5 | `test_logical_new_thread_wrong_container` | New thread in wrong container |
| 6 | `test_logical_endpoint_at_wrong_descriptor_index` | Endpoint at descriptor index 1 |
| 7 | `test_logical_push_removes_old` | Push removes existing element |
| 8 | `test_logical_set_insert_idempotent_grows` | Re-inserting existing element changes set |
| 9 | `test_logical_last_descriptor_not_none` | Last descriptor (127) is not None |
| 10 | `test_logical_quota_zero_subtract_grows` | Zero-subtract increases quota |

---

## Conclusion

The specification for `syscall_new_thread_with_endpoint` correctly rejects all 28 adversarial queries across boundary violations, behavioral mutations, and logical inconsistencies. The spec demonstrates:

1. **Boundary robustness**: Invalid endpoint indices, zero/exceeded quotas, and full lists are properly guarded.
2. **Behavioral precision**: Mutations to quota amounts, domain sizes, thread lists, endpoint descriptors, and page mappings are all detected.
3. **Logical consistency**: The spec rejects non-determinism claims, stronger-than-specified invariants, cross-function misuse, and structural assumption violations.

No spec weaknesses were identified in the tested property space.
