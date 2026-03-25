# Adversarial Test Summary: `syscall_new_thread`

**Target**: `kernel__syscall_new_thread__impl0__syscall_new_thread.rs`

## Specification Under Test

The `syscall_new_thread` function creates a new thread in a kernel. Key spec:

- **Requires**: `old(self).wf()` ∧ `old(self).thread_dom().contains(thread_ptr)`
- **Ensures**: `self.wf()` ∧ `(requirement == false <==> ret.is_error())`

The `syscall_new_thread_requirement` checks 4 conditions:
1. Process thread list not full (`< 128`)
2. Container has 4k quota (`mem_4k >= 1`)
3. Scheduler not full (`< 10`)
4. Free pages available (`> 0`)

## Results Summary

| Category | Tests | All Failed? | Status |
|----------|-------|-------------|--------|
| Boundary | 7 | ✅ Yes (7/7) | PASS |
| Behavioral Mutation | 10 | ✅ Yes (10/10) | PASS |
| Logical | 10 | ✅ Yes (10/10) | PASS |
| **Combined (correctness_tests.rs)** | **27** | **✅ Yes (27/27)** | **PASS** |

All 27 adversarial tests correctly **fail verification**, confirming the spec rejects these invalid properties.

## Boundary Tests (7 tests)

| # | Property Tested | Result |
|---|----------------|--------|
| 1 | thread_ptr not in thread_dom | ✅ Rejected |
| 2 | Empty thread_dom contains element | ✅ Rejected |
| 3 | Thread list at MAX (128) is not full | ✅ Rejected |
| 4 | Zero quota satisfies mem_4k ≥ 1 | ✅ Rejected |
| 5 | Scheduler at MAX (10) is not full | ✅ Rejected |
| 6 | Zero free pages satisfies > 0 | ✅ Rejected |
| 7 | Quota subtract underflow (0 - 1 = MAX) | ✅ Rejected |

## Behavioral Mutation Tests (10 tests)

| # | Property Tested | Result |
|---|----------------|--------|
| 1 | Quota subtracted by 2 instead of 1 | ✅ Rejected |
| 2 | mem_2m mutated during subtract | ✅ Rejected |
| 3 | ioid mutated during subtract | ✅ Rejected |
| 4 | Success case returns error | ✅ Rejected |
| 5 | Failure case returns success | ✅ Rejected |
| 6 | thread_dom unchanged on success | ✅ Rejected |
| 7 | page_closure unchanged on success | ✅ Rejected |
| 8 | proc_dom mutated on success | ✅ Rejected |
| 9 | container_dom mutated on success | ✅ Rejected |
| 10 | pcid changed for owning process | ✅ Rejected |

## Logical Tests (10 tests)

| # | Property Tested | Result |
|---|----------------|--------|
| 1 | Determinism of new thread ptr | ✅ Rejected |
| 2 | Quota stays positive after subtract | ✅ Rejected |
| 3 | Thread domain grows by 2 | ✅ Rejected |
| 4 | Unrelated proc's pcid changes | ✅ Rejected |
| 5 | Free pages decrease by 2 | ✅ Rejected |
| 6 | Allocated page not in free set | ✅ Rejected |
| 7 | Free/allocated sets overlap | ✅ Rejected |
| 8 | Set insert of existing grows set | ✅ Rejected |
| 9 | wf implies positive quota | ✅ Rejected |
| 10 | Container owned pages change | ✅ Rejected |

## Conclusion

The specification correctly rejects all 27 adversarial queries across boundary violations, behavioral mutations, and logical overreach. No specification weaknesses were detected in the tested properties.
