use vstd::prelude::*;

fn main() {}

verus!{

pub type CpuId = usize;
pub type ThreadPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type ProcPtr = usize;
pub type PagePtr = usize;
pub type Pcid = usize;

pub const NUM_CPUS: usize = 32;
pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThreadState {
    SCHEDULED,
    BLOCKED,
    RUNNING,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EndpointState {
    RECEIVE,
    SEND,
}

#[derive(Clone, Copy, Debug)]
pub enum SwitchDecision {
    NoSwitch,
    NoThread,
    Switch,
}

#[derive(Clone, Copy)]
pub enum RetValueType {
    Error,
    Else,
}

#[derive(Clone, Copy)]
pub struct SyscallReturnStruct {
    pub error_code: RetValueType,
    pub pcid: Option<Pcid>,
    pub cr3: Option<usize>,
    pub switch_decision: SwitchDecision,
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize {
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize {
    (i * 4096) as usize
}

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs/postconditions, then asserts
// a WRONG output or relation (mutated from the correct one).
// All tests SHOULD FAIL verification.

// Test 1: NoSwitchNew ensures error_code matches input.
// Mutate: claim error_code does NOT match.
// SHOULD FAIL
proof fn test_mutation_noswitchnew_wrong_error_code(
    ret_error_code: RetValueType,
    input_error_code: RetValueType,
)
    requires
        ret_error_code == input_error_code,
        input_error_code == RetValueType::Error,
{
    assert(ret_error_code == RetValueType::Else);
}

// Test 2: NoSwitchNew ensures pcid.is_None().
// Mutate: claim pcid is Some.
// SHOULD FAIL
proof fn test_mutation_noswitchnew_pcid_not_none(pcid: Option<Pcid>)
    requires
        pcid.is_None(),
{
    assert(pcid.is_Some());
}

// Test 3: NoSwitchNew ensures switch_decision == NoSwitch.
// Mutate: claim switch_decision == Switch.
// SHOULD FAIL
proof fn test_mutation_noswitchnew_wrong_switch(switch: SwitchDecision)
    requires
        switch == SwitchDecision::NoSwitch,
{
    assert(switch == SwitchDecision::Switch);
}

// Test 4: NoSwitchNew ensures cr3.is_None().
// Mutate: claim cr3 is Some.
// SHOULD FAIL
proof fn test_mutation_noswitchnew_cr3_not_none(cr3: Option<usize>)
    requires
        cr3.is_None(),
{
    assert(cr3.is_Some());
}

// Test 5: is_send() ensures ret == (self == SEND).
// Mutate: RECEIVE endpoint claims is_send == true.
// SHOULD FAIL
proof fn test_mutation_is_send_on_receive(state: EndpointState, ret: bool)
    requires
        state == EndpointState::RECEIVE,
        ret == (state == EndpointState::SEND),
{
    assert(ret == true);
}

// Test 6: is_receive() ensures ret == (self == RECEIVE).
// Mutate: SEND endpoint claims is_receive == true.
// SHOULD FAIL
proof fn test_mutation_is_receive_on_send(state: EndpointState, ret: bool)
    requires
        state == EndpointState::SEND,
        ret == (state == EndpointState::RECEIVE),
{
    assert(ret == true);
}

// Test 7: schedule_running_thread ensures the current thread's state
// becomes SCHEDULED. Mutate: claim state is still RUNNING.
// SHOULD FAIL
proof fn test_mutation_scheduled_thread_still_running(new_state: ThreadState)
    requires
        new_state == ThreadState::SCHEDULED,
{
    assert(new_state == ThreadState::RUNNING);
}

// Test 8: schedule_running_thread ensures cpu.current_thread.is_None() after.
// Mutate: claim current_thread is still Some.
// SHOULD FAIL
proof fn test_mutation_schedule_cpu_still_has_thread(current_thread: Option<ThreadPtr>)
    requires
        current_thread.is_None(),
{
    assert(current_thread.is_Some());
}

// Test 9: schedule_running_thread ensures other CPUs unchanged.
// Mutate: claim a different CPU changed.
// SHOULD FAIL
proof fn test_mutation_schedule_other_cpu_changed(
    old_cpu_thread: Option<ThreadPtr>,
    new_cpu_thread: Option<ThreadPtr>,
)
    requires
        old_cpu_thread == new_cpu_thread,
{
    assert(old_cpu_thread != new_cpu_thread);
}

// Test 10: run_blocked_thread ensures the blocked thread's state becomes RUNNING.
// Mutate: claim state is BLOCKED.
// SHOULD FAIL
proof fn test_mutation_unblocked_thread_still_blocked(new_state: ThreadState)
    requires
        new_state == ThreadState::RUNNING,
{
    assert(new_state == ThreadState::BLOCKED);
}

// Test 11: run_blocked_thread ensures endpoint queue is old queue skipped by 1.
// Mutate: claim queue is unchanged (not skipped).
// SHOULD FAIL
proof fn test_mutation_queue_not_dequeued(
    old_queue: Seq<ThreadPtr>,
    new_queue: Seq<ThreadPtr>,
)
    requires
        old_queue.len() > 0,
        new_queue == old_queue.skip(1),
{
    assert(new_queue =~= old_queue);
}

// Test 12: run_blocked_thread ensures cpu gets the blocked thread.
// Mutate: claim cpu current_thread is None after.
// SHOULD FAIL
proof fn test_mutation_run_blocked_cpu_no_thread(current_thread: Option<ThreadPtr>)
    requires
        current_thread.is_Some(),
{
    assert(current_thread.is_None());
}

// Test 13: schedule_running_thread ensures procs are preserved.
// Mutate: claim procs changed.
// SHOULD FAIL
proof fn test_mutation_schedule_procs_changed(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
)
    requires
        new_proc_dom =~= old_proc_dom,
{
    assert(!(new_proc_dom =~= old_proc_dom));
}

// Test 14: schedule_running_thread ensures scheduler adds the current thread.
// Mutate: claim scheduler is unchanged (thread not added).
// SHOULD FAIL
proof fn test_mutation_scheduler_thread_not_added(
    old_scheduler: Seq<ThreadPtr>,
    new_scheduler: Seq<ThreadPtr>,
    thread: ThreadPtr,
)
    requires
        new_scheduler == old_scheduler.push(thread),
{
    assert(new_scheduler =~= old_scheduler);
}

// Test 15: page_ptr2page_index and page_index2page_ptr are inverses.
// Mutate: claim they give wrong result.
// SHOULD FAIL
proof fn test_mutation_ptr_index_roundtrip_wrong() {
    let i: usize = 100;
    assert(spec_page_ptr2page_index(spec_page_index2page_ptr(i)) != i);
}

}
