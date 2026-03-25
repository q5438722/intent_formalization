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

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_usize2pa))]
pub fn usize2pa(v: usize) -> (ret: PAddr)
    ensures
        ret =~= spec_usize2pa(v),
        MEM_valid(ret),
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
// COMPLETENESS ROUND 3: NEGATED POSTCONDITIONS
// All tests should FAIL (verification errors)
// =============================================================

// Test 1: Assert NoSwitchNew returns Some pcid — contradicts ensures pcid.is_None()
fn test_negated_no_switch_has_pcid() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::Error);
    assert(ret.pcid.is_Some()); // FAIL: ensures says pcid.is_None()
}

// Test 2: Assert NoSwitchNew has Switch decision — contradicts ensures NoSwitch
fn test_negated_no_switch_is_switch() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::Error);
    assert(ret.switch_decision == SwitchDecision::Switch); // FAIL: ensures says NoSwitch
}

// Test 3: Assert NoSwitchNew has Some cr3 — contradicts ensures cr3.is_None()
fn test_negated_no_switch_has_cr3() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::Error);
    assert(ret.cr3.is_Some()); // FAIL: ensures says cr3.is_None()
}

// Test 4: Assert usize2pa result is NOT MEM_valid — contradicts ensures
fn test_negated_usize2pa_not_mem_valid() {
    let pa = usize2pa(0usize);
    assert(!MEM_valid(pa)); // FAIL: ensures says MEM_valid(ret)
}

// Test 5: Assert page_ptr2page_index gives wrong result — contradicts ensures
fn test_negated_page_ptr2index_wrong() {
    let idx = page_ptr2page_index(4096usize);
    assert(idx != spec_page_ptr2page_index(4096usize)); // FAIL: ensures says idx == spec result
}

} // verus!
