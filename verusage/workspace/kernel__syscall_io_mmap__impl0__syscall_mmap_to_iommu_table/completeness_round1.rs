use vstd::prelude::*;

fn main() {}

verus! {

// ===== Type aliases =====
pub type PAddr = usize;
pub type Pcid = usize;

// ===== Constants =====
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;

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

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

// ===== External body functions =====

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

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_usize2pa))]
pub fn usize2pa(v: usize) -> (ret: PAddr)
    ensures
        ret =~= spec_usize2pa(v),
        MEM_valid(ret),
{
    unimplemented!()
}


// =============================================================
// COMPLETENESS ROUND 1: PRECONDITION VIOLATIONS
// All tests should FAIL (verification errors)
// =============================================================

// Test 1: Call page_ptr2page_index with unaligned pointer (violates ptr % 0x1000 == 0)
fn test_precondition_violation_unaligned_ptr() {
    let idx = page_ptr2page_index(4095usize); // FAIL: 4095 % 0x1000 != 0
}

// Test 2: Call page_ptr2page_index with odd pointer (violates ptr % 0x1000 == 0)
fn test_precondition_violation_odd_ptr() {
    let idx = page_ptr2page_index(1usize); // FAIL: 1 % 0x1000 != 0
}

// Test 3: Call page_index2page_ptr with index >= NUM_PAGES (violates 0 <= i < NUM_PAGES)
fn test_precondition_violation_index_too_large() {
    let ptr = page_index2page_ptr(NUM_PAGES); // FAIL: NUM_PAGES is not < NUM_PAGES
}

// Test 4: Call page_index2page_ptr with very large index
fn test_precondition_violation_index_way_too_large() {
    let ptr = page_index2page_ptr(NUM_PAGES + 1000usize); // FAIL: out of range
}

// Test 5: Call page_ptr2page_index with partially aligned ptr
fn test_precondition_violation_partial_align() {
    let idx = page_ptr2page_index(2048usize); // FAIL: 2048 % 0x1000 != 0
}

} // verus!
