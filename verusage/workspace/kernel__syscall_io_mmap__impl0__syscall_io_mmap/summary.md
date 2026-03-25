# Test Summary: `kernel__syscall_io_mmap__impl0__syscall_io_mmap`

## Target Function
`syscall_io_mmap(&mut self, thread_ptr: ThreadPtr, va_range: VaRange4K) -> SyscallReturnStruct`

## Key Specification Observations
- **`syscall_io_mmap` has NO `ensures` clause** — this is the most significant spec weakness. The function's postconditions are entirely unspecified, meaning nothing about the return value or post-state can be verified.
- `range_alloc_and_map_io` ensures only `self.wf()` — no guarantees about the returned page count or allocated pages.
- `NoSwitchNew` has complete field-level ensures (error_code, pcid, cr3, switch_decision).
- `page_ptr2page_index` / `page_index2page_ptr` have precondition-guarded specs but no result bounding.

---

## Results

### Boundary Tests (10/10 FAILED ✓)
All precondition violations were correctly rejected.

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `page_ptr2page_index(1)` | Unaligned ptr (requires `ptr % 0x1000 == 0`) | FAIL ✓ |
| 2 | `page_ptr2page_index(0x1001)` | Odd offset ptr | FAIL ✓ |
| 3 | `page_index2page_ptr(NUM_PAGES)` | Index at upper bound (requires `i < NUM_PAGES`) | FAIL ✓ |
| 4 | `page_index2page_ptr(usize::MAX)` | Extreme out-of-range index | FAIL ✓ |
| 5 | Thread not in empty domain | `thread_dom().contains(thread_ptr)` violated | FAIL ✓ |
| 6 | `va_range.len = usize::MAX` | `len * 4 < usize::MAX` overflows | FAIL ✓ |
| 7 | Proc not in empty domain | `proc_dom().contains(proc_ptr)` violated | FAIL ✓ |
| 8 | Quota = 0, len = 1 | `quota.mem_4k >= 4 * len` violated | FAIL ✓ |
| 9 | Free pages = 3, len = 1 | `free_pages >= 4 * len` violated | FAIL ✓ |
| 10 | Start near `usize::MAX` | `start + len * 4096 < usize::MAX` violated | FAIL ✓ |

### Behavioral Mutation Tests (10/10 FAILED ✓)
All mutated output relations were correctly rejected.

| # | Test | Mutation | Result |
|---|------|---------|--------|
| 1 | `NoSwitchNew(ErrorNoQuota)` | Assert error_code is `Error` instead | FAIL ✓ |
| 2 | `NoSwitchNew(Error)` | Assert switch_decision is `Switch` | FAIL ✓ |
| 3 | `NoSwitchNew(Error)` | Assert pcid is Some | FAIL ✓ |
| 4 | `NoSwitchNew(Error)` | Assert cr3 is Some | FAIL ✓ |
| 5 | `spec_page_ptr2page_index(4096)` | Assert result is 2 (not 1) | FAIL ✓ |
| 6 | `spec_page_index2page_ptr(1)` | Assert result is 8192 (not 4096) | FAIL ✓ |
| 7 | `spec_usize2page_entry_perm(0)` | Assert present is true (not false) | FAIL ✓ |
| 8 | `spec_usize2pa(0)` | Assert result is nonzero | FAIL ✓ |
| 9 | Empty io_space domain | Assert a VA is contained | FAIL ✓ |
| 10 | `spec_va_4k_valid(0)` | Assert VA 0 is valid (it isn't) | FAIL ✓ |

### Logical Tests (10/10 FAILED ✓)
All unguaranteed properties were correctly rejected.

| # | Test | Unguaranteed Property | Result |
|---|------|----------------------|--------|
| 1 | Arbitrary switch decision | syscall return's switch_decision (no ensures) | FAIL ✓ |
| 2 | Arbitrary error code | syscall return's error_code when ioid is None (no ensures) | FAIL ✓ |
| 3 | `page_ptr2page_index` bounded | Result `< NUM_PAGES` (not guaranteed) | FAIL ✓ |
| 4 | `page_index2page_ptr` valid | Result satisfies `page_ptr_valid` (not guaranteed) | FAIL ✓ |
| 5 | `page_ptr2page_index` injective | Different ptrs → different indices (not for unaligned) | FAIL ✓ |
| 6 | `range_alloc_and_map_io` count | Returned count = va_range.len (not guaranteed) | FAIL ✓ |
| 7 | syscall preserves wf | Post-state is wf (no ensures on syscall_io_mmap) | FAIL ✓ |
| 8 | IO space violation at index 0 | Specific collision index (existential, not constructive) | FAIL ✓ |
| 9 | NoSwitchNew distinguishability | Different inputs ⇒ different results (opaque Ghost fields) | FAIL ✓ |
| 10 | Quota sufficiency for any len | `quota = 4` enough for arbitrary len | FAIL ✓ |

---

## Spec Weakness Analysis

### Critical: Missing `ensures` on `syscall_io_mmap`
The main function has **no postcondition**. The spec cannot prove:
- What error code is returned in any case
- That the kernel remains well-formed after the call
- That `NoSwitch` is always the switch decision
- Any relationship between the return value and the input parameters

### Notable: Weak `ensures` on `range_alloc_and_map_io`
Only guarantees `self.wf()`. Does not specify:
- How many pages were allocated
- That the IO mapping was actually created
- Quota reduction

### Sound: `NoSwitchNew`, `page_ptr2page_index`, `page_index2page_ptr`, `va_4k_valid`
These functions have complete specs that correctly reject all mutations and admit no unintended reasoning.
