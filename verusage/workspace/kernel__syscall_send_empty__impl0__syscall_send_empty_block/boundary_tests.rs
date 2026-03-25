use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type EndpointPtr = usize;
pub type EndpointIdx = usize;
pub type ContainerPtr = usize;
pub type ProcPtr = usize;
pub type PagePtr = usize;

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

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) or
// uses edge-case values. All tests SHOULD FAIL verification.

// Test 1: syscall_send_empty_block requires
//   0 <= blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS.
// Passing MAX_NUM_ENDPOINT_DESCRIPTORS (128) violates the upper bound.
// SHOULD FAIL
proof fn test_boundary_endpoint_index_at_max() {
    let idx: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(0 <= idx && idx < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 2: syscall_send_empty_block requires
//   thread_dom().contains(sender_thread_ptr).
// If the thread is not in the domain, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_thread_not_in_domain(
    thread_dom: Set<ThreadPtr>,
    sender_thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(sender_thread_ptr),
{
    assert(thread_dom.contains(sender_thread_ptr));
}

// Test 3: syscall_send_empty_block requires
//   get_thread(sender_thread_ptr).state == ThreadState::RUNNING.
// If thread is BLOCKED, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_thread_blocked_not_running() {
    let state = ThreadState::BLOCKED;
    assert(state == ThreadState::RUNNING);
}

// Test 4: syscall_send_empty_block requires
//   get_thread(sender_thread_ptr).state == ThreadState::RUNNING.
// If thread is SCHEDULED, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_thread_scheduled_not_running() {
    let state = ThreadState::SCHEDULED;
    assert(state == ThreadState::RUNNING);
}

// Test 5: schedule_blocked_thread requires
//   get_endpoint(endpoint_ptr).queue.len() > 0.
// With 0-length queue, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_schedule_empty_queue() {
    let queue_len: usize = 0;
    assert(queue_len > 0);
}

// Test 6: schedule_blocked_thread requires
//   get_container(...).scheduler.len() < MAX_CONTAINER_SCHEDULER_LEN.
// At capacity (10), precondition is violated.
// SHOULD FAIL
proof fn test_boundary_scheduler_at_capacity() {
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN;
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN);
}

// Test 7: block_running_thread_and_set_trap_frame requires
//   get_endpoint(...).queue.len() < MAX_NUM_THREADS_PER_ENDPOINT.
// At max capacity (128), precondition is violated.
// SHOULD FAIL
proof fn test_boundary_endpoint_queue_full() {
    let queue_len: usize = MAX_NUM_THREADS_PER_ENDPOINT;
    assert(queue_len < MAX_NUM_THREADS_PER_ENDPOINT);
}

// Test 8: block_running_thread_and_set_trap_frame requires
//   endpoint_descriptors@[endpoint_index].is_Some().
// If None, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_endpoint_descriptor_none() {
    let descriptor: Option<EndpointPtr> = None;
    assert(descriptor.is_Some());
}

// Test 9: page_ptr2page_index requires ptr % 0x1000 == 0.
// Passing ptr = 0x1001 (not 4k-aligned) violates precondition.
// SHOULD FAIL
proof fn test_boundary_page_ptr_not_aligned() {
    let ptr: usize = 0x1001;
    assert(ptr % 0x1000 == 0);
}

// Test 10: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// Passing i = NUM_PAGES is out of range.
// SHOULD FAIL
proof fn test_boundary_page_index_at_max() {
    let i: usize = NUM_PAGES;
    assert(0 <= i && i < NUM_PAGES);
}

// Test 11: syscall_send_empty_block requires blocking_endpoint_index
// in valid range. Using usize::MAX is far beyond MAX_NUM_ENDPOINT_DESCRIPTORS.
// SHOULD FAIL
proof fn test_boundary_endpoint_index_overflow() {
    let idx: EndpointIdx = usize::MAX;
    assert(idx < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 12: get_endpoint requires endpoint_dom contains the ptr.
// If it doesn't, precondition violated.
// SHOULD FAIL
proof fn test_boundary_endpoint_not_in_domain(
    endpoint_dom: Set<EndpointPtr>,
    endpoint_ptr: EndpointPtr,
)
    requires
        !endpoint_dom.contains(endpoint_ptr),
{
    assert(endpoint_dom.contains(endpoint_ptr));
}

}
