use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus!{

pub type IOid = usize;

pub type CpuId = usize;

pub type ThreadPtr = usize;

pub type ProcPtr = usize;

pub type EndpointIdx = usize;

pub type EndpointPtr = usize;

pub type ContainerPtr = usize;

pub type PagePtr = usize;

pub type PageMapPtr = usize;

pub type Pcid = usize;

pub type PAddr = usize;

pub type VAddr = usize;

pub type L4Index = usize;

pub type L3Index = usize;

pub type L2Index = usize;

pub type L1Index = usize;

pub type SLLIndex = i32;

pub type PagePerm4k = PointsTo<[u8; PAGE_SZ_4k]>;

pub type PagePerm2m = PointsTo<[u8; PAGE_SZ_2m]>;

pub type PagePerm1g = PointsTo<[u8; PAGE_SZ_1g]>;

pub const NUM_CPUS: usize = 32;

pub const MAX_NUM_THREADS_PER_PROC: usize = 128;

pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;

pub const CONTAINER_PROC_LIST_LEN: usize = 10;

pub const CONTAINER_CHILD_LIST_LEN: usize = 10;

pub const PROC_CHILD_LIST_LEN: usize = 10;

pub const CONTAINER_ENDPOINT_LIST_LEN: usize = 10;

pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const PAGE_SZ_4k: usize = 1usize << 12;

pub const PAGE_SZ_2m: usize = 1usize << 21;

pub const PAGE_SZ_1g: usize = 1usize << 30;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;

pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize
    recommends
        page_ptr_valid(ptr),
{
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize
    recommends
        page_index_valid(i),
{
    (i * 4096) as usize
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

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_va_4k_valid))]
pub fn va_4k_valid(va: usize) -> (ret: bool)
    ensures
        ret == spec_va_4k_valid(va),
{
    unimplemented!()
}


// ==========================================
// BOUNDARY TEST 1: page_ptr2page_index with non-aligned pointer
// The requires clause demands ptr % 0x1000 == 0.
// Calling with ptr=1 violates this precondition.
// SHOULD FAIL
// ==========================================
fn test_boundary_page_ptr2page_index_non_aligned() {
    // ptr = 1 is not aligned to 0x1000, violating the precondition
    let ret = page_ptr2page_index(1);  // SHOULD FAIL
}

// ==========================================
// BOUNDARY TEST 2: page_index2page_ptr with index out of range
// The requires clause demands 0 <= i < NUM_PAGES (2*1024*1024).
// Calling with i = NUM_PAGES violates this precondition.
// SHOULD FAIL
// ==========================================
fn test_boundary_page_index2page_ptr_out_of_range() {
    // i = NUM_PAGES is out of the valid range
    let ret = page_index2page_ptr(NUM_PAGES);  // SHOULD FAIL
}

// ==========================================
// BOUNDARY TEST 3: page_ptr2page_index with odd number
// ptr = 0x1001 is not aligned to 4096, violating requires
// SHOULD FAIL
// ==========================================
fn test_boundary_page_ptr2page_index_off_by_one() {
    let ret = page_ptr2page_index(0x1001);  // SHOULD FAIL
}

// ==========================================
// BOUNDARY TEST 4: page_index2page_ptr with usize::MAX
// Massively out of range, violating requires
// SHOULD FAIL
// ==========================================
fn test_boundary_page_index2page_ptr_max_value() {
    let ret = page_index2page_ptr(usize::MAX);  // SHOULD FAIL
}

// ==========================================
// BOUNDARY TEST 5: page_ptr2page_index with 0 then assert wrong result
// ptr = 0 IS aligned to 0x1000 (0 % 0x1000 == 0), so requires is met.
// The spec says ret == ptr / 4096 == 0.
// We assert ret != 0 to check the spec rejects this wrong claim.
// SHOULD FAIL
// ==========================================
fn test_boundary_page_ptr2page_index_zero_wrong_result() {
    let ret = page_ptr2page_index(0);
    assert(ret != 0);  // SHOULD FAIL: spec says ret == 0
}

}
