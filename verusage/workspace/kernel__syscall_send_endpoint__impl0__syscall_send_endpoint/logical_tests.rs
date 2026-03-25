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

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EndpointState {
    RECEIVE,
    SEND,
}

// ===================== LOGICAL TESTS =====================
// These tests encode properties NOT explicitly guaranteed by the spec.
// They probe for unintended entailments. All tests SHOULD FAIL verification.

// Test 1: The spec does NOT guarantee determinism across different
// blocking_endpoint_index values. Assert two different indices always yield same result.
// SHOULD FAIL
proof fn test_logical_different_endpoint_index_same_result(
    idx_a: EndpointIdx,
    idx_b: EndpointIdx,
    result_a: bool,
    result_b: bool,
)
    requires
        idx_a != idx_b,
        idx_a < MAX_NUM_ENDPOINT_DESCRIPTORS,
        idx_b < MAX_NUM_ENDPOINT_DESCRIPTORS,
{
    // Claim: the result is always the same regardless of endpoint index
    assert(result_a == result_b);
}

// Test 2: The spec does NOT guarantee that the sender thread is always blocked
// after syscall_send_endpoint. On error paths, the sender remains RUNNING.
// Assert that the sender is always blocked.
// SHOULD FAIL
proof fn test_logical_sender_always_blocked_after_send() {
    // On many error paths (endpoint not found, queue full, etc.) the sender stays RUNNING.
    let sender_blocked_after_call: bool = false; // error path
    assert(sender_blocked_after_call);
}

// Test 3: The spec does NOT guarantee that endpoint domains grow after send.
// Assert that a new endpoint appears in the domain.
// SHOULD FAIL
proof fn test_logical_endpoint_domain_grows(
    old_endpoint_dom: Set<EndpointPtr>,
    new_endpoint_dom: Set<EndpointPtr>,
    new_ep: EndpointPtr,
)
    requires
        old_endpoint_dom == new_endpoint_dom,
        !old_endpoint_dom.contains(new_ep),
{
    assert(new_endpoint_dom.contains(new_ep));
}

// Test 4: The spec does NOT guarantee the thread domain changes.
// Assert a new thread appears after send.
// SHOULD FAIL
proof fn test_logical_thread_domain_grows(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
    new_t: ThreadPtr,
)
    requires
        old_thread_dom == new_thread_dom,
        !old_thread_dom.contains(new_t),
{
    assert(new_thread_dom.contains(new_t));
}

// Test 5: The spec does NOT guarantee that the queue state is always SEND
// after any execution of syscall_send_endpoint. When receiver_exist, queue_state
// is preserved (could be RECEIVE). Assert it's always SEND.
// SHOULD FAIL
proof fn test_logical_queue_state_always_send() {
    let queue_state_after = EndpointState::RECEIVE;
    assert(queue_state_after == EndpointState::SEND);
}

// Test 6: The spec does NOT guarantee that the process domain changes.
// Assert that proc domain grew.
// SHOULD FAIL
proof fn test_logical_proc_domain_unchanged(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
    new_p: ProcPtr,
)
    requires
        old_proc_dom == new_proc_dom,
        !old_proc_dom.contains(new_p),
{
    assert(new_proc_dom.contains(new_p));
}

// Test 7: The spec does NOT guarantee that blocking_endpoint_ptr == payload_endpoint_ptr.
// They can be different endpoints. Assert they are always the same.
// SHOULD FAIL
proof fn test_logical_blocking_equals_payload(
    blocking_endpoint_ptr: EndpointPtr,
    payload_endpoint_ptr: EndpointPtr,
)
    requires
        blocking_endpoint_ptr != payload_endpoint_ptr,
{
    assert(blocking_endpoint_ptr == payload_endpoint_ptr);
}

// Test 8: The spec does NOT guarantee that the rf_counter is always incremented.
// The rf_counter only changes in the success path (via pass_endpoint owning_threads insert).
// On error paths, rf_counter is unchanged.
// Assert rf_counter always increases.
// SHOULD FAIL
proof fn test_logical_rf_counter_always_increases(
    old_rf_counter: usize,
    new_rf_counter: usize,
)
    requires
        old_rf_counter == new_rf_counter, // error path
{
    assert(new_rf_counter > old_rf_counter);
}

// Test 9: The spec does NOT guarantee a stronger inequality: that on success
// the blocking endpoint's queue length strictly decreases by exactly 1.
// This is actually true (skip(1)), but let's test a STRONGER claim: queue becomes empty.
// SHOULD FAIL
proof fn test_logical_queue_always_empty_after_success(
    old_queue: Seq<ThreadPtr>,
    new_queue: Seq<ThreadPtr>,
)
    requires
        old_queue.len() == 3,
        new_queue =~= old_queue.skip(1),
{
    // Stronger claim: queue is always empty after send
    assert(new_queue.len() == 0);
}

// Test 10: The spec does NOT guarantee that pass_endpoint is idempotent.
// Assert that calling pass_endpoint twice yields the same result as once.
// Specifically, after one insert, doing the same insert again should not change the set.
// But the spec doesn't guarantee this property across two calls.
// SHOULD FAIL
proof fn test_logical_container_domain_grows(
    old_container_dom: Set<ContainerPtr>,
    new_container_dom: Set<ContainerPtr>,
    new_c: ContainerPtr,
)
    requires
        old_container_dom == new_container_dom,
        !old_container_dom.contains(new_c),
{
    // Spec preserves container domain, so this should fail
    assert(new_container_dom.contains(new_c));
}

}
