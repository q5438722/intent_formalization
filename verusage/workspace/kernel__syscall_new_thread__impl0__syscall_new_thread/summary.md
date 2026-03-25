# Test Summary: `syscall_new_thread`

## Target
`kernel__syscall_new_thread__impl0__syscall_new_thread.rs`

## Key Specification Under Test
```
pub fn syscall_new_thread(&mut self, thread_ptr: ThreadPtr, pt_regs: Registers) -> SyscallReturnStruct
    requires: old(self).wf(), old(self).thread_dom().contains(thread_ptr)
    ensures:  self.wf(), syscall_new_thread_requirement(*old(self), thread_ptr) == false <==> ret.is_error()
```

The `syscall_new_thread_requirement` checks four conditions: thread list not full, mem_4k quota ≥ 1, scheduler not full, free pages > 0.

---

## Results

| File | Tests | All Failed (as expected) |
|------|-------|------------------------|
| `boundary_tests.rs` | 10 | ✅ Yes (10/10 errors) |
| `behavioral_mutation_tests.rs` | 10 | ✅ Yes (10/10 errors) |
| `logical_tests.rs` | 10 | ✅ Yes (10/10 errors) |

**Total: 30/30 tests correctly FAILED verification.**

---

## Boundary Tests (10 tests)
All probe precondition violations and edge-case values:

| # | Test | Target | Result |
|---|------|--------|--------|
| 1 | `test_boundary_thread_not_in_domain` | thread_ptr not in thread_dom | ✅ FAIL |
| 2 | `test_boundary_thread_list_at_max` | owned_threads.len() == 128 | ✅ FAIL |
| 3 | `test_boundary_zero_mem_quota` | mem_4k == 0 | ✅ FAIL |
| 4 | `test_boundary_scheduler_at_max` | scheduler.len() == 10 | ✅ FAIL |
| 5 | `test_boundary_zero_free_pages` | free_pages == 0 | ✅ FAIL |
| 6 | `test_boundary_unaligned_page_ptr` | page_ptr % 0x1000 != 0 | ✅ FAIL |
| 7 | `test_boundary_page_index_at_num_pages` | index == NUM_PAGES | ✅ FAIL |
| 8 | `test_boundary_page_index_overflow` | index == usize::MAX | ✅ FAIL |
| 9 | `test_boundary_multiple_violations` | All conditions violated | ✅ FAIL |
| 10 | `test_boundary_alloc_with_empty_free_list` | free_list_len == 0 | ✅ FAIL |

## Behavioral Mutation Tests (10 tests)
All start from valid postconditions and mutate expected behavior:

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_success_returns_error` | Claim success is error | ✅ FAIL |
| 2 | `test_mutation_failure_returns_success` | Claim failure is success | ✅ FAIL |
| 3 | `test_mutation_new_thread_not_in_dom` | New thread not in thread_dom | ✅ FAIL |
| 4 | `test_mutation_proc_dom_changed` | proc_dom gained a member | ✅ FAIL |
| 5 | `test_mutation_container_dom_shrank` | container_dom lost a member | ✅ FAIL |
| 6 | `test_mutation_endpoint_dom_changed` | endpoint_dom gained a member | ✅ FAIL |
| 7 | `test_mutation_page_closure_unchanged` | page_closure didn't grow | ✅ FAIL |
| 8 | `test_mutation_no_switch_new_pcid_some` | pcid is Some after NoSwitchNew | ✅ FAIL |
| 9 | `test_mutation_page_already_allocated` | Page was already allocated | ✅ FAIL |
| 10 | `test_mutation_quota_unchanged` | mem_4k stays the same | ✅ FAIL |

## Logical Tests (10 tests)
All assert properties NOT guaranteed by the specification:

| # | Test | Unentailed Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_determinism` | Same inputs → same thread ptr | ✅ FAIL |
| 2 | `test_logical_new_thread_is_zero` | New thread ptr is always 0 | ✅ FAIL |
| 3 | `test_logical_thread_dom_grows_by_two` | Domain grows by 2 | ✅ FAIL |
| 4 | `test_logical_unrelated_proc_changed` | Unrelated proc removed | ✅ FAIL |
| 5 | `test_logical_requirement_always_true` | Requirement always holds | ✅ FAIL |
| 6 | `test_logical_success_causes_switch` | Success causes context switch | ✅ FAIL |
| 7 | `test_logical_container_pages_changed` | Container pages changed | ✅ FAIL |
| 8 | `test_logical_distinguish_failure_modes` | Failure modes distinguishable | ✅ FAIL |
| 9 | `test_logical_mem_4k_never_decreases` | mem_4k monotonically increases | ✅ FAIL |
| 10 | `test_logical_free_pages_shrink_by_two` | Free pages shrink by 2 | ✅ FAIL |

---

## Conclusion

The specification for `syscall_new_thread` correctly rejects all 30 adversarial properties across boundary violations, behavioral mutations, and unentailed logical claims. No spec weakness was found — the specification is **consistent** with respect to all tested semantic queries.
