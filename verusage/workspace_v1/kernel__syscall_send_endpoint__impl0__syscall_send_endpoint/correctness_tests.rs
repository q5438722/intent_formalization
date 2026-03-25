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

// ===================== BOUNDARY TESTS =====================

// BOUNDARY TEST 1: sender_thread_ptr NOT in thread_dom
// SHOULD FAIL
proof fn test_boundary_sender_not_in_thread_dom()
{
    let thread_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2).insert(3);
    let sender_thread_ptr: ThreadPtr = 999;
    assert(thread_dom.contains(sender_thread_ptr)); // SHOULD FAIL
}

// BOUNDARY TEST 2: blocking_endpoint_index at upper bound
// SHOULD FAIL
proof fn test_boundary_blocking_endpoint_index_at_max()
{
    let blocking_endpoint_index: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// BOUNDARY TEST 3: sender_endpoint_payload at upper bound
// SHOULD FAIL
proof fn test_boundary_sender_endpoint_payload_at_max()
{
    let sender_endpoint_payload: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(sender_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// BOUNDARY TEST 4: sender_endpoint_payload overflow
// SHOULD FAIL
proof fn test_boundary_sender_endpoint_payload_overflow()
{
    let sender_endpoint_payload: EndpointIdx = usize::MAX;
    assert(sender_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// BOUNDARY TEST 5: receiver_endpoint_payload >= MAX
// SHOULD FAIL
proof fn test_boundary_receiver_endpoint_payload_at_max()
{
    let receiver_endpoint_payload: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(receiver_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// BOUNDARY TEST 6: Queue at MAX is not full (false)
// SHOULD FAIL
proof fn test_boundary_queue_at_max_is_not_full()
{
    let queue_len: usize = MAX_NUM_THREADS_PER_ENDPOINT;
    assert(queue_len < MAX_NUM_THREADS_PER_ENDPOINT); // SHOULD FAIL
}

// BOUNDARY TEST 7: Scheduler at MAX is not full (false)
// SHOULD FAIL
proof fn test_boundary_scheduler_at_max_is_not_full()
{
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN;
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN); // SHOULD FAIL
}

// BOUNDARY TEST 8: rf_counter at usize::MAX
// SHOULD FAIL
proof fn test_boundary_rf_counter_at_max()
{
    let rf_counter: usize = usize::MAX;
    assert(rf_counter != usize::MAX); // SHOULD FAIL
}

// BOUNDARY TEST 9: Empty thread domain
// SHOULD FAIL
proof fn test_boundary_empty_thread_domain()
{
    let thread_dom: Set<ThreadPtr> = Set::empty();
    let sender_thread_ptr: ThreadPtr = 1;
    assert(thread_dom.contains(sender_thread_ptr)); // SHOULD FAIL
}

// BOUNDARY TEST 10: blocking_endpoint_index overflow
// SHOULD FAIL
proof fn test_boundary_blocking_endpoint_index_overflow()
{
    let blocking_endpoint_index: EndpointIdx = usize::MAX;
    assert(blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// ================ BEHAVIORAL MUTATION TESTS ================

// BEHAVIORAL TEST 1: Queue push order mutation
// SHOULD FAIL
proof fn test_behavioral_queue_push_order_mutated()
{
    let old_queue: Seq<ThreadPtr> = seq![10usize, 20usize, 30usize];
    let sender_thread_ptr: ThreadPtr = 42;
    let new_queue: Seq<ThreadPtr> = old_queue.push(sender_thread_ptr);
    let mutated_queue: Seq<ThreadPtr> = seq![42usize, 10usize, 20usize, 30usize];
    assert(new_queue =~= mutated_queue); // SHOULD FAIL
}

// BEHAVIORAL TEST 2: Success queue skip(2) instead of skip(1)
// SHOULD FAIL
proof fn test_behavioral_success_skip_count_mutated()
{
    let old_queue: Seq<ThreadPtr> = seq![100usize, 200usize, 300usize];
    let correct_new_queue = old_queue.skip(1);
    let mutated_new_queue = old_queue.skip(2);
    assert(correct_new_queue =~= mutated_new_queue); // SHOULD FAIL
}

// BEHAVIORAL TEST 3: Endpoint descriptor update at wrong index
// SHOULD FAIL
proof fn test_behavioral_endpoint_descriptor_wrong_index()
{
    let old_descriptors: Seq<Option<EndpointPtr>> = seq![
        None, None, None, None, None,
        None, None, None, None, None,
    ];
    let sender_endpoint_ptr: EndpointPtr = 0xABCD;
    let correct_update = old_descriptors.update(3, Some(sender_endpoint_ptr));
    let wrong_update = old_descriptors.update(7, Some(sender_endpoint_ptr));
    assert(correct_update =~= wrong_update); // SHOULD FAIL
}

// BEHAVIORAL TEST 4: Owning threads insert with wrong pair
// SHOULD FAIL
proof fn test_behavioral_owning_threads_wrong_pair()
{
    let old_owning: Set<(ThreadPtr, EndpointIdx)> = Set::empty();
    let correct_new = old_owning.insert((42usize, 3usize));
    let wrong_new = old_owning.insert((42usize, 9usize));
    assert(correct_new =~= wrong_new); // SHOULD FAIL
}

// BEHAVIORAL TEST 5: Queue state not changed to SEND
// SHOULD FAIL
proof fn test_behavioral_queue_state_not_changed_to_send()
{
    let old_state = EndpointState::RECEIVE;
    let expected_new_state = EndpointState::SEND;
    assert(old_state == expected_new_state); // SHOULD FAIL
}

// BEHAVIORAL TEST 6: Sender descriptors mutated
// SHOULD FAIL
proof fn test_behavioral_sender_descriptors_mutated()
{
    let old_descriptors: Seq<Option<EndpointPtr>> = seq![
        Some(100usize), None, Some(200usize), None, None,
    ];
    let mutated_descriptors: Seq<Option<EndpointPtr>> = seq![
        Some(100usize), Some(999usize), Some(200usize), None, None,
    ];
    assert(old_descriptors =~= mutated_descriptors); // SHOULD FAIL
}

// BEHAVIORAL TEST 7: Wrong endpoint value passed
// SHOULD FAIL
proof fn test_behavioral_wrong_endpoint_value_passed()
{
    let old_descriptors: Seq<Option<EndpointPtr>> = seq![None, None, None, None, None];
    let correct = old_descriptors.update(2, Some(0x1234usize));
    let wrong = old_descriptors.update(2, Some(0x5678usize));
    assert(correct =~= wrong); // SHOULD FAIL
}

// BEHAVIORAL TEST 8: Queue emptied instead of skipped
// SHOULD FAIL
proof fn test_behavioral_queue_emptied_instead_of_skipped()
{
    let old_queue: Seq<ThreadPtr> = seq![100usize, 200usize, 300usize];
    let correct_skip = old_queue.skip(1);
    let mutated_empty: Seq<ThreadPtr> = Seq::empty();
    assert(correct_skip =~= mutated_empty); // SHOULD FAIL
}

// BEHAVIORAL TEST 9: Owning threads wrong thread
// SHOULD FAIL
proof fn test_behavioral_owning_threads_wrong_thread()
{
    let old_owning: Set<(ThreadPtr, EndpointIdx)> = Set::empty();
    let correct = old_owning.insert((42usize, 5usize));
    let wrong = old_owning.insert((99usize, 5usize));
    assert(correct =~= wrong); // SHOULD FAIL
}

// BEHAVIORAL TEST 10: Queue pop-last instead of skip
// SHOULD FAIL
proof fn test_behavioral_queue_pop_last_instead_of_skip()
{
    let old_queue: Seq<ThreadPtr> = seq![100usize, 200usize, 300usize];
    let correct_skip = old_queue.skip(1);
    let mutated_pop_last = old_queue.subrange(0, old_queue.len() as int - 1);
    assert(correct_skip =~= mutated_pop_last); // SHOULD FAIL
}

// =================== LOGICAL TESTS ===================

// LOGICAL TEST 1: Push commutativity (not entailed)
// SHOULD FAIL
proof fn test_logical_push_commutativity()
{
    let q: Seq<ThreadPtr> = Seq::empty();
    let q1 = q.push(1usize).push(2usize);
    let q2 = q.push(2usize).push(1usize);
    assert(q1 =~= q2); // SHOULD FAIL
}

// LOGICAL TEST 2: skip preserves length (false)
// SHOULD FAIL
proof fn test_logical_skip_preserves_length()
{
    let q: Seq<ThreadPtr> = seq![10usize, 20usize, 30usize];
    let q_skipped = q.skip(1);
    assert(q_skipped.len() == q.len()); // SHOULD FAIL
}

// LOGICAL TEST 3: Endpoint state always SEND (false)
// SHOULD FAIL
proof fn test_logical_endpoint_state_always_send()
{
    let state = EndpointState::RECEIVE;
    assert(state == EndpointState::SEND); // SHOULD FAIL
}

// LOGICAL TEST 4: All error paths preserve state (false for blocking)
// SHOULD FAIL
proof fn test_logical_all_errors_preserve_state()
{
    let old_queue: Seq<ThreadPtr> = seq![10usize, 20usize];
    let new_queue = old_queue.push(30usize);
    assert(old_queue =~= new_queue); // SHOULD FAIL
}

// LOGICAL TEST 5: Insert different elements yields same set (false)
// SHOULD FAIL
proof fn test_logical_insert_different_elements_equal()
{
    let s: Set<(ThreadPtr, EndpointIdx)> = Set::empty();
    assert(s.insert((1usize, 2usize)) =~= s.insert((3usize, 4usize))); // SHOULD FAIL
}

// LOGICAL TEST 6: skip on empty reduces length (false)
// SHOULD FAIL
proof fn test_logical_skip_empty_reduces_length()
{
    let q: Seq<ThreadPtr> = Seq::empty();
    let q_skipped = q.skip(1);
    assert(q_skipped.len() < q.len()); // SHOULD FAIL
}

// LOGICAL TEST 7: SEND == RECEIVE (false)
// SHOULD FAIL
proof fn test_logical_send_receive_are_same()
{
    assert(EndpointState::SEND == EndpointState::RECEIVE); // SHOULD FAIL
}

// LOGICAL TEST 8: Update at different indices yields same seq (false)
// SHOULD FAIL
proof fn test_logical_update_different_indices_equal()
{
    let s: Seq<Option<EndpointPtr>> = seq![None, None, None, None, None];
    let v: Option<EndpointPtr> = Some(42usize);
    assert(s.update(1, v) =~= s.update(3, v)); // SHOULD FAIL
}

// LOGICAL TEST 9: push then skip is identity (false)
// SHOULD FAIL
proof fn test_logical_push_then_skip_is_identity()
{
    let q: Seq<ThreadPtr> = seq![10usize, 20usize];
    let after = q.push(30usize).skip(1);
    assert(q =~= after); // SHOULD FAIL
}

// LOGICAL TEST 10: sender == receiver (contradicts pass_endpoint precondition)
// SHOULD FAIL
proof fn test_logical_sender_equals_receiver()
{
    let sender: ThreadPtr = 42;
    let receiver: ThreadPtr = 42;
    assert(sender != receiver); // SHOULD FAIL
}

} // verus!
