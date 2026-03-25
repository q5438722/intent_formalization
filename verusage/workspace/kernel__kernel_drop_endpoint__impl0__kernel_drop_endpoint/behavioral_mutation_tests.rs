use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type ProcPtr = usize;
pub type PagePtr = usize;

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs and postconditions, then asserts
// an INCORRECT behavior. All tests SHOULD FAIL verification.

// Test 1: kernel_drop_endpoint ensures endpoint_descriptors are updated
// at edp_idx to None. Mutate: assert the descriptor is NOT None after drop.
// SHOULD FAIL
proof fn test_behavioral_descriptor_not_cleared(
    old_descriptors: Seq<Option<EndpointPtr>>,
    new_descriptors: Seq<Option<EndpointPtr>>,
    edp_idx: int,
)
    requires
        0 <= edp_idx < MAX_NUM_ENDPOINT_DESCRIPTORS as int,
        old_descriptors.len() == MAX_NUM_ENDPOINT_DESCRIPTORS as int,
        new_descriptors =~= old_descriptors.update(edp_idx, None),
{
    // The spec says new_descriptors[edp_idx] == None.
    // Mutate: claim it's still Some.
    assert(new_descriptors[edp_idx].is_Some());
}

// Test 2: kernel_drop_endpoint ensures thread state is preserved.
// Mutate: assert the state changed from SCHEDULED to BLOCKED.
// SHOULD FAIL
proof fn test_behavioral_thread_state_changed(
    old_state: u8,
    new_state: u8,
)
    requires
        old_state == new_state,
{
    // The spec says state is preserved. Mutate: claim it changed.
    assert(old_state != new_state);
}

// Test 3: kernel_drop_endpoint ensures threads_unchanged_except for thread_ptr.
// Mutate: claim that a different thread (other_thread) also changed.
// SHOULD FAIL
proof fn test_behavioral_other_thread_changed(
    thread_ptr: ThreadPtr,
    other_thread: ThreadPtr,
    changed_set: Set<ThreadPtr>,
)
    requires
        changed_set =~= set![thread_ptr],
        other_thread != thread_ptr,
{
    // The spec says only thread_ptr is changed. Mutate: claim other_thread
    // is also in the changed set.
    assert(changed_set.contains(other_thread));
}

// Test 4: kernel_drop_endpoint ensures proc_dom unchanged.
// Mutate: claim proc_dom changed (a process was removed).
// SHOULD FAIL
proof fn test_behavioral_proc_dom_shrank(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
    some_proc: ProcPtr,
)
    requires
        old_proc_dom == new_proc_dom,
        old_proc_dom.contains(some_proc),
{
    // The spec preserves proc_dom. Mutate: claim some_proc is gone.
    assert(!new_proc_dom.contains(some_proc));
}

// Test 5: kernel_drop_endpoint ensures container_dom unchanged.
// Mutate: claim container_dom shrank.
// SHOULD FAIL
proof fn test_behavioral_container_dom_changed(
    old_container_dom: Set<ContainerPtr>,
    new_container_dom: Set<ContainerPtr>,
    some_container: ContainerPtr,
)
    requires
        old_container_dom == new_container_dom,
        old_container_dom.contains(some_container),
{
    assert(!new_container_dom.contains(some_container));
}

// Test 6: kernel_drop_endpoint ensures blocking_endpoint_index is preserved.
// Mutate: claim blocking_endpoint_index changed.
// SHOULD FAIL
proof fn test_behavioral_blocking_index_changed(
    old_blocking: Option<EndpointIdx>,
    new_blocking: Option<EndpointIdx>,
)
    requires
        new_blocking == old_blocking,
{
    assert(new_blocking != old_blocking);
}

// Test 7: kernel_drop_endpoint ensures owning_proc is preserved.
// Mutate: claim owning_proc changed to a different value.
// SHOULD FAIL
proof fn test_behavioral_owning_proc_changed(
    old_owning_proc: ProcPtr,
    new_owning_proc: ProcPtr,
)
    requires
        old_owning_proc == new_owning_proc,
{
    assert(old_owning_proc != new_owning_proc);
}

// Test 8: kernel_drop_endpoint ensures processes_unchanged.
// Mutate: claim a process's owning_container changed.
// SHOULD FAIL
proof fn test_behavioral_process_container_changed(
    old_container: ContainerPtr,
    new_container: ContainerPtr,
)
    requires
        old_container == new_container,
{
    assert(old_container != new_container);
}

}
