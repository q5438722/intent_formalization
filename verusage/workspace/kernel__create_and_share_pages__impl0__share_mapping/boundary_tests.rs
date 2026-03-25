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

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) of share_mapping
// or uses edge-case values. All tests SHOULD FAIL verification.

// Test 1: share_mapping requires proc_dom().contains(src_proc_ptr).
// If src proc is not in the domain, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_src_proc_not_in_domain(
    proc_dom: Set<ProcPtr>,
    src_proc_ptr: ProcPtr,
)
    requires
        !proc_dom.contains(src_proc_ptr),
{
    assert(proc_dom.contains(src_proc_ptr));
}

// Test 2: share_mapping requires proc_dom().contains(target_proc_ptr).
// If target proc is not in the domain, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_target_proc_not_in_domain(
    proc_dom: Set<ProcPtr>,
    target_proc_ptr: ProcPtr,
)
    requires
        !proc_dom.contains(target_proc_ptr),
{
    assert(proc_dom.contains(target_proc_ptr));
}

// Test 3: share_mapping requires va_4k_valid(target_va).
// VA = 0 is in kernel address space and should fail validity.
// SHOULD FAIL
proof fn test_boundary_target_va_zero() {
    let va: usize = 0;
    assert(spec_va_4k_valid(va));
}

// Test 4: share_mapping requires page_ptr_valid(entry.addr).
// A misaligned pointer (not 0x1000 aligned) should fail.
// SHOULD FAIL
proof fn test_boundary_entry_addr_misaligned() {
    let addr: usize = 0x1001;
    assert(page_ptr_valid(addr));
}

// Test 5: share_mapping requires page_ptr_valid(entry.addr).
// An address with page index >= NUM_PAGES should fail.
// SHOULD FAIL
proof fn test_boundary_entry_addr_out_of_range() {
    let addr: usize = (NUM_PAGES * 0x1000) as usize;
    assert(page_ptr_valid(addr));
}

// Test 6: share_mapping requires that the target VA is NOT already
// in the target proc's address space. If it is, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_target_va_already_mapped(
    address_space: Map<VAddr, MapEntry>,
    target_va: VAddr,
)
    requires
        address_space.dom().contains(target_va),
{
    assert(address_space.dom().contains(target_va) == false);
}

// Test 7: share_mapping requires get_address_space(src_proc_ptr).dom().contains(src_va).
// If src_va is NOT in the src address space, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_src_va_not_mapped(
    address_space: Map<VAddr, MapEntry>,
    src_va: VAddr,
)
    requires
        !address_space.dom().contains(src_va),
{
    assert(address_space.dom().contains(src_va));
}

// Test 8: share_mapping requires get_physical_page_reference_counter(entry.addr) <= usize::MAX - 1.
// If reference counter is at usize::MAX, it would overflow.
// SHOULD FAIL
proof fn test_boundary_ref_counter_overflow() {
    let ref_counter: nat = usize::MAX as nat;
    assert(ref_counter <= (usize::MAX - 1) as nat);
}

// Test 9: share_mapping requires va_4k_valid(target_va).
// A VA with non-zero low bits (not 4k aligned) should fail validity.
// SHOULD FAIL
proof fn test_boundary_target_va_not_aligned() {
    let va: usize = 0x1001usize;
    assert(spec_va_4k_valid(va));
}

// Test 10: share_mapping requires page_is_mapped(entry.addr).
// If the page is NOT mapped, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_page_not_mapped(
    mapped: bool,
)
    requires
        mapped == false,
{
    assert(mapped == true);
}

}
