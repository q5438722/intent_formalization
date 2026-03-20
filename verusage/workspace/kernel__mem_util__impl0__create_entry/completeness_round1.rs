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
pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;
pub const PAGE_ENTRY_USER_SHIFT: u64 = 2;
pub const PAGE_ENTRY_PS_SHIFT: u64 = 7;
pub const PAGE_ENTRY_EXECUTE_SHIFT: u64 = 63;
pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;
pub const PAGE_ENTRY_USER_MASK: u64 = 0x1u64 << PAGE_ENTRY_USER_SHIFT;
pub const PAGE_ENTRY_PS_MASK: u64 = 0x1u64 << PAGE_ENTRY_PS_SHIFT;
pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x1u64 << PAGE_ENTRY_EXECUTE_SHIFT;

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

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
}

pub open spec fn usize2present(v: usize) -> bool {
    (v & PAGE_ENTRY_PRESENT_MASK as usize) != 0
}

pub open spec fn usize2write(v: usize) -> bool {
    (v & PAGE_ENTRY_WRITE_MASK as usize) != 0
}

pub open spec fn usize2user(v: usize) -> bool {
    (v & PAGE_ENTRY_USER_MASK as usize) != 0
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}


// =========================================================================
// COMPLETENESS ROUND 1: Precondition Violations
// All tests should FAIL (verification errors expected)
// =========================================================================

// Test 1: Quota subtract with underflow (mem_4k=2, k=3)
proof fn test_fail_quota_subtract_too_much() {
    let q = Quota { mem_4k: 2, mem_2m: 10, mem_1g: 5, pcid: 2, ioid: 1 };
    let new_q = Quota { mem_4k: 0, mem_2m: 10, mem_1g: 5, pcid: 2, ioid: 1 };
    // 2 - 3 = -1 (int), not 0
    assert(q.spec_subtract_mem_4k(new_q, 3));
}

// Test 2: Quota subtract with mismatched mem_2m field
proof fn test_fail_quota_subtract_mismatched_fields() {
    let old_q = Quota { mem_4k: 10, mem_2m: 20, mem_1g: 5, pcid: 2, ioid: 1 };
    let new_q = Quota { mem_4k: 7, mem_2m: 30, mem_1g: 5, pcid: 2, ioid: 1 };
    assert(old_q.spec_subtract_mem_4k(new_q, 3));
}

// Test 3: page_ptr_valid with unaligned address
proof fn test_fail_page_ptr_valid_unaligned() {
    assert(page_ptr_valid(1usize));
}

// Test 4: page_index_valid with out-of-range index
proof fn test_fail_page_index_valid_out_of_range() {
    assert(page_index_valid(NUM_PAGES as usize));
}

// Test 5: MEM_valid with non-aligned value
proof fn test_fail_mem_valid_with_low_bits() {
    assert(MEM_valid(0xFFFusize)) by (bit_vector);
}

// Test 6: Assert present when bit 0 is not set
proof fn test_fail_present_when_bit0_clear() {
    assert(usize2present(2usize)) by (bit_vector);
}

// Test 7: Assert write when bit 1 is not set
proof fn test_fail_write_when_bit1_clear() {
    assert(usize2write(1usize)) by (bit_vector);
}

// Test 8: Assert user when bit 2 is not set
proof fn test_fail_user_when_bit2_clear() {
    assert(usize2user(1usize)) by (bit_vector);
}


} // verus!
