use vstd::prelude::*;

fn main() {}

verus!{

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub type ThreadPtr = usize;
pub type EndpointPtr = usize;
pub type EndpointIdx = usize;
pub type ContainerPtr = usize;

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition or uses edge-case values
// for syscall_send_empty_no_block to check if invalid inputs
// are properly rejected by the specification.
// All tests SHOULD FAIL verification.

// Test 1: sender_thread_ptr must be in thread_dom.
// Violating this precondition should be rejected.
// SHOULD FAIL
proof fn test_boundary_sender_thread_not_in_domain(
    thread_dom: Set<ThreadPtr>,
    sender_thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(sender_thread_ptr),
{
    // Precondition requires thread_dom.contains(sender_thread_ptr)
    assert(thread_dom.contains(sender_thread_ptr));
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
    let endpoint_index: EndpointIdx = 256usize;
    assert(endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 4: The sender thread must be in RUNNING state.
// A BLOCKED thread should not satisfy the precondition.
// We encode ThreadState as integers: RUNNING=2, BLOCKED=1, SCHEDULED=0.
// SHOULD FAIL
proof fn test_boundary_sender_thread_not_running(
    thread_state: int,
)
    requires
        thread_state == 1, // BLOCKED, not RUNNING (2)
{
    assert(thread_state == 2); // must be RUNNING
}

// Test 5: The sender thread must be in RUNNING state.
// A SCHEDULED thread should not satisfy the precondition.
// SHOULD FAIL
proof fn test_boundary_sender_thread_scheduled(
    thread_state: int,
)
    requires
        thread_state == 0, // SCHEDULED, not RUNNING (2)
{
    assert(thread_state == 2); // must be RUNNING
}

// Test 6: page_ptr2page_index requires ptr % 0x1000 == 0.
// Passing a non-aligned pointer violates the precondition.
// SHOULD FAIL
proof fn test_boundary_page_ptr_not_aligned() {
    let ptr: usize = 0x1001usize;
    assert(ptr % 0x1000usize == 0);
}

// Test 7: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// Passing NUM_PAGES as index violates the upper bound.
// SHOULD FAIL
proof fn test_boundary_page_index_at_max() {
    let i: usize = NUM_PAGES;
    assert(i < NUM_PAGES);
}

// Test 8: The endpoint queue must have room (len < MAX_NUM_THREADS_PER_ENDPOINT)
// for some operations. At exactly MAX, queue is full.
// SHOULD FAIL
proof fn test_boundary_endpoint_queue_full() {
    let queue_len: usize = MAX_NUM_THREADS_PER_ENDPOINT;
    assert(queue_len < MAX_NUM_THREADS_PER_ENDPOINT);
}

// Test 9: The scheduler must have room to schedule a thread.
// At exactly MAX_CONTAINER_SCHEDULER_LEN, scheduling should fail.
// SHOULD FAIL
proof fn test_boundary_scheduler_at_max_capacity() {
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN;
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN);
}

// Test 10: blocking_endpoint_index must be >= 0.
// Attempting negative-like index (in usize context, wrapping) should be rejected.
// In usize, -1 wraps to usize::MAX, which is >= MAX_NUM_ENDPOINT_DESCRIPTORS.
// SHOULD FAIL
proof fn test_boundary_endpoint_index_usize_max() {
    let endpoint_index: usize = usize::MAX;
    assert(endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

}
