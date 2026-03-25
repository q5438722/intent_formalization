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
pub type IOid = usize;
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

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) of
// create_iommu_table_entry or uses edge-case values.
// All tests SHOULD FAIL verification.

// Test 1: proc_ptr must be in proc_dom.
// Violate: proc_ptr NOT in proc_dom.
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

// Test 2: process must have IOMMU table (ioid.is_Some()).
// Violate: ioid is None.
// SHOULD FAIL
proof fn test_boundary_no_iommu_table(ioid: Option<IOid>)
    requires
        ioid.is_None(),
{
    assert(ioid.is_Some());
}

// Test 3: container quota mem_4k must be >= 3.
// Violate: quota is 2.
// SHOULD FAIL
proof fn test_boundary_insufficient_quota() {
    let mem_4k: usize = 2;
    assert(mem_4k >= 3);
}

// Test 4: get_num_of_free_pages() must be >= 3.
// Violate: only 2 free pages.
// SHOULD FAIL
proof fn test_boundary_insufficient_free_pages() {
    let free_pages: usize = 2;
    assert(free_pages >= 3);
}

// Test 5: va must be 4k-valid.
// Violate: va = 0 is in kernel space (L4 index 0 < KERNEL_MEM_END_L4INDEX).
// SHOULD FAIL
proof fn test_boundary_va_zero_not_valid() {
    let va: usize = 0;
    assert(spec_va_4k_valid(va));
}

// Test 6: va must NOT already be in IO space.
// Violate: va IS already in IO space.
// SHOULD FAIL
proof fn test_boundary_va_already_in_io_space(
    io_space_dom: Set<VAddr>,
    va: VAddr,
)
    requires
        io_space_dom.contains(va),
{
    assert(io_space_dom.contains(va) == false);
}

// Test 7: quota edge case: quota == 0 (far below 3).
// SHOULD FAIL
proof fn test_boundary_zero_quota() {
    let mem_4k: usize = 0;
    assert(mem_4k >= 3);
}

// Test 8: free pages edge case: 0 free pages.
// SHOULD FAIL
proof fn test_boundary_zero_free_pages() {
    let free_pages: usize = 0;
    assert(free_pages >= 3);
}

// Test 9: va with misaligned bits (not 4k-aligned).
// va = 1 has low bits set, violating alignment.
// SHOULD FAIL
proof fn test_boundary_va_misaligned() {
    let va: usize = 1;
    assert(spec_va_4k_valid(va));
}

// Test 10: quota exactly at boundary: quota == 3 is valid,
// but quota == 1 should be rejected.
// SHOULD FAIL
proof fn test_boundary_quota_one() {
    let mem_4k: usize = 1;
    assert(mem_4k >= 3);
}

}
