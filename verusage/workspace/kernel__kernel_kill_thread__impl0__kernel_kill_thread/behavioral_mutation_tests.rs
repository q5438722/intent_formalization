use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type EndpointIdx = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs and mutates expected outputs or relations.
// All tests SHOULD FAIL verification.

// --- kernel_kill_thread postcondition mutations ---

// Test 1: kernel_kill_thread ensures thread_dom is reduced by exactly thread_ptr.
// Mutant: the thread is NOT removed (thread_dom stays the same).
// SHOULD FAIL
proof fn test_mutation_thread_not_removed(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        old_thread_dom.contains(thread_ptr),
        new_thread_dom =~= old_thread_dom.remove(thread_ptr),
{
    // Mutated assertion: new domain equals old domain (thread NOT removed)
    assert(new_thread_dom =~= old_thread_dom);
}

// Test 2: kernel_kill_thread ensures proc_dom is unchanged.
// Mutant: proc_dom changes (a process is removed).
// SHOULD FAIL
proof fn test_mutation_proc_dom_changed(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
    some_proc: ProcPtr,
)
    requires
        old_proc_dom.contains(some_proc),
        new_proc_dom =~= old_proc_dom,
{
    // Mutated: assert proc_dom lost a member
    assert(new_proc_dom =~= old_proc_dom.remove(some_proc));
}

// Test 3: kernel_kill_thread ensures container_dom is unchanged.
// Mutant: container_dom changes.
// SHOULD FAIL
proof fn test_mutation_container_dom_changed(
    old_container_dom: Set<ContainerPtr>,
    new_container_dom: Set<ContainerPtr>,
    some_container: ContainerPtr,
)
    requires
        old_container_dom.contains(some_container),
        new_container_dom =~= old_container_dom,
{
    // Mutated: assert a container was removed
    assert(new_container_dom =~= old_container_dom.remove(some_container));
}

// Test 4: kernel_kill_thread ensures owned_threads length decreases by 1.
// Mutant: length stays the same.
// SHOULD FAIL
proof fn test_mutation_owned_threads_len_unchanged(
    old_len: int,
    new_len: int,
)
    requires
        old_len > 0,
        new_len == old_len - 1,
{
    // Mutated: assert length is unchanged
    assert(new_len == old_len);
}

// Test 5: kernel_kill_thread ensures owned_threads@ == old.remove_value(thread_ptr).
// Mutant: the wrong thread is removed from the list.
// SHOULD FAIL
proof fn test_mutation_wrong_thread_removed(
    old_threads: Seq<ThreadPtr>,
    new_threads: Seq<ThreadPtr>,
    thread_ptr: ThreadPtr,
    wrong_ptr: ThreadPtr,
)
    requires
        thread_ptr != wrong_ptr,
        old_threads.contains(thread_ptr),
        old_threads.contains(wrong_ptr),
        new_threads =~= old_threads.remove_value(thread_ptr),
{
    // Mutated: assert the wrong thread was removed instead
    assert(new_threads =~= old_threads.remove_value(wrong_ptr));
}

// --- kernel_drop_endpoint postcondition mutations ---

// Test 6: kernel_drop_endpoint ensures endpoint_descriptors is updated
// at edp_idx to None. Mutant: a different index is updated.
// SHOULD FAIL
proof fn test_mutation_drop_wrong_endpoint_index(
    old_descriptors: Seq<Option<usize>>,
    new_descriptors: Seq<Option<usize>>,
    edp_idx: int,
    wrong_idx: int,
)
    requires
        0 <= edp_idx < 128,
        0 <= wrong_idx < 128,
        edp_idx != wrong_idx,
        old_descriptors.len() == 128,
        new_descriptors =~= old_descriptors.update(edp_idx, None),
{
    // Mutated: assert the wrong index was updated
    assert(new_descriptors =~= old_descriptors.update(wrong_idx, None));
}

// Test 7: kernel_drop_endpoint ensures thread state is unchanged.
// Mutant: the thread state changes.
// SHOULD FAIL
proof fn test_mutation_drop_endpoint_changes_state(
    old_state: int,
    new_state: int,
)
    requires
        old_state == new_state,
{
    // Mutated: assert state changed
    assert(new_state != old_state);
}

// --- kill_scheduled_thread postcondition mutations ---

// Test 8: kill_scheduled_thread ensures page_closure shrinks by ret.0.
// Mutant: page_closure stays the same (page not freed).
// SHOULD FAIL
proof fn test_mutation_page_not_freed(
    old_page_closure: Set<PagePtr>,
    new_page_closure: Set<PagePtr>,
    freed_page: PagePtr,
)
    requires
        old_page_closure.contains(freed_page),
        new_page_closure =~= old_page_closure.remove(freed_page),
{
    // Mutated: assert page_closure unchanged
    assert(new_page_closure =~= old_page_closure);
}

// Test 9: threads_unchanged_except ensures unchanged threads are identical.
// Mutant: an unchanged thread has been modified.
// SHOULD FAIL
proof fn test_mutation_unchanged_thread_modified(
    old_val: int,
    new_val: int,
    in_changed_set: bool,
)
    requires
        in_changed_set == false,
        old_val == new_val,
{
    // Mutated: assert the thread was changed even though it shouldn't be
    assert(old_val != new_val);
}

// Test 10: kernel_kill_thread passes set![] as the changed set to
// threads_unchanged_except, meaning NO thread (besides the removed one)
// should change. Mutant: a surviving thread was changed.
// SHOULD FAIL
proof fn test_mutation_surviving_thread_changed(
    old_thread_val: int,
    new_thread_val: int,
    thread_in_new_dom: bool,
)
    requires
        thread_in_new_dom == true,
        old_thread_val == new_thread_val,
{
    // Mutated: assert the surviving thread's value changed
    assert(new_thread_val == old_thread_val + 1);
}

}
