use vstd::prelude::*;

fn main() {}

verus!{

pub type CpuId = usize;
pub type ThreadPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type ProcPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;

pub const NUM_CPUS: usize = 32;
pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThreadState {
    SCHEDULED,
    BLOCKED,
    RUNNING,
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize {
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize {
    (i * 4096) as usize
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) of
// syscall_send_empty_try_schedule or its helper functions.
// All tests SHOULD FAIL verification.

// Test 1: syscall_send_empty_try_schedule requires 0 <= cpu_id < NUM_CPUS.
// cpu_id == NUM_CPUS (32) is out of range.
// SHOULD FAIL
proof fn test_boundary_cpu_id_at_max() {
    let cpu_id: CpuId = NUM_CPUS;
    assert(0 <= cpu_id && cpu_id < NUM_CPUS);
}

// Test 2: syscall_send_empty_try_schedule requires 0 <= cpu_id < NUM_CPUS.
// cpu_id == usize::MAX is far out of range.
// SHOULD FAIL
proof fn test_boundary_cpu_id_overflow() {
    let cpu_id: CpuId = usize::MAX;
    assert(cpu_id < NUM_CPUS);
}

// Test 3: syscall_send_empty_try_schedule requires
// 0 <= blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS (128).
// blocking_endpoint_index == 128 violates this.
// SHOULD FAIL
proof fn test_boundary_endpoint_index_at_max() {
    let idx: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(0 <= idx && idx < MAX_NUM_ENDPOINT_DESCRIPTORS);
}

// Test 4: syscall_send_empty_try_schedule requires sender thread state == RUNNING.
// If state is BLOCKED, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_sender_not_running() {
    let state = ThreadState::BLOCKED;
    assert(state == ThreadState::RUNNING);
}

// Test 5: syscall_send_empty_try_schedule requires sender thread state == RUNNING.
// If state is SCHEDULED, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_sender_scheduled_not_running() {
    let state = ThreadState::SCHEDULED;
    assert(state == ThreadState::RUNNING);
}

// Test 6: syscall_send_empty_try_schedule requires
// get_cpu(cpu_id).current_thread.is_some().
// If current_thread is None, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_cpu_no_current_thread(current_thread: Option<ThreadPtr>)
    requires
        current_thread.is_None(),
{
    assert(current_thread.is_Some());
}

// Test 7: syscall_send_empty_try_schedule requires get_cpu(cpu_id).active.
// If CPU is not active, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_cpu_not_active(active: bool)
    requires
        active == false,
{
    assert(active);
}

// Test 8: syscall_send_empty_try_schedule requires
// current_thread.unwrap() == sender_thread_ptr.
// If they differ, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_sender_not_current_thread(
    current_thread: ThreadPtr,
    sender_thread_ptr: ThreadPtr,
)
    requires
        current_thread != sender_thread_ptr,
{
    assert(current_thread == sender_thread_ptr);
}

// Test 9: syscall_send_empty_try_schedule requires
// thread_dom().contains(sender_thread_ptr).
// If not in domain, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_sender_not_in_thread_dom(
    thread_dom: Set<ThreadPtr>,
    sender_thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(sender_thread_ptr),
{
    assert(thread_dom.contains(sender_thread_ptr));
}

// Test 10: syscall_send_empty_try_schedule requires
// cpu owning_container == sender owning_container.
// If containers differ, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_container_mismatch(
    cpu_container: ContainerPtr,
    sender_container: ContainerPtr,
)
    requires
        cpu_container != sender_container,
{
    assert(cpu_container == sender_container);
}

// Test 11: page_ptr2page_index requires ptr % 0x1000 == 0.
// ptr = 1 is not aligned.
// SHOULD FAIL
proof fn test_boundary_page_ptr_not_aligned() {
    let ptr: usize = 1;
    assert(ptr % 0x1000 == 0);
}

// Test 12: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// i = NUM_PAGES is out of range.
// SHOULD FAIL
proof fn test_boundary_page_index_at_max() {
    let i: usize = NUM_PAGES;
    assert(0 <= i && i < NUM_PAGES);
}

// Test 13: schedule_running_thread requires scheduler.len() < MAX_CONTAINER_SCHEDULER_LEN.
// If scheduler is full, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_scheduler_full(scheduler_len: usize)
    requires
        scheduler_len >= MAX_CONTAINER_SCHEDULER_LEN,
{
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN);
}

// Test 14: run_blocked_thread requires endpoint queue.len() > 0.
// If queue is empty, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_empty_endpoint_queue(queue_len: usize)
    requires
        queue_len == 0,
{
    assert(queue_len > 0);
}

// Test 15: run_blocked_thread requires cpu current_thread.is_none().
// If current_thread is some, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_run_blocked_cpu_has_thread(current_thread: Option<ThreadPtr>)
    requires
        current_thread.is_Some(),
{
    assert(current_thread.is_None());
}

}
