use vstd::prelude::*;

fn main() {}

verus!{

pub type IOid = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type ThreadPtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;
pub const PAGE_ENTRY_USER_SHIFT: u64 = 2;
pub const PAGE_ENTRY_PS_SHIFT: u64 = 7;
pub const PAGE_ENTRY_EXECUTE_SHIFT: u64 = 63;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;
pub const PAGE_ENTRY_USER_MASK: u64 = 0x1u64 << PAGE_ENTRY_USER_SHIFT;
pub const PAGE_ENTRY_PS_MASK: u64 = 0x1u64 << PAGE_ENTRY_PS_SHIFT;
pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x1u64 << PAGE_ENTRY_EXECUTE_SHIFT;

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
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
        ps: usize2ps(v),
        write: usize2write(v),
        execute_disable: usize2execute_disable(v),
        user: usize2user(v),
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
#[verifier(when_used_as_spec(spec_usize2pa))]
pub fn usize2pa(v: usize) -> (ret: PAddr)
    ensures
        ret =~= spec_usize2pa(v),
        MEM_valid(ret),
{
    unimplemented!()
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize {
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize {
    (i * 4096) as usize
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
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

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs but asserts INCORRECT output relations.
// These mutate expected behaviors to check if the spec rejects them.
// All tests SHOULD FAIL verification.

// Test 1: NoSwitchNew should return the given error_code.
// Mutate: assert that the returned error_code is Error when we passed ErrorNoQuota.
// SHOULD FAIL
fn test_mutation_noswitch_wrong_error_code() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::ErrorNoQuota);
    assert(ret.error_code == RetValueType::Error);
}

// Test 2: NoSwitchNew guarantees switch_decision == NoSwitch.
// Mutate: assert it is Switch instead.
// SHOULD FAIL
fn test_mutation_noswitch_wrong_switch_decision() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::Error);
    assert(ret.switch_decision == SwitchDecision::Switch);
}

// Test 3: NoSwitchNew guarantees pcid.is_None().
// Mutate: assert pcid is Some.
// SHOULD FAIL
fn test_mutation_noswitch_pcid_is_some() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::Error);
    assert(ret.pcid.is_Some());
}

// Test 4: NoSwitchNew guarantees cr3.is_None().
// Mutate: assert cr3 is Some.
// SHOULD FAIL
fn test_mutation_noswitch_cr3_is_some() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::Error);
    assert(ret.cr3.is_Some());
}

// Test 5: spec_page_ptr2page_index(4096) == 1.
// Mutate: assert it equals 2.
// SHOULD FAIL
proof fn test_mutation_page_ptr2index_wrong() {
    assert(spec_page_ptr2page_index(4096) == 2);
}

// Test 6: spec_page_index2page_ptr(1) == 4096.
// Mutate: assert it equals 8192.
// SHOULD FAIL
proof fn test_mutation_page_index2ptr_wrong() {
    assert(spec_page_index2page_ptr(1) == 8192);
}

// Test 7: usize2page_entry_perm(0) should have present == false.
// Mutate: assert present == true for input 0.
// SHOULD FAIL
proof fn test_mutation_perm_zero_present_true() {
    let perm = spec_usize2page_entry_perm(0);
    assert(perm.present == true);
}

// Test 8: spec_usize2pa(0) should be 0 (0 & MEM_MASK == 0).
// Mutate: assert it is non-zero.
// SHOULD FAIL
proof fn test_mutation_usize2pa_zero_nonzero() {
    assert(spec_usize2pa(0) != 0);
}

// Test 9: For io_space_range_free, if all entries in va_range are NOT
// in the io_space domain, the range is free. Mutate: assert that
// an entry IS in the domain when the domain is empty.
// SHOULD FAIL
proof fn test_mutation_io_space_range_free_wrong() {
    let io_space_dom = Map::<VAddr, MapEntry>::empty();
    let va: VAddr = 0x1000usize;
    assert(io_space_dom.dom().contains(va));
}

// Test 10: spec_va_4k_valid should return false for va = 0.
// (va = 0: (0 >> 39) & 0x1ff = 0, which is < 1 = KERNEL_MEM_END_L4INDEX)
// Mutate: assert it is true.
// SHOULD FAIL
proof fn test_mutation_va_4k_valid_zero_is_valid() {
    assert(spec_va_4k_valid(0));
}

}
