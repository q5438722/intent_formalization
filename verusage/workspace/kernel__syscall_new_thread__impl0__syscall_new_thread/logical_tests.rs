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
pub type EndpointPtr = usize;

// ===================== LOGICAL TESTS =====================
// Each test asserts a property NOT explicitly guaranteed by the
// syscall_new_thread specification, testing whether the spec
// allows unintended reasoning.
// All tests SHOULD FAIL verification.

// Test 1: The spec does NOT guarantee determinism of the new thread pointer.
// Two calls with the same preconditions could produce different pointers.
// SHOULD FAIL
proof fn test_logical_determinism(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_ptr_1: ThreadPtr,
    new_thread_ptr_2: ThreadPtr,
)
    requires
        !old_thread_dom.contains(new_thread_ptr_1),
        !old_thread_dom.contains(new_thread_ptr_2),
{
    assert(new_thread_ptr_1 == new_thread_ptr_2);
}

// Test 2: The new thread pointer is NOT guaranteed to be 0.
// SHOULD FAIL
proof fn test_logical_new_thread_is_zero(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_ptr: ThreadPtr,
)
    requires
        !old_thread_dom.contains(new_thread_ptr),
{
    assert(new_thread_ptr == 0usize);
}

// Test 3: The spec does not guarantee that thread_dom grows by exactly 2.
// Only one thread is created, so it grows by 1.
// SHOULD FAIL
proof fn test_logical_thread_dom_grows_by_two(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
    new_thread_ptr: ThreadPtr,
)
    requires
        old_thread_dom.finite(),
        !old_thread_dom.contains(new_thread_ptr),
        new_thread_dom =~= old_thread_dom.insert(new_thread_ptr),
{
    assert(new_thread_dom.len() == old_thread_dom.len() + 2);
}

// Test 4: The spec does NOT guarantee that all procs for other
// processes are affected. In fact, they are preserved.
// Asserting a proc_ptr changed for an unrelated process is unsound.
// SHOULD FAIL
proof fn test_logical_unrelated_proc_changed(
    proc_ptr: ProcPtr,
    other_proc: ProcPtr,
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
)
    requires
        old_proc_dom.contains(proc_ptr),
        old_proc_dom.contains(other_proc),
        proc_ptr != other_proc,
        new_proc_dom =~= old_proc_dom,
{
    // Try to claim other_proc was removed
    assert(!new_proc_dom.contains(other_proc));
}

// Test 5: The spec does NOT guarantee that the requirement function
// is always true. It depends on the kernel state.
// SHOULD FAIL
proof fn test_logical_requirement_always_true(
    thread_list_full: bool,
    mem_4k: usize,
) {
    // The requirement depends on multiple conditions.
    // We cannot prove it always holds without assumptions.
    assert(!thread_list_full && mem_4k >= 1);
}

// Test 6: The spec does NOT guarantee that on success,
// the return struct has switch_decision == Switch.
// NoSwitchNew guarantees switch_decision == NoSwitch.
// SHOULD FAIL
proof fn test_logical_success_causes_switch(
    is_no_switch: bool,
)
    requires
        is_no_switch,  // NoSwitchNew guarantees NoSwitch
{
    assert(!is_no_switch);
}

// Test 7: The spec does NOT imply that container_owned_pages
// change after syscall_new_thread. They are preserved.
// Asserting they changed is unentailed.
// SHOULD FAIL
proof fn test_logical_container_pages_changed(
    old_pages: Set<PagePtr>,
    new_pages: Set<PagePtr>,
    extra_page: PagePtr,
)
    requires
        new_pages =~= old_pages,
        !old_pages.contains(extra_page),
{
    assert(new_pages.contains(extra_page));
}

// Test 8: The spec does NOT guarantee that the error code
// on failure reveals which specific condition failed.
// Two different failure modes produce the same Error variant.
// We cannot distinguish between "thread list full" and "no quota".
// SHOULD FAIL
proof fn test_logical_distinguish_failure_modes(
    failure_reason_1: bool,
    failure_reason_2: bool,
)
    requires
        failure_reason_1 != failure_reason_2,
{
    // Both produce is_error() == true, but we can't distinguish them
    assert(failure_reason_1 == failure_reason_2);
}

// Test 9: The spec does NOT guarantee that quota fields other
// than mem_4k are unchanged unless spec_subtract_mem_4k states so.
// Actually it DOES guarantee this via spec_subtract_mem_4k.
// But we test a stronger claim: that mem_4k doesn't decrease at all.
// SHOULD FAIL
proof fn test_logical_mem_4k_never_decreases(
    old_mem_4k: usize,
    new_mem_4k: usize,
)
    requires
        old_mem_4k >= 1,
        old_mem_4k - 1 == new_mem_4k,
{
    assert(new_mem_4k >= old_mem_4k);
}

// Test 10: The spec does NOT guarantee that the free_pages list
// shrinks by exactly 2 after one thread creation. It shrinks by 1
// (one page allocated for the thread stack).
// SHOULD FAIL
proof fn test_logical_free_pages_shrink_by_two(
    old_free_len: usize,
    new_free_len: usize,
)
    requires
        old_free_len > 0,
        new_free_len == old_free_len - 1,
{
    assert(new_free_len == old_free_len - 2);
}

}
