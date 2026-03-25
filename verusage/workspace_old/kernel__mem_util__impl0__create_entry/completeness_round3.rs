use vstd::prelude::*;

fn main() {}

verus! {

pub type PAddr = usize;
pub type VAddr = usize;
pub type L4Index = usize;
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

pub struct PageEntryPerm {
    pub present: bool,
    pub ps: bool,
    pub write: bool,
    pub execute_disable: bool,
    pub user: bool,
}

pub struct PageEntry {
    pub addr: PAddr,
    pub perm: PageEntryPerm,
}

impl PageEntry {
    pub open spec fn is_empty(&self) -> bool {
        &&& self.addr == 0
        &&& self.perm.present == false
        &&& self.perm.ps == false
        &&& self.perm.write == false
        &&& self.perm.execute_disable == false
        &&& self.perm.user == false
    }
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

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
}

pub open spec fn usize2present(v: usize) -> bool {
    (v & PAGE_ENTRY_PRESENT_MASK as usize) != 0
}

pub open spec fn usize2ps(v: usize) -> bool {
    (v & PAGE_ENTRY_PS_MASK as usize) != 0
}

pub open spec fn usize2write(v: usize) -> bool {
    (v & PAGE_ENTRY_WRITE_MASK as usize) != 0
}

pub open spec fn usize2execute_disable(v: usize) -> bool {
    (v & PAGE_ENTRY_EXECUTE_MASK as usize) != 0
}

pub open spec fn usize2user(v: usize) -> bool {
    (v & PAGE_ENTRY_USER_MASK as usize) != 0
}

pub open spec fn spec_usize2page_entry_perm(v: usize) -> PageEntryPerm {
    PageEntryPerm {
        present: usize2present(v),
        ps: usize2ps(v),
        write: usize2write(v),
        execute_disable: usize2execute_disable(v),
        user: usize2user(v),
    }
}

pub open spec fn spec_usize2page_entry(v: usize) -> PageEntry {
    PageEntry { addr: spec_usize2pa(v), perm: spec_usize2page_entry_perm(v) }
}

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

pub open spec fn spec_v2l1index(va: usize) -> L1Index {
    (va >> 12 & 0x1ff) as usize
}


// =========================================================================
// COMPLETENESS ROUND 3: Negated/Contradicted Postconditions
// All tests should FAIL (verification errors expected)
// =========================================================================

// Test 1: Assert present is false when bit 0 IS set
proof fn test_fail_negate_present() {
    assert(!usize2present(1usize)) by (bit_vector);
}

// Test 2: Assert write is false when bit 1 IS set
proof fn test_fail_negate_write() {
    assert(!usize2write(2usize)) by (bit_vector);
}

// Test 3: Assert user is false when bit 2 IS set
proof fn test_fail_negate_user() {
    assert(!usize2user(4usize)) by (bit_vector);
}

// Test 4: Assert ps is false when bit 7 IS set
proof fn test_fail_negate_ps() {
    assert(!usize2ps(0x80usize)) by (bit_vector);
}

// Test 5: Assert MEM_valid(0x1000) is false (it IS valid)
proof fn test_fail_negate_mem_valid() {
    assert(!MEM_valid(0x1000usize)) by (bit_vector);
}

// Test 6: Assert page entry from 0 is NOT empty (it IS empty)
proof fn test_fail_negate_is_empty() {
    let entry = spec_usize2page_entry(0usize);
    assert(spec_usize2pa(0usize) == 0usize) by (bit_vector);
    assert(!usize2present(0usize)) by (bit_vector);
    assert(!usize2ps(0usize)) by (bit_vector);
    assert(!usize2write(0usize)) by (bit_vector);
    assert(!usize2execute_disable(0usize)) by (bit_vector);
    assert(!usize2user(0usize)) by (bit_vector);
    assert(!entry.is_empty());
}

// Test 7: Assert v2l1index of 0 is NOT 0 (it IS 0)
proof fn test_fail_negate_l1index_zero() {
    assert(spec_v2l1index(0usize) != 0usize) by (bit_vector);
}

// Test 8: Assert quota subtract DOESN'T hold when it DOES
proof fn test_fail_negate_quota_subtract() {
    let old_q = Quota { mem_4k: 10, mem_2m: 20, mem_1g: 5, pcid: 2, ioid: 1 };
    let new_q = Quota { mem_4k: 7, mem_2m: 20, mem_1g: 5, pcid: 2, ioid: 1 };
    assert(!old_q.spec_subtract_mem_4k(new_q, 3));
}


} // verus!
