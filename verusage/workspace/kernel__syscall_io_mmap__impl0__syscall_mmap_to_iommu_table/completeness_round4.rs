use vstd::prelude::*;

fn main() {}

verus! {

// ===== Type aliases =====
pub type PAddr = usize;

// ===== Constants =====
pub const NUM_PAGES: usize = 2 * 1024 * 1024;

// ===== Open spec functions =====

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

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

pub open spec fn page_index_2m_valid(i: usize) -> bool {
    &&& i % 512 == 0
    &&& 0 <= i < NUM_PAGES
}

pub open spec fn page_ptr_2m_valid(ptr: usize) -> bool {
    ((ptr % (0x200000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn spec_page_index_truncate_2m(index: usize) -> usize {
    (index / 512usize * 512usize) as usize
}

// ===== Structs =====

#[derive(Clone,Debug)]
pub struct PageEntryPerm {
    pub present: bool,
    pub ps: bool,
    pub write: bool,
    pub execute_disable: bool,
    pub user: bool,
}

#[derive(Clone,Debug)]
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


// =============================================================
// COMPLETENESS ROUND 4: WRONG CONCRETE VALUES
// All tests should FAIL (verification errors)
// =============================================================

// Test 1: Assert spec_page_ptr2page_index(4096) == 2 — wrong, should be 1
proof fn test_wrong_value_ptr2index() {
    assert(spec_page_ptr2page_index(4096usize) == 2usize); // FAIL: correct answer is 1
}

// Test 2: Assert spec_page_index2page_ptr(1) == 8192 — wrong, should be 4096
proof fn test_wrong_value_index2ptr() {
    assert(spec_page_index2page_ptr(1usize) == 8192usize); // FAIL: correct answer is 4096
}

// Test 3: Assert page_ptr_valid(4095) — wrong, 4095 is not aligned
proof fn test_wrong_value_ptr_valid_unaligned() {
    assert(page_ptr_valid(4095usize)); // FAIL: 4095 % 0x1000 != 0
}

// Test 4: Assert page_index_valid(NUM_PAGES) — wrong, NUM_PAGES is out of range
proof fn test_wrong_value_index_valid_max() {
    assert(page_index_valid(NUM_PAGES)); // FAIL: NUM_PAGES is not < NUM_PAGES
}

// Test 5: Assert !page_ptr_valid(0) — wrong, 0 IS a valid page ptr
proof fn test_wrong_value_zero_invalid() {
    assert(!page_ptr_valid(0usize)); // FAIL: 0 is valid (0%0x1000==0, 0/0x1000==0 < NUM_PAGES)
}

// Test 6: Assert spec_page_index_truncate_2m(513) == 0 — wrong, should be 512
proof fn test_wrong_value_truncate_2m() {
    assert(spec_page_index_truncate_2m(513usize) == 0usize); // FAIL: 513/512*512 = 512
}

// Test 7: Assert PageEntry with nonzero addr is empty — wrong
proof fn test_wrong_value_page_entry_empty() {
    let entry = PageEntry {
        addr: 4096,
        perm: PageEntryPerm {
            present: false,
            ps: false,
            write: false,
            execute_disable: false,
            user: false,
        },
    };
    assert(entry.is_empty()); // FAIL: addr != 0
}

} // verus!
