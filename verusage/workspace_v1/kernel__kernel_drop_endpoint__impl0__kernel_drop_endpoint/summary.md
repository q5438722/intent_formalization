# Test Summary: `kernel_drop_endpoint`

## Target
`kernel__kernel_drop_endpoint__impl0__kernel_drop_endpoint.rs` — drops an endpoint descriptor from a thread in the Atmosphere verified kernel.

## Results

| File | Tests | All Failed (as expected) |
|------|-------|------------------------|
| `boundary_tests.rs` | 7 | ✅ 7/7 |
| `behavioral_mutation_tests.rs` | 7 | ✅ 7/7 |
| `logical_tests.rs` | 7 | ✅ 7/7 |
| **Total** | **21** | **✅ 21/21** |

All 21 tests correctly fail verification, confirming that the specification rejects invalid inputs, incorrect behaviors, and unintended logical inferences.

---

## Boundary Tests (7/7 FAIL ✅)

| # | Test | Failure Mode |
|---|------|-------------|
| 1 | `test_boundary_thread_not_in_domain` | Access thread data for thread not in `thread_dom` |
| 2 | `test_boundary_edp_idx_at_max` | Access `endpoint_descriptors[128]` (off-by-one, max index is 127) |
| 3 | `test_boundary_wf_nonempty_threads` | `wf()` does not imply `thread_dom` is non-empty |
| 4 | `test_boundary_zero_thread_in_dom` | `wf()` does not imply `0` is in `thread_dom` |
| 5 | `test_boundary_edp_idx_huge` | Access `endpoint_descriptors[228]` (far out of bounds) |
| 6 | `test_boundary_thread_in_container_dom` | Thread ptr in `thread_dom` does not imply membership in `container_dom` |
| 7 | `test_boundary_thread_in_endpoint_dom` | Thread ptr in `thread_dom` does not imply membership in `endpoint_dom` |

## Behavioral Mutation Tests (7/7 FAIL ✅)

| # | Test | Mutation |
|---|------|---------|
| 1 | `test_mutation_descriptor_stays_some` | Assert dropped descriptor remains `Some` (should be `None`) |
| 2 | `test_mutation_thread_state_changes` | Assert thread state changed to `RUNNING` (should be preserved) |
| 3 | `test_mutation_thread_removed` | Assert thread removed from `thread_dom` (should be preserved) |
| 4 | `test_mutation_proc_dom_changes` | Assert `proc_dom` size changed (should be preserved) |
| 5 | `test_mutation_owning_proc_changes` | Assert `owning_proc` changed to arbitrary value (should be preserved) |
| 6 | `test_mutation_other_thread_changed` | Assert non-target thread's descriptors changed (should be unchanged) |
| 7 | `test_mutation_container_removed` | Assert container removed from `container_dom` (should be preserved) |

## Logical Tests (7/7 FAIL ✅)

| # | Test | Unintended Property |
|---|------|-------------------|
| 1 | `test_logical_determinism` | Two independent post-states have identical `allocated_pages_4k` |
| 2 | `test_logical_endpoint_dom_shrinks` | `endpoint_dom` strictly shrinks (not guaranteed) |
| 3 | `test_logical_blocking_cleared` | `blocking_endpoint_index` becomes `None` (spec says preserved, not cleared) |
| 4 | `test_logical_all_descriptors_cleared` | All descriptors become `None` (only the targeted one does) |
| 5 | `test_logical_page_closure_strict_subset` | `page_closure` strictly shrinks (can be equal when `ret.is_None()`) |
| 6 | `test_logical_ipc_payload_preserved` | `ipc_payload` is preserved (not stated in postconditions — **spec gap**) |
| 7 | `test_logical_mem_man_unchanged` | `mem_man.page_closure()` unchanged (not stated in postconditions — **spec gap**) |

## Spec Gaps Identified

Two logical tests (6 & 7) reveal potential **spec incompleteness** — the postconditions of `kernel_drop_endpoint` do not explicitly state whether `ipc_payload` or `mem_man.page_closure()` are preserved or modified. These tests correctly fail (the spec does not entail these properties), but a developer may intend them to hold. These are candidates for spec strengthening.
