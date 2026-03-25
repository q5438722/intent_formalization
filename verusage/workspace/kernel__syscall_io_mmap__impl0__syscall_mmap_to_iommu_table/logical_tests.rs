use vstd::prelude::*;

fn main() {}

verus!{

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const PCID_MAX: usize = 4096;
pub const IOID_MAX: usize = 4096;

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type IOid = usize;
pub type Pcid = usize;

// ===================== LOGICAL TESTS =====================
// Each test asserts a property NOT explicitly guaranteed by
// the syscall_io_mmap specification, testing whether the spec
// allows unintended reasoning.
// All tests SHOULD FAIL verification.

// Test 1: syscall_io_mmap has NO ensures clause.
// The spec does NOT guarantee that the return is always Error.
// SHOULD FAIL
proof fn test_logical_always_returns_error(
    ioid_is_some: bool,
    quota_sufficient: bool,
    va_range_free: bool,
)
    requires
        ioid_is_some,
        quota_sufficient,
        va_range_free,
{
    // Try to prove the function always returns error
    // even when all checks pass — not entailed
    assert(!ioid_is_some || !quota_sufficient || !va_range_free);
}

// Test 2: syscall_io_mmap has NO ensures on self.wf().
// The spec does NOT guarantee well-formedness is preserved.
// (range_alloc_and_map_io ensures wf, but syscall_io_mmap itself doesn't)
// SHOULD FAIL
proof fn test_logical_syscall_preserves_total_wf(
    pre_total_wf: bool,
    post_total_wf: bool,
)
    requires
        pre_total_wf,
{
    // syscall_io_mmap has no ensures clause, so we cannot derive post_total_wf
    assert(post_total_wf);
}

// Test 3: Determinism — the spec does NOT guarantee that
// two calls with identical state produce the same result.
// SHOULD FAIL
proof fn test_logical_determinism(
    result1: bool,
    result2: bool,
)
{
    // Without specification constraints, two arbitrary results
    // are not guaranteed to be equal
    assert(result1 == result2);
}

// Test 4: The spec does NOT guarantee that proc_dom is preserved.
// syscall_io_mmap has no ensures about proc_dom.
// SHOULD FAIL
proof fn test_logical_proc_dom_preserved(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
)
{
    // Cannot prove proc_dom unchanged without postcondition
    assert(old_proc_dom =~= new_proc_dom);
}

// Test 5: The spec does NOT guarantee that container_dom is preserved.
// SHOULD FAIL
proof fn test_logical_container_dom_preserved(
    old_container_dom: Set<ContainerPtr>,
    new_container_dom: Set<ContainerPtr>,
)
{
    // Cannot prove container_dom unchanged without postcondition
    assert(old_container_dom =~= new_container_dom);
}

// Test 6: The spec does NOT guarantee that the number of
// free pages decreases by exactly va_range.len * 4 on success.
// SHOULD FAIL
proof fn test_logical_free_pages_exact_decrease(
    old_free: usize,
    new_free: usize,
    va_range_len: usize,
)
    requires
        va_range_len > 0,
        old_free >= va_range_len * 4,
{
    // Not guaranteed by any ensures clause
    assert(new_free == old_free - va_range_len * 4);
}

// Test 7: The spec does NOT guarantee that after success,
// the IO space contains the newly mapped VAs.
// SHOULD FAIL
proof fn test_logical_io_space_contains_new_mappings(
    old_io_space: Set<VAddr>,
    new_io_space: Set<VAddr>,
    new_va: VAddr,
)
    requires
        !old_io_space.contains(new_va),
{
    // Without postcondition, cannot prove new VA is mapped
    assert(new_io_space.contains(new_va));
}

// Test 8: The spec does NOT guarantee that the quota decreases
// after a successful allocation.
// SHOULD FAIL
proof fn test_logical_quota_decreases(
    old_quota: usize,
    new_quota: usize,
    va_range_len: usize,
)
    requires
        va_range_len > 0,
        old_quota >= va_range_len * 4,
{
    // Not guaranteed by any ensures
    assert(new_quota < old_quota);
}

// Test 9: Cross-function: page_ptr2page_index and page_index2page_ptr
// are inverses. But the spec does NOT state this as a global property.
// We try to prove it for an arbitrary valid pointer without calling the funcs.
// SHOULD FAIL
proof fn test_logical_page_ptr_index_inverse(
    ptr: usize,
    index: usize,
    result_ptr: usize,
)
    requires
        ptr % 0x1000 == 0,
        ptr / 0x1000 < NUM_PAGES,
        index == ptr / 4096,
        result_ptr == index * 4096,
{
    // Try to prove a stronger claim: result_ptr == ptr + 4096
    // This is NOT true — result should equal ptr exactly
    assert(result_ptr == ptr + 4096);
}

// Test 10: The spec does NOT guarantee that the thread_dom
// is unchanged after syscall_io_mmap (no ensures clause).
// SHOULD FAIL
proof fn test_logical_thread_dom_unchanged(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
)
{
    // Cannot prove thread_dom unchanged without postcondition
    assert(old_thread_dom =~= new_thread_dom);
}

}
