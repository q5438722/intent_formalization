use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type EndpointPtr = usize;
pub type EndpointIdx = usize;
pub type ContainerPtr = usize;
pub type ProcPtr = usize;
pub type PagePtr = usize;
pub type Pcid = usize;

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwitchDecision {
    NoSwitch,
    NoThread,
    Switch,
}

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs (matching postconditions), then
// asserts a WRONG output or relation (mutated from the correct postcondition).
// All tests SHOULD FAIL verification.

// Test 1: block_running_thread_and_set_trap_frame ensures
//   self.get_thread(thread_ptr).state == ThreadState::BLOCKED.
// Mutate: claim thread stays RUNNING after blocking.
// SHOULD FAIL
proof fn test_mutation_thread_stays_running_after_block(
    old_state: ThreadState,
    new_state: ThreadState,
)
    requires
        old_state == ThreadState::RUNNING,
        new_state == ThreadState::BLOCKED,
{
    assert(new_state == ThreadState::RUNNING);
}

// Test 2: block_running_thread_and_set_trap_frame ensures endpoint queue
//   gets the thread pushed: queue@ == old_queue@.push(thread_ptr).
// Mutate: claim queue is unchanged.
// SHOULD FAIL
proof fn test_mutation_queue_unchanged_after_block(
    old_queue: Seq<ThreadPtr>,
    new_queue: Seq<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        new_queue =~= old_queue.push(thread_ptr),
{
    assert(new_queue =~= old_queue);
}

// Test 3: schedule_blocked_thread ensures endpoint queue skips first:
//   self.get_endpoint(endpoint_ptr).queue@ == old.queue@.skip(1).
// Mutate: claim queue is unchanged (no skip).
// SHOULD FAIL
proof fn test_mutation_queue_not_skipped_after_schedule(
    old_queue: Seq<ThreadPtr>,
    new_queue: Seq<ThreadPtr>,
)
    requires
        old_queue.len() > 0,
        new_queue == old_queue.skip(1),
{
    assert(new_queue =~= old_queue);
}

// Test 4: schedule_blocked_thread ensures endpoint_dom unchanged:
//   self.endpoint_dom() == old(self).endpoint_dom().
// Mutate: claim a new endpoint appears in domain.
// SHOULD FAIL
proof fn test_mutation_endpoint_dom_changed_after_schedule(
    old_dom: Set<EndpointPtr>,
    new_dom: Set<EndpointPtr>,
    extra_ptr: EndpointPtr,
)
    requires
        new_dom == old_dom,
        !old_dom.contains(extra_ptr),
{
    assert(new_dom.contains(extra_ptr));
}

// Test 5: NoSwitchNew ensures switch_decision == SwitchDecision::NoSwitch.
// Mutate: claim decision is Switch.
// SHOULD FAIL
proof fn test_mutation_no_switch_returns_switch(
    decision: SwitchDecision,
)
    requires
        decision == SwitchDecision::NoSwitch,
{
    assert(decision == SwitchDecision::Switch);
}

// Test 6: NoNextThreadNew ensures pcid.is_None().
// Mutate: claim pcid is Some.
// SHOULD FAIL
proof fn test_mutation_no_next_thread_has_pcid(
    pcid: Option<Pcid>,
)
    requires
        pcid.is_None(),
{
    assert(pcid.is_Some());
}

// Test 7: block_running_thread_and_set_trap_frame ensures endpoint
//   queue_state is unchanged for the blocking endpoint.
// Mutate: claim queue_state changes from SEND to RECEIVE.
// SHOULD FAIL
proof fn test_mutation_queue_state_changed_by_block(
    old_state: EndpointState,
    new_state: EndpointState,
)
    requires
        old_state == EndpointState::SEND,
        new_state == old_state,
{
    assert(new_state == EndpointState::RECEIVE);
}

// Test 8: schedule_blocked_thread ensures thread_dom unchanged:
//   self.thread_dom() == old(self).thread_dom().
// Mutate: claim thread was removed from domain.
// SHOULD FAIL
proof fn test_mutation_thread_removed_after_schedule(
    old_dom: Set<ThreadPtr>,
    new_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        old_dom.contains(thread_ptr),
        new_dom == old_dom,
{
    assert(!new_dom.contains(thread_ptr));
}

// Test 9: block_running_thread_and_change_queue_state ensures
//   queue_state becomes the provided target state.
// Mutate: claim queue_state becomes the opposite.
// SHOULD FAIL
proof fn test_mutation_queue_state_wrong_after_change(
    target_state: EndpointState,
    actual_state: EndpointState,
)
    requires
        target_state == EndpointState::SEND,
        actual_state == target_state,
{
    assert(actual_state == EndpointState::RECEIVE);
}

// Test 10: block_running_thread_and_set_trap_frame ensures
//   endpoint_descriptors are preserved for the blocked thread.
// Mutate: claim descriptors length becomes 0.
// SHOULD FAIL
proof fn test_mutation_descriptors_changed_after_block(
    old_descriptors: Seq<Option<EndpointPtr>>,
    new_descriptors: Seq<Option<EndpointPtr>>,
)
    requires
        new_descriptors =~= old_descriptors,
        old_descriptors.len() > 0,
{
    assert(new_descriptors.len() == 0);
}

// Test 11: schedule_blocked_thread ensures proc_dom unchanged:
//   self.proc_dom() =~= old(self).proc_dom().
// Mutate: claim proc_dom gains a new element.
// SHOULD FAIL
proof fn test_mutation_proc_dom_grows_after_schedule(
    old_dom: Set<ProcPtr>,
    new_dom: Set<ProcPtr>,
    new_proc: ProcPtr,
)
    requires
        new_dom =~= old_dom,
        !old_dom.contains(new_proc),
{
    assert(new_dom.contains(new_proc));
}

// Test 12: NoSwitchNew ensures cr3.is_None().
// Mutate: claim cr3 is Some.
// SHOULD FAIL
proof fn test_mutation_no_switch_has_cr3(
    cr3: Option<usize>,
)
    requires
        cr3.is_None(),
{
    assert(cr3.is_Some());
}

}
