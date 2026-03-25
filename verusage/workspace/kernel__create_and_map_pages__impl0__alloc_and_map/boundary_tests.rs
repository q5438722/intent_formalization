use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;
pub type PageMapPtr = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) or
// uses edge-case values. All tests SHOULD FAIL verification.

// Test 1: alloc_and_map requires get_num_of_free_pages() >= 1.
// With 0 free pages, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_zero_free_pages() {
    let free_pages: usize = 0;
    assert(free_pages >= 1);
}

// Test 2: alloc_and_map requires proc_dom().contains(target_proc_ptr).
// If the proc is not in the domain, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_proc_not_in_domain(
    proc_dom: Set<ProcPtr>,
    target_proc_ptr: ProcPtr,
)
    requires
        !proc_dom.contains(target_proc_ptr),
{
    assert(proc_dom.contains(target_proc_ptr));
}

// Test 3: alloc_and_map requires mem_4k >= 1 for the owning container.
// With quota.mem_4k == 0, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_zero_quota() {
    let mem_4k: usize = 0;
    assert(mem_4k >= 1);
}

// Test 4: alloc_and_map requires va_4k_valid(target_va).
// VA = 0 is in kernel address space and should not be valid.
// SHOULD FAIL
proof fn test_boundary_va_zero_not_4k_valid() {
    let va: usize = 0;
    assert(spec_va_4k_valid(va));
}

// Test 5: alloc_and_map requires the target VA is NOT already in the
// address space. If it IS present, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_va_already_in_address_space(
    addr_space_dom: Set<VAddr>,
    target_va: VAddr,
)
    requires
        addr_space_dom.contains(target_va),
{
    assert(addr_space_dom.contains(target_va) == false);
}

// Test 6: page_ptr2page_index requires ptr % 0x1000 == 0.
// Passing ptr = 1 (not 4k-aligned) violates the precondition.
// SHOULD FAIL
proof fn test_boundary_page_ptr_not_aligned() {
    let ptr: usize = 1;
    assert(ptr % 0x1000 == 0);
}

// Test 7: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// Passing i = NUM_PAGES is out of range.
// SHOULD FAIL
proof fn test_boundary_page_index_at_max() {
    let i: usize = NUM_PAGES;
    assert(0 <= i && i < NUM_PAGES);
}

// Test 8: pagetable_map_4k_page requires KERNEL_MEM_END_L4INDEX <= target_l4i < 512.
// Using target_l4i = 0 (< KERNEL_MEM_END_L4INDEX) violates the lower bound.
// SHOULD FAIL
proof fn test_boundary_l4_index_below_kernel_end() {
    let l4i: L4Index = 0;
    assert(KERNEL_MEM_END_L4INDEX <= l4i && l4i < 512);
}

// Test 9: pagetable_map_4k_page requires 0 <= target_l1i < 512.
// Using target_l1i = 512 is out of range.
// SHOULD FAIL
proof fn test_boundary_l1_index_at_512() {
    let l1i: L1Index = 512;
    assert(0 <= l1i && l1i < 512);
}

// Test 10: pagetable_map_4k_page requires page_ptr_valid(target_entry.addr).
// A page_ptr that is not 4k-aligned is invalid.
// SHOULD FAIL
proof fn test_boundary_page_ptr_unaligned_for_entry() {
    let addr: usize = 0x1001;
    assert(page_ptr_valid(addr));
}

// Test 11: alloc_and_map_4k requires free_pages_4k.len() > 0.
// With length 0, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_alloc_map_4k_empty_free_list() {
    let free_len: usize = 0;
    assert(free_len > 0);
}

// Test 12: pagetable_map_4k_page requires page not already in page_closure.
// If the page is already in the closure, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_page_already_in_closure(
    page_closure: Set<PagePtr>,
    page_ptr: PagePtr,
)
    requires
        page_closure.contains(page_ptr),
{
    assert(page_closure.contains(page_ptr) == false);
}

}
