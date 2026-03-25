use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type ProcPtr = usize;
pub type PagePtr = usize;

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) of
// kernel_drop_endpoint or drop_endpoint.
// All tests SHOULD FAIL verification.

// Test 1: kernel_drop_endpoint requires 0 <= edp_idx < MAX_NUM_ENDPOINT_DESCRIPTORS.
// Using edp_idx == MAX_NUM_ENDPOINT_DESCRIPTORS (128) violates the upper bound.
// SHOULD FAIL
proof fn test_boundary_edp_idx_at_max() {
    let edp_idx: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(0 <= edp_idx && edp_idx < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 2: kernel_drop_endpoint requires 0 <= edp_idx < MAX_NUM_ENDPOINT_DESCRIPTORS.
// Using a very large value (usize::MAX) should violate the bound.
// SHOULD FAIL
proof fn test_boundary_edp_idx_usize_max() {
    let edp_idx: EndpointIdx = usize::MAX;
    assert(edp_idx < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 3: kernel_drop_endpoint requires thread_dom().contains(thread_ptr).
// If thread_ptr is not in the domain, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_thread_not_in_domain(
    thread_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(thread_ptr),
{
    assert(thread_dom.contains(thread_ptr));
}

// Test 4: kernel_drop_endpoint requires that if the thread is BLOCKED,
// its blocking_endpoint_index.unwrap() != edp_idx.
// Here we violate this: the blocked thread's blocking index IS the same
// as the endpoint we try to drop.
// SHOULD FAIL
proof fn test_boundary_blocked_thread_drops_blocking_endpoint(
    blocking_idx: EndpointIdx,
    edp_idx: EndpointIdx,
)
    requires
        blocking_idx == edp_idx,
        0 <= edp_idx < MAX_NUM_ENDPOINT_DESCRIPTORS,
{
    // The precondition says blocking_endpoint_index.unwrap() != edp_idx
    // but we set them equal, so this should be rejected.
    assert(blocking_idx != edp_idx);
}

// Test 5: kernel_drop_endpoint requires old(self).wf().
// If the kernel is not well-formed, the function should not be callable.
// We encode this as: a false wf should not let us proceed.
// SHOULD FAIL
proof fn test_boundary_kernel_not_wf(kernel_wf: bool)
    requires
        kernel_wf == false,
{
    assert(kernel_wf);
}

// Test 6: Edge case: edp_idx = 0 is valid, but we test that edp_idx = -1
// (as usize, which wraps) violates the bound. Since EndpointIdx is usize,
// usize::MAX is the wrap-around of -1.
// SHOULD FAIL
proof fn test_boundary_edp_idx_negative_wrap() {
    let edp_idx: usize = 200;
    assert(edp_idx < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

}
