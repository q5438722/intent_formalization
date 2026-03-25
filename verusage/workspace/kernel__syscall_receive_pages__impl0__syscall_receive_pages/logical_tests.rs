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

pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;

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

    pub open spec fn spec_is_error(&self) -> bool {
        match self.error_code {
            RetValueType::Error => true,
            _ => false,
        }
    }

    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_is_error))]
    pub fn is_error(&self) -> (ret: bool)
        ensures
            ret == self.is_error(),
    {
        unimplemented!()
    }

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


// ==========================================
// LOGICAL TEST 1: page_ptr2page_index and page_index2page_ptr are exact inverses
// The spec only guarantees page_ptr2page_index returns ptr/4096 and page_index2page_ptr
// returns i*4096. The round-trip property (index2ptr(ptr2index(ptr)) == ptr) may not
// be provable for all valid ptrs because the requires for page_index2page_ptr demands
// i < NUM_PAGES, which is not ensured by the ensures of page_ptr2page_index.
// SHOULD FAIL
// ==========================================
proof fn test_logical_roundtrip_not_guaranteed() {
    // spec_page_ptr2page_index does NOT guarantee result < NUM_PAGES.
    // The spec simply returns ptr / 4096 without bounding it.
    // We test a property that SHOULD NOT hold: for an arbitrary aligned ptr,
    // the index is always < NUM_PAGES.
    // Using a quantifier to express this unbounded claim:
    assert(forall|ptr: usize| ptr % 0x1000 == 0 ==> spec_page_ptr2page_index(ptr) < NUM_PAGES);
    // SHOULD FAIL: spec doesn't bound the result; large aligned ptrs give idx >= NUM_PAGES
}

// ==========================================
// LOGICAL TEST 2: va_4k_valid determinism - asserting stronger property
// The spec defines va_4k_valid via bit operations. We claim that va_4k_valid(0) == true
// but actually 0 >> 39 & 0x1ff == 0 which is < KERNEL_MEM_END_L4INDEX (1), so it's false.
// This tests whether the spec correctly rejects this false claim.
// SHOULD FAIL
// ==========================================
proof fn test_logical_va_4k_valid_zero_is_valid() {
    let ret = va_4k_valid(0);
    assert(ret == true);  // SHOULD FAIL: va=0 has (0>>39 & 0x1ff) == 0 < 1
}

// ==========================================
// LOGICAL TEST 3: NoSwitchNew always returns NoSwitch, but we assert a stronger property:
// that for ANY SyscallReturnStruct with error_code Error, pcid must be None.
// The spec only guarantees this for values produced by NoSwitchNew, not for arbitrary structs.
// We construct an arbitrary struct violating this to test the spec boundary.
// SHOULD FAIL
// ==========================================
proof fn test_logical_arbitrary_struct_pcid_none_with_error() {
    // We can construct a SyscallReturnStruct directly without using NoSwitchNew
    let s = SyscallReturnStruct {
        error_code: RetValueType::Error,
        pcid: Some(42usize),
        cr3: None,
        switch_decision: SwitchDecision::NoSwitch,
    };
    // The spec does NOT guarantee that all error structs have pcid == None
    // Only NoSwitchNew's ensures clause does. So this should verify that pcid is Some.
    // But we assert it's None to test the spec doesn't overgeneralize.
    assert(s.pcid.is_None());  // SHOULD FAIL: we explicitly set pcid to Some(42)
}

// ==========================================
// LOGICAL TEST 4: page_ptr2page_index strict injectivity not guaranteed by spec
// The spec says ret == ptr / 4096. We test whether the verifier can prove that
// two DIFFERENT valid pointers always produce DIFFERENT indices even when the spec
// lacks an explicit injectivity postcondition. Here we deliberately try to prove
// a false claim: that two different aligned ptrs map to the SAME index.
// SHOULD FAIL
// ==========================================
proof fn test_logical_page_ptr2page_index_collision() {
    let idx1 = page_ptr2page_index(0x1000);  // spec: 1
    let idx2 = page_ptr2page_index(0x2000);  // spec: 2
    assert(idx1 == idx2);  // SHOULD FAIL: 1 != 2
}

// ==========================================
// LOGICAL TEST 5: Claiming NoSwitchNew(Error).cr3 is Some
// The spec explicitly ensures cr3.is_None() for NoSwitchNew results.
// We assert cr3.is_Some() to verify the spec rejects this.
// SHOULD FAIL
// ==========================================
fn test_logical_no_switch_new_cr3_some() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::Else);
    assert(ret.cr3.is_Some());  // SHOULD FAIL: spec ensures cr3.is_None()
}

}
