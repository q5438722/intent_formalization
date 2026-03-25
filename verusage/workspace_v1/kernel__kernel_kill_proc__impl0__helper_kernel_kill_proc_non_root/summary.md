# Test Summary: `helper_kernel_kill_proc_non_root`

## Target
`kernel__kernel_kill_proc__impl0__helper_kernel_kill_proc_non_root.rs`

## Results: 21/21 tests FAIL as expected ✅

All adversarial tests were correctly rejected by the specification, meaning the spec is consistent with respect to the queried properties.

---

### Boundary Tests (7/7 FAIL ✅)

| # | Test | Violated Precondition | Result |
|---|------|-----------------------|--------|
| 1 | `test_boundary_proc_not_in_domain` | `proc_dom().contains(proc_ptr)` | FAIL ✅ |
| 2 | `test_boundary_proc_has_ioid` | `ioid.is_None()` | FAIL ✅ |
| 3 | `test_boundary_proc_has_threads` | `owned_threads@ == empty()` | FAIL ✅ |
| 4 | `test_boundary_proc_has_children` | `children@ == empty()` | FAIL ✅ |
| 5 | `test_boundary_proc_is_root` | `depth != 0` | FAIL ✅ |
| 6 | `test_boundary_wf_implies_nonempty_proc_dom` | N/A (stronger claim) | FAIL ✅ |
| 7 | `test_boundary_zero_proc_ptr_in_dom` | N/A (concrete value) | FAIL ✅ |

### Behavioral Mutation Tests (7/7 FAIL ✅)

| # | Test | Mutated Relation | Result |
|---|------|------------------|--------|
| 1 | `test_mutation_killed_proc_still_in_dom` | proc_ptr still in proc_dom | FAIL ✅ |
| 2 | `test_mutation_thread_dom_changed` | thread_dom lost element | FAIL ✅ |
| 3 | `test_mutation_container_dom_changed` | container_dom lost element | FAIL ✅ |
| 4 | `test_mutation_parent_children_still_has_proc` | parent still has proc_ptr | FAIL ✅ |
| 5 | `test_mutation_parent_children_len_wrong` | children.len decreased by 2 | FAIL ✅ |
| 6 | `test_mutation_container_tree_changed` | container depth changed | FAIL ✅ |
| 7 | `test_mutation_other_proc_children_changed` | non-parent proc children changed | FAIL ✅ |

### Logical Tests (7/7 FAIL ✅)

| # | Test | Unentailed Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_deterministic_page_alloc` | Deterministic page allocator state | FAIL ✅ |
| 2 | `test_logical_pcid_still_active` | Killed proc's pcid remains active | FAIL ✅ |
| 3 | `test_logical_endpoints_unchanged` | Endpoint ref counters preserved | FAIL ✅ |
| 4 | `test_logical_cpu_state_preserved` | CPU state unchanged | FAIL ✅ |
| 5 | `test_logical_scheduler_unchanged` | Scheduler state unchanged | FAIL ✅ |
| 6 | `test_logical_total_allocated_preserved` | Total allocated pages unchanged | FAIL ✅ |
| 7 | `test_logical_subtree_set_fully_unchanged` | Ancestor subtree_set fully unchanged | FAIL ✅ |

## Conclusion

The specification for `helper_kernel_kill_proc_non_root` correctly:
- **Rejects invalid inputs**: All precondition violations are caught
- **Rejects incorrect behaviors**: Mutated output relations are not entailed
- **Rejects unintended reasoning**: Stronger/unguaranteed properties are not derivable

No specification weaknesses were detected across all 21 adversarial queries.
