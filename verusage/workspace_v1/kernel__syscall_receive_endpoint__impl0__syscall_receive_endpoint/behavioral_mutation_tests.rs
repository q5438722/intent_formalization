use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus! {

// ============================================================
// Minimal type/const definitions from target file
// ============================================================

pub type IOid = usize;
pub type CpuId = usize;
pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type Pcid = usize;
pub type PAddr = usize;
pub type VAddr = usize;
pub type SLLIndex = i32;

pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EndpointState {
    RECEIVE,
    SEND,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThreadState {
    SCHEDULED,
    BLOCKED,
    RUNNING,
}

// ============================================================
// BEHAVIORAL MUTATION TEST 1: Queue push order mutation
// When a thread is blocked (no_sender case), the spec says the
// queue is extended by push(thread_ptr) at the END.
// Asserting it was prepended (inserted at front) should fail.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_queue_push_order_mutated()
{
    let old_queue: Seq<ThreadPtr> = seq![10usize, 20usize, 30usize];
    let thread_ptr: ThreadPtr = 42;
    let new_queue: Seq<ThreadPtr> = old_queue.push(thread_ptr);

    // Mutated claim: the thread was inserted at position 0 instead of pushed to end
    let mutated_queue: Seq<ThreadPtr> = seq![42usize, 10usize, 20usize, 30usize];
    assert(new_queue =~= mutated_queue); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 2: Success queue skip mutation
// On success, the spec says the shared endpoint's queue becomes
// old_queue.skip(1) (removing the head sender).
// Asserting skip(2) (removing two elements) should fail.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_success_skip_count_mutated()
{
    let old_queue: Seq<ThreadPtr> = seq![100usize, 200usize, 300usize];
    let correct_new_queue = old_queue.skip(1); // removes first element
    let mutated_new_queue = old_queue.skip(2); // removes first two

    assert(correct_new_queue =~= mutated_new_queue); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 3: Endpoint descriptor update target mutation
// On success, the dst_thread's endpoint_descriptors is updated at
// position `to` (receiver_endpoint_payload) with the payload endpoint.
// Asserting update at a DIFFERENT index should fail.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_endpoint_descriptor_wrong_index()
{
    let old_descriptors: Seq<Option<EndpointPtr>> = seq![
        None, None, None, None, None,
        None, None, None, None, None,
    ];
    let payload_endpoint_ptr: EndpointPtr = 0xABCD;
    let correct_index: int = 3;
    let wrong_index: int = 5;

    let correct_update = old_descriptors.update(correct_index, Some(payload_endpoint_ptr));
    let wrong_update = old_descriptors.update(wrong_index, Some(payload_endpoint_ptr));

    assert(correct_update =~= wrong_update); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 4: Owning threads insert mutation
// On success, the payload endpoint's owning_threads set should
// have (dst_thread_ptr, to) inserted.
// Asserting a different (thread, index) pair was inserted should fail.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_owning_threads_wrong_pair()
{
    let old_owning: Set<(ThreadPtr, EndpointIdx)> = Set::empty();
    let dst_thread_ptr: ThreadPtr = 42;
    let correct_to: EndpointIdx = 3;
    let wrong_to: EndpointIdx = 7;

    let correct_new = old_owning.insert((dst_thread_ptr, correct_to));
    let wrong_new = old_owning.insert((dst_thread_ptr, wrong_to));

    assert(correct_new =~= wrong_new); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 5: Fail case — state should be unchanged for general error
// In the fail spec, when the endpoint does NOT exist (neither no_sender 
// nor sender_queue_empty), old =~= new (no state change).
// Mutating to claim the state changed should fail.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_fail_state_changed()
{
    let old_queue: Seq<ThreadPtr> = seq![10usize, 20usize];
    let new_queue: Seq<ThreadPtr> = seq![10usize]; // one element removed

    // In the general-error fail case, old =~= new, so queues must be identical
    assert(old_queue =~= new_queue); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 6: Queue state mutation in fail-with-block case
// When sender_queue_empty is true, the queue_state should change to RECEIVE.
// Asserting it stays SEND should fail.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_queue_state_not_changed_to_receive()
{
    let new_state = EndpointState::RECEIVE;
    let expected_wrong = EndpointState::SEND;
    assert(new_state =~= expected_wrong); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 7: IPC payload mutation
// When blocking, the thread's ipc_payload should be set to the 
// receiver_endpoint_payload value. Asserting a different payload
// value was stored should fail.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_ipc_payload_wrong_value()
{
    let receiver_endpoint_payload: EndpointIdx = 5;
    let stored_payload: Option<EndpointIdx> = Some(receiver_endpoint_payload);
    let wrong_payload: Option<EndpointIdx> = Some(99); // wrong value

    assert(stored_payload =~= wrong_payload); // SHOULD FAIL
}

}
