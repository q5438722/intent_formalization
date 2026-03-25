use vstd::prelude::*;

fn main() {}

verus! {

pub type PAddr = usize;
pub type VAddr = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;

pub const KERNEL_MEM_END_L4INDEX: usize = 1;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;

#[derive(Clone, Copy, Debug)]
pub struct Quota {
    pub mem_4k: usize,
    pub mem_2m: usize,
    pub mem_1g: usize,
    pub pcid: usize,
    pub ioid: usize,
}

impl Quota {
    pub open spec fn spec_set_mem_4k(&self, v: usize) -> Self {
        Self {
            mem_4k: v,
            mem_2m: self.mem_2m,
            mem_1g: self.mem_1g,
            pcid: self.pcid,
            ioid: self.ioid,
        }
    }

    pub open spec fn spec_subtract_mem_4k(&self, new: Self, k: usize) -> bool {
        &&& self.mem_4k - k == new.mem_4k
        &&& self.mem_2m == new.mem_2m
        &&& self.mem_1g == new.mem_1g
        &&& self.pcid == new.pcid
        &&& self.ioid == new.ioid
    }
}

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

pub open spec fn spec_v2l1index(va: usize) -> L1Index {
    (va >> 12 & 0x1ff) as usize
}

pub open spec fn spec_v2l2index(va: usize) -> L2Index {
    (va >> 21 & 0x1ff) as usize
}

pub open spec fn spec_v2l3index(va: usize) -> L3Index {
    (va >> 30 & 0x1ff) as usize
}

pub open spec fn spec_v2l4index(va: usize) -> L4Index {
    (va >> 39 & 0x1ff) as usize
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn spec_page_index_truncate_2m(index: usize) -> usize {
    (index / 512usize * 512usize) as usize
}


// =========================================================================
// COMPLETENESS ROUND 2: Overly Strong Postconditions
// All tests should FAIL (verification errors expected)
// =========================================================================

// Test 1: Assert v2l1index < 256 (spec only guarantees < 512)
proof fn test_fail_l1index_too_tight_bound(va: usize) {
    assert(spec_v2l1index(va) < 256usize) by (bit_vector);
}

// Test 2: Assert v2l2index < 256 (spec only guarantees < 512)
proof fn test_fail_l2index_too_tight_bound(va: usize) {
    assert(spec_v2l2index(va) < 256usize) by (bit_vector);
}

// Test 3: Assert v2l3index < 256 (spec only guarantees < 512)
proof fn test_fail_l3index_too_tight_bound(va: usize) {
    assert(spec_v2l3index(va) < 256usize) by (bit_vector);
}

// Test 4: Assert v2l4index < 256 (spec only guarantees < 512)
proof fn test_fail_l4index_too_tight_bound(va: usize) {
    assert(spec_v2l4index(va) < 256usize) by (bit_vector);
}

// Test 5: Assert spec_usize2pa preserves full value (it masks bits)
proof fn test_fail_pa_equals_input(v: usize) {
    assert(spec_usize2pa(v) == v) by (bit_vector);
}

// Test 6: Assert page_ptr_valid for any page-aligned pointer (needs < NUM_PAGES too)
proof fn test_fail_aligned_implies_valid(ptr: usize)
    requires ptr % 0x1000 == 0
{
    assert(page_ptr_valid(ptr));
}

// Test 7: Assert quota subtract with wrong k
proof fn test_fail_quota_subtract_wrong_k() {
    let old_q = Quota { mem_4k: 10, mem_2m: 20, mem_1g: 5, pcid: 2, ioid: 1 };
    let new_q = old_q.spec_set_mem_4k(7);
    // Claiming k=2 when it should be k=3
    assert(old_q.spec_subtract_mem_4k(new_q, 2));
}

// Test 8: Assert truncate_2m is identity (it truncates)
proof fn test_fail_truncate_2m_identity(index: usize) {
    assert(spec_page_index_truncate_2m(index) == index);
}


} // verus!
