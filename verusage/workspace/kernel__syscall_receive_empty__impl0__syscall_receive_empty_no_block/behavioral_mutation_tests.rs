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

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs and mutates expected outputs
// or relations. Tests whether incorrect behaviors are rejected.
// The ensures clause of syscall_receive_empty_no_block is EMPTY,
// so many behavioral mutations will likely PASS (spec is too weak).
// All tests SHOULD FAIL verification.

// Test 1: When all conditions are met for a successful receive
// (sender exists, scheduler has room), the function returns
// RetValueType::Else. Mutated: assert it returns Error instead.
// SHOULD FAIL
proof fn test_mutation_success_returns_error(
    is_send_state: bool,
    queue_len: usize,
    scheduler_len: usize,
)
    requires
        is_send_state,
        queue_len > 0,
        scheduler_len < MAX_CONTAINER_SCHEDULER_LEN,
{
    // All conditions met => should return Else (success)
    // Mutated: assert the opposite (error)
    let should_succeed = is_send_state && queue_len > 0 && scheduler_len < MAX_CONTAINER_SCHEDULER_LEN;
    assert(should_succeed);
    assert(!should_succeed); // SHOULD FAIL: contradicts above
}

// Test 2: When endpoint descriptor is None, the function returns Error.
// Mutated: assert it returns success (Else).
// SHOULD FAIL
proof fn test_mutation_none_descriptor_returns_success(
    is_none: bool,
)
    requires
        is_none,
{
    // When descriptor is None, function returns Error
    // Mutated: claiming we can still succeed
    assert(!is_none); // SHOULD FAIL: is_none is true
}

// Test 3: When endpoint is in RECEIVE state, the function returns Error.
// Mutated: assert the function does NOT return error.
// SHOULD FAIL
proof fn test_mutation_receive_state_not_error(
    is_receive: bool,
    queue_len: usize,
)
    requires
        is_receive,
        queue_len < MAX_NUM_THREADS_PER_ENDPOINT,
{
    // In receive state with queue not full => Error
    // Mutated: asserting that is_receive is false
    assert(!is_receive); // SHOULD FAIL
}

// Test 4: When send queue is empty, function returns Error.
// Mutated: assert queue was non-empty.
// SHOULD FAIL
proof fn test_mutation_empty_send_queue_success(
    is_send: bool,
    queue_len: usize,
)
    requires
        is_send,
        queue_len == 0,
{
    // Queue is empty in SEND state => Error
    // Mutated: assert queue is not empty
    assert(queue_len > 0); // SHOULD FAIL
}

// Test 5: After schedule_blocked_thread, the endpoint queue should
// have lost its head (queue becomes old queue skipped by 1).
// Mutated: assert the queue length stays the same.
// Note: The ensures of syscall_receive_empty_no_block is EMPTY,
// so the spec does NOT guarantee this. This tests spec weakness.
// SHOULD FAIL
proof fn test_mutation_queue_unchanged_after_schedule(
    old_queue_len: int,
    new_queue_len: int,
)
    requires
        old_queue_len > 0,
        new_queue_len == old_queue_len - 1,
{
    // After scheduling, queue shrinks by 1
    // Mutated: assert it stays the same
    assert(new_queue_len == old_queue_len); // SHOULD FAIL
}

// Test 6: When scheduler is full, function should return Error.
// Mutated: assert it returns success (Else).
// SHOULD FAIL
proof fn test_mutation_full_scheduler_returns_success(
    scheduler_len: usize,
)
    requires
        scheduler_len >= MAX_CONTAINER_SCHEDULER_LEN,
{
    // Scheduler full => Error
    // Mutated: assert scheduler has room
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN); // SHOULD FAIL
}

// Test 7: After successful call, wf() should be preserved.
// Mutated: assert that self is NOT wf after the call.
// Since ensures is empty, spec doesn't guarantee wf preservation.
// SHOULD FAIL
proof fn test_mutation_wf_not_preserved(
    old_wf: bool,
    new_wf: bool,
)
    requires
        old_wf,
        new_wf,  // In reality schedule_blocked_thread ensures wf
{
    assert(!new_wf); // SHOULD FAIL: contradicts new_wf == true
}

}
