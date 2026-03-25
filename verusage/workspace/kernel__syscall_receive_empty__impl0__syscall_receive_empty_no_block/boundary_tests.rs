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

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition or uses edge-case values
// for syscall_receive_empty_no_block to check if invalid inputs
// are properly rejected by the specification.
// All tests SHOULD FAIL verification.

// Test 1: receiver_thread_ptr must be in thread_dom.
// Violating this precondition should be rejected.
// SHOULD FAIL
proof fn test_boundary_thread_not_in_domain(
    thread_dom: Set<ThreadPtr>,
    receiver_thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(receiver_thread_ptr),
{
    // Precondition requires thread_dom.contains(receiver_thread_ptr)
    assert(thread_dom.contains(receiver_thread_ptr));
}

// Test 2: blocking_endpoint_index must be < MAX_NUM_ENDPOINT_DESCRIPTORS.
// Using exactly MAX_NUM_ENDPOINT_DESCRIPTORS violates the upper bound.
// SHOULD FAIL
proof fn test_boundary_endpoint_index_at_max() {
    let endpoint_index: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 3: blocking_endpoint_index must be < MAX_NUM_ENDPOINT_DESCRIPTORS.
// Using a value well beyond the upper bound violates the precondition.
// SHOULD FAIL
proof fn test_boundary_endpoint_index_exceeds_max() {
    let endpoint_index: EndpointIdx = 200usize;
    assert(endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 4: The scheduler must have room to schedule the sender.
// At exactly MAX_CONTAINER_SCHEDULER_LEN, scheduling should fail.
// SHOULD FAIL
proof fn test_boundary_scheduler_at_max_capacity() {
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN;
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN);
}

// Test 5: The endpoint queue len must be > 0 for sender_exist
// (i.e., the send queue is non-empty). With len == 0, there is no sender.
// SHOULD FAIL
proof fn test_boundary_send_queue_empty() {
    let queue_len: usize = 0;
    let is_send_state = true;
    // sender_exist requires queue_state == SEND and queue.len() != 0
    assert(is_send_state && queue_len != 0);
}

// Test 6: The endpoint descriptor at the given index must not be None.
// If it is None, the function returns Error early.
// SHOULD FAIL
proof fn test_boundary_endpoint_descriptor_is_none(
    endpoint_desc: Option<EndpointPtr>,
)
    requires
        endpoint_desc is None,
{
    // The function requires endpoint_desc to be Some for the happy path
    assert(endpoint_desc is Some);
}

// Test 7: The queue is in RECEIVE state with non-full queue.
// This should be an error (no sender). We test that the receive-state
// non-full condition is not accepted as valid for proceeding.
// SHOULD FAIL
proof fn test_boundary_receive_state_queue_not_full() {
    let is_receive = true;
    let queue_len: usize = 0;
    // In receive state with < MAX threads, function returns Error
    // Cannot proceed; asserting we can proceed is wrong
    assert(!(is_receive && queue_len < MAX_NUM_THREADS_PER_ENDPOINT));
}

}
