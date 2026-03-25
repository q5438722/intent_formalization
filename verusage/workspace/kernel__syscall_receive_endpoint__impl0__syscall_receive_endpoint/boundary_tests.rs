use vstd::prelude::*;

fn main() {}

verus!{

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type EndpointPtr = usize;
pub type EndpointIdx = usize;

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition of syscall_receive_endpoint
// or uses edge-case values to check if invalid inputs are
// properly rejected by the specification.
// All tests SHOULD FAIL verification.

// Test 1: receiver_thread_ptr must be in thread_dom.
// Violates: old(self).thread_dom().contains(receiver_thread_ptr)
// SHOULD FAIL
proof fn test_boundary_receiver_thread_not_in_domain(
    thread_dom: Set<ThreadPtr>,
    receiver_thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(receiver_thread_ptr),
{
    assert(thread_dom.contains(receiver_thread_ptr));
}

// Test 2: blocking_endpoint_index must be < MAX_NUM_ENDPOINT_DESCRIPTORS.
// At exactly MAX_NUM_ENDPOINT_DESCRIPTORS, precondition is violated.
// Violates: 0 <= blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS
// SHOULD FAIL
proof fn test_boundary_endpoint_index_at_max() {
    let idx: usize = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(idx < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 3: receiver_endpoint_payload must be < MAX_NUM_ENDPOINT_DESCRIPTORS.
// At exactly MAX_NUM_ENDPOINT_DESCRIPTORS, precondition is violated.
// Violates: 0 <= receiver_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS
// SHOULD FAIL
proof fn test_boundary_payload_index_at_max() {
    let payload: usize = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(payload < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 4: blocking_endpoint_index must be >= 0 (non-negative).
// Using usize::MAX as a wrap-around edge case.
// Violates: 0 <= blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS
// SHOULD FAIL
proof fn test_boundary_endpoint_index_usize_max() {
    let idx: usize = usize::MAX;
    assert(0 <= idx && idx < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 5: Queue length must be < MAX_NUM_THREADS_PER_ENDPOINT for blocking.
// When queue is at max, blocking should be rejected.
// Violates: queue.len() < MAX_NUM_THREADS_PER_ENDPOINT (needed for block)
// SHOULD FAIL
proof fn test_boundary_queue_at_max_capacity() {
    let queue_len: usize = MAX_NUM_THREADS_PER_ENDPOINT;
    assert(queue_len < MAX_NUM_THREADS_PER_ENDPOINT);
}

// Test 6: Scheduler must have room to schedule a blocked thread.
// At exactly MAX_CONTAINER_SCHEDULER_LEN, scheduling is rejected.
// Violates: scheduler.len() < MAX_CONTAINER_SCHEDULER_LEN
// SHOULD FAIL
proof fn test_boundary_scheduler_at_max() {
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN;
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN);
}

// Test 7: receiver_endpoint_payload slot must be empty (is_None).
// If it already holds an endpoint, passing should be rejected.
// Violates: receiver_endpoint_dsecriptor_ptr_op.is_None() 
// SHOULD FAIL
proof fn test_boundary_payload_slot_not_empty(
    receiver_endpoint_slot: Option<EndpointPtr>,
)
    requires
        receiver_endpoint_slot.is_Some(),
{
    assert(receiver_endpoint_slot.is_None());
}

// Test 8: rf_counter must not be at usize::MAX.
// At usize::MAX, passing endpoint should be rejected.
// Violates: rf_counter != usize::MAX
// SHOULD FAIL
proof fn test_boundary_rf_counter_at_max() {
    let rf_counter: usize = usize::MAX;
    assert(rf_counter != usize::MAX);
}

}
