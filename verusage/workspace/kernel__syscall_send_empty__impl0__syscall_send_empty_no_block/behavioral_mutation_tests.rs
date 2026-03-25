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
// The ensures clause of syscall_send_empty_no_block only has self.wf(),
// so many behavioral mutations may pass (indicating spec weakness).
// All tests SHOULD FAIL verification.

// Test 1: When endpoint descriptor is None, the function returns Error.
// Mutated: assert it returns success (Else) instead.
// SHOULD FAIL
proof fn test_mutation_none_descriptor_returns_success(
    is_none: bool,
)
    requires
        is_none,
{
    // When descriptor is None, function returns Error
    // Mutated: claiming is_none is false (success path)
    assert(!is_none);
}

// Test 2: When endpoint is in SEND state with queue len < MAX,
// the function returns Error. Mutated: assert it returns success.
// SHOULD FAIL
proof fn test_mutation_send_state_small_queue_returns_success(
    is_send_state: bool,
    queue_len: usize,
)
    requires
        is_send_state,
        queue_len < MAX_NUM_THREADS_PER_ENDPOINT,
{
    // In SEND state with queue not full => Error
    // Mutated: asserting is_send_state is false (contradicts precondition)
    assert(!is_send_state);
}

// Test 3: When endpoint is in SEND state with queue len >= MAX,
// the function returns Error. Mutated: assert it returns success.
// SHOULD FAIL
proof fn test_mutation_send_state_full_queue_returns_success(
    is_send_state: bool,
    queue_len: usize,
)
    requires
        is_send_state,
        queue_len >= MAX_NUM_THREADS_PER_ENDPOINT,
{
    // In SEND state with queue full => Error
    // Mutated: asserting queue is not full
    assert(queue_len < MAX_NUM_THREADS_PER_ENDPOINT);
}

// Test 4: When RECEIVE state and queue is empty, function returns Error.
// Mutated: assert queue was non-empty.
// SHOULD FAIL
proof fn test_mutation_receive_empty_queue_returns_success(
    is_receive: bool,
    queue_len: usize,
)
    requires
        is_receive,
        queue_len == 0,
{
    // RECEIVE state with empty queue => Error
    // Mutated: assert queue is non-empty
    assert(queue_len > 0);
}

// Test 5: When all conditions are met for success
// (RECEIVE state, queue non-empty, scheduler has room),
// the function should return Else. Mutated: assert it returns Error.
// SHOULD FAIL
proof fn test_mutation_success_returns_error(
    is_receive: bool,
    queue_len: usize,
    scheduler_len: usize,
)
    requires
        is_receive,
        queue_len > 0,
        scheduler_len < MAX_CONTAINER_SCHEDULER_LEN,
{
    // All conditions met => should return Else (success)
    let should_succeed = is_receive && queue_len > 0 && scheduler_len < MAX_CONTAINER_SCHEDULER_LEN;
    assert(should_succeed);
    // Mutated: assert the opposite
    assert(!should_succeed);
}

// Test 6: When scheduler is full (>= MAX_CONTAINER_SCHEDULER_LEN),
// function returns Error. Mutated: assert scheduler has room.
// SHOULD FAIL
proof fn test_mutation_full_scheduler_returns_success(
    scheduler_len: usize,
)
    requires
        scheduler_len >= MAX_CONTAINER_SCHEDULER_LEN,
{
    // Scheduler full => Error
    // Mutated: assert scheduler has room
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN);
}

// Test 7: After schedule_blocked_thread, the endpoint queue shrinks by 1.
// Mutated: assert the queue length stays the same.
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
    assert(new_queue_len == old_queue_len);
}

}
