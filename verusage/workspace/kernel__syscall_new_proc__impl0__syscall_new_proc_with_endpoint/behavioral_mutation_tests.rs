use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;
pub type ThreadPtr = usize;
pub type EndpointPtr = usize;
pub type EndpointIdx = usize;

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const CONTAINER_PROC_LIST_LEN: usize = 10;
pub const PROC_CHILD_LIST_LEN: usize = 10;

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs but mutates the expected output
// or post-condition relation. All tests SHOULD FAIL verification.

// Test 1: When requirement is satisfied (no error conditions), the spec says
// ret.is_error() == false. Mutate: assert ret IS an error even when requirement holds.
// SHOULD FAIL
proof fn test_mutant_success_returns_error(
    requirement_holds: bool,
    is_error: bool,
)
    requires
        requirement_holds == true,
        requirement_holds == false <==> is_error,
{
    assert(is_error);
}

// Test 2: When requirement fails, spec says old == new (kernel unchanged).
// Mutate: assert old != new when requirement fails.
// SHOULD FAIL
proof fn test_mutant_failed_req_changes_kernel(
    requirement_holds: bool,
    old_val: int,
    new_val: int,
)
    requires
        requirement_holds == false,
        requirement_holds == false ==> old_val == new_val,
{
    assert(old_val != new_val);
}

// Test 3: Spec says new proc's owned_threads contains exactly one thread (the new thread).
// Mutate: assert the new proc has zero threads.
// SHOULD FAIL
proof fn test_mutant_new_proc_has_no_threads(
    new_proc_threads: Seq<ThreadPtr>,
    new_thread_ptr: ThreadPtr,
)
    requires
        new_proc_threads =~= Seq::<ThreadPtr>::empty().push(new_thread_ptr),
{
    assert(new_proc_threads.len() == 0);
}

// Test 4: Spec says new proc's owning_container == container_ptr.
// Mutate: assert the new proc belongs to a different container.
// SHOULD FAIL
proof fn test_mutant_new_proc_wrong_container(
    new_proc_container: ContainerPtr,
    container_ptr: ContainerPtr,
)
    requires
        new_proc_container == container_ptr,
{
    assert(new_proc_container != container_ptr);
}

// Test 5: Spec says old.container_dom() =~= new.container_dom() (no container added/removed).
// Mutate: assert an extra container appears.
// SHOULD FAIL
proof fn test_mutant_container_dom_changes(
    old_container_dom: Set<ContainerPtr>,
    new_container_dom: Set<ContainerPtr>,
    extra: ContainerPtr,
)
    requires
        old_container_dom =~= new_container_dom,
        !old_container_dom.contains(extra),
{
    assert(new_container_dom.contains(extra));
}

// Test 6: Spec says old.endpoint_dom() =~= new.endpoint_dom() (no endpoint added/removed).
// Mutate: assert endpoint domain changed.
// SHOULD FAIL
proof fn test_mutant_endpoint_dom_changes(
    old_endpoint_dom: Set<EndpointPtr>,
    new_endpoint_dom: Set<EndpointPtr>,
    extra: EndpointPtr,
)
    requires
        old_endpoint_dom =~= new_endpoint_dom,
        !old_endpoint_dom.contains(extra),
{
    assert(new_endpoint_dom.contains(extra));
}

// Test 7: Spec says new.proc_dom() == old.proc_dom().insert(new_proc_ptr).
// Mutate: assert old proc_dom already contained the new_proc_ptr.
// SHOULD FAIL
proof fn test_mutant_new_proc_already_existed(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
    new_proc_ptr: ProcPtr,
)
    requires
        old_proc_dom.insert(new_proc_ptr) =~= new_proc_dom,
        !old_proc_dom.contains(new_proc_ptr),
{
    assert(old_proc_dom.contains(new_proc_ptr));
}

// Test 8: Spec says old threads are unchanged.
// Mutate: assert an old thread was modified.
// SHOULD FAIL
proof fn test_mutant_old_thread_changed(
    old_thread_container: ContainerPtr,
    new_thread_container: ContainerPtr,
)
    requires
        old_thread_container == new_thread_container,
{
    assert(old_thread_container != new_thread_container);
}

// Test 9: Spec says new thread's endpoint_descriptors[0] == Some(endpoint_ptr).
// Mutate: assert endpoint_descriptors[0] is None.
// SHOULD FAIL
proof fn test_mutant_new_thread_no_endpoint(
    descriptors: Seq<Option<EndpointPtr>>,
    endpoint_ptr: EndpointPtr,
)
    requires
        descriptors =~= Seq::new(MAX_NUM_ENDPOINT_DESCRIPTORS as nat, |i: int| { None::<EndpointPtr> }).update(0, Some(endpoint_ptr)),
{
    assert(descriptors[0].is_None());
}

// Test 10: Spec says the container's owned_procs is appended with new_proc_ptr.
// Mutate: assert the container's owned_procs is unchanged.
// SHOULD FAIL
proof fn test_mutant_container_procs_unchanged(
    old_procs: Seq<ProcPtr>,
    new_procs: Seq<ProcPtr>,
    new_proc_ptr: ProcPtr,
)
    requires
        new_procs =~= old_procs.push(new_proc_ptr),
{
    assert(old_procs =~= new_procs);
}

// Test 11: Spec says owned_endpoints is unchanged for the container.
// Mutate: assert an extra endpoint was added.
// SHOULD FAIL
proof fn test_mutant_container_endpoints_changed(
    old_endpoints: Set<EndpointPtr>,
    new_endpoints: Set<EndpointPtr>,
    extra: EndpointPtr,
)
    requires
        new_endpoints =~= old_endpoints,
        !old_endpoints.contains(extra),
{
    assert(new_endpoints.contains(extra));
}

// Test 12: Spec says the return value pair is (new_proc_ptr, new_thread_ptr).
// Mutate: assert the return values are swapped (thread_ptr, proc_ptr).
// SHOULD FAIL
proof fn test_mutant_return_values_swapped(
    ret_val1: usize,
    ret_val2: usize,
    new_proc_ptr: ProcPtr,
    new_thread_ptr: ThreadPtr,
)
    requires
        ret_val1 == new_proc_ptr,
        ret_val2 == new_thread_ptr,
        new_proc_ptr != new_thread_ptr,
{
    assert(ret_val1 == new_thread_ptr);
}

}
