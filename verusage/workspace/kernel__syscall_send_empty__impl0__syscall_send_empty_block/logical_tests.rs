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
pub const NUM_PAGES: usize = 2 * 1024 * 1024;

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

#[derive(Clone, Copy)]
pub enum RetValueType {
    Error,
    Else,
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize {
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize {
    (i * 4096) as usize
}

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// These test for unintended reasoning: determinism, stronger bounds,
// structural/global assumptions, cross-function misuse.
// All tests SHOULD FAIL verification.

// Test 1: syscall_send_empty_block only ensures self.wf().
// It does NOT guarantee thread domains are preserved.
// Claim: thread_dom is always preserved (stronger than spec).
// SHOULD FAIL
proof fn test_logical_thread_dom_preserved(
    old_dom: Set<ThreadPtr>,
    new_dom: Set<ThreadPtr>,
    kernel_wf: bool,
)
    requires
        kernel_wf == true,
{
    assert(old_dom == new_dom);
}

// Test 2: syscall_send_empty_block only ensures self.wf().
// It does NOT specify the return value's error_code.
// Claim: return is always Error.
// SHOULD FAIL
proof fn test_logical_return_always_error(
    ret_code: RetValueType,
    kernel_wf: bool,
)
    requires
        kernel_wf,
{
    assert(ret_code is Error);
}

// Test 3: syscall_send_empty_block only ensures self.wf().
// It does NOT guarantee sender thread state changes to BLOCKED.
// Claim: sender is always BLOCKED after the call.
// SHOULD FAIL
proof fn test_logical_sender_always_blocked_after(
    kernel_wf: bool,
    sender_state_after: ThreadState,
)
    requires
        kernel_wf,
{
    assert(sender_state_after == ThreadState::BLOCKED);
}

// Test 4: syscall_send_empty_block does NOT guarantee determinism.
// Two calls with the same preconditions could produce different results.
// Claim: switch decisions are always the same.
// SHOULD FAIL
proof fn test_logical_determinism(
    decision1: SwitchDecision,
    decision2: SwitchDecision,
    kernel_wf1: bool,
    kernel_wf2: bool,
)
    requires
        kernel_wf1,
        kernel_wf2,
{
    assert(decision1 == decision2);
}

// Test 5: syscall_send_empty_block does NOT guarantee container_dom preserved.
// Claim: container domain is unchanged after the call.
// SHOULD FAIL
proof fn test_logical_container_dom_preserved(
    old_container_dom: Set<ContainerPtr>,
    new_container_dom: Set<ContainerPtr>,
    kernel_wf: bool,
)
    requires
        kernel_wf,
{
    assert(old_container_dom == new_container_dom);
}

// Test 6: syscall_send_empty_block does NOT guarantee endpoint state unchanged.
// Claim: endpoint queue state is always preserved across the call.
// SHOULD FAIL
proof fn test_logical_endpoint_state_always_preserved(
    old_state: EndpointState,
    new_state: EndpointState,
    kernel_wf: bool,
)
    requires
        kernel_wf,
{
    assert(old_state == new_state);
}

// Test 7: schedule_blocked_thread does NOT guarantee the dequeued thread
// transitions to RUNNING state. It only specifies queue manipulation.
// Claim: dequeued thread is always RUNNING.
// SHOULD FAIL
proof fn test_logical_dequeued_thread_becomes_running(
    thread_state: ThreadState,
    queue_old: Seq<ThreadPtr>,
    queue_new: Seq<ThreadPtr>,
)
    requires
        queue_old.len() > 0,
        queue_new == queue_old.skip(1),
{
    assert(thread_state == ThreadState::RUNNING);
}

// Test 8: syscall_send_empty_block does NOT guarantee that the
// return switch_decision is NoSwitch. In fact, NoNextThread is
// possible too. Claim: decision is always NoSwitch.
// SHOULD FAIL
proof fn test_logical_return_always_no_switch(
    decision: SwitchDecision,
    kernel_wf: bool,
)
    requires
        kernel_wf,
{
    assert(decision == SwitchDecision::NoSwitch);
}

// Test 9: The spec does NOT connect page_ptr2page_index and
// page_index2page_ptr for out-of-range values.
// Claim: roundtrip holds for NUM_PAGES (out of valid range).
// SHOULD FAIL
proof fn test_logical_roundtrip_out_of_range() {
    let big_i: usize = NUM_PAGES;
    assert(big_i < NUM_PAGES);
}

// Test 10: syscall_send_empty_block does NOT guarantee page_closure
// is unchanged after the call. Claim: page closure preserved.
// SHOULD FAIL
proof fn test_logical_page_closure_preserved(
    old_closure: Set<PagePtr>,
    new_closure: Set<PagePtr>,
    kernel_wf: bool,
)
    requires
        kernel_wf,
{
    assert(old_closure =~= new_closure);
}

// Test 11: syscall_send_empty_block does NOT guarantee endpoint_dom
// is non-empty. Claim: endpoint domain always has elements.
// SHOULD FAIL
proof fn test_logical_endpoint_dom_always_nonempty(
    endpoint_dom: Set<EndpointPtr>,
    kernel_wf: bool,
)
    requires
        kernel_wf,
{
    assert(endpoint_dom.len() > 0);
}

// Test 12: The spec does NOT tie the sender's thread_ptr value to
// any property of the endpoint. Claim: sender_thread_ptr is always
// less than MAX_NUM_THREADS_PER_ENDPOINT.
// SHOULD FAIL
proof fn test_logical_sender_ptr_bounded(
    sender_thread_ptr: ThreadPtr,
    kernel_wf: bool,
)
    requires
        kernel_wf,
{
    assert(sender_thread_ptr < MAX_NUM_THREADS_PER_ENDPOINT);
}

}
