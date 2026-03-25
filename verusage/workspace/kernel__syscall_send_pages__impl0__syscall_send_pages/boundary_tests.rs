use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ThreadPtr = usize;
pub type ContainerPtr = usize;
pub type EndpointPtr = usize;
pub type EndpointIdx = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize
    recommends
        page_ptr_valid(ptr),
{
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize
    recommends
        page_index_valid(i),
{
    (i * 4096) as usize
}

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

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Quota {
    pub mem_4k: usize,
    pub mem_2m: usize,
    pub mem_1g: usize,
    pub pcid: usize,
    pub ioid: usize,
}

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) of
// syscall_send_pages or related functions.
// All tests SHOULD FAIL verification.

// Test 1: syscall_send_pages requires 0 <= sender_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS.
// With sender_endpoint_payload == MAX_NUM_ENDPOINT_DESCRIPTORS (128), precondition violated.
// SHOULD FAIL
proof fn test_boundary_endpoint_payload_at_max() {
    let sender_endpoint_payload: usize = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(0 <= sender_endpoint_payload && sender_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 2: syscall_send_pages requires sender_thread_ptr is in thread_dom.
// If thread is not in the domain, precondition violated.
// SHOULD FAIL
proof fn test_boundary_thread_not_in_domain(
    thread_dom: Set<ThreadPtr>,
    sender_thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(sender_thread_ptr),
{
    assert(thread_dom.contains(sender_thread_ptr));
}

// Test 3: syscall_send_pages requires sender thread state == RUNNING.
// With state == BLOCKED, precondition violated.
// SHOULD FAIL
proof fn test_boundary_thread_not_running() {
    let state = ThreadState::BLOCKED;
    assert(state == ThreadState::RUNNING);
}

// Test 4: syscall_send_pages requires sender thread state == RUNNING.
// With state == SCHEDULED, precondition violated.
// SHOULD FAIL
proof fn test_boundary_thread_scheduled_not_running() {
    let state = ThreadState::SCHEDULED;
    assert(state == ThreadState::RUNNING);
}

// Test 5: page_ptr2page_index requires ptr % 0x1000 == 0.
// ptr = 1 is not page-aligned.
// SHOULD FAIL
proof fn test_boundary_page_ptr_not_aligned() {
    let ptr: usize = 1;
    assert(ptr % 0x1000 == 0);
}

// Test 6: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// With i == NUM_PAGES, precondition violated.
// SHOULD FAIL
proof fn test_boundary_page_index_at_max() {
    let i: usize = NUM_PAGES;
    assert(0 <= i && i < NUM_PAGES);
}

// Test 7: sender_endpoint_payload large overflow value.
// With sender_endpoint_payload == usize::MAX, precondition violated.
// SHOULD FAIL
proof fn test_boundary_endpoint_payload_overflow() {
    let sender_endpoint_payload: usize = usize::MAX;
    assert(sender_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 8: VA range well-formedness requires start + len * 4096 < usize::MAX.
// With very large len, this overflows. Using a concrete large value.
// SHOULD FAIL
proof fn test_boundary_va_range_overflow(len: usize)
    requires
        len > usize::MAX / 4096,
{
    let start: usize = 0;
    assert(start + len * 4096 < usize::MAX);
}

// Test 9: VA = 0 is in kernel space and should not be va_4k_valid.
// SHOULD FAIL
proof fn test_boundary_va_zero_not_valid() {
    assert(spec_va_4k_valid(0));
}

// Test 10: block_running_thread requires queue.len() < MAX_NUM_THREADS_PER_ENDPOINT.
// With queue at max, precondition violated.
// SHOULD FAIL
proof fn test_boundary_endpoint_queue_full() {
    let queue_len: usize = MAX_NUM_THREADS_PER_ENDPOINT;
    assert(queue_len < MAX_NUM_THREADS_PER_ENDPOINT);
}

// Test 11: schedule_blocked_thread requires scheduler.len() < MAX_CONTAINER_SCHEDULER_LEN.
// With scheduler at max, precondition violated.
// SHOULD FAIL
proof fn test_boundary_scheduler_full() {
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN;
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN);
}

// Test 12: range_create_and_share_mapping requires src_proc_ptr != target_proc_ptr.
// With same proc, precondition violated.
// SHOULD FAIL
proof fn test_boundary_same_sender_receiver_proc(
    proc_ptr: ProcPtr,
)
{
    let src = proc_ptr;
    let target = proc_ptr;
    assert(src != target);
}

}
