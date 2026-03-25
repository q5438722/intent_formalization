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

// SyscallReturnStruct types

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwitchDecision {
    NoSwitch,
    NoThread,
    Switch,
}

#[allow(inconsistent_fields)]
#[derive(Clone, Copy)]
pub enum RetValueType {
    SuccessUsize { value: usize },
    SuccessSeqUsize { value: Ghost<Seq<usize>> },
    SuccessPairUsize { value1: usize, value2: usize },
    SuccessThreeUsize { value1: usize, value2: usize, value3: usize },
    ErrorNoQuota,
    ErrorVaInUse,
    CpuIdle,
    Error,
    Else,
    NoQuota,
    VaInUse,
}

#[derive(Clone, Copy)]
pub struct SyscallReturnStruct {
    pub error_code: RetValueType,
    pub pcid: Option<Pcid>,
    pub cr3: Option<usize>,
    pub switch_decision: SwitchDecision,
}

impl SyscallReturnStruct {
    #[verifier::external_body]
    pub fn NoSwitchNew(error_code: RetValueType) -> (ret: Self)
        ensures
            ret.error_code == error_code,
            ret.pcid.is_None(),
            ret.cr3.is_None(),
            ret.switch_decision == SwitchDecision::NoSwitch,
    {
        unimplemented!()
    }
}


// =============================================================
// COMPLETENESS ROUND 5: CROSS-FUNCTION MISUSE & EDGE CASES
// All tests should FAIL (verification errors)
// =============================================================

// Test 1: Assert ptr2index == index2ptr for same input — they are inverses, not equal
proof fn test_cross_function_swap(i: usize)
    requires
        page_index_valid(i),
        i > 1usize,
{
    // ptr2index(i) = i/4096, index2ptr(i) = i*4096; these are NOT equal for i > 1
    assert(spec_page_ptr2page_index(i) == spec_page_index2page_ptr(i)); // FAIL
}

// Test 2: Assert page_index_valid implies page_index_2m_valid — wrong, 2m requires 512-alignment
proof fn test_cross_index_valid_implies_2m(i: usize)
    requires
        page_index_valid(i),
{
    assert(page_index_2m_valid(i)); // FAIL: page_index_valid doesn't require i % 512 == 0
}

// Test 3: Assert page_ptr_valid implies page_ptr_2m_valid — wrong, 2m requires 0x200000 alignment
proof fn test_cross_ptr_valid_implies_2m(ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(page_ptr_2m_valid(ptr)); // FAIL: 4096-alignment doesn't imply 0x200000-alignment
}

// Test 4: Assert truncate_2m is identity for any index — only true for 512-aligned
proof fn test_cross_truncate_is_identity(i: usize)
    requires
        page_index_valid(i),
{
    assert(spec_page_index_truncate_2m(i) == i); // FAIL: not true for unaligned i
}

// Test 5: Assert NoSwitchNew error_code is always Error regardless of input
fn test_cross_no_switch_wrong_error_code() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::ErrorNoQuota);
    assert(ret.error_code == RetValueType::Error); // FAIL: we passed ErrorNoQuota, not Error
}

} // verus!
