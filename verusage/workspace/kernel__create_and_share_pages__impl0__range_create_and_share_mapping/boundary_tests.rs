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

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition of range_create_and_share_mapping
// or create_entry_and_share. All tests SHOULD FAIL verification.

// Test 1: range_create_and_share_mapping requires proc_dom().contains(src_proc_ptr).
// Violate: src_proc_ptr is NOT in proc domain.
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

// Test 2: range_create_and_share_mapping requires proc_dom().contains(target_proc_ptr).
// Violate: target_proc_ptr is NOT in proc domain.
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

// Test 3: range_create_and_share_mapping requires src_proc_ptr != target_proc_ptr.
// Violate: same proc for source and target.
// SHOULD FAIL
proof fn test_boundary_same_src_and_target_proc(
    proc_ptr: ProcPtr,
)
{
    let src = proc_ptr;
    let target = proc_ptr;
    assert(src != target);
}

// Test 4: range_create_and_share_mapping requires
// get_container_quota(...).mem_4k >= 3 * src_va_range.len.
// Violate: quota is 0 but range length is 1.
// SHOULD FAIL
proof fn test_boundary_insufficient_quota(
    quota_mem_4k: usize,
    range_len: usize,
)
    requires
        quota_mem_4k == 0,
        range_len == 1,
{
    assert(quota_mem_4k >= 3 * range_len);
}

// Test 5: range_create_and_share_mapping requires
// get_num_of_free_pages() >= 3 * src_va_range.len.
// Violate: only 2 free pages but range length is 1 (need 3).
// SHOULD FAIL
proof fn test_boundary_insufficient_free_pages(
    free_pages: usize,
    range_len: usize,
)
    requires
        free_pages == 2,
        range_len == 1,
{
    assert(free_pages >= 3 * range_len);
}

// Test 6: range_create_and_share_mapping requires
// src_va_range.len == target_va_range.len.
// Violate: different lengths.
// SHOULD FAIL
proof fn test_boundary_range_length_mismatch(
    src_len: usize,
    target_len: usize,
)
    requires
        src_len == 5,
        target_len == 3,
{
    assert(src_len == target_len);
}

// Test 7: create_entry_and_share requires va_4k_valid(src_va).
// VA = 0 is in kernel space, should not be valid for user.
// SHOULD FAIL
proof fn test_boundary_src_va_zero() {
    let va: usize = 0;
    assert(spec_va_4k_valid(va));
}

// Test 8: create_entry_and_share requires va_4k_valid(target_va).
// Misaligned VA (not 4k aligned) should fail.
// SHOULD FAIL
proof fn test_boundary_target_va_not_aligned() {
    let va: usize = 0x1001usize;
    assert(spec_va_4k_valid(va));
}

// Test 9: create_entry_and_share requires
// get_address_space(target_proc_ptr).dom().contains(target_va) == false.
// Violate: target_va is already in the target address space.
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

// Test 10: create_entry_and_share requires
// get_address_space(src_proc_ptr).dom().contains(src_va) == true.
// Violate: src_va is NOT in the source address space.
// SHOULD FAIL
proof fn test_boundary_src_va_not_in_address_space(
    address_space: Map<VAddr, MapEntry>,
    src_va: VAddr,
)
    requires
        !address_space.dom().contains(src_va),
{
    assert(address_space.dom().contains(src_va));
}

// Test 11: create_entry_and_share requires
// get_physical_page_reference_counter(...) <= usize::MAX - 1.
// Violate: ref counter is at usize::MAX (would overflow on increment).
// SHOULD FAIL
proof fn test_boundary_ref_counter_at_max() {
    let ref_counter: nat = usize::MAX as nat;
    assert(ref_counter <= (usize::MAX - 1) as nat);
}

// Test 12: range_create_and_share_mapping requires
// address_space_range_shareable(src_proc_ptr, src_va_range).
// Part of this requires ref counters bounded by usize::MAX - va_range.len.
// Violate: ref counter too high for the range.
// SHOULD FAIL
proof fn test_boundary_ref_counter_overflow_for_range(
    ref_counter: nat,
    range_len: usize,
)
    requires
        range_len == 10,
        ref_counter == (usize::MAX - 5) as nat,
{
    assert(ref_counter <= (usize::MAX - range_len) as nat);
}

}
