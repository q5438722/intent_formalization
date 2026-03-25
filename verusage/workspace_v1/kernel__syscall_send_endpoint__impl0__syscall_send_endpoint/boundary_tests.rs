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

// ============================================================
// BOUNDARY TEST 1: sender_thread_ptr NOT in thread_dom
// Precondition: old(self).thread_dom().contains(sender_thread_ptr)
// A thread pointer outside the domain violates this.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_sender_not_in_thread_dom()
{
    let thread_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2).insert(3);
    let sender_thread_ptr: ThreadPtr = 999;
    assert(thread_dom.contains(sender_thread_ptr)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 2: blocking_endpoint_index at upper bound (== MAX)
// Precondition: 0 <= blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS
// Using exactly 128 is out of range.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_blocking_endpoint_index_at_max()
{
    let blocking_endpoint_index: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS; // == 128
    assert(blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 3: sender_endpoint_payload at upper bound (== MAX)
// Precondition: 0 <= sender_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS
// Using exactly 128 is out of range.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_sender_endpoint_payload_at_max()
{
    let sender_endpoint_payload: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(sender_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 4: sender_endpoint_payload overflow (usize::MAX)
// Precondition: 0 <= sender_endpoint_payload < 128
// A very large value must be rejected.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_sender_endpoint_payload_overflow()
{
    let sender_endpoint_payload: EndpointIdx = usize::MAX;
    assert(sender_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 5: receiver_endpoint_payload >= MAX_NUM_ENDPOINT_DESCRIPTORS
// The spec checks: if receiver_endpoint_payload >= MAX_NUM_ENDPOINT_DESCRIPTORS
// then old =~= new. Asserting the opposite should fail.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_receiver_endpoint_payload_at_max()
{
    let receiver_endpoint_payload: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(receiver_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 6: Queue length at MAX_NUM_THREADS_PER_ENDPOINT
// The spec checks sender_queue_full: queue.len() >= MAX_NUM_THREADS_PER_ENDPOINT.
// An endpoint queue at exactly 128 triggers the full condition.
// Asserting it is NOT full should fail.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_queue_at_max_is_not_full()
{
    let queue_len: usize = MAX_NUM_THREADS_PER_ENDPOINT; // == 128
    assert(queue_len < MAX_NUM_THREADS_PER_ENDPOINT); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 7: Scheduler length at MAX_CONTAINER_SCHEDULER_LEN
// The spec checks: if scheduler.len() >= MAX_CONTAINER_SCHEDULER_LEN
// then old =~= new (error). Asserting not full should fail.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_scheduler_at_max_is_not_full()
{
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN; // == 10
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 8: rf_counter at usize::MAX
// The spec checks: if rf_counter == usize::MAX then old =~= new.
// Asserting rf_counter != usize::MAX when it IS usize::MAX should fail.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_rf_counter_at_max()
{
    let rf_counter: usize = usize::MAX;
    assert(rf_counter != usize::MAX); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 9: Empty thread domain
// Precondition: old(self).thread_dom().contains(sender_thread_ptr)
// An empty domain contains no thread pointer.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_empty_thread_domain()
{
    let thread_dom: Set<ThreadPtr> = Set::empty();
    let sender_thread_ptr: ThreadPtr = 1;
    assert(thread_dom.contains(sender_thread_ptr)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 10: blocking_endpoint_index negative-like (usize::MAX)
// Precondition: 0 <= blocking_endpoint_index < 128
// usize::MAX wraps as a very large value.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_blocking_endpoint_index_overflow()
{
    let blocking_endpoint_index: EndpointIdx = usize::MAX;
    assert(blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

} // verus!
