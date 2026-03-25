# Test Summary: `kernel_proc_kill_all_threads`

## Target Function

`Kernel::kernel_proc_kill_all_threads(&mut self, proc_ptr: ProcPtr)`

**Preconditions:** `old(self).wf()`, `old(self).proc_dom().contains(proc_ptr)`

**Postconditions:**
- `self.wf()` — kernel remains well-formed
- `self.proc_dom().contains(proc_ptr)` — process still exists
- `self.get_proc(proc_ptr).owned_threads.len() == 0` — all threads killed
- `self.proc_dom() == old(self).proc_dom()` — process domain unchanged
- `process_tree_unchanged(...)` — process tree structure preserved
- `self.container_dom() == old(self).container_dom()` — container domain unchanged
- `containers_tree_unchanged(...)` — container tree structure preserved

---

## Results Overview

| Category | Tests | Failed (expected) | Passed (unexpected) |
|---|---|---|---|
| Boundary | 7 | 7 | 0 |
| Behavioral Mutation | 7 | 7 | 0 |
| Logical | 7 | 7 | 0 |
| **Total** | **21** | **21** | **0** |

All 21 tests **failed verification** as intended — the specification correctly rejects all tested invalid inputs, incorrect behaviors, and unintended logical properties.

---

## Boundary Tests (7/7 FAIL ✓)

| # | Test | Failure Mode |
|---|---|---|
| 1 | `test_boundary_proc_not_in_domain` | Precondition violation: proc_ptr not in proc_dom |
| 2 | `test_boundary_kernel_not_wf` | Precondition violation: kernel not well-formed |
| 3 | `test_boundary_zero_proc_ptr_in_dom` | Edge case: 0 not necessarily a valid proc_ptr |
| 4 | `test_boundary_wf_implies_nonempty_proc_dom` | wf() does not imply non-empty proc_dom |
| 5 | `test_boundary_proc_in_thread_dom` | Domain separation: proc_dom ≠ thread_dom |
| 6 | `test_boundary_proc_container_arbitrary` | owning_container must be in container_dom |
| 7 | `test_boundary_proc_has_threads` | wf() does not imply process has threads |

## Behavioral Mutation Tests (7/7 FAIL ✓)

| # | Test | Failure Mode |
|---|---|---|
| 1 | `test_mutation_threads_not_empty_after_kill` | Mutated: owned_threads.len()==1 (should be 0) |
| 2 | `test_mutation_proc_removed_from_dom` | Mutated: proc_ptr removed from proc_dom |
| 3 | `test_mutation_container_dom_shrank` | Mutated: container removed from container_dom |
| 4 | `test_mutation_proc_dom_grew` | Mutated: extra proc added to proc_dom |
| 5 | `test_mutation_wf_violated` | Mutated: wf() is false after operation |
| 6 | `test_mutation_process_tree_changed` | Mutated: process parent set to None |
| 7 | `test_mutation_container_depth_changed` | Mutated: container depth altered |

## Logical Tests (7/7 FAIL ✓)

| # | Test | Failure Mode |
|---|---|---|
| 1 | `test_logical_determinism_thread_dom` | Determinism: two post-states have same thread_dom |
| 2 | `test_logical_thread_dom_strictly_smaller` | Stronger: thread_dom strictly shrinks |
| 3 | `test_logical_page_alloc_unchanged` | Cross-module: page allocator state preserved |
| 4 | `test_logical_scheduler_unchanged` | Unspecified: container scheduler preserved |
| 5 | `test_logical_other_proc_threads_unchanged` | Unspecified: other proc's threads unchanged |
| 6 | `test_logical_container_owned_threads_unchanged` | Unspecified: container owned_threads preserved |
| 7 | `test_logical_cpu_state_unchanged` | Unspecified: CPU state preserved |

---

## Conclusion

The specification for `kernel_proc_kill_all_threads` is **consistent** with respect to all 21 adversarial queries tested:

- **Boundary**: Invalid inputs are properly rejected by preconditions.
- **Behavioral**: Incorrect output mutations are rejected by postconditions.
- **Logical**: The spec does not entail unintended properties (determinism, stronger guarantees, cross-module invariants).

**Notable observations:**
- Logical tests 4–7 reveal that the spec intentionally leaves scheduler state, other processes' thread lists, container owned_threads, and CPU state **unspecified** — they may change. This is appropriate since `kernel_proc_kill_all_threads` internally calls `kernel_kill_thread` which may have broader side effects on scheduling and container thread tracking.
- The spec correctly uses `process_tree_unchanged` and `containers_tree_unchanged` to preserve *structural* properties (parent/children/depth/subtree) while allowing *operational* state (threads, scheduler, CPUs) to evolve.
