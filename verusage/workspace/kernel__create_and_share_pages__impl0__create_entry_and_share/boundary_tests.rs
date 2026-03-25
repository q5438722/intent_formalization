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
// Each test violates a precondition (requires clause) of
// create_entry_and_share or related functions.
// All tests SHOULD FAIL verification.

// Test 1: create_entry_and_share requires get_num_of_free_pages() >= 3.
// With 0 free pages, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_zero_free_pages() {
    let free_pages: usize = 0;
    assert(free_pages >= 3);
}

// Test 2: create_entry_and_share requires get_num_of_free_pages() >= 3.
// With exactly 2 free pages, still insufficient.
// SHOULD FAIL
proof fn test_boundary_two_free_pages() {
    let free_pages: usize = 2;
    assert(free_pages >= 3);
}

// Test 3: create_entry_and_share requires quota.mem_4k >= 3.
// With quota.mem_4k == 0, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_zero_quota() {
    let mem_4k: usize = 0;
    assert(mem_4k >= 3);
}

// Test 4: create_entry_and_share requires quota.mem_4k >= 3.
// With quota.mem_4k == 2, still insufficient.
// SHOULD FAIL
proof fn test_boundary_quota_two() {
    let mem_4k: usize = 2;
    assert(mem_4k >= 3);
}

// Test 5: create_entry_and_share requires proc_dom contains src_proc_ptr.
// If src_proc is not in the domain, precondition is violated.
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

// Test 6: create_entry_and_share requires proc_dom contains target_proc_ptr.
// If target_proc is not in the domain, precondition is violated.
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

// Test 7: create_entry_and_share requires va_4k_valid(src_va).
// VA = 0 is in kernel space and should not be valid.
// SHOULD FAIL
proof fn test_boundary_src_va_zero_not_valid() {
    let va: usize = 0;
    assert(spec_va_4k_valid(va));
}

// Test 8: create_entry_and_share requires va_4k_valid(target_va).
// VA = 1 is not 4k-aligned, so not valid.
// SHOULD FAIL
proof fn test_boundary_target_va_not_aligned() {
    let va: usize = 1;
    assert(spec_va_4k_valid(va));
}

// Test 9: create_entry_and_share requires target_va NOT in target's address space.
// If target_va IS present, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_target_va_already_mapped(
    addr_space_dom: Set<VAddr>,
    target_va: VAddr,
)
    requires
        addr_space_dom.contains(target_va),
{
    assert(addr_space_dom.contains(target_va) == false);
}

// Test 10: create_entry_and_share requires src_va IS in src's address space.
// If src_va is NOT present, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_src_va_not_mapped(
    addr_space_dom: Set<VAddr>,
    src_va: VAddr,
)
    requires
        !addr_space_dom.contains(src_va),
{
    assert(addr_space_dom.contains(src_va) == true);
}

// Test 11: create_entry_and_share requires ref counter <= usize::MAX - 1.
// With ref_counter == usize::MAX, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_ref_counter_at_max() {
    let ref_counter: usize = usize::MAX;
    assert(ref_counter <= usize::MAX - 1);
}

// Test 12: create_entry_and_share ensures ret <= 3.
// Claiming ret > 3 contradicts the postcondition.
// SHOULD FAIL
proof fn test_boundary_ret_exceeds_three(ret: usize)
    requires
        ret <= 3,
{
    assert(ret > 3);
}

}
