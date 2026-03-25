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
    recommends page_ptr_valid(ptr),
{ (ptr / 4096usize) as usize }

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize
    recommends page_index_valid(i),
{ (i * 4096) as usize }

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_page_ptr2page_index))]
pub fn page_ptr2page_index(ptr: usize) -> (ret: usize)
    requires ptr % 0x1000 == 0,
    ensures ret == spec_page_ptr2page_index(ptr),
{ unimplemented!() }

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_page_index2page_ptr))]
pub fn page_index2page_ptr(i: usize) -> (ret: usize)
    requires 0 <= i < NUM_PAGES,
    ensures ret == spec_page_index2page_ptr(i),
{ unimplemented!() }

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64) >= KERNEL_MEM_END_L4INDEX as u64
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_va_4k_valid))]
pub fn va_4k_valid(va: usize) -> (ret: bool)
    ensures ret == spec_va_4k_valid(va),
{ unimplemented!() }

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwitchDecision { NoSwitch, NoThread, Switch }

#[allow(inconsistent_fields)]
#[derive(Clone, Copy)]
pub enum RetValueType {
    SuccessUsize { value: usize },
    SuccessSeqUsize { value: Ghost<Seq<usize>> },
    SuccessPairUsize { value1: usize, value2: usize },
    SuccessThreeUsize { value1: usize, value2: usize, value3: usize },
    ErrorNoQuota, ErrorVaInUse, CpuIdle, Error, Else, NoQuota, VaInUse,
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
        match self.error_code { RetValueType::Error => true, _ => false }
    }

    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_is_error))]
    pub fn is_error(&self) -> (ret: bool)
        ensures ret == self.is_error(),
    { unimplemented!() }

    #[verifier::external_body]
    pub fn NoSwitchNew(error_code: RetValueType) -> (ret: Self)
        ensures
            ret.error_code == error_code,
            ret.pcid.is_None(),
            ret.cr3.is_None(),
            ret.switch_decision == SwitchDecision::NoSwitch,
    { unimplemented!() }
}


// =====================================================================
// BOUNDARY TESTS: Violate preconditions (requires) with invalid inputs
// =====================================================================

// SHOULD FAIL: ptr=1 is not 4096-aligned
fn test_boundary_1_non_aligned_ptr() {
    let ret = page_ptr2page_index(1);
}

// SHOULD FAIL: i=NUM_PAGES is out of valid range [0, NUM_PAGES)
fn test_boundary_2_index_out_of_range() {
    let ret = page_index2page_ptr(NUM_PAGES);
}

// SHOULD FAIL: ptr=0x1001 is not 4096-aligned
fn test_boundary_3_off_by_one_alignment() {
    let ret = page_ptr2page_index(0x1001);
}

// SHOULD FAIL: i=usize::MAX is massively out of range
fn test_boundary_4_max_index() {
    let ret = page_index2page_ptr(usize::MAX);
}

// SHOULD FAIL: ptr=0 gives ret=0, asserting ret!=0 is wrong
fn test_boundary_5_zero_ptr_wrong_result() {
    let ret = page_ptr2page_index(0);
    assert(ret != 0);
}


// =====================================================================
// BEHAVIORAL MUTATION TESTS: Mutate expected outputs or relations
// =====================================================================

// SHOULD FAIL: ptr=0x2000 gives ret=2, not 3
fn test_behavioral_1_wrong_index() {
    let ret = page_ptr2page_index(0x2000);
    assert(ret == 3);
}

// SHOULD FAIL: i=5 gives ret=20480, not 0
fn test_behavioral_2_wrong_ptr() {
    let ret = page_index2page_ptr(5);
    assert(ret == 0);
}

// SHOULD FAIL: NoSwitchNew(Error) IS an error
fn test_behavioral_3_error_is_not_error() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::Error);
    assert(!ret.is_error());
}

// SHOULD FAIL: NoSwitchNew(Else) is NOT an error
fn test_behavioral_4_else_is_error() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::Else);
    assert(ret.is_error());
}

// SHOULD FAIL: NoSwitchNew guarantees NoSwitch, not Switch
fn test_behavioral_5_switch_decision_mutated() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::Error);
    assert(ret.switch_decision == SwitchDecision::Switch);
}


// =====================================================================
// LOGICAL TESTS: Properties NOT explicitly guaranteed by the spec
// =====================================================================

// SHOULD FAIL: spec_page_ptr2page_index doesn't bound result < NUM_PAGES for all aligned ptrs
proof fn test_logical_1_unbounded_index() {
    assert(forall|ptr: usize| ptr % 0x1000 == 0 ==> spec_page_ptr2page_index(ptr) < NUM_PAGES);
}

// SHOULD FAIL: va=0 has (0>>39 & 0x1ff)==0 < KERNEL_MEM_END_L4INDEX, so va_4k_valid(0)==false
proof fn test_logical_2_va_zero_valid() {
    let ret = va_4k_valid(0);
    assert(ret == true);
}

// SHOULD FAIL: directly constructed struct with pcid=Some(42) is not None
proof fn test_logical_3_arbitrary_struct_pcid() {
    let s = SyscallReturnStruct {
        error_code: RetValueType::Error,
        pcid: Some(42usize),
        cr3: None,
        switch_decision: SwitchDecision::NoSwitch,
    };
    assert(s.pcid.is_None());
}

// SHOULD FAIL: different aligned ptrs (0x1000, 0x2000) give different indices (1 vs 2)
proof fn test_logical_4_false_collision() {
    let idx1 = spec_page_ptr2page_index(0x1000);
    let idx2 = spec_page_ptr2page_index(0x2000);
    assert(idx1 == idx2);
}

// SHOULD FAIL: NoSwitchNew ensures cr3.is_None(), not is_Some()
fn test_logical_5_cr3_some() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::Else);
    assert(ret.cr3.is_Some());
}

}
