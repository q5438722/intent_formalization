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

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
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
// These test for unintended reasoning: determinism, injectivity,
// stronger bounds, cross-function misuse.
// All tests SHOULD FAIL verification.

// Test 1: alloc_and_map is NOT deterministic — same inputs do not
// guarantee the same returned page. Claim: two allocations
// from identical states return identical pages.
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

// Test 2: alloc_and_map ensures ret.addr is page_ptr_valid, but does
// NOT guarantee ret.addr has any specific value. Claim: ret.addr
// is always exactly 0x1000.
// SHOULD FAIL
proof fn test_logical_ret_addr_always_specific(ret_addr: PagePtr)
    requires
        page_ptr_valid(ret_addr),
{
    assert(ret_addr == 0x1000);
}

// Test 3: va_4k_valid does NOT imply page_ptr_valid. These are
// different validity domains (virtual vs physical).
// SHOULD FAIL
proof fn test_logical_va_valid_implies_page_ptr_valid(va: VAddr)
    requires
        spec_va_4k_valid(va),
{
    assert(page_ptr_valid(va));
}

// Test 4: page_ptr2page_index is NOT injective for non-aligned
// inputs. Two different non-aligned ptrs may map to the same index.
// Claim: it's always injective.
// SHOULD FAIL
proof fn test_logical_page_ptr2index_injective_non_aligned() {
    let ptr1: usize = 0x1000;
    let ptr2: usize = 0x1001;
    // ptr2 is not aligned, but truncation means index is same as ptr1
    assert(spec_page_ptr2page_index(ptr1) != spec_page_ptr2page_index(ptr2));
}

// Test 5: alloc_and_map returns MapEntry with write=true and
// execute_disable=false. The spec does NOT guarantee that
// ret.addr == target_va (VA != PA). Claim: they are equal.
// SHOULD FAIL
proof fn test_logical_ret_addr_equals_va(
    ret_addr: PagePtr,
    target_va: VAddr,
)
    requires
        page_ptr_valid(ret_addr),
        spec_va_4k_valid(target_va),
{
    assert(ret_addr == target_va);
}

// Test 6: Two different target VAs do NOT guarantee different
// returned pages across separate calls. Claim: different VAs
// always produce different page addrs.
// SHOULD FAIL
proof fn test_logical_different_va_different_page(
    va1: VAddr,
    va2: VAddr,
    page1: PagePtr,
    page2: PagePtr,
)
    requires
        va1 != va2,
        page_ptr_valid(page1),
        page_ptr_valid(page2),
{
    assert(page1 != page2);
}

// Test 7: alloc_and_map_4k ensures page_io_mappings(ret) is empty.
// The spec does NOT guarantee that page_io_mappings for OTHER
// pages are also empty. Claim: all io_mappings are empty.
// SHOULD FAIL
proof fn test_logical_all_io_mappings_empty(
    io_mappings_ret: Set<(IOid, VAddr)>,
    io_mappings_other: Set<(IOid, VAddr)>,
)
    requires
        io_mappings_ret =~= Set::<(IOid, VAddr)>::empty(),
{
    assert(io_mappings_other =~= Set::<(IOid, VAddr)>::empty());
}

// Test 8: The spec says quota subtracts exactly 1 for alloc_and_map.
// It does NOT guarantee that quota.mem_4k > 0 AFTER the call
// (it could go to 0). Claim: quota is always >= 1 after.
// SHOULD FAIL
proof fn test_logical_quota_positive_after_alloc(
    old_quota: Quota,
    new_quota: Quota,
)
    requires
        old_quota.spec_subtract_mem_4k(new_quota, 1),
        old_quota.mem_4k >= 1,
{
    assert(new_quota.mem_4k >= 1);
}

// Test 9: The spec preserves container fields for the target
// container (owned_procs, parent, children, etc.), but does NOT
// explicitly preserve can_have_children (not in ensures).
// Claim: the 'root_process' field is also explicitly guaranteed
// unchanged by the spec — but it is NOT in alloc_and_map's ensures.
// SHOULD FAIL
proof fn test_logical_root_process_preserved(
    old_root: Option<ProcPtr>,
    new_root: Option<ProcPtr>,
)
    requires
        old_root.is_Some(),
{
    // The spec doesn't explicitly preserve root_process for the
    // target container in alloc_and_map's ensures
    assert(old_root == new_root);
}

// Test 10: usize2pa is NOT injective: different inputs can map to
// the same physical address because masking discards low bits.
// Claim: it's always injective.
// SHOULD FAIL
proof fn test_logical_usize2pa_injective() {
    let v1: usize = 0x1000;
    let v2: usize = 0x1001;
    assert(spec_usize2pa(v1) != spec_usize2pa(v2));
}

// Test 11: alloc_and_map ensures page_mapping[ret.addr] ==
// Set::empty().insert((target_proc_ptr, target_va)).
// The spec does NOT guarantee ret.addr is the smallest free page.
// Claim: ret.addr is always the minimum element of free_pages.
// SHOULD FAIL
proof fn test_logical_ret_is_minimum_free_page(
    ret: PagePtr,
    other_free: PagePtr,
)
    requires
        page_ptr_valid(ret),
        page_ptr_valid(other_free),
        other_free != ret,
{
    assert(ret < other_free);
}

// Test 12: alloc_and_map ensures container_owned_pages for
// the owning container gains ret.addr. The spec does NOT
// guarantee that the TOTAL number of owned pages across ALL
// containers increases by exactly 1.
// SHOULD FAIL
proof fn test_logical_total_owned_pages_increases_by_one(
    old_total: int,
    new_total: int,
    old_owned: Set<PagePtr>,
    new_owned: Set<PagePtr>,
    ret: PagePtr,
)
    requires
        new_owned =~= old_owned.insert(ret),
        !old_owned.contains(ret),
{
    // The spec only guarantees the owning container gains ret.
    // It says nothing about a "total" across all containers.
    assert(new_total == old_total + 1);
}

}
