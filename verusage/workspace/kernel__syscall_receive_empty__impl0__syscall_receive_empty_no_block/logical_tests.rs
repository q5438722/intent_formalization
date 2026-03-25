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
// syscall_receive_empty_no_block specification, testing whether
// the spec allows unintended reasoning.
// The ensures clause is EMPTY, making the spec very permissive.
// All tests SHOULD FAIL verification.

// Test 1: The spec does NOT guarantee determinism of the return value.
// Two calls with identical preconditions could theoretically produce
// different results since ensures is empty.
// SHOULD FAIL
proof fn test_logical_determinism(
    ret1_is_error: bool,
    ret2_is_error: bool,
    is_send: bool,
    queue_len: usize,
)
    requires
        is_send,
        queue_len > 0,
{
    // Two calls with same preconditions don't guarantee same result
    assert(ret1_is_error == ret2_is_error);
}

// Test 2: The spec does NOT guarantee that the return value
// is always Error. It could be Else on the success path.
// SHOULD FAIL
proof fn test_logical_always_error(
    is_send: bool,
    queue_len: usize,
    scheduler_len: usize,
    returns_error: bool,
)
    requires
        is_send,
        queue_len > 0,
        scheduler_len < MAX_CONTAINER_SCHEDULER_LEN,
{
    // Cannot conclude it always returns error
    assert(returns_error);
}

// Test 3: The spec does NOT guarantee that the return value
// is always Else (success). Error paths exist.
// SHOULD FAIL
proof fn test_logical_always_success(
    is_receive: bool,
    returns_else: bool,
)
    requires
        is_receive,
{
    // Cannot conclude it always returns success
    assert(returns_else);
}

// Test 4: The spec does NOT imply that thread_dom changes after the call.
// In fact, syscall_receive_empty_no_block does not create/destroy threads
// in its own ensures (which is empty anyway).
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

// Test 5: The spec does NOT guarantee that the endpoint queue state
// flips from SEND to RECEIVE after the call.
// SHOULD FAIL
proof fn test_logical_queue_state_flips(
    old_is_send: bool,
    new_is_receive: bool,
)
    requires
        old_is_send,
{
    // No spec says queue state flips
    assert(new_is_receive);
}

// Test 6: The spec does NOT guarantee that the sender thread pointer
// is always equal to the receiver thread pointer. They are different threads.
// SHOULD FAIL
proof fn test_logical_sender_equals_receiver(
    receiver_thread_ptr: ThreadPtr,
    sender_thread_ptr: ThreadPtr,
)
    requires
        receiver_thread_ptr != sender_thread_ptr,
{
    assert(receiver_thread_ptr == sender_thread_ptr);
}

// Test 7: The spec does NOT guarantee that switch_decision is Switch.
// The function always returns NoSwitch via NoSwitchNew.
// SHOULD FAIL
proof fn test_logical_switch_decision_is_switch(
    no_switch: bool,
)
    requires
        no_switch,
{
    // NoSwitchNew ensures switch_decision == NoSwitch
    // Asserting it IS Switch is wrong
    assert(!no_switch);
}

// Test 8: The spec does NOT guarantee that the endpoint's rf_counter
// is decremented after the call. The empty ensures says nothing about
// rf_counter changes.
// SHOULD FAIL
proof fn test_logical_rf_counter_decremented(
    old_rf_counter: usize,
    new_rf_counter: usize,
)
    requires
        old_rf_counter > 0,
        new_rf_counter == old_rf_counter,
{
    // rf_counter is unchanged (per schedule_blocked_thread postcondition)
    // Asserting it decremented is wrong
    assert(new_rf_counter == old_rf_counter - 1);
}

}
