use vstd::prelude::*;

fn main() {}

verus! {

// ===== Type aliases =====
pub type PAddr = usize;
pub type Pcid = usize;

// ===== Constants =====
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;
pub const PAGE_ENTRY_USER_SHIFT: u64 = 2;
pub const PAGE_ENTRY_PS_SHIFT: u64 = 7;
pub const PAGE_ENTRY_EXECUTE_SHIFT: u64 = 63;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;
pub const PAGE_ENTRY_USER_MASK: u64 = 0x1u64 << PAGE_ENTRY_USER_SHIFT;
pub const PAGE_ENTRY_PS_MASK: u64 = 0x1u64 << PAGE_ENTRY_PS_SHIFT;
pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x1u64 << PAGE_ENTRY_EXECUTE_SHIFT;

// ===== Open spec functions =====

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
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

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

pub open spec fn usize2present(v: usize) -> bool {
    (v & PAGE_ENTRY_PRESENT_MASK as usize) != 0
}

pub open spec fn usize2write(v: usize) -> bool {
    (v & PAGE_ENTRY_WRITE_MASK as usize) != 0
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

pub open spec fn spec_usize2page_entry_perm(v: usize) -> PageEntryPerm {
    PageEntryPerm {
        present: usize2present(v),
        ps: (v & PAGE_ENTRY_PS_MASK as usize) != 0,
        write: usize2write(v),
        execute_disable: (v & PAGE_ENTRY_EXECUTE_MASK as usize) != 0,
        user: (v & PAGE_ENTRY_USER_MASK as usize) != 0,
    }
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_usize2page_entry_perm))]
pub fn usize2page_entry_perm(v: usize) -> (ret: PageEntryPerm)
    ensures
        ret =~= spec_usize2page_entry_perm(v),
        v == 0 ==> ret.present == false && ret.ps == false && ret.write == false
            && ret.execute_disable == false && ret.user == false,
{
    unimplemented!()
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_page_ptr2page_index))]
pub fn page_ptr2page_index(ptr: usize) -> (ret: usize)
    requires
        ptr % 0x1000 == 0,
    ensures
        ret == spec_page_ptr2page_index(ptr),
{
    unimplemented!()
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_page_index2page_ptr))]
pub fn page_index2page_ptr(i: usize) -> (ret: usize)
    requires
        0 <= i < NUM_PAGES,
    ensures
        ret == spec_page_index2page_ptr(i),
{
    unimplemented!()
}


// =============================================================
// COMPLETENESS ROUND 2: OVERLY STRONG POSTCONDITIONS
// All tests should FAIL (verification errors)
// =============================================================

// Test 1: Assert page_ptr2page_index always returns 0 — too strong
fn test_overly_strong_always_zero() {
    let idx = page_ptr2page_index(4096usize);
    assert(idx == 0usize); // FAIL: spec says idx == 4096/4096 == 1, not 0
}

// Test 2: Assert page_index2page_ptr always returns 0 — too strong
fn test_overly_strong_ptr_always_zero() {
    let ptr = page_index2page_ptr(1usize);
    assert(ptr == 0usize); // FAIL: spec says ptr == 1*4096 == 4096, not 0
}

// Test 3: Assert page_ptr2page_index result is bounded by 10 for any valid ptr — too strong
proof fn test_overly_strong_tight_bound(ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(spec_page_ptr2page_index(ptr) < 10usize); // FAIL: valid ptrs can have index up to NUM_PAGES-1
}

// Test 4: Assert usize2page_entry_perm for arbitrary v always has present == false — too strong
fn test_overly_strong_perm_always_not_present(v: usize) {
    let perm = usize2page_entry_perm(v);
    assert(perm.present == false); // FAIL: only guaranteed for v == 0
}

// Test 5: Assert page_ptr_valid implies ptr < 4096 — too restrictive
proof fn test_overly_strong_valid_implies_small(ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(ptr < 4096usize); // FAIL: valid ptrs range from 0 to (NUM_PAGES-1)*4096
}

} // verus!
