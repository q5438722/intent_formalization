use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type IOid = usize;
pub type Pcid = usize;
pub type PagePtr = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;
pub const IOID_MAX: usize = 4096;

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) of
// check_io_space_va_range_free or related functions.
// All tests SHOULD FAIL verification.

// Test 1: check_io_space_va_range_free requires proc_dom().contains(target_proc_ptr).
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

// Test 2: check_io_space_va_range_free requires get_proc(target_proc_ptr).ioid.is_Some().
// If the process has no ioid, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_ioid_is_none(ioid: Option<IOid>)
    requires
        ioid.is_None(),
{
    assert(ioid.is_Some());
}

// Test 3: check_io_space_va_range_free requires va_range.wf().
// A VaRange4K with start not 4k-aligned violates wf().
// spec_va_4k_valid(0) should be false since 0 is in kernel space.
// SHOULD FAIL
proof fn test_boundary_va_range_start_zero() {
    let va: usize = 0;
    assert(spec_va_4k_valid(va));
}

// Test 4: va_range.wf() requires start + len * 4096 < usize::MAX.
// With large start and len, this exceeds usize::MAX.
// SHOULD FAIL
proof fn test_boundary_va_range_len_overflow(start: usize, len: usize)
    requires
        start == 0x1000usize,
        len == 0x1000usize,
{
    // start + len * 4096 = 0x1000 + 0x1000 * 4096 = 0x1000 + 0x1000000 which is fine.
    // But we assert something impossible: that len * 4096 overflows
    assert(start + len * 4096 >= usize::MAX);
}

// Test 5: resolve_iommu_table_mapping requires ioid_active(ioid).
// A free ioid is not active.
// SHOULD FAIL
proof fn test_boundary_ioid_not_active(
    free_ioids: Set<IOid>,
    ioid: IOid,
)
    requires
        free_ioids.contains(ioid),
{
    // ioid_active requires !free_ioids.contains(ioid)
    assert(!free_ioids.contains(ioid));
}

// Test 6: resolve_iommu_table_mapping requires va_4k_valid(va).
// An unaligned va (low bits set) violates this.
// SHOULD FAIL
proof fn test_boundary_va_not_4k_aligned() {
    let va: usize = 0x1001;
    assert(spec_va_4k_valid(va));
}

// Test 7: resolve_iommu_table_mapping requires ioid in range [0, IOID_MAX).
// Using ioid == IOID_MAX is out of bounds.
// SHOULD FAIL
proof fn test_boundary_ioid_at_max() {
    let ioid: usize = IOID_MAX;
    assert(0 <= ioid && ioid < IOID_MAX);
}

// Test 8: va_range.wf() requires no_duplicates in the va sequence.
// If two entries are the same, wf() is violated.
// SHOULD FAIL
proof fn test_boundary_va_range_duplicates(va_seq: Seq<VAddr>)
    requires
        va_seq.len() == 2,
        va_seq[0] == va_seq[1],
{
    assert(va_seq.no_duplicates());
}

// Test 9: va_range.wf() requires all elements are va_4k_valid.
// A non-valid element at index 0 violates this.
// SHOULD FAIL
proof fn test_boundary_va_range_element_invalid(va_seq: Seq<VAddr>)
    requires
        va_seq.len() >= 1,
        va_seq[0] == 1, // 1 is not 4k-aligned
{
    assert(spec_va_4k_valid(va_seq[0]));
}

// Test 10: check_io_space_va_range_free requires self.wf().
// Without wf(), the ensures clause cannot be derived.
// Test that kernel_wf cannot be assumed from nothing.
// SHOULD FAIL
proof fn test_boundary_kernel_wf_from_nothing(kernel_wf: bool)
    requires
        kernel_wf == false,
{
    assert(kernel_wf == true);
}

// Test 11: va_range.len could be 0 (empty range). The function should
// return true for empty ranges. But claiming it returns false is wrong.
// The io_space_range_free spec vacuously holds for len=0.
// SHOULD FAIL
proof fn test_boundary_empty_range_returns_false() {
    let len: usize = 0;
    // forall|j: int| 0 <= j < 0 ==> ... is vacuously true
    // so io_space_range_free == true, asserting false should fail
    let range_free: bool = true; // vacuously true for len=0
    assert(range_free == false);
}

// Test 12: page_ptr2page_index requires ptr % 0x1000 == 0.
// Passing ptr = 0xFFF (not aligned) violates the precondition.
// SHOULD FAIL
proof fn test_boundary_page_ptr_not_aligned() {
    let ptr: usize = 0xFFF;
    assert(ptr % 0x1000 == 0);
}

}
