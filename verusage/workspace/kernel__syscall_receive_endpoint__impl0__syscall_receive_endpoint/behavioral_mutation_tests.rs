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

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs matching the spec postconditions
// and mutates expected outputs or relations.
// Tests whether incorrect behaviors are rejected by the spec.
// All tests SHOULD FAIL verification.

// Test 1: When the endpoint does not exist (blocking_endpoint_ptr_op.is_none()),
// the syscall returns error and old =~= new. Mutated: claim kernel changed.
// Models: ret.is_error() ==> syscall_receive_endpoint_fail ... old =~= new
// SHOULD FAIL
proof fn test_mutation_no_endpoint_kernel_changes(
    old_val: usize,
    new_val: usize,
)
    requires
        old_val == new_val,  // models old =~= new (state unchanged on fail)
{
    // mutated: assert kernel state changed
    assert(old_val != new_val);
}

// Test 2: On success (no error), the shared endpoint queue must lose its head.
// queue@ == old.queue@.skip(1). Mutated: claim queue is unchanged.
// Models: new.get_endpoint(shared_endpoint).queue@ =~= old.get_endpoint(shared_endpoint).queue@.skip(1)
// SHOULD FAIL
proof fn test_mutation_success_queue_unchanged(
    old_queue: Seq<ThreadPtr>,
    new_queue: Seq<ThreadPtr>,
)
    requires
        old_queue.len() > 0,
        new_queue =~= old_queue.skip(1),
{
    // mutated: assert queue hasn't changed
    assert(new_queue =~= old_queue);
}

// Test 3: On fail with no_sender, the thread should be blocked and pushed
// onto the queue. Mutated: claim queue didn't grow.
// Models: new.get_endpoint(endpoint_ptr).queue@ =~= old.get_endpoint(endpoint_ptr).queue@.push(thread_ptr)
// SHOULD FAIL
proof fn test_mutation_block_queue_not_grown(
    old_queue: Seq<ThreadPtr>,
    new_queue: Seq<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        new_queue =~= old_queue.push(thread_ptr),
{
    // mutated: assert queue is same length
    assert(new_queue.len() == old_queue.len());
}

// Test 4: On success, dst thread endpoint_descriptors must be updated at the
// receiver_endpoint_payload index. Mutated: claim descriptors unchanged.
// Models: new.get_thread(dst).endpoint_descriptors@ =~= old...update(to, Some(src_payload_ptr))
// SHOULD FAIL
proof fn test_mutation_dst_descriptors_unchanged(
    old_descriptors: Seq<Option<EndpointPtr>>,
    new_descriptors: Seq<Option<EndpointPtr>>,
    payload_idx: int,
    src_payload_ptr: EndpointPtr,
)
    requires
        0 <= payload_idx < old_descriptors.len() as int,
        new_descriptors =~= old_descriptors.update(payload_idx, Some(src_payload_ptr)),
{
    // mutated: assert descriptors didn't change
    assert(new_descriptors =~= old_descriptors);
}

// Test 5: On success, the payload endpoint's owning_threads must include
// the new (dst_thread, to) pair. Mutated: claim it wasn't inserted.
// Models: new.get_endpoint(src_payload_ptr).owning_threads@ =~= old...insert((dst, to))
// SHOULD FAIL
proof fn test_mutation_owning_threads_not_updated(
    old_owners: Set<(ThreadPtr, EndpointIdx)>,
    new_owners: Set<(ThreadPtr, EndpointIdx)>,
    dst_thread_ptr: ThreadPtr,
    to_idx: EndpointIdx,
)
    requires
        new_owners =~= old_owners.insert((dst_thread_ptr, to_idx)),
{
    // mutated: assert owning_threads unchanged
    assert(new_owners =~= old_owners);
}

// Test 6: On failure, error return should have is_error() == true.
// Mutated: claim error return is NOT an error.
// Models: ret.is_error() ==> syscall_receive_endpoint_fail(...)
// SHOULD FAIL
proof fn test_mutation_error_not_flagged(
    is_error: bool,
)
    requires
        is_error == true,
{
    // mutated: assert is_error is false
    assert(!is_error);
}

// Test 7: On failure with sender_queue_empty, queue_state should change
// to RECEIVE. Mutated: claim queue_state stayed as SEND.
// Models: new.get_endpoint(ptr).queue_state =~= EndpointState::RECEIVE
// SHOULD FAIL
proof fn test_mutation_queue_state_not_changed_to_receive(
    new_queue_state: u8,
)
    requires
        new_queue_state == 0,  // 0 = RECEIVE
{
    // mutated: assert queue_state is still SEND (1)
    assert(new_queue_state == 1);
}

// Test 8: On success, the return should NOT be an error.
// Mutated: claim success path returns error.
// Models: !ret.is_error() ==> syscall_receive_endpoint_success(...)
// SHOULD FAIL
proof fn test_mutation_success_returns_error(
    is_error: bool,
)
    requires
        is_error == false,
{
    // mutated: assert is_error is true
    assert(is_error);
}

}
