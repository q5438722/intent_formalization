use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;

pub const MAX_NUM_THREADS_PER_PROC: usize = 128;

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition of kernel_proc_kill_all_threads
// or a related executable function.
// All tests SHOULD FAIL verification.

// --- kernel_proc_kill_all_threads boundary tests ---

// Test 1: kernel_proc_kill_all_threads requires proc_ptr in proc_dom.
// If proc_ptr is NOT in the domain, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_proc_not_in_dom(
    proc_dom: Set<ProcPtr>,
    proc_ptr: ProcPtr,
)
    requires
        !proc_dom.contains(proc_ptr),
{
    // Attempt to assert the precondition holds — it should not.
    assert(proc_dom.contains(proc_ptr));
}

// Test 2: kernel_proc_kill_all_threads requires self.wf().
// If wf is false, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_no_wf(wf: bool)
    requires
        wf == false,
{
    assert(wf);
}

// Test 3: proc_ptr == 0 with an empty proc_dom.
// Edge case: zero pointer in empty domain.
// SHOULD FAIL
proof fn test_boundary_proc_ptr_zero_empty_dom() {
    let proc_dom: Set<ProcPtr> = Set::empty();
    let proc_ptr: ProcPtr = 0;
    assert(proc_dom.contains(proc_ptr));
}

// Test 4: proc_ptr == usize::MAX.
// Edge case: maximum possible pointer value not in domain.
// SHOULD FAIL
proof fn test_boundary_proc_ptr_max(
    proc_dom: Set<ProcPtr>,
)
    requires
        !proc_dom.contains(usize::MAX),
{
    assert(proc_dom.contains(usize::MAX));
}

// Test 5: The loop inside kernel_proc_kill_all_threads calls
// get_head() which requires len() > 0. If len == 0, the precondition
// of get_head is violated. Test that we cannot assert len > 0 when len == 0.
// SHOULD FAIL
proof fn test_boundary_get_head_on_empty_list(len: usize)
    requires
        len == 0,
{
    assert(len > 0);
}

// Test 6: The loop calls kernel_kill_thread which requires
// thread_ptr in thread_dom. If a thread is not in the domain,
// the precondition fails.
// SHOULD FAIL
proof fn test_boundary_kill_thread_not_in_dom(
    thread_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        !thread_dom.contains(thread_ptr),
{
    assert(thread_dom.contains(thread_ptr));
}

// Test 7: kernel_kill_thread requires self.wf() AND thread in domain.
// Both preconditions must hold simultaneously. Test that one without
// the other is insufficient.
// SHOULD FAIL
proof fn test_boundary_wf_but_thread_not_in_dom(
    wf: bool,
    thread_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        wf == true,
        !thread_dom.contains(thread_ptr),
{
    // wf is true but thread not in domain — both needed
    assert(wf && thread_dom.contains(thread_ptr));
}

// Test 8: The loop iterates num_threads times. If num_threads
// overflows or is negative (impossible for usize, but test
// the invariant that owned_threads.len() matches loop bound).
// Test that we cannot assert a mismatched loop bound.
// SHOULD FAIL
proof fn test_boundary_loop_count_mismatch(
    num_threads: usize,
    actual_len: usize,
)
    requires
        num_threads == 5,
        actual_len == 3,
{
    // The loop requires num_threads == owned_threads.len()
    assert(num_threads == actual_len);
}

// Test 9: ProcessManager.get_proc requires proc_perms_wf AND
// process_fields_wf AND proc_dom contains proc_ptr.
// Missing process_fields_wf should fail.
// SHOULD FAIL
proof fn test_boundary_get_proc_missing_fields_wf(
    proc_perms_wf: bool,
    process_fields_wf: bool,
    in_dom: bool,
)
    requires
        proc_perms_wf == true,
        process_fields_wf == false,
        in_dom == true,
{
    // All three must be true
    assert(proc_perms_wf && process_fields_wf && in_dom);
}

// Test 10: The loop invariant requires self.proc_dom().contains(proc_ptr)
// at each iteration. If proc is removed mid-loop, this breaks.
// SHOULD FAIL
proof fn test_boundary_proc_removed_during_loop(
    proc_dom_before: Set<ProcPtr>,
    proc_dom_after: Set<ProcPtr>,
    proc_ptr: ProcPtr,
)
    requires
        proc_dom_before.contains(proc_ptr),
        proc_dom_after =~= proc_dom_before.remove(proc_ptr),
{
    // The loop invariant requires proc_ptr stays in domain
    assert(proc_dom_after.contains(proc_ptr));
}

}
