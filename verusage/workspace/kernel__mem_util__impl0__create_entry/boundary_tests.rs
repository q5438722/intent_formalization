use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type PageMapPtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type ThreadPtr = usize;
pub type EndpointPtr = usize;
pub type Pcid = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;

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

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) of create_entry
// or uses edge-case values. All tests SHOULD FAIL verification.

// Test 1: create_entry requires old(self).proc_dom().contains(proc_ptr).
// If proc_ptr is NOT in proc_dom, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_proc_not_in_domain(
    proc_dom: Set<ProcPtr>,
    proc_ptr: ProcPtr,
)
    requires
        !proc_dom.contains(proc_ptr),
{
    assert(proc_dom.contains(proc_ptr));
}

// Test 2: create_entry requires mem_4k quota >= 3.
// If mem_4k == 2, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_insufficient_quota() {
    let mem_4k: usize = 2;
    assert(mem_4k >= 3);
}

// Test 3: create_entry requires get_num_of_free_pages() >= 3.
// Having only 2 free pages violates this.
// SHOULD FAIL
proof fn test_boundary_insufficient_free_pages() {
    let free_pages: usize = 2;
    assert(free_pages >= 3);
}

// Test 4: create_entry requires va_4k_valid(va).
// va = 0 is a kernel address (L4 index < KERNEL_MEM_END_L4INDEX), so not valid.
// SHOULD FAIL
proof fn test_boundary_va_zero_not_valid() {
    let va: usize = 0;
    assert(spec_va_4k_valid(va));
}

// Test 5: create_entry requires the va is NOT already in the address space.
// If address space already contains the va, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_va_already_mapped(
    addr_space_dom: Set<VAddr>,
    target_va: VAddr,
)
    requires
        addr_space_dom.contains(target_va),
{
    assert(addr_space_dom.contains(target_va) == false);
}

// Test 6: create_entry requires mem_4k quota >= 3.
// Edge case: quota == 0 (empty container).
// SHOULD FAIL
proof fn test_boundary_zero_quota() {
    let mem_4k: usize = 0;
    assert(mem_4k >= 3);
}

// Test 7: create_entry requires get_num_of_free_pages() >= 3.
// Edge case: 0 free pages.
// SHOULD FAIL
proof fn test_boundary_zero_free_pages() {
    let free_pages: usize = 0;
    assert(free_pages >= 3);
}

// Test 8: create_entry requires va_4k_valid(va).
// A misaligned VA (not 4k-aligned) should fail the validity check.
// SHOULD FAIL
proof fn test_boundary_va_not_aligned() {
    let va: usize = 0x1001;
    assert(spec_va_4k_valid(va));
}

// Test 9: create_entry ensures ret.0 <= 3.
// If we assume ret.0 == 4, this violates the postcondition bound.
// SHOULD FAIL
proof fn test_boundary_ret_exceeds_max(ret0: usize)
    requires
        ret0 <= 3,
{
    assert(ret0 == 4);
}

// Test 10: create_entry requires both quota >= 3 AND free_pages >= 3.
// If only one holds (quota=3 but free=1), the combined requirement is not met.
// SHOULD FAIL
proof fn test_boundary_quota_ok_free_not() {
    let mem_4k: usize = 3;
    let free_pages: usize = 1;
    assert(mem_4k >= 3 && free_pages >= 3);
}

// Test 11: page_ptr2page_index requires ptr % 0x1000 == 0.
// Passing an odd pointer violates this.
// SHOULD FAIL
proof fn test_boundary_page_ptr_not_aligned() {
    let ptr: usize = 7;
    assert(ptr % 0x1000 == 0);
}

// Test 12: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// Passing i = NUM_PAGES is out of range.
// SHOULD FAIL
proof fn test_boundary_page_index_at_max() {
    let i: usize = NUM_PAGES;
    assert(0 <= i && i < NUM_PAGES);
}

}
