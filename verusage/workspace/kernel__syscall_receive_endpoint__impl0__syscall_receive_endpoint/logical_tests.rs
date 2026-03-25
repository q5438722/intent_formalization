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

// ===================== LOGICAL TESTS =====================
// Each test generates a property NOT explicitly guaranteed
// by the specification: determinism, stronger inequalities,
// structural/global assumptions, cross-function misuse.
// Tests whether the spec allows unintended reasoning.
// All tests SHOULD FAIL verification.

// Test 1: The spec does NOT guarantee that blocking_endpoint_index ==
// receiver_endpoint_payload. They are independent parameters.
// Stronger property: claim they must be equal.
// SHOULD FAIL
proof fn test_logical_endpoint_indices_must_be_equal(
    blocking_endpoint_index: EndpointIdx,
    receiver_endpoint_payload: EndpointIdx,
)
    requires
        0 <= blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS,
        0 <= receiver_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS,
{
    // Not guaranteed: these two indices must be equal
    assert(blocking_endpoint_index == receiver_endpoint_payload);
}

// Test 2: The spec does NOT guarantee that old thread_dom is a
// strict subset of new thread_dom. thread_dom should be preserved.
// Stronger property: claim new thread_dom is strictly larger.
// SHOULD FAIL
proof fn test_logical_thread_dom_grows(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
)
    requires
        new_thread_dom =~= old_thread_dom,  // spec: domains unchanged
{
    // Not guaranteed: new domain is strictly larger
    assert(old_thread_dom.len() < new_thread_dom.len());
}

// Test 3: The spec does NOT guarantee that the syscall is deterministic
// in terms of uniquely determining new kernel state from old + inputs.
// But we query: given same old state and inputs, is the result unique?
// Stronger property: claim two different new states are impossible.
// SHOULD FAIL
proof fn test_logical_determinism_of_result(
    ret1_is_error: bool,
    ret2_is_error: bool,
)
    requires
        ret1_is_error != ret2_is_error,
{
    // Not guaranteed: same inputs produce same result
    assert(ret1_is_error == ret2_is_error);
}

// Test 4: The spec does NOT guarantee that the endpoint queue is
// always non-empty after the syscall. It could be empty in failure
// cases. Stronger property: claim queue is always non-empty.
// SHOULD FAIL
proof fn test_logical_queue_always_nonempty(
    queue: Seq<ThreadPtr>,
) {
    // Not guaranteed: queue is always non-empty
    assert(queue.len() > 0);
}

// Test 5: The spec does NOT guarantee that receiver and sender are
// in the same container. They can be in different containers as
// long as the ancestor check passes.
// Stronger property: claim they must be in same container.
// SHOULD FAIL
proof fn test_logical_same_container_required(
    receiver_container: ContainerPtr,
    sender_container: ContainerPtr,
    is_ancestor: bool,
)
    requires
        is_ancestor || receiver_container == sender_container,
        is_ancestor,
        receiver_container != sender_container,
{
    // Not guaranteed: receiver and sender in same container
    assert(receiver_container == sender_container);
}

// Test 6: The spec does NOT guarantee that pass_endpoint preserves
// the rf_counter value exactly. It may change (increment by 1).
// Stronger property: claim rf_counter is unchanged.
// SHOULD FAIL
proof fn test_logical_rf_counter_unchanged(
    old_rf: usize,
    new_owning_threads_len: usize,
)
    requires
        new_owning_threads_len == old_rf + 1,  // endpoint_perms_wf: rf_counter == owning_threads@.len()
        old_rf < usize::MAX,
{
    // Not guaranteed: rf_counter stays the same
    assert(new_owning_threads_len == old_rf);
}

// Test 7: The spec does NOT guarantee that all endpoints remain
// unchanged. At least the shared endpoint and payload endpoint
// can change. Stronger property: claim ALL endpoints unchanged.
// SHOULD FAIL
proof fn test_logical_all_endpoints_unchanged(
    old_queue: Seq<ThreadPtr>,
    new_queue: Seq<ThreadPtr>,
)
    requires
        old_queue.len() > 0,
        new_queue =~= old_queue.skip(1),
{
    // Not guaranteed: all endpoints unchanged
    assert(new_queue =~= old_queue);
}

// Test 8: The spec does NOT guarantee that the sender_thread_ptr
// is different from the receiver_thread_ptr. However, in the
// success path, pass_endpoint requires src != dst.
// Stronger property: claim they could be equal in the precondition.
// SHOULD FAIL
proof fn test_logical_sender_equals_receiver(
    src: ThreadPtr,
    dst: ThreadPtr,
)
    requires
        src != dst,  // pass_endpoint requires src_thread_ptr != dst_thread_ptr
{
    // Not guaranteed: sender can equal receiver
    assert(src == dst);
}

}
