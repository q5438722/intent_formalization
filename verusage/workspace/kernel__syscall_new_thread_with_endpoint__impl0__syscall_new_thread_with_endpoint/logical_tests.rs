use vstd::prelude::*;

fn main() {}

verus!{

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;

// ===================== LOGICAL TESTS =====================
// Each test asserts a property NOT explicitly guaranteed by the
// syscall_new_thread_with_endpoint specification, testing whether
// the spec allows unintended reasoning.
// All tests SHOULD FAIL verification.

// Test 1: The spec does NOT guarantee determinism — the new thread pointer
// could be any value. Asserting two calls produce the same new_thread_ptr
// is not entailed.
// SHOULD FAIL
proof fn test_logical_determinism(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_ptr_1: ThreadPtr,
    new_thread_ptr_2: ThreadPtr,
    new_dom_1: Set<ThreadPtr>,
    new_dom_2: Set<ThreadPtr>,
)
    requires
        new_dom_1 =~= old_thread_dom.insert(new_thread_ptr_1),
        new_dom_2 =~= old_thread_dom.insert(new_thread_ptr_2),
{
    assert(new_thread_ptr_1 == new_thread_ptr_2);
}

// Test 2: The new thread pointer is NOT guaranteed to be 0.
// The spec allows any thread pointer not already in the domain.
// SHOULD FAIL
proof fn test_logical_new_thread_ptr_is_zero(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_ptr: ThreadPtr,
)
    requires
        !old_thread_dom.contains(new_thread_ptr),
{
    assert(new_thread_ptr == 0usize);
}

// Test 3: Inserting one thread does NOT increase thread_dom size by 2.
// Only one thread is created per syscall.
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

// Test 4: The spec does not guarantee that the new thread's owning_container
// equals the process's owning_container. It guarantees it equals the
// CALLING thread's owning_container. These could differ if threads and
// processes have different container ownership. Assert they must be a
// specific value (0).
// SHOULD FAIL
proof fn test_logical_owning_container_is_zero(
    new_thread_owning_container: ContainerPtr,
    target_container_ptr: ContainerPtr,
)
    requires
        new_thread_owning_container == target_container_ptr,
{
    assert(new_thread_owning_container == 0usize);
}

// Test 5: Quota mem_4k subtraction by 1 does NOT imply mem_4k becomes 0.
// The old value could be much larger.
// SHOULD FAIL
proof fn test_logical_quota_becomes_zero(
    old_mem_4k: usize,
    new_mem_4k: usize,
)
    requires
        old_mem_4k > 0,
        old_mem_4k - 1 == new_mem_4k,
{
    assert(new_mem_4k == 0usize);
}

// Test 6: The endpoint_descriptors of the new thread have Some at index 0
// but this does NOT mean ALL slots are Some.
// SHOULD FAIL
proof fn test_logical_all_endpoint_slots_filled(
    endpoint_descriptors: Seq<Option<EndpointPtr>>,
    target_endpoint_ptr: EndpointPtr,
)
    requires
        endpoint_descriptors =~= Seq::new(
            MAX_NUM_ENDPOINT_DESCRIPTORS as nat,
            |i: int| { None::<EndpointPtr> },
        ).update(0int, Some(target_endpoint_ptr)),
        MAX_NUM_ENDPOINT_DESCRIPTORS > 1,
{
    assert(endpoint_descriptors[1int].is_Some());
}

// Test 7: The spec says other containers' owned_threads should NOT
// contain the new thread. This does NOT mean non-target containers
// have empty owned_threads.
// SHOULD FAIL
proof fn test_logical_other_containers_have_no_threads(
    owned_threads: Set<ThreadPtr>,
    new_thread_ptr: ThreadPtr,
)
    requires
        !owned_threads.contains(new_thread_ptr),
{
    assert(owned_threads =~= Set::empty());
}

// Test 8: The spec preserves address space mappings for all procs.
// This does NOT mean all procs have empty address spaces.
// SHOULD FAIL
proof fn test_logical_address_space_empty(
    old_mapping: Map<VAddr, usize>,
    new_mapping: Map<VAddr, usize>,
)
    requires
        new_mapping =~= old_mapping,
{
    assert(new_mapping =~= Map::empty());
}

// Test 9: Pushing to owned_threads does NOT reorder existing elements.
// old.push(x) puts x at the end. Assert x is at index 0 instead.
// SHOULD FAIL
proof fn test_logical_new_thread_at_wrong_index(
    old_threads: Seq<ThreadPtr>,
    new_threads: Seq<ThreadPtr>,
    new_thread_ptr: ThreadPtr,
)
    requires
        old_threads.len() > 0,
        new_threads =~= old_threads.push(new_thread_ptr),
{
    assert(new_threads[0int] == new_thread_ptr);
}

// Test 10: The endpoint's queue_state is preserved, so it does NOT
// flip from RECEIVE to SEND (or vice versa). Assert a state change.
// We model queue_state as a bool for simplicity.
// SHOULD FAIL
proof fn test_logical_queue_state_flipped(
    old_state: bool,
    new_state: bool,
)
    requires
        new_state =~= old_state,
{
    assert(new_state != old_state);
}

}
