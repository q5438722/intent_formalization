use vstd::prelude::*;

fn main() {}

verus!{

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;

pub type ThreadPtr = usize;
pub type EndpointPtr = usize;
pub type EndpointIdx = usize;
pub type ContainerPtr = usize;

// ===================== LOGICAL TESTS =====================
// Each test asserts a property NOT explicitly guaranteed by the
// syscall_send_empty_no_block specification, testing whether
// the spec allows unintended reasoning.
// The ensures clause only has self.wf(), making the spec
// very permissive about return values and state transitions.
// All tests SHOULD FAIL verification.

// Test 1: The spec does NOT guarantee determinism of return value.
// Two calls with identical preconditions could theoretically produce
// different results since ensures doesn't constrain the return value.
// SHOULD FAIL
proof fn test_logical_determinism(
    ret1_is_error: bool,
    ret2_is_error: bool,
    is_receive: bool,
    queue_len: usize,
)
    requires
        is_receive,
        queue_len > 0,
{
    // Two calls with same preconditions don't guarantee same result
    assert(ret1_is_error == ret2_is_error);
}

// Test 2: The ensures clause does NOT constrain the return value.
// Cannot prove the return is always Error from the spec alone.
// SHOULD FAIL
proof fn test_logical_always_error(
    is_receive: bool,
    queue_len: usize,
    scheduler_len: usize,
    returns_error: bool,
)
    requires
        is_receive,
        queue_len > 0,
        scheduler_len < MAX_CONTAINER_SCHEDULER_LEN,
{
    // Cannot conclude it always returns error
    assert(returns_error);
}

// Test 3: The ensures clause does NOT constrain the return value.
// Cannot prove the return is always Else (success) from the spec alone.
// SHOULD FAIL
proof fn test_logical_always_success(
    is_send: bool,
    returns_else: bool,
)
    requires
        is_send,
{
    // Cannot conclude it always returns success
    assert(returns_else);
}

// Test 4: The spec does NOT imply thread_dom changes after the call.
// Asserting a thread was removed should fail.
// SHOULD FAIL
proof fn test_logical_thread_dom_changes(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
    t: ThreadPtr,
)
    requires
        old_thread_dom.contains(t),
        new_thread_dom =~= old_thread_dom,
{
    // Thread dom should be unchanged, but asserting a thread was removed
    assert(!new_thread_dom.contains(t));
}

// Test 5: The spec does NOT guarantee the endpoint queue state
// flips from RECEIVE to SEND after the call.
// SHOULD FAIL
proof fn test_logical_queue_state_flips(
    old_is_receive: bool,
    new_is_send: bool,
)
    requires
        old_is_receive,
{
    // No spec says queue state flips
    assert(new_is_send);
}

// Test 6: The ensures clause does NOT guarantee switch_decision is Switch.
// The function always returns NoSwitch via NoSwitchNew in the implementation,
// but the ensures only says self.wf().
// SHOULD FAIL
proof fn test_logical_switch_decision_is_switch(
    switch_is_noswitch: bool,
)
    requires
        switch_is_noswitch,
{
    // Asserting it is Switch, not NoSwitch
    assert(!switch_is_noswitch);
}

// Test 7: The spec does NOT imply a stronger bound on endpoint_index.
// Trying to prove endpoint_index < 64 (half of max) from the spec.
// SHOULD FAIL
proof fn test_logical_stronger_endpoint_bound(
    endpoint_index: EndpointIdx,
)
    requires
        0 <= endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS,
{
    // Spec only guarantees < 128, not < 64
    assert(endpoint_index < 64);
}

// Test 8: The spec does NOT guarantee the sender_thread_ptr
// equals the receiver_thread_ptr. They are different threads.
// SHOULD FAIL
proof fn test_logical_sender_equals_receiver(
    sender_thread_ptr: ThreadPtr,
    receiver_thread_ptr: ThreadPtr,
)
    requires
        sender_thread_ptr != receiver_thread_ptr,
{
    assert(sender_thread_ptr == receiver_thread_ptr);
}

// Test 9: The spec does NOT guarantee that endpoint_dom changes.
// Asserting the endpoint domain grew should fail.
// SHOULD FAIL
proof fn test_logical_endpoint_dom_grows(
    old_endpoint_dom: Set<EndpointPtr>,
    new_endpoint_dom: Set<EndpointPtr>,
    new_ep: EndpointPtr,
)
    requires
        !old_endpoint_dom.contains(new_ep),
        new_endpoint_dom =~= old_endpoint_dom,
{
    // Spec preserves endpoint_dom, asserting a new endpoint was added
    assert(new_endpoint_dom.contains(new_ep));
}

}
