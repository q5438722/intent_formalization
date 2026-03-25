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

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize {
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize {
    (i * 4096) as usize
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

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// These test for unintended reasoning: determinism, stronger bounds,
// injectivity, cross-function misuse, structural assumptions.
// All tests SHOULD FAIL verification.

// Test 1: Determinism — same inputs do NOT guarantee the same returned page.
// The spec says ret comes from free_pages but does not fix which one.
// SHOULD FAIL
proof fn test_logical_alloc_determinism(
    ret1: PagePtr,
    ret2: PagePtr,
    free_pages: Set<PagePtr>,
)
    requires
        free_pages.contains(ret1),
        free_pages.contains(ret2),
        free_pages.len() > 1,
{
    assert(ret1 == ret2);
}

// Test 2: Injectivity — two allocations with different VAs do NOT
// guarantee different returned pages (the spec doesn't link VA to page identity).
// SHOULD FAIL
proof fn test_logical_va_does_not_determine_page(
    va1: VAddr,
    va2: VAddr,
    ret1: PagePtr,
    ret2: PagePtr,
)
    requires
        va1 != va2,
        page_ptr_valid(ret1),
        page_ptr_valid(ret2),
{
    assert(ret1 != ret2);
}

// Test 3: The spec does NOT guarantee that the returned page address
// is related to the target VA. Claim: ret.addr == target_va.
// SHOULD FAIL
proof fn test_logical_ret_addr_equals_va(
    target_va: VAddr,
    ret_addr: PAddr,
)
    requires
        page_ptr_valid(ret_addr),
        spec_va_4k_valid(target_va),
{
    assert(ret_addr == target_va);
}

// Test 4: The spec does NOT guarantee that page_ptr_valid implies
// the page was previously free. Claim: valid ptr means it was free.
// SHOULD FAIL
proof fn test_logical_valid_ptr_implies_free(
    ret: PagePtr,
    free_pages: Set<PagePtr>,
)
    requires
        page_ptr_valid(ret),
        free_pages.len() > 0,
{
    assert(free_pages.contains(ret));
}

// Test 5: The spec does NOT guarantee that two distinct IOids map
// to distinct pages. Claim: different IOids => different pages.
// SHOULD FAIL
proof fn test_logical_distinct_ioids_distinct_pages(
    ioid1: IOid,
    ioid2: IOid,
    ret1: PagePtr,
    ret2: PagePtr,
)
    requires
        ioid1 != ioid2,
        page_ptr_valid(ret1),
        page_ptr_valid(ret2),
{
    assert(ret1 != ret2);
}

// Test 6: The spec does NOT guarantee quota cannot go to zero.
// Stronger claim: after allocation, quota is always >= 1.
// SHOULD FAIL
proof fn test_logical_quota_always_positive(
    old_quota: Quota,
)
    requires
        old_quota.mem_4k >= 1,
{
    let new_mem_4k: usize = (old_quota.mem_4k - 1) as usize;
    assert(new_mem_4k >= 1);
}

// Test 7: The spec does NOT guarantee page_ptr_valid(ret) implies
// spec_page_ptr2page_index(ret) < some specific bound smaller than NUM_PAGES.
// Claim: page index is always < NUM_PAGES / 2.
// SHOULD FAIL
proof fn test_logical_page_index_half_bound(
    ret: PagePtr,
)
    requires
        page_ptr_valid(ret),
{
    assert(spec_page_ptr2page_index(ret) < NUM_PAGES / 2);
}

// Test 8: Cross-function misuse — alloc_and_map_io_4k's postcondition says
// page_mappings(ret) is empty, but alloc_and_map_io wraps it. The spec does
// NOT promise that Kernel.get_address_space reflects the IO mapping.
// Claim: IO mapping entry appears in address space.
// SHOULD FAIL
proof fn test_logical_io_mapping_in_address_space(
    addr_space: Map<VAddr, MapEntry>,
    target_va: VAddr,
)
    requires
        !addr_space.dom().contains(target_va),
{
    assert(addr_space.dom().contains(target_va));
}

// Test 9: The spec does NOT guarantee that repeated alloc_and_map_io calls
// with the same (ioid, va) pair are rejected. The precondition only checks
// IO space domain. But if we assume the first call succeeded, the second
// should fail because va is already in domain. This tests the boundary
// of idempotency — claiming two identical allocations are fine.
// SHOULD FAIL
proof fn test_logical_double_alloc_same_va(
    io_space_after_first: Map<VAddr, MapEntry>,
    target_va: VAddr,
)
    requires
        io_space_after_first.dom().contains(target_va),
{
    // Attempting to prove the precondition for a second call
    assert(!io_space_after_first.dom().contains(target_va));
}

// Test 10: The spec does NOT guarantee that the container owning the
// target process is unique (i.e., no other process shares the container).
// Claim: owning_container determines the proc uniquely.
// SHOULD FAIL
proof fn test_logical_container_unique_proc(
    proc_dom: Set<ProcPtr>,
    p1: ProcPtr,
    p2: ProcPtr,
    c: ContainerPtr,
)
    requires
        proc_dom.contains(p1),
        proc_dom.contains(p2),
        p1 != p2,
        // Both could share the same owning_container
{
    // Claim they must be in different containers — not guaranteed
    assert(p1 == p2);
}

// Test 11: spec_page_ptr2page_index and spec_page_index2page_ptr are NOT
// guaranteed to be inverses for all inputs, only for valid ones. Claim
// roundtrip for arbitrary values.
// SHOULD FAIL
proof fn test_logical_roundtrip_arbitrary(
    v: usize,
)
{
    assert(spec_page_index2page_ptr(spec_page_ptr2page_index(v)) == v);
}

// Test 12: The spec does NOT guarantee that alloc_and_map_io preserves
// the total number of mapped+free pages. Claim: mapped + free is constant.
// SHOULD FAIL
proof fn test_logical_total_pages_constant(
    old_mapped: nat,
    old_free: nat,
    new_mapped: nat,
    new_free: nat,
)
    requires
        new_free == old_free - 1,
        new_mapped == old_mapped + 1,
        old_mapped > 0,
        old_free > 0,
{
    // This actually holds arithmetically, but the spec doesn't state it
    // as an explicit invariant across all page categories.
    // We test a WRONG version: total increases.
    assert(new_mapped + new_free > old_mapped + old_free);
}

}
