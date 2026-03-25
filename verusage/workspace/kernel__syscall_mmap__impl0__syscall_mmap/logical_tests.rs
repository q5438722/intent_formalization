use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type EndpointPtr = usize;
pub type VAddr = usize;

// ===================== LOGICAL TESTS =====================
// Each test generates properties NOT explicitly guaranteed
// by the syscall_mmap specification: determinism, stronger
// inequalities, structural assumptions, cross-function misuse.
// All tests SHOULD FAIL verification.

// Test 1: Determinism - the spec does not guarantee that two calls
// to syscall_mmap with the same inputs produce the same mapped pages.
// The physical pages allocated are chosen nondeterministically.
// SHOULD FAIL
proof fn test_logical_determinism_of_mapped_pages(
    pages_run1: Seq<PagePtr>,
    pages_run2: Seq<PagePtr>,
)
    requires
        pages_run1.len() == 5,
        pages_run2.len() == 5,
        forall|i: int| 0 <= i < 5 ==> pages_run1[i] != 0,
        forall|i: int| 0 <= i < 5 ==> pages_run2[i] != 0,
{
    // The spec does not enforce deterministic page selection
    assert(pages_run1 =~= pages_run2);
}

// Test 2: Stronger inequality - the spec says free pages decrease by
// ret.0, but does NOT guarantee free pages decrease by exactly va_range.len.
// The actual decrease could be more (e.g., page table pages allocated).
// SHOULD FAIL
proof fn test_logical_free_pages_decrease_exact(
    old_free: usize,
    new_free: usize,
    va_range_len: usize,
    num_page: usize,
)
    requires
        new_free == old_free - num_page,
        va_range_len > 0,
        num_page >= va_range_len,
{
    // The spec does not guarantee num_page == va_range_len
    assert(num_page == va_range_len);
}

// Test 3: The spec does not explicitly guarantee newly allocated pages
// are distinct from each other (uniqueness of mmapped_physical_pages_seq).
// Although va_range has no_duplicates, the pages could theoretically overlap.
// SHOULD FAIL
proof fn test_logical_allocated_pages_unique(
    pages: Seq<PagePtr>,
)
    requires
        pages.len() == 3,
{
    // Spec does not explicitly assert pages.no_duplicates()
    assert(pages[0] != pages[1]);
}

// Test 4: The spec does not guarantee the mapped pages are contiguous
// in physical memory. Asserting contiguity is an unwarranted assumption.
// SHOULD FAIL
proof fn test_logical_pages_contiguous(
    pages: Seq<PagePtr>,
)
    requires
        pages.len() >= 2,
{
    // No contiguity guarantee in the spec
    assert(pages[1] == pages[0] + 4096);
}

// Test 5: The spec guarantees that containers other than the owning one
// are unchanged, but does NOT explicitly state the owning container's
// quota decreased by EXACTLY va_range.len * 4. The spec uses
// spec_subtract_mem_4k(new_quota, ret.0) which may differ.
// SHOULD FAIL
proof fn test_logical_quota_decrease_equals_va_range_len(
    old_quota_mem_4k: usize,
    new_quota_mem_4k: usize,
    va_range_len: usize,
    num_page: usize,
)
    requires
        old_quota_mem_4k - num_page == new_quota_mem_4k,
        va_range_len > 0,
        num_page >= va_range_len,
{
    // Spec does not guarantee num_page == va_range_len * 4
    // (page table pages may also be allocated)
    assert(old_quota_mem_4k - va_range_len == new_quota_mem_4k);
}

// Test 6: Cross-function misuse - using page_ptr2page_index result
// as a page_index2page_ptr input without verifying bounds.
// page_ptr2page_index requires ptr % 0x1000 == 0 but page_index2page_ptr
// requires i < NUM_PAGES. Neither guarantees the other's precondition.
// SHOULD FAIL
proof fn test_logical_ptr_index_roundtrip_without_bounds(
    ptr: usize,
)
    requires
        ptr % 0x1000 == 0,
{
    // page_ptr2page_index gives ptr / 4096, but we can't assert it's < NUM_PAGES
    let index = ptr / 4096usize;
    assert(index < 2 * 1024 * 1024);
}

// Test 7: The spec does not guarantee that the address space for
// the target process after mmap has ONLY the old entries plus
// the new va_range entries. There could be other entries (from
// concurrent operations or spec gaps).
// SHOULD FAIL
proof fn test_logical_address_space_exact_composition(
    old_addr_dom: Set<VAddr>,
    new_addr_dom: Set<VAddr>,
    va_range_set: Set<VAddr>,
    extra_va: VAddr,
)
    requires
        // The spec guarantees: outside va_range, dom is same; inside va_range, dom contains va
        forall|va: VAddr| !va_range_set.contains(va) ==> new_addr_dom.contains(va) == old_addr_dom.contains(va),
        forall|va: VAddr| va_range_set.contains(va) ==> new_addr_dom.contains(va),
        !old_addr_dom.contains(extra_va),
        !va_range_set.contains(extra_va),
{
    // The spec does NOT say new_addr_dom == old_addr_dom + va_range_set exactly
    // It doesn't prevent extra entries appearing from other sources
    // But with the preservation clause above, extra_va is preserved as not in dom
    // Actually this should fail because we're asserting something the spec preserves
    assert(new_addr_dom.contains(extra_va));
}

// Test 8: The spec does not guarantee that on ErrorVaInUse,
// the specific conflicting VA is identified. It only says
// address_space_range_free == false. Cannot infer which VA conflicts.
// SHOULD FAIL
proof fn test_logical_identify_conflicting_va(
    addr_dom: Set<VAddr>,
    va_range: Seq<VAddr>,
)
    requires
        va_range.len() == 3,
        // range is NOT free: at least one VA is in domain
        exists|j: int| 0 <= j < 3 && addr_dom.contains(va_range[j]),
{
    // Cannot conclude which specific VA conflicts
    assert(addr_dom.contains(va_range[0]));
}

// Test 9: The spec does not guarantee that all newly mapped pages
// have zero content. The spec tracks mappings but not page content.
// SHOULD FAIL
proof fn test_logical_new_pages_zeroed(
    page_content: Seq<u8>,
)
    requires
        page_content.len() == 4096,
{
    // No content guarantee in the spec
    assert(page_content[0] == 0);
}

// Test 10: The spec does not give an upper bound on how many pages
// are consumed (ret.0). It could consume more than va_range.len
// pages due to page table overhead. Cannot assume tight bound.
// SHOULD FAIL
proof fn test_logical_pages_consumed_upper_bound(
    num_page: usize,
    va_range_len: usize,
)
    requires
        va_range_len == 10,
        num_page > va_range_len,
{
    // The spec allows num_page > va_range_len (page table pages)
    // Cannot assert a tight upper bound
    assert(num_page <= va_range_len);
}

}
