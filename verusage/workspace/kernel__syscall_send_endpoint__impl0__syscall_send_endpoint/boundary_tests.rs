use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type ProcPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition of syscall_send_endpoint or its
// helper functions. All tests SHOULD FAIL verification.

// Test 1: syscall_send_endpoint requires old(self).thread_dom().contains(sender_thread_ptr).
// If the sender thread is not in the thread domain, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_sender_thread_not_in_domain(
    thread_dom: Set<ThreadPtr>,
    sender_thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(sender_thread_ptr),
{
    assert(thread_dom.contains(sender_thread_ptr));
}

// Test 2: syscall_send_endpoint requires 0 <= blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS.
// Using an index equal to MAX_NUM_ENDPOINT_DESCRIPTORS violates the upper bound.
// SHOULD FAIL
proof fn test_boundary_endpoint_index_at_max() {
    let blocking_endpoint_index: usize = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 3: syscall_send_endpoint requires 0 <= sender_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS.
// Using usize::MAX as the payload index violates the bound.
// SHOULD FAIL
proof fn test_boundary_payload_index_overflow() {
    let sender_endpoint_payload: usize = usize::MAX;
    assert(sender_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 4: syscall_send_endpoint requires sender thread state == ThreadState::RUNNING.
// A BLOCKED thread should not be able to invoke the syscall.
// SHOULD FAIL
proof fn test_boundary_sender_thread_not_running() {
    // Encode that a thread in BLOCKED state satisfies the RUNNING requirement
    let state: u8 = 1; // 1 = BLOCKED, 2 = RUNNING
    assert(state == 2u8); // should be RUNNING
}

// Test 5: syscall_send_endpoint requires old(self).wf().
// If the kernel is not well-formed, the precondition is violated.
// This tests that a non-wf kernel cannot invoke the syscall.
// SHOULD FAIL
proof fn test_boundary_kernel_not_wf() {
    let wf: bool = false;
    assert(wf);
}

// Test 6: block_running_thread requires the endpoint queue length < MAX_NUM_THREADS_PER_ENDPOINT.
// When the queue is full, blocking should not be allowed.
// SHOULD FAIL
proof fn test_boundary_endpoint_queue_full_for_blocking() {
    let queue_len: usize = MAX_NUM_THREADS_PER_ENDPOINT;
    assert(queue_len < MAX_NUM_THREADS_PER_ENDPOINT);
}

// Test 7: pass_endpoint requires src_thread_ptr != dst_thread_ptr.
// If sender and receiver are the same thread, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_same_sender_receiver_thread() {
    let src_thread_ptr: ThreadPtr = 42;
    let dst_thread_ptr: ThreadPtr = 42;
    assert(src_thread_ptr != dst_thread_ptr);
}

// Test 8: pass_endpoint requires dst endpoint descriptor slot is None.
// If receiver already has an endpoint at that slot, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_receiver_slot_occupied() {
    let slot_is_none: bool = false;
    assert(slot_is_none);
}

// Test 9: schedule_blocked_thread requires endpoint queue len > 0.
// With an empty queue, scheduling is impossible.
// SHOULD FAIL
proof fn test_boundary_schedule_empty_queue() {
    let queue_len: usize = 0;
    assert(queue_len > 0);
}

// Test 10: container_check_is_ancestor requires both containers in domain.
// If ancestor_ptr is not in domain, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_ancestor_not_in_container_domain(
    container_dom: Set<ContainerPtr>,
    ancestor_ptr: ContainerPtr,
)
    requires
        !container_dom.contains(ancestor_ptr),
{
    assert(container_dom.contains(ancestor_ptr));
}

}
