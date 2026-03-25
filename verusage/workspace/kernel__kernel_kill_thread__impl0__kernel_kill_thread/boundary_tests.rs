use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition of an executable function.
// All tests SHOULD FAIL verification.

// --- kernel_kill_thread boundary tests ---

// Test 1: kernel_kill_thread requires thread_ptr in thread_dom.
// If thread_ptr is NOT in the domain, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_kill_thread_not_in_dom(
    thread_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(thread_ptr),
{
    // Attempt to assert the precondition holds — it should not.
    assert(thread_dom.contains(thread_ptr));
}

// Test 2: kernel_kill_thread requires self.wf().
// If wf is false, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_kill_thread_no_wf(wf: bool)
    requires
        wf == false,
{
    assert(wf);
}

// --- kernel_drop_endpoint boundary tests ---

// Test 3: kernel_drop_endpoint requires 0 <= edp_idx < MAX_NUM_ENDPOINT_DESCRIPTORS.
// Using edp_idx == MAX_NUM_ENDPOINT_DESCRIPTORS violates upper bound.
// SHOULD FAIL
proof fn test_boundary_drop_endpoint_index_at_max() {
    let edp_idx: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(0 <= edp_idx && edp_idx < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 4: kernel_drop_endpoint requires edp_idx < 128.
// edp_idx == usize::MAX is far out of range.
// SHOULD FAIL
proof fn test_boundary_drop_endpoint_index_overflow() {
    let edp_idx: EndpointIdx = usize::MAX;
    assert(edp_idx < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 5: kernel_drop_endpoint requires thread_ptr in thread_dom.
// If the thread is not in the domain, it's invalid.
// SHOULD FAIL
proof fn test_boundary_drop_endpoint_thread_not_in_dom(
    thread_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(thread_ptr),
{
    assert(thread_dom.contains(thread_ptr));
}

// --- page_ptr2page_index boundary tests ---

// Test 6: page_ptr2page_index requires ptr % 0x1000 == 0.
// A non-aligned pointer (ptr = 1) violates the precondition.
// SHOULD FAIL
proof fn test_boundary_page_ptr_not_aligned() {
    let ptr: usize = 1;
    assert(ptr % 0x1000 == 0);
}

// Test 7: page_ptr2page_index requires ptr % 0x1000 == 0.
// ptr = 0xFFF is not page-aligned.
// SHOULD FAIL
proof fn test_boundary_page_ptr_just_below_alignment() {
    let ptr: usize = 0xFFF;
    assert(ptr % 0x1000 == 0);
}

// --- page_index2page_ptr boundary tests ---

// Test 8: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// i == NUM_PAGES is out of bounds.
// SHOULD FAIL
proof fn test_boundary_page_index_at_max() {
    let i: usize = NUM_PAGES;
    assert(0 <= i && i < NUM_PAGES);
}

// Test 9: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// i == usize::MAX is far out of bounds.
// SHOULD FAIL
proof fn test_boundary_page_index_overflow() {
    let i: usize = usize::MAX;
    assert(i < NUM_PAGES);
}

// --- kill_scheduled_thread boundary tests ---

// Test 10: kill_scheduled_thread requires all endpoint descriptors to be None.
// If any descriptor is Some, the precondition is violated.
// This tests the constraint that ALL endpoints must be dropped before killing.
// SHOULD FAIL
proof fn test_boundary_kill_scheduled_endpoint_not_none(
    descriptor_is_none: bool,
)
    requires
        descriptor_is_none == false,
{
    // The spec requires all endpoints be None; assert the opposite
    assert(descriptor_is_none);
}

}
