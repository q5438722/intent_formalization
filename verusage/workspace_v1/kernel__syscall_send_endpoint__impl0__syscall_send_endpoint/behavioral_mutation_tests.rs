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

// ============================================================
// BEHAVIORAL MUTATION TEST 1: Queue push order mutation (no_receiver case)
// When sender is blocked (no_receiver), the spec pushes sender_thread_ptr
// to the END of the blocking endpoint queue. Asserting it was prepended
// (inserted at the front) should fail.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_queue_push_order_mutated()
{
    let old_queue: Seq<ThreadPtr> = seq![10usize, 20usize, 30usize];
    let sender_thread_ptr: ThreadPtr = 42;
    let new_queue: Seq<ThreadPtr> = old_queue.push(sender_thread_ptr);

    // Mutated: claim thread was prepended instead of appended
    let mutated_queue: Seq<ThreadPtr> = seq![42usize, 10usize, 20usize, 30usize];
    assert(new_queue =~= mutated_queue); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 2: Success path queue skip mutation
// On success, blocking endpoint queue becomes old_queue.skip(1).
// Asserting skip(2) (removing two elements) should fail.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_success_skip_count_mutated()
{
    let old_queue: Seq<ThreadPtr> = seq![100usize, 200usize, 300usize];
    let correct_new_queue = old_queue.skip(1);
    let mutated_new_queue = old_queue.skip(2);

    assert(correct_new_queue =~= mutated_new_queue); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 3: Endpoint descriptor update at wrong index
// On success, receiver_thread's endpoint_descriptors is updated at
// receiver_endpoint_payload index. Updating at a different index
// should produce a different result.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_endpoint_descriptor_wrong_index()
{
    let old_descriptors: Seq<Option<EndpointPtr>> = seq![
        None, None, None, None, None,
        None, None, None, None, None,
    ];
    let sender_endpoint_ptr: EndpointPtr = 0xABCD;
    let correct_index: int = 3;
    let wrong_index: int = 7;

    let correct_update = old_descriptors.update(correct_index, Some(sender_endpoint_ptr));
    let wrong_update = old_descriptors.update(wrong_index, Some(sender_endpoint_ptr));

    assert(correct_update =~= wrong_update); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 4: Owning threads insert with wrong pair
// On success, sender_endpoint's owning_threads gets
// (receiver_thread_ptr, receiver_endpoint_payload) inserted.
// Inserting a different pair should yield a different set.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_owning_threads_wrong_pair()
{
    let old_owning: Set<(ThreadPtr, EndpointIdx)> = Set::empty();
    let receiver_thread_ptr: ThreadPtr = 42;
    let correct_payload: EndpointIdx = 3;
    let wrong_payload: EndpointIdx = 9;

    let correct_new = old_owning.insert((receiver_thread_ptr, correct_payload));
    let wrong_new = old_owning.insert((receiver_thread_ptr, wrong_payload));

    assert(correct_new =~= wrong_new); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 5: Queue state mutation (receiver_queue_empty case)
// When receiver_queue_empty, the spec sets queue_state to EndpointState::SEND.
// Asserting it stays RECEIVE should fail.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_queue_state_not_changed_to_send()
{
    let old_state = EndpointState::RECEIVE;
    let expected_new_state = EndpointState::SEND;

    // Mutated: claim state remains RECEIVE
    assert(old_state == expected_new_state); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 6: Sender endpoint descriptors preserved
// In the no_receiver case, the spec ensures sender's endpoint_descriptors
// are unchanged. Asserting they were modified should fail.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_sender_descriptors_mutated()
{
    let old_descriptors: Seq<Option<EndpointPtr>> = seq![
        Some(100usize), None, Some(200usize), None, None,
    ];
    // Mutated: one entry changed
    let mutated_descriptors: Seq<Option<EndpointPtr>> = seq![
        Some(100usize), Some(999usize), Some(200usize), None, None,
    ];

    assert(old_descriptors =~= mutated_descriptors); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 7: Success path with wrong endpoint value
// On success, receiver's descriptor at receiver_endpoint_payload gets
// sender_endpoint_ptr. Asserting a different value was placed should fail.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_wrong_endpoint_value_passed()
{
    let old_descriptors: Seq<Option<EndpointPtr>> = seq![None, None, None, None, None];
    let sender_endpoint_ptr: EndpointPtr = 0x1234;
    let wrong_endpoint_ptr: EndpointPtr = 0x5678;
    let index: int = 2;

    let correct = old_descriptors.update(index, Some(sender_endpoint_ptr));
    let wrong = old_descriptors.update(index, Some(wrong_endpoint_ptr));

    assert(correct =~= wrong); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 8: Success path queue becomes empty vs skip
// On success, blocking endpoint queue becomes old.skip(1), not empty.
// For a multi-element queue, skip(1) != empty.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_queue_emptied_instead_of_skipped()
{
    let old_queue: Seq<ThreadPtr> = seq![100usize, 200usize, 300usize];
    let correct_skip = old_queue.skip(1); // [200, 300]
    let mutated_empty: Seq<ThreadPtr> = Seq::empty();

    assert(correct_skip =~= mutated_empty); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 9: Owning threads insert wrong thread
// On success, owning_threads gets (receiver_thread_ptr, payload) inserted.
// Inserting (sender_thread_ptr, payload) instead should fail.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_owning_threads_wrong_thread()
{
    let old_owning: Set<(ThreadPtr, EndpointIdx)> = Set::empty();
    let receiver_thread_ptr: ThreadPtr = 42;
    let sender_thread_ptr: ThreadPtr = 99;
    let payload: EndpointIdx = 5;

    let correct = old_owning.insert((receiver_thread_ptr, payload));
    let wrong = old_owning.insert((sender_thread_ptr, payload));

    assert(correct =~= wrong); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 10: Queue skip vs pop-last
// On success, the queue becomes old.skip(1) (remove head).
// Removing the last element instead should produce a different result
// for queues with > 1 distinct elements.
// SHOULD FAIL
// ============================================================
proof fn test_behavioral_queue_pop_last_instead_of_skip()
{
    let old_queue: Seq<ThreadPtr> = seq![100usize, 200usize, 300usize];
    let correct_skip = old_queue.skip(1); // [200, 300]
    // Remove last element instead
    let mutated_pop_last = old_queue.subrange(0, old_queue.len() as int - 1); // [100, 200]

    assert(correct_skip =~= mutated_pop_last); // SHOULD FAIL
}

} // verus!
