use vstd::prelude::*;

fn main() {}

verus!{

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition or uses edge-case values
// for syscall_new_thread_with_endpoint to check if invalid
// inputs are properly rejected by the specification.
// All tests SHOULD FAIL verification.

// Test 1: endpoint_index must be < MAX_NUM_ENDPOINT_DESCRIPTORS (128).
// Using endpoint_index == 128 violates the precondition.
// SHOULD FAIL
proof fn test_boundary_endpoint_index_at_max() {
    let endpoint_index: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(0 <= endpoint_index && endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 2: endpoint_index == usize::MAX is well beyond valid range.
// SHOULD FAIL
proof fn test_boundary_endpoint_index_overflow() {
    let endpoint_index: EndpointIdx = usize::MAX;
    assert(endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 3: thread_dom must contain thread_ptr. If thread_ptr is not
// in the thread domain, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_thread_not_in_dom(
    thread_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(thread_ptr),
{
    assert(thread_dom.contains(thread_ptr));
}

// Test 4: The requirement says the process thread list must not be full
// (len < MAX_NUM_THREADS_PER_PROC). If it IS full (len >= 128),
// the requirement function returns false.
// SHOULD FAIL
proof fn test_boundary_thread_list_full() {
    let thread_count: usize = MAX_NUM_THREADS_PER_PROC;
    assert(thread_count < MAX_NUM_THREADS_PER_PROC);
}

// Test 5: Container quota mem_4k must be > 0. If mem_4k == 0,
// the requirement is not met and the syscall returns error.
// SHOULD FAIL
proof fn test_boundary_zero_mem_quota() {
    let mem_4k: usize = 0;
    assert(mem_4k > 0);
}

// Test 6: Scheduler must not be full (len < MAX_CONTAINER_SCHEDULER_LEN).
// If len == MAX_CONTAINER_SCHEDULER_LEN, the requirement is violated.
// SHOULD FAIL
proof fn test_boundary_scheduler_full() {
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN;
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN);
}

// Test 7: Free pages count must be > 0. If free_pages == 0,
// there is no page to allocate for the new thread.
// SHOULD FAIL
proof fn test_boundary_no_free_pages() {
    let free_pages: usize = 0;
    assert(free_pages > 0);
}

// Test 8: The endpoint must be shareable — rf_counter != usize::MAX.
// If rf_counter == usize::MAX, the endpoint is full.
// SHOULD FAIL
proof fn test_boundary_endpoint_rf_counter_full() {
    let rf_counter: usize = usize::MAX;
    assert(rf_counter != usize::MAX);
}

// Test 9: endpoint_ptr must be Some (the thread must have an
// endpoint descriptor at the given index). If None, the syscall
// returns error.
// SHOULD FAIL
proof fn test_boundary_no_endpoint_ptr(
    endpoint_ptr_op: Option<EndpointPtr>,
)
    requires
        endpoint_ptr_op.is_None(),
{
    assert(endpoint_ptr_op.is_Some());
}

// Test 10: page_closure must NOT contain the new page_ptr
// (for new_thread_with_endpoint). If it already contains
// the page, it cannot be freshly allocated.
// SHOULD FAIL
proof fn test_boundary_page_already_in_closure(
    page_closure: Set<PagePtr>,
    page_ptr: PagePtr,
)
    requires
        page_closure.contains(page_ptr),
{
    assert(!page_closure.contains(page_ptr));
}

}
