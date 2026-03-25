use vstd::prelude::*;

fn main() {}

verus! {

pub type PAddr = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;

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

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize
    recommends page_ptr_valid(ptr),
{
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize
    recommends page_index_valid(i),
{
    (i * 4096) as usize
}

pub open spec fn spec_page_index_truncate_2m(index: usize) -> usize {
    (index / 512usize * 512usize) as usize
}


// =========================================================================
// COMPLETENESS ROUND 4: Wrong Specific Values
// All tests should FAIL (verification errors expected)
// =========================================================================

// Test 1: Wrong l1 index for 0x1000 (should be 1, asserting 2)
proof fn test_fail_wrong_l1index_for_0x1000() {
    assert(spec_v2l1index(0x1000usize) == 2usize) by (bit_vector);
}

// Test 2: Wrong l2 index for 0x200000 (should be 1, asserting 0)
proof fn test_fail_wrong_l2index_for_0x200000() {
    assert(spec_v2l2index(0x200000usize) == 0usize) by (bit_vector);
}

// Test 3: Wrong l3 index for 0x40000000 (should be 1, asserting 2)
proof fn test_fail_wrong_l3index_for_0x40000000() {
    assert(spec_v2l3index(0x40000000usize) == 2usize) by (bit_vector);
}

// Test 4: Wrong PA for 0x1001 (should be 0x1000, asserting 0x1001)
proof fn test_fail_wrong_pa_for_0x1001() {
    assert(spec_usize2pa(0x1001usize) == 0x1001usize) by (bit_vector);
}

// Test 5: Wrong page index for 4096 (should be 1, asserting 0)
proof fn test_fail_wrong_page_index_for_4096() {
    assert(spec_page_ptr2page_index(4096usize) == 0usize);
}

// Test 6: Wrong page pointer for index 1 (should be 4096, asserting 8192)
proof fn test_fail_wrong_page_ptr_for_1() {
    assert(spec_page_index2page_ptr(1usize) == 8192usize);
}

// Test 7: Wrong truncation result (should be 512, asserting 513)
proof fn test_fail_wrong_truncate_2m() {
    assert(spec_page_index_truncate_2m(513usize) == 513usize);
}

// Test 8: Wrong quota subtract (10-3=7, not 8)
proof fn test_fail_wrong_quota_subtract() {
    let old_q = Quota { mem_4k: 10, mem_2m: 20, mem_1g: 5, pcid: 2, ioid: 1 };
    let wrong_new_q = Quota { mem_4k: 8, mem_2m: 20, mem_1g: 5, pcid: 2, ioid: 1 };
    assert(old_q.spec_subtract_mem_4k(wrong_new_q, 3));
}


} // verus!
