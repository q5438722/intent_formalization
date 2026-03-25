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

#[derive(Clone, Copy, Debug, PartialEq)]
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

// ============================================================
// LOGICAL TEST 1: Determinism — same queue, different thread ptrs
// The spec does not guarantee that push order is commutative.
// push(a).push(b) != push(b).push(a) for distinct a, b.
// Asserting commutativity of queue push is not entailed.
// SHOULD FAIL
// ============================================================
proof fn test_logical_push_commutativity()
{
    let q: Seq<ThreadPtr> = Seq::empty();
    let a: ThreadPtr = 1;
    let b: ThreadPtr = 2;

    let q1 = q.push(a).push(b);
    let q2 = q.push(b).push(a);

    assert(q1 =~= q2); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 2: Stronger inequality — skip(1) preserves length
// The spec says queue becomes old.skip(1) on success. But skip(1)
// reduces length by 1 for non-empty queues. Asserting length
// is preserved is false.
// SHOULD FAIL
// ============================================================
proof fn test_logical_skip_preserves_length()
{
    let q: Seq<ThreadPtr> = seq![10usize, 20usize, 30usize];
    let q_skipped = q.skip(1);
    assert(q_skipped.len() == q.len()); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 3: Structural assumption — endpoint state is always SEND
// The spec handles RECEIVE and SEND branches differently.
// Asserting the state must always be SEND is not entailed.
// SHOULD FAIL
// ============================================================
proof fn test_logical_endpoint_state_always_send()
{
    let state = EndpointState::RECEIVE;
    assert(state == EndpointState::SEND); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 4: Global assumption — all error paths imply old =~= new
// The no_receiver path does NOT leave old =~= new; it modifies
// the endpoint queue (push) and sender's ipc_payload.
// Claiming all non-success paths preserve state exactly is false.
// SHOULD FAIL
// ============================================================
proof fn test_logical_all_errors_preserve_state()
{
    // In the no_receiver case, the queue is modified (push).
    // So "error implies no change" is too strong.
    let old_queue: Seq<ThreadPtr> = seq![10usize, 20usize];
    let sender: ThreadPtr = 30;
    let new_queue = old_queue.push(sender);
    // Claiming queue unchanged after blocking (error path)
    assert(old_queue =~= new_queue); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 5: Cross-function misuse — insert is idempotent
// The spec inserts (receiver_thread_ptr, payload) into owning_threads.
// Set insert is NOT idempotent when checking the resulting set
// against an expected different set.
// Asserting insert(x) == insert(y) for x != y should fail.
// SHOULD FAIL
// ============================================================
proof fn test_logical_insert_different_elements_equal()
{
    let s: Set<(ThreadPtr, EndpointIdx)> = Set::empty();
    let x: (ThreadPtr, EndpointIdx) = (1, 2);
    let y: (ThreadPtr, EndpointIdx) = (3, 4);
    assert(s.insert(x) =~= s.insert(y)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 6: Stronger property — skip on empty still empty
// skip(1) on an empty sequence is still empty.
// But asserting skip(1) makes length strictly less is false for empty.
// SHOULD FAIL
// ============================================================
proof fn test_logical_skip_empty_reduces_length()
{
    let q: Seq<ThreadPtr> = Seq::empty();
    let q_skipped = q.skip(1);
    assert(q_skipped.len() < q.len()); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 7: Determinism — queue state after no_receiver vs receiver_queue_empty
// These are two distinct branches producing different outcomes.
// no_receiver: queue_state unchanged. receiver_queue_empty: set to SEND.
// Asserting both produce the same endpoint state is false when
// the original state is RECEIVE (it stays RECEIVE in no_receiver
// but the no_receiver branch requires state == SEND, so this tests
// that SEND != RECEIVE).
// SHOULD FAIL
// ============================================================
proof fn test_logical_send_receive_are_same()
{
    assert(EndpointState::SEND == EndpointState::RECEIVE); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 8: Structural assumption — sequence update is order-insensitive
// The spec updates a specific index. Two updates at different indices
// then checking extensional equality should fail.
// SHOULD FAIL
// ============================================================
proof fn test_logical_update_different_indices_equal()
{
    let s: Seq<Option<EndpointPtr>> = seq![None, None, None, None, None];
    let v: Option<EndpointPtr> = Some(42);
    let s1 = s.update(1, v);
    let s2 = s.update(3, v);
    assert(s1 =~= s2); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 9: Global assumption — push then skip(1) is identity
// push(x) then skip(1) is NOT identity on a non-empty sequence.
// It shifts all elements and drops the original head.
// SHOULD FAIL
// ============================================================
proof fn test_logical_push_then_skip_is_identity()
{
    let q: Seq<ThreadPtr> = seq![10usize, 20usize];
    let x: ThreadPtr = 30;
    let after_push = q.push(x);
    let after_skip = after_push.skip(1);
    // push appends to end, skip removes from front — not identity
    assert(q =~= after_skip); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 10: Cross-function misuse — sender_thread_ptr == receiver_thread_ptr
// The spec (via pass_endpoint) requires src_thread_ptr != dst_thread_ptr.
// Asserting they can be equal contradicts this precondition.
// SHOULD FAIL
// ============================================================
proof fn test_logical_sender_equals_receiver()
{
    let sender: ThreadPtr = 42;
    let receiver: ThreadPtr = 42;
    // pass_endpoint requires src_thread_ptr != dst_thread_ptr
    assert(sender != receiver); // SHOULD FAIL
}

} // verus!
