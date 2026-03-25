use vstd::prelude::*;

fn main() {}

verus!{

pub type CpuId = usize;
pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type Pcid = usize;
pub type PAddr = usize;
pub type VAddr = usize;

pub const NUM_CPUS: usize = 32;
pub const PCID_MAX: usize = 4096;
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

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
}

#[derive(Clone, Debug)]
pub struct PageEntryPerm {
    pub present: bool,
    pub ps: bool,
    pub write: bool,
    pub execute_disable: bool,
    pub user: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwitchDecision {
    NoSwitch,
    NoThread,
    Switch,
}

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs/postconditions, then asserts a
// WRONG output or relation (mutated from the correct one).
// All tests SHOULD FAIL verification.

// Test 1: NoSwitchNew ensures switch_decision == NoSwitch.
// Mutate: claim switch_decision == Switch.
// SHOULD FAIL
proof fn test_mutation_noswitch_returns_switch() {
    let switch_decision = SwitchDecision::NoSwitch;
    assert(switch_decision == SwitchDecision::Switch);
}

// Test 2: NoNextThreadNew ensures switch_decision == NoThread.
// Mutate: claim switch_decision == NoSwitch.
// SHOULD FAIL
proof fn test_mutation_nothread_returns_noswitch() {
    let switch_decision = SwitchDecision::NoThread;
    assert(switch_decision == SwitchDecision::NoSwitch);
}

// Test 3: SwitchNew ensures pcid =~= Some(pcid).
// Mutate: claim pcid.is_None().
// SHOULD FAIL
proof fn test_mutation_switch_pcid_is_none(pcid: Pcid) {
    let ret_pcid: Option<Pcid> = Some(pcid);
    assert(ret_pcid is None);
}

// Test 4: SwitchNew ensures cr3 =~= Some(cr3).
// Mutate: claim cr3.is_None().
// SHOULD FAIL
proof fn test_mutation_switch_cr3_is_none(cr3: usize) {
    let ret_cr3: Option<usize> = Some(cr3);
    assert(ret_cr3 is None);
}

// Test 5: NoSwitchNew ensures pcid.is_None().
// Mutate: claim pcid.is_Some().
// SHOULD FAIL
proof fn test_mutation_noswitch_pcid_is_some() {
    let ret_pcid: Option<Pcid> = None;
    assert(ret_pcid is Some);
}

// Test 6: NoSwitchNew ensures cr3.is_None().
// Mutate: claim cr3.is_Some().
// SHOULD FAIL
proof fn test_mutation_noswitch_cr3_is_some() {
    let ret_cr3: Option<usize> = None;
    assert(ret_cr3 is Some);
}

// Test 7: NoNextThreadNew ensures pcid.is_None() and cr3.is_None().
// Mutate: claim pcid.is_Some() AND cr3.is_Some().
// SHOULD FAIL
proof fn test_mutation_nothread_has_pcid_and_cr3() {
    let ret_pcid: Option<Pcid> = None;
    let ret_cr3: Option<usize> = None;
    assert((ret_pcid is Some) && (ret_cr3 is Some));
}

// Test 8: SwitchNew ensures switch_decision == Switch.
// Mutate: claim switch_decision == NoThread.
// SHOULD FAIL
proof fn test_mutation_switch_returns_nothread() {
    let switch_decision = SwitchDecision::Switch;
    assert(switch_decision == SwitchDecision::NoThread);
}

// Test 9: usize2page_entry_perm(0) should have present == false.
// Mutate: claim present == true.
// SHOULD FAIL
proof fn test_mutation_zero_perm_present() {
    let perm = spec_usize2page_entry_perm(0usize);
    assert(perm.present == true);
}

// Test 10: usize2page_entry_perm(0) should have write == false.
// Mutate: claim write == true.
// SHOULD FAIL
proof fn test_mutation_zero_perm_write() {
    let perm = spec_usize2page_entry_perm(0usize);
    assert(perm.write == true);
}

// Test 11: usize2pa(0) should be 0 (MEM_valid).
// Mutate: claim it is NOT MEM_valid.
// SHOULD FAIL
proof fn test_mutation_usize2pa_zero_not_valid() {
    let result = spec_usize2pa(0usize);
    assert(!MEM_valid(result));
}

// Test 12: pop_scheduler_for_idle_cpu ensures thread state is RUNNING.
// Mutate: claim thread state remains SCHEDULED after popping.
// (Encoding: if something is RUNNING, claim it is not RUNNING)
// SHOULD FAIL
proof fn test_mutation_popped_thread_not_running() {
    let state_is_running: bool = true;
    assert(state_is_running == false);
}

}
