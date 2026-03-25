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
pub type PageMapPtr = usize;
pub type Pcid = usize;
pub type PAddr = usize;
pub type VAddr = usize;
pub type SLLIndex = i32;

pub const NUM_CPUS: usize = 32;
pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const CONTAINER_PROC_LIST_LEN: usize = 10;
pub const CONTAINER_CHILD_LIST_LEN: usize = 10;
pub const PROC_CHILD_LIST_LEN: usize = 10;
pub const CONTAINER_ENDPOINT_LIST_LEN: usize = 10;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const PAGE_SZ_4k: usize = 1usize << 12;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const PCID_MAX: usize = 4096;
pub const IOID_MAX: usize = 4096;

// ============================================================
// BOUNDARY TEST 1: receiver_thread_ptr NOT in thread_dom
// Precondition: old(self).thread_dom().contains(receiver_thread_ptr)
// Using a thread_ptr outside the domain violates this.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_receiver_not_in_thread_dom()
{
    let thread_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2).insert(3);
    let receiver_thread_ptr: ThreadPtr = 999; // not in domain
    assert(thread_dom.contains(receiver_thread_ptr)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 2: blocking_endpoint_index at upper bound (== MAX)
// Precondition: 0 <= blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS
// Using exactly MAX_NUM_ENDPOINT_DESCRIPTORS (128) is out of range.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_endpoint_index_at_max()
{
    let blocking_endpoint_index: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS; // == 128
    assert(0 <= blocking_endpoint_index && blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 3: blocking_endpoint_index overflow (very large)
// Precondition: 0 <= blocking_endpoint_index < 128
// A very large value should be rejected.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_endpoint_index_overflow()
{
    let blocking_endpoint_index: EndpointIdx = usize::MAX;
    assert(blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 4: receiver_endpoint_payload at upper bound (== MAX)
// Precondition: 0 <= receiver_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS
// Using exactly 128 violates the strict upper bound.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_payload_index_at_max()
{
    let receiver_endpoint_payload: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS; // == 128
    assert(receiver_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 5: thread state is not RUNNING
// Precondition: old(self).get_thread(receiver_thread_ptr).state == ThreadState::RUNNING
// If the thread is BLOCKED or SCHEDULED, the precondition is violated.
// SHOULD FAIL
// ============================================================

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThreadState {
    SCHEDULED,
    BLOCKED,
    RUNNING,
}

proof fn test_boundary_thread_not_running_blocked()
{
    let state = ThreadState::BLOCKED;
    assert(state == ThreadState::RUNNING); // SHOULD FAIL
}

proof fn test_boundary_thread_not_running_scheduled()
{
    let state = ThreadState::SCHEDULED;
    assert(state == ThreadState::RUNNING); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 6: empty thread domain
// If thread_dom is empty, no thread_ptr satisfies the precondition.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_empty_thread_dom()
{
    let thread_dom: Set<ThreadPtr> = Set::empty();
    let receiver_thread_ptr: ThreadPtr = 0;
    assert(thread_dom.contains(receiver_thread_ptr)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 7: both endpoint indices at boundary
// Both blocking_endpoint_index and receiver_endpoint_payload at 128.
// Both violate preconditions simultaneously.
// SHOULD FAIL
// ============================================================
proof fn test_boundary_both_indices_at_max()
{
    let blocking_endpoint_index: EndpointIdx = 128;
    let receiver_endpoint_payload: EndpointIdx = 128;
    assert(
        blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS
        && receiver_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS
    ); // SHOULD FAIL
}

}
