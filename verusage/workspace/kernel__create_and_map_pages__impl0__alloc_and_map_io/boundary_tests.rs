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
pub type ThreadPtr = usize;
pub type EndpointPtr = usize;

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

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Quota {
    pub mem_4k: usize,
    pub mem_2m: usize,
    pub mem_1g: usize,
    pub pcid: usize,
    pub ioid: usize,
}

impl Quota {
    pub open spec fn spec_subtract_mem_4k(&self, new: Self, k: usize) -> bool {
        &&& self.mem_4k - k == new.mem_4k
        &&& self.mem_2m == new.mem_2m
        &&& self.mem_1g == new.mem_1g
        &&& self.pcid == new.pcid
        &&& self.ioid == new.ioid
    }
}

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) or uses
// edge-case values. All tests SHOULD FAIL verification.

// Test 1: alloc_and_map_io requires get_num_of_free_pages() >= 1.
// Zero free pages violates the precondition.
// SHOULD FAIL
proof fn test_boundary_zero_free_pages() {
    let free_pages: usize = 0;
    assert(free_pages >= 1);
}

// Test 2: alloc_and_map_io requires proc_dom().contains(target_proc_ptr).
// If proc is NOT in the domain, precondition is violated.
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

// Test 3: alloc_and_map_io requires container quota mem_4k >= 1.
// With mem_4k = 0, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_zero_container_quota() {
    let mem_4k: usize = 0;
    assert(mem_4k >= 1);
}

// Test 4: alloc_and_map_io requires va_4k_valid(target_va).
// VA = 0 is in kernel address space (L4 index 0 < KERNEL_MEM_END_L4INDEX).
// SHOULD FAIL
proof fn test_boundary_va_zero_not_valid() {
    assert(spec_va_4k_valid(0usize));
}

// Test 5: alloc_and_map_io requires va_4k_valid(target_va).
// VA = 1 is not 4k-aligned.
// SHOULD FAIL
proof fn test_boundary_va_not_aligned() {
    assert(spec_va_4k_valid(1usize));
}

// Test 6: alloc_and_map_io requires target_va NOT already in IO space.
// If target_va is already mapped, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_va_already_in_io_space(
    io_space_dom: Set<VAddr>,
    target_va: VAddr,
)
    requires
        io_space_dom.contains(target_va),
{
    assert(io_space_dom.contains(target_va) == false);
}

// Test 7: alloc_and_map_io requires get_proc_has_iommu_table(target_proc_ptr).
// A process without an IOMMU table (ioid == None) violates precondition.
// SHOULD FAIL
proof fn test_boundary_proc_no_iommu_table() {
    let ioid: Option<IOid> = None;
    assert(ioid.is_Some());
}

// Test 8: page_ptr2page_index requires ptr % 0x1000 == 0.
// Passing ptr = 1 (not 4k-aligned) violates precondition.
// SHOULD FAIL
proof fn test_boundary_page_ptr_not_aligned() {
    let ptr: usize = 1;
    assert(ptr % 0x1000 == 0);
}

// Test 9: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// Passing i = NUM_PAGES is out of range.
// SHOULD FAIL
proof fn test_boundary_page_index_at_max() {
    let i: usize = NUM_PAGES;
    assert(0 <= i && i < NUM_PAGES);
}

// Test 10: va2index requires va_4k_valid(va) || va_2m_valid(va) || va_1g_valid(va).
// A completely invalid VA (e.g., 0x7) satisfies none.
// SHOULD FAIL
proof fn test_boundary_va2index_invalid_va() {
    let va: usize = 0x7;
    assert(spec_va_4k_valid(va));
}

// Test 11: alloc_and_map_io requires the IOMMU L2 resolution is_Some().
// If the L2 page table entry doesn't exist, the precondition fails.
// SHOULD FAIL
proof fn test_boundary_iommu_l2_resolution_none() {
    let resolved: Option<usize> = None;
    assert(resolved.is_Some());
}

// Test 12: Quota subtraction with underflow: mem_4k = 0 and k = 1.
// The spec says self.mem_4k - k == new.mem_4k, but 0 - 1 underflows.
// SHOULD FAIL
proof fn test_boundary_quota_underflow() {
    let old_quota = Quota { mem_4k: 0, mem_2m: 0, mem_1g: 0, pcid: 0, ioid: 0 };
    let new_quota = Quota { mem_4k: 0, mem_2m: 0, mem_1g: 0, pcid: 0, ioid: 0 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 1));
}

}
