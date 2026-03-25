# Adversarial Proof Test Summary: `kernel_kill_thread`

## Target
`kernel__kernel_kill_thread__impl0__kernel_kill_thread.rs` — the `kernel_kill_thread` function in the Atmosphere verified kernel.

## Specification Under Test

**Preconditions:**
- `old(self).wf()` — kernel is well-formed
- `old(self).thread_dom().contains(thread_ptr)` — thread exists

**Postconditions:**
- `self.wf()` — kernel remains well-formed
- `self.thread_dom() == old(self).thread_dom().remove(thread_ptr)` — thread removed
- `threads_unchanged_except(old(self).proc_man, self.proc_man, set![])` — all surviving threads unchanged
- `self.proc_dom() == old(self).proc_dom()` — process domain preserved
- `process_tree_unchanged(...)` — process tree preserved
- `self.container_dom() == old(self).container_dom()` — container domain preserved
- `containers_tree_unchanged(...)` — container tree preserved
- Owning process's `owned_threads` updated with `remove_value(thread_ptr)`, length decreases by 1

---

## Results Summary

| Category | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary | 7 | 7 | 0 |
| Behavioral Mutation | 7 | 7 | 0 |
| Logical | 7 | 7 | 0 |
| **Total** | **21** | **21** | **0** |

**All 21 adversarial tests correctly FAIL verification**, confirming the specification rejects:
- Invalid inputs (boundary violations)
- Incorrect output relations (behavioral mutations)
- Unintended logical entailments (stronger-than-specified properties)

---

## Boundary Tests (7/7 FAIL ✓)

| # | Test | Failure Mode |
|---|------|-------------|
| 1 | `test_boundary_thread_not_in_domain` | Thread not in domain → can't derive proc membership |
| 2 | `test_boundary_wf_implies_nonempty_thread_dom` | wf() does not guarantee non-empty thread_dom |
| 3 | `test_boundary_zero_thread_ptr_in_dom` | Zero pointer not necessarily in thread_dom |
| 4 | `test_boundary_edp_idx_at_max` | Off-by-one: endpoint index at MAX is out of bounds |
| 5 | `test_boundary_thread_in_container_dom` | Memory disjointness: thread_dom ∩ container_dom = ∅ |
| 6 | `test_boundary_thread_in_proc_dom` | Memory disjointness: thread_dom ∩ proc_dom = ∅ |
| 7 | `test_boundary_thread_owning_proc_arbitrary` | Thread's owning_proc must be in proc_dom |

## Behavioral Mutation Tests (7/7 FAIL ✓)

| # | Test | Failure Mode |
|---|------|-------------|
| 1 | `test_mutation_killed_thread_still_in_dom` | Killed thread removed from thread_dom |
| 2 | `test_mutation_proc_dom_changed` | proc_dom is preserved (no new processes) |
| 3 | `test_mutation_container_dom_changed` | container_dom is preserved (no containers lost) |
| 4 | `test_mutation_owned_threads_len_wrong` | owned_threads.len() decreases by exactly 1, not 2 |
| 5 | `test_mutation_other_thread_changed` | Other threads unchanged (threads_unchanged_except) |
| 6 | `test_mutation_process_tree_changed` | Process tree structure preserved |
| 7 | `test_mutation_container_tree_changed` | Container tree structure preserved |

## Logical Tests (7/7 FAIL ✓)

| # | Test | Failure Mode |
|---|------|-------------|
| 1 | `test_logical_deterministic_page_closure` | Page allocator state not deterministic from spec |
| 2 | `test_logical_mem_man_unchanged` | Memory manager page_closure not guaranteed unchanged |
| 3 | `test_logical_endpoints_unchanged` | Endpoint state may change (queue removal for blocked) |
| 4 | `test_logical_cpu_state_preserved` | CPU state not guaranteed identical |
| 5 | `test_logical_scheduler_unchanged` | Scheduler state not guaranteed preserved |
| 6 | `test_logical_total_allocated_preserved` | Allocated pages change (freed back to allocator) |
| 7 | `test_logical_wrong_state_postcondition` | Cannot derive SCHEDULED from RUNNING precondition |

---

## Conclusion

The specification for `kernel_kill_thread` is **consistent** with respect to all 21 adversarial queries:
- It correctly **rejects invalid inputs** at the precondition boundary.
- It correctly **rejects mutated outputs** that violate postcondition guarantees.
- It correctly **does not entail** stronger logical properties beyond what is specified.

No spec weaknesses were detected. The specification precisely constrains the semantic boundary of `kernel_kill_thread`.
