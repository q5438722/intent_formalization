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

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;

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

#[derive(Clone, Copy, Debug)]
pub enum SwitchDecision {
    NoSwitch,
    NoThread,
    Switch,
}

#[allow(inconsistent_fields)]
#[derive(Clone, Copy)]
pub enum RetValueType {
    SuccessUsize { value: usize },
    Error,
    Else,
}

#[derive(Clone, Copy)]
pub struct SyscallReturnStruct {
    pub error_code: RetValueType,
    pub pcid: Option<Pcid>,
    pub cr3: Option<usize>,
    pub switch_decision: SwitchDecision,
}

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs and mutates expected output relations.
// All tests SHOULD FAIL verification.

// Test 1: When endpoint does not exist (blocking_endpoint_ptr_op.is_none()),
// the spec says old =~= new (no state change). Mutate: assert state changed.
// SHOULD FAIL
proof fn test_mutation_no_endpoint_should_not_modify_state(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
    extra_thread: ThreadPtr,
)
    requires
        old_thread_dom == new_thread_dom,
        !old_thread_dom.contains(extra_thread),
{
    // Mutate: claim the new state gained a thread
    assert(new_thread_dom.contains(extra_thread));
}

// Test 2: When sender queue is full, spec says old =~= new.
// Mutate: claim the endpoint queue changed.
// SHOULD FAIL
proof fn test_mutation_full_queue_should_not_change_queue(
    old_queue_len: usize,
    new_queue_len: usize,
)
    requires
        old_queue_len >= MAX_NUM_THREADS_PER_ENDPOINT,
        old_queue_len == new_queue_len,
{
    // Mutate: claim queue grew
    assert(new_queue_len == old_queue_len + 1);
}

// Test 3: When no_receiver (SEND state, queue not full), the sender gets blocked
// and the spec says the sender's endpoint_descriptors are preserved.
// Mutate: claim the sender's descriptors changed.
// SHOULD FAIL
proof fn test_mutation_no_receiver_descriptors_should_be_preserved(
    old_descriptors: Seq<Option<EndpointPtr>>,
    new_descriptors: Seq<Option<EndpointPtr>>,
)
    requires
        old_descriptors =~= new_descriptors,
{
    // Mutate: claim a descriptor slot changed to Some
    assert(new_descriptors.len() > 0 ==>
        new_descriptors[0] != old_descriptors[0]);
}

// Test 4: On successful send, the blocking endpoint's queue should lose
// its head (receiver dequeued). Mutate: claim the queue did not change.
// SHOULD FAIL
proof fn test_mutation_success_queue_should_dequeue_head(
    old_queue: Seq<ThreadPtr>,
    new_queue: Seq<ThreadPtr>,
)
    requires
        old_queue.len() > 0,
        new_queue =~= old_queue.skip(1),
{
    // Mutate: claim the queue is unchanged
    assert(new_queue =~= old_queue);
}

// Test 5: On successful send, the receiver's endpoint descriptors get updated
// with the sender's endpoint. Mutate: claim receiver descriptors unchanged.
// SHOULD FAIL
proof fn test_mutation_success_receiver_descriptors_should_update(
    old_recv_descriptors: Seq<Option<EndpointPtr>>,
    new_recv_descriptors: Seq<Option<EndpointPtr>>,
    receiver_endpoint_payload: EndpointIdx,
    sender_endpoint_ptr: EndpointPtr,
)
    requires
        0 <= receiver_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS,
        old_recv_descriptors.len() == MAX_NUM_ENDPOINT_DESCRIPTORS as int,
        new_recv_descriptors.len() == MAX_NUM_ENDPOINT_DESCRIPTORS as int,
        new_recv_descriptors =~= old_recv_descriptors.update(
            receiver_endpoint_payload as int,
            Some(sender_endpoint_ptr),
        ),
        old_recv_descriptors[receiver_endpoint_payload as int].is_None(),
{
    // Mutate: claim receiver descriptors did not change
    assert(new_recv_descriptors =~= old_recv_descriptors);
}

// Test 6: On successful send, the owning_threads set of the passed endpoint
// gains a new entry. Mutate: claim owning_threads is unchanged.
// SHOULD FAIL
proof fn test_mutation_success_owning_threads_should_grow(
    old_owning: Set<(ThreadPtr, EndpointIdx)>,
    new_owning: Set<(ThreadPtr, EndpointIdx)>,
    receiver_thread_ptr: ThreadPtr,
    receiver_endpoint_payload: EndpointIdx,
)
    requires
        old_owning.finite(),
        new_owning =~= old_owning.insert((receiver_thread_ptr, receiver_endpoint_payload)),
        !old_owning.contains((receiver_thread_ptr, receiver_endpoint_payload)),
{
    // Mutate: claim owning threads set is unchanged
    assert(new_owning =~= old_owning);
}

// Test 7: The return value on success should have error_code == Else.
// Mutate: claim it returns Error instead.
// SHOULD FAIL
proof fn test_mutation_success_should_not_return_error() {
    // The spec on the success path returns RetValueType::Else.
    // Mutate by asserting it should be Error.
    let ret_is_else: bool = true;
    assert(!ret_is_else);
}

// Test 8: When no_receiver case, the sender thread ipc_payload should
// contain the sender_endpoint_payload. Mutate: claim different payload.
// SHOULD FAIL
proof fn test_mutation_blocked_payload_should_match(
    sender_endpoint_payload: EndpointIdx,
    stored_payload: Option<EndpointIdx>,
)
    requires
        stored_payload == Some(sender_endpoint_payload),
        sender_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS,
{
    // Mutate: claim a different payload was stored
    let wrong_payload: EndpointIdx = if sender_endpoint_payload == 0 { 1 } else { 0 };
    assert(stored_payload == Some(wrong_payload));
}

// Test 9: When receiver_queue_empty, the blocking endpoint queue_state
// should change to SEND. Mutate: claim it stays RECEIVE.
// SHOULD FAIL
proof fn test_mutation_empty_receiver_queue_state_should_become_send() {
    let new_state_is_send: bool = true;
    let new_state_is_receive: bool = !new_state_is_send;
    assert(new_state_is_receive);
}

// Test 10: On success path, the sender's endpoint_descriptors should be preserved.
// Mutate: claim the sender lost its payload endpoint.
// SHOULD FAIL
proof fn test_mutation_success_sender_descriptors_preserved(
    old_sender_desc: Seq<Option<EndpointPtr>>,
    new_sender_desc: Seq<Option<EndpointPtr>>,
)
    requires
        old_sender_desc =~= new_sender_desc,
        old_sender_desc.len() > 0,
        old_sender_desc[0].is_Some(),
{
    // Mutate: claim sender's first descriptor was cleared
    assert(new_sender_desc[0].is_None());
}

}
