use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type PageMapPtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize {
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize {
    (i * 4096) as usize
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn spec_v2l4index(va: usize) -> L4Index {
    (va >> 39 & 0x1ff) as usize
}

pub open spec fn spec_v2l3index(va: usize) -> L3Index {
    (va >> 30 & 0x1ff) as usize
}

pub open spec fn spec_v2l2index(va: usize) -> L2Index {
    (va >> 21 & 0x1ff) as usize
}

pub open spec fn spec_v2l1index(va: usize) -> L1Index {
    (va >> 12 & 0x1ff) as usize
}

pub open spec fn spec_va2index(va: usize) -> (L4Index, L3Index, L2Index, L1Index) {
    (spec_v2l4index(va), spec_v2l3index(va), spec_v2l2index(va), spec_v2l1index(va))
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

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// Tests probe for determinism, stronger inequalities, structural
// assumptions, and cross-function misuse.
// All tests SHOULD FAIL verification.

// Test 1: create_entry ensures ret.0 <= 3.
// The spec does NOT guarantee ret.0 >= 1 (could be 0 if all levels exist).
// Claim: it always allocates at least 1 page.
// SHOULD FAIL
proof fn test_logical_create_entry_uses_at_least_1(ret0: usize)
    requires
        ret0 <= 3,
{
    assert(ret0 >= 1);
}

// Test 2: create_entry ensures ret.0 <= 3.
// The spec does NOT guarantee ret.0 == 3 always.
// Claim: it always uses exactly 3 pages.
// SHOULD FAIL
proof fn test_logical_create_entry_always_uses_3(ret0: usize)
    requires
        ret0 <= 3,
{
    assert(ret0 == 3);
}

// Test 3: usize2pa is NOT injective — different inputs can produce
// the same PA (due to masking out low bits). Claim injectivity.
// SHOULD FAIL
proof fn test_logical_usize2pa_injective() {
    let v1: usize = 0x1000;
    let v2: usize = 0x1001;
    assert(spec_usize2pa(v1) != spec_usize2pa(v2));
}

// Test 4: page_ptr2page_index and page_index2page_ptr roundtrip
// is NOT valid for non-aligned pointers. Claim it works for any ptr.
// SHOULD FAIL
proof fn test_logical_ptr_index_identity_arbitrary() {
    let ptr: usize = 0x1234; // not 4k-aligned
    assert(spec_page_index2page_ptr(spec_page_ptr2page_index(ptr)) == ptr);
}

// Test 5: Two calls to create_entry with the same state could allocate
// different pages. The spec doesn't guarantee deterministic PageMapPtr.
// Claim: return PageMapPtrs are always the same.
// SHOULD FAIL
proof fn test_logical_create_entry_determinism(
    ptr1: PageMapPtr,
    ptr2: PageMapPtr,
)
    requires
        page_ptr_valid(ptr1),
        page_ptr_valid(ptr2),
{
    assert(ptr1 == ptr2);
}

// Test 6: va_4k_valid does NOT imply the VA is page_ptr_valid.
// Virtual addresses and physical page pointers have different domains.
// Claim: va_4k_valid implies page_ptr_valid.
// SHOULD FAIL
proof fn test_logical_va_valid_implies_page_ptr_valid(va: VAddr)
    requires
        spec_va_4k_valid(va),
{
    assert(page_ptr_valid(va));
}

// Test 7: create_entry's quota subtraction does not guarantee exact amount.
// The spec says spec_subtract_mem_4k with ret.0, but ret.0 can be 0..3.
// Claim: quota always decreases by exactly 3.
// SHOULD FAIL
proof fn test_logical_quota_always_decreases_by_3(
    old_quota: Quota,
    new_quota: Quota,
    ret0: usize,
)
    requires
        old_quota.spec_subtract_mem_4k(new_quota, ret0),
        ret0 <= 3,
{
    assert(old_quota.mem_4k - new_quota.mem_4k == 3);
}

// Test 8: The L2 mapping resolve result (ret.1) is not guaranteed to be
// page_ptr_valid. The spec only says it equals the l2_entry addr.
// Claim: ret.1 is always page_ptr_valid.
// SHOULD FAIL
proof fn test_logical_ret_pagemap_ptr_valid(ret1: PageMapPtr)
    requires
        ret1 != 0, // nontrivial
{
    assert(page_ptr_valid(ret1));
}

// Test 9: The PCID-to-proc mapping is preserved, but the spec does NOT
// guarantee that ALL pcids remain active. Claim: pcid activity is preserved.
// SHOULD FAIL
proof fn test_logical_pcid_activity_preserved(
    old_active: bool,
    new_active: bool,
)
    requires
        old_active == true,
{
    assert(new_active == true);
}

// Test 10: create_entry does NOT guarantee that the new L2 page map
// is different from all existing page maps. Claim: uniqueness of returned ptr.
// SHOULD FAIL
proof fn test_logical_returned_ptr_unique(
    existing_ptrs: Set<PageMapPtr>,
    new_ptr: PageMapPtr,
)
    requires
        existing_ptrs.len() > 0,
{
    assert(!existing_ptrs.contains(new_ptr));
}

// Test 11: The spec says address space for other procs is preserved,
// but it does NOT say the target proc's va becomes mapped.
// The spec only guarantees L2 resolution, not L1-level mapping.
// Claim: va is in the target proc's address space after create_entry.
// SHOULD FAIL
proof fn test_logical_va_mapped_after_create_entry(
    addr_space: Map<VAddr, MapEntry>,
    va: VAddr,
)
    requires
        !addr_space.dom().contains(va),
{
    assert(addr_space.dom().contains(va));
}

// Test 12: The spec preserves container owned pages for ALL containers.
// This does NOT imply that the free page count equals any specific formula.
// Claim: free pages == old free pages - 3 always.
// SHOULD FAIL
proof fn test_logical_free_pages_always_minus_3(
    old_free: usize,
    new_free: usize,
    ret0: usize,
)
    requires
        new_free == old_free - ret0,
        ret0 <= 3,
        old_free >= 3,
{
    assert(new_free == old_free - 3);
}

}
