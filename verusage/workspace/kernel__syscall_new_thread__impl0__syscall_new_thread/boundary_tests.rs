use vstd::prelude::*;

fn main() {}

verus!{

pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition or uses edge-case values
// for syscall_new_thread to check if invalid inputs are
// properly rejected by the specification.
// All tests SHOULD FAIL verification.

// Test 1: thread_ptr must be in thread_dom. If it is not,
// the precondition of syscall_new_thread is violated.
// SHOULD FAIL
proof fn test_boundary_thread_not_in_domain(
    thread_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(thread_ptr),
{
    assert(thread_dom.contains(thread_ptr));
}

// Test 2: The process's owned_threads list must not be full
// (len < MAX_NUM_THREADS_PER_PROC). Testing at exact max boundary.
// SHOULD FAIL
proof fn test_boundary_thread_list_at_max() {
    let thread_count: usize = MAX_NUM_THREADS_PER_PROC;
    assert(thread_count < MAX_NUM_THREADS_PER_PROC);
}

// Test 3: Container quota mem_4k must be >= 1 for the requirement
// to hold. With mem_4k == 0, the requirement returns false.
// SHOULD FAIL
proof fn test_boundary_zero_mem_quota() {
    let mem_4k: usize = 0;
    assert(mem_4k >= 1);
}

// Test 4: Scheduler must not be full. At exactly MAX_CONTAINER_SCHEDULER_LEN,
// the requirement returns false.
// SHOULD FAIL
proof fn test_boundary_scheduler_at_max() {
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN;
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN);
}

// Test 5: Free pages must be > 0 for allocation. Zero free pages
// violates the alloc_page_4k precondition.
// SHOULD FAIL
proof fn test_boundary_zero_free_pages() {
    let free_pages: usize = 0;
    assert(free_pages > 0);
}

// Test 6: page_ptr2page_index requires ptr % 0x1000 == 0.
// An unaligned pointer (e.g., 1) violates this.
// SHOULD FAIL
proof fn test_boundary_unaligned_page_ptr() {
    let ptr: usize = 1;
    assert(ptr % 0x1000 == 0);
}

// Test 7: page_index2page_ptr requires 0 <= i < NUM_PAGES.
// An index at NUM_PAGES violates this upper bound.
// SHOULD FAIL
proof fn test_boundary_page_index_at_num_pages() {
    let i: usize = NUM_PAGES;
    assert(i < NUM_PAGES);
}

// Test 8: page_index2page_ptr with usize::MAX is way beyond range.
// SHOULD FAIL
proof fn test_boundary_page_index_overflow() {
    let i: usize = usize::MAX;
    assert(i < NUM_PAGES);
}

// Test 9: The combined requirement check: all four conditions must hold
// simultaneously. Here thread list is full AND mem_4k is zero —
// doubly invalid. Cannot satisfy requirement.
// SHOULD FAIL
proof fn test_boundary_multiple_violations() {
    let thread_count: usize = MAX_NUM_THREADS_PER_PROC;
    let mem_4k: usize = 0;
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN;
    let free_pages: usize = 0;
    // Try to claim this state satisfies the requirement
    assert(
        thread_count < MAX_NUM_THREADS_PER_PROC
        && mem_4k >= 1
        && scheduler_len < MAX_CONTAINER_SCHEDULER_LEN
        && free_pages > 0
    );
}

// Test 10: alloc_page_4k requires old(self).wf() AND
// old(self).free_pages_4k.len() > 0. Attempting with len == 0.
// SHOULD FAIL
proof fn test_boundary_alloc_with_empty_free_list() {
    let free_list_len: usize = 0;
    assert(free_list_len > 0);
}

}
