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

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs (matching the spec postconditions)
// and mutates expected output or relations.
// Tests whether incorrect behaviors are rejected by the spec.
// All tests SHOULD FAIL verification.

// Test 1: After a successful call, the new thread must be in thread_dom.
// Mutated: assert the new thread is NOT in the new thread_dom.
// Models: postcondition old.thread_dom().insert(new_thread_ptr) =~= new.thread_dom()
// SHOULD FAIL
proof fn test_mutation_new_thread_not_in_dom(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
    new_thread_ptr: ThreadPtr,
)
    requires
        new_thread_dom =~= old_thread_dom.insert(new_thread_ptr),
{
    assert(!new_thread_dom.contains(new_thread_ptr));
}

// Test 2: proc_dom must be unchanged after the syscall.
// Mutated: assert a new proc appeared in proc_dom.
// Models: postcondition old.proc_dom() =~= new.proc_dom()
// SHOULD FAIL
proof fn test_mutation_proc_dom_changed(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
    extra_proc: ProcPtr,
)
    requires
        new_proc_dom =~= old_proc_dom,
        !old_proc_dom.contains(extra_proc),
{
    assert(new_proc_dom.contains(extra_proc));
}

// Test 3: container_dom must be unchanged.
// Mutated: assert container_dom shrank.
// Models: postcondition old.container_dom() =~= new.container_dom()
// SHOULD FAIL
proof fn test_mutation_container_dom_shrank(
    old_container_dom: Set<ContainerPtr>,
    new_container_dom: Set<ContainerPtr>,
    c: ContainerPtr,
)
    requires
        new_container_dom =~= old_container_dom,
        old_container_dom.contains(c),
{
    assert(!new_container_dom.contains(c));
}

// Test 4: endpoint_dom must be unchanged.
// Mutated: assert endpoint_dom changed.
// Models: postcondition old.endpoint_dom() == new.endpoint_dom()
// SHOULD FAIL
proof fn test_mutation_endpoint_dom_changed(
    old_endpoint_dom: Set<EndpointPtr>,
    new_endpoint_dom: Set<EndpointPtr>,
    extra_ep: EndpointPtr,
)
    requires
        new_endpoint_dom == old_endpoint_dom,
        !old_endpoint_dom.contains(extra_ep),
{
    assert(new_endpoint_dom.contains(extra_ep));
}

// Test 5: The new thread's endpoint_descriptors should have exactly
// one entry at index 0, the rest None.
// Mutated: assert the entry at index 0 is None.
// Models: postcondition endpoint_descriptors@ =~= Seq::new(...).update(0, Some(target_endpoint_ptr))
// SHOULD FAIL
proof fn test_mutation_new_thread_no_endpoint(
    endpoint_descriptors: Seq<Option<EndpointPtr>>,
    target_endpoint_ptr: EndpointPtr,
)
    requires
        endpoint_descriptors =~= Seq::new(
            MAX_NUM_ENDPOINT_DESCRIPTORS as nat,
            |i: int| { None::<EndpointPtr> },
        ).update(0int, Some(target_endpoint_ptr)),
{
    assert(endpoint_descriptors[0int].is_None());
}

// Test 6: The process's owned_threads should have new_thread_ptr pushed.
// Mutated: assert the owned_threads didn't grow.
// Models: postcondition new.get_proc(p).owned_threads@ == old.push(ret)
// SHOULD FAIL
proof fn test_mutation_owned_threads_not_pushed(
    old_threads: Seq<ThreadPtr>,
    new_threads: Seq<ThreadPtr>,
    new_thread_ptr: ThreadPtr,
)
    requires
        new_threads =~= old_threads.push(new_thread_ptr),
{
    assert(new_threads.len() == old_threads.len());
}

// Test 7: The endpoint's owning_threads must include the new (thread, 0) pair.
// Mutated: assert the pair is NOT in the new owning_threads.
// Models: postcondition owning_threads@ =~= old.owning_threads@.insert((new_thread_ptr, 0))
// SHOULD FAIL
proof fn test_mutation_endpoint_owning_threads_no_new(
    old_owning: Set<(ThreadPtr, EndpointIdx)>,
    new_owning: Set<(ThreadPtr, EndpointIdx)>,
    new_thread_ptr: ThreadPtr,
)
    requires
        new_owning =~= old_owning.insert((new_thread_ptr, 0usize)),
{
    assert(!new_owning.contains((new_thread_ptr, 0usize)));
}

// Test 8: Quota mem_4k should decrease by 1 after success.
// Mutated: assert it stayed the same.
// Models: postcondition old.quota.spec_subtract_mem_4k(new.quota, 1)
// SHOULD FAIL
proof fn test_mutation_quota_unchanged(
    old_mem_4k: usize,
    new_mem_4k: usize,
)
    requires
        old_mem_4k > 0,
        old_mem_4k - 1 == new_mem_4k,
{
    assert(old_mem_4k == new_mem_4k);
}

// Test 9: The container's owned_procs should be preserved.
// Mutated: assert owned_procs changed (extra proc appeared).
// Models: postcondition new.get_container(c).owned_procs =~= old.get_container(c).owned_procs
// SHOULD FAIL
proof fn test_mutation_owned_procs_changed(
    old_procs: Seq<ProcPtr>,
    new_procs: Seq<ProcPtr>,
    extra_proc: ProcPtr,
)
    requires
        new_procs =~= old_procs,
{
    assert(new_procs.len() > old_procs.len());
}

// Test 10: When the requirement is not met, the kernel state should
// be unchanged (new =~= old). Mutated: assert they differ.
// Models: postcondition when requirement == false: new =~= old
// SHOULD FAIL
proof fn test_mutation_error_path_state_changed(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
    extra: ThreadPtr,
)
    requires
        new_thread_dom =~= old_thread_dom,
        !old_thread_dom.contains(extra),
{
    assert(new_thread_dom.contains(extra));
}

}
