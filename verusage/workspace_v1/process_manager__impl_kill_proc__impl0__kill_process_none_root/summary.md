# Test Summary: `kill_process_none_root`

## Target
`process_manager__impl_kill_proc__impl0__kill_process_none_root.rs`

Function `kill_process_none_root` removes a non-root process (with no threads/children) from the ProcessManager, returning its page.

---

## Results: All 21 tests FAILED verification ✅

All adversarial tests were correctly rejected, indicating the specification is **consistent** with respect to the queried properties.

### Boundary Tests (7/7 FAILED ✅)

| # | Test | Violated Precondition | Result |
|---|------|----------------------|--------|
| 1 | `test_boundary_proc_not_in_domain` | `proc_dom().contains(proc_ptr)` | FAIL ✅ |
| 2 | `test_boundary_proc_has_threads` | `owned_threads@ == empty()` | FAIL ✅ |
| 3 | `test_boundary_proc_has_children` | `children@ == empty()` | FAIL ✅ |
| 4 | `test_boundary_proc_is_root` | `depth != 0` | FAIL ✅ |
| 5 | `test_boundary_wf_implies_nonempty_proc_dom` | N/A (tests wf strength) | FAIL ✅ |
| 6 | `test_boundary_zero_proc_ptr_in_dom` | N/A (tests wf strength) | FAIL ✅ |
| 7 | `test_boundary_depth_nonzero_implies_parent` | N/A (tests wf → parent) | FAIL ✅ |

**Conclusion**: Preconditions are necessary — removing any one prevents reasoning about postconditions.

### Behavioral Mutation Tests (7/7 FAILED ✅)

| # | Test | Mutated Property | Result |
|---|------|-----------------|--------|
| 1 | `test_mutation_proc_still_in_domain` | proc_ptr still in proc_dom | FAIL ✅ |
| 2 | `test_mutation_container_dom_changed` | container removed | FAIL ✅ |
| 3 | `test_mutation_thread_dom_changed` | thread removed | FAIL ✅ |
| 4 | `test_mutation_ret_page_not_in_closure` | ret page not in closure | FAIL ✅ |
| 5 | `test_mutation_ret_page_is_container` | ret page is container | FAIL ✅ |
| 6 | `test_mutation_parent_children_unchanged` | parent children length same | FAIL ✅ |
| 7 | `test_mutation_page_closure_grew` | extra page appeared | FAIL ✅ |

**Conclusion**: The spec correctly rejects all behavioral mutations — incorrect outputs are not derivable.

### Logical Tests (7/7 FAILED ✅)

| # | Test | Unintended Property | Result |
|---|------|-------------------|--------|
| 1 | `test_logical_ret_equals_proc_ptr` | ret_page == proc_ptr (determinism) | FAIL ✅ |
| 2 | `test_logical_owning_container_removed` | owning container removed | FAIL ✅ |
| 3 | `test_logical_root_container_unchanged` | root_container field preserved | FAIL ✅ |
| 4 | `test_logical_endpoint_dom_unchanged` | endpoint_dom preserved | FAIL ✅ |
| 5 | `test_logical_cpu_list_unchanged` | cpu_list preserved | FAIL ✅ |
| 6 | `test_logical_owning_container_preserved` | owning_container in processes_fields_unchanged | FAIL ✅ |
| 7 | `test_logical_container_scheduler_unchanged` | scheduler in containers_tree_unchanged | FAIL ✅ |

**Conclusion**: The spec does not entail unintended logical consequences.

### Notable Findings

- **Logical tests 3-5** reveal that `root_container`, `endpoint_perms.dom()`, and `cpu_list` are NOT explicitly preserved by postconditions. While the spec rejects reasoning about these (good), a client of this function cannot assume these are unchanged without additional guarantees. This is a potential **spec gap** — the postconditions may be weaker than intended regarding these fields.
- **Logical test 6** shows `processes_fields_unchanged` does NOT include `owning_container`, so callers cannot derive it's preserved (though it likely should be since processes stay in the same container).
- **Logical test 7** shows `containers_tree_unchanged` only covers tree-structural fields, not `scheduler` or `owned_threads`. This is intentional but means callers must rely on `wf()` to reason about these.

Overall, the specification is **consistent**: it rejects all tested invalid inputs, incorrect behaviors, and unintended logical inferences.
