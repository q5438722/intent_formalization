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
pub type PagePtr = usize;

pub const NUM_CPUS: usize = 32;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const PCID_MAX: usize = 4096;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
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

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwitchDecision {
    NoSwitch,
    NoThread,
    Switch,
}

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// These test for unintended reasoning: determinism, injectivity,
// stronger bounds, cross-function misuse, and structural assumptions.
// All tests SHOULD FAIL verification.

// Test 1: schedule_idle_cpu ensures only self.wf(). It does NOT
// guarantee the return is always Switch. Claim it always switches.
// SHOULD FAIL
proof fn test_logical_schedule_always_switches(switch_decision: SwitchDecision)
    requires
        switch_decision == SwitchDecision::NoSwitch
        || switch_decision == SwitchDecision::NoThread
        || switch_decision == SwitchDecision::Switch,
{
    assert(switch_decision == SwitchDecision::Switch);
}

// Test 2: schedule_idle_cpu's return value is NOT deterministic.
// Two calls with same wf() kernel could return different decisions.
// Claim: two SwitchDecisions must be equal.
// SHOULD FAIL
proof fn test_logical_schedule_deterministic(
    decision1: SwitchDecision,
    decision2: SwitchDecision,
)
{
    assert(decision1 == decision2);
}

// Test 3: usize2pa is NOT injective: different inputs can map to
// the same physical address (masking loses bits).
// Claim: different inputs always give different outputs.
// SHOULD FAIL
proof fn test_logical_usize2pa_injective() {
    let v1: usize = 0x1000;
    let v2: usize = 0x1001;
    assert(spec_usize2pa(v1) != spec_usize2pa(v2));
}

// Test 4: page_ptr2page_index followed by page_index2page_ptr
// is NOT identity for non-aligned values.
// Claim: roundtrip works for any value.
// SHOULD FAIL
proof fn test_logical_ptr_index_roundtrip_arbitrary() {
    let ptr: usize = 0x1234;
    assert(spec_page_index2page_ptr(spec_page_ptr2page_index(ptr)) == ptr);
}

// Test 5: va_4k_valid does NOT imply page_ptr_valid.
// They operate on different address domains (virtual vs physical).
// Claim: va_4k_valid implies page_ptr_valid.
// SHOULD FAIL
proof fn test_logical_va_valid_implies_page_ptr_valid(va: VAddr)
    requires
        spec_va_4k_valid(va),
{
    assert(page_ptr_valid(va));
}

// Test 6: The spec does NOT guarantee that NoSwitch and NoThread
// can never both describe the same scenario. They are distinct variants
// but the spec doesn't prove they are distinguishable from ensures alone.
// Claim: NoSwitch == NoThread.
// SHOULD FAIL
proof fn test_logical_noswitch_equals_nothread() {
    assert(SwitchDecision::NoSwitch == SwitchDecision::NoThread);
}

// Test 7: schedule_idle_cpu ensures self.wf() but does NOT guarantee
// that scheduler length decreases by exactly 1 in all cases.
// (It only decreases when Switch happens, not on early returns.)
// Claim: scheduler always shrinks by 1.
// SHOULD FAIL
proof fn test_logical_scheduler_always_shrinks(
    old_len: usize,
    new_len: usize,
)
    requires
        old_len > 0,
{
    assert(new_len == old_len - 1);
}

// Test 8: pop_scheduler_for_idle_cpu preserves all proc data.
// But the spec does NOT guarantee that all THREAD data is preserved.
// (The popped thread's state changes to RUNNING.)
// Claim: the thread that was popped was already RUNNING before being popped.
// This contradicts the scheduler design where scheduled threads are SCHEDULED.
// SHOULD FAIL
proof fn test_logical_popped_thread_was_already_running(
    old_state_is_running: bool,
    new_state_is_running: bool,
)
    requires
        new_state_is_running == true,
        // old state should have been SCHEDULED, not RUNNING
        old_state_is_running == false,
{
    // Wrong claim: old state was already RUNNING
    assert(old_state_is_running == true);
}

// Test 9: schedule_idle_cpu returns SwitchNew with some cr3.
// The spec does NOT guarantee cr3 > 0 or any particular value.
// Claim: cr3 returned is always non-zero.
// SHOULD FAIL
proof fn test_logical_cr3_always_nonzero(cr3: usize) {
    assert(cr3 != 0);
}

// Test 10: schedule_idle_cpu returns SwitchNew with some pcid.
// The spec does NOT guarantee pcid < NUM_CPUS.
// Claim: pcid < NUM_CPUS always.
// SHOULD FAIL
proof fn test_logical_pcid_less_than_num_cpus(pcid: Pcid) {
    assert(pcid < NUM_CPUS);
}

// Test 11: The spec says container domains are preserved by
// pop_scheduler_for_idle_cpu. But it does NOT say the
// owning_container's scheduler content is fully determined.
// Claim: scheduler is always empty after pop.
// SHOULD FAIL
proof fn test_logical_scheduler_empty_after_pop(
    old_len: usize,
    new_len: usize,
)
    requires
        old_len > 0,
        // pop_scheduler ensures old scheduler.skip(1) == new scheduler
{
    assert(new_len == 0);
}

// Test 12: schedule_idle_cpu does NOT guarantee that the
// endpoint_dom, container_dom, thread_dom, and proc_dom
// are all identical. Only pop_scheduler_for_idle_cpu ensures
// domain preservation. From schedule_idle_cpu's ensures (self.wf()),
// we cannot derive specific domain relationships.
// Claim: thread_dom is always a subset of proc_dom.
// SHOULD FAIL
proof fn test_logical_thread_dom_subset_proc_dom(
    thread_dom: Set<ThreadPtr>,
    proc_dom: Set<ProcPtr>,
) {
    assert(thread_dom.subset_of(proc_dom));
}

}
