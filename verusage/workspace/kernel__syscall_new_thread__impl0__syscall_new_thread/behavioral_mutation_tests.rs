use vstd::prelude::*;

fn main() {}

verus!{

pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type EndpointPtr = usize;
pub type EndpointIdx = usize;

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs matching the spec postconditions
// and mutates expected outputs or relations.
// Tests whether incorrect behaviors are rejected by the spec.
// All tests SHOULD FAIL verification.

// Test 1: When requirement IS satisfied (all conditions met), the spec
// guarantees ret.is_error() == false. Mutated: claim it IS an error.
// Models: ensures syscall_new_thread_requirement == false <==> ret.is_error()
// SHOULD FAIL
proof fn test_mutation_success_returns_error(
    thread_list_full: bool,
    mem_4k: usize,
    scheduler_full: bool,
    free_pages: usize,
)
    requires
        !thread_list_full,
        mem_4k >= 1,
        !scheduler_full,
        free_pages > 0,
{
    // requirement is true, so is_error must be false
    // mutated: assert is_error is true
    let requirement_met = !thread_list_full && mem_4k >= 1 && !scheduler_full && free_pages > 0;
    assert(requirement_met);
    // If requirement is met, is_error should be false, but we assert true
    assert(!requirement_met);
}

// Test 2: When requirement is NOT satisfied (thread list full),
// the spec guarantees ret.is_error() == true.
// Mutated: claim it is NOT an error.
// SHOULD FAIL
proof fn test_mutation_failure_returns_success() {
    let thread_count: usize = MAX_NUM_THREADS_PER_PROC;
    let is_full = thread_count >= MAX_NUM_THREADS_PER_PROC;
    assert(is_full);
    // requirement fails, so is_error should be true
    // mutated: assert is_error is false (i.e., requirement holds)
    assert(!is_full);
}

// Test 3: After successful syscall_new_thread, new thread MUST be
// added to thread_dom. Mutated: assert new thread is NOT in thread_dom.
// Models: ensures self.thread_dom() == old(self).thread_dom().insert(ret)
// (from new_thread postcondition)
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

// Test 4: proc_dom must be unchanged after syscall_new_thread.
// Mutated: assert a new proc appeared in proc_dom.
// Models: ensures self.proc_dom() =~= old(self).proc_dom()
// (from new_thread postcondition)
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

// Test 5: container_dom must be unchanged after syscall_new_thread.
// Mutated: assert a container was removed.
// Models: ensures self.container_dom() == old(self).container_dom()
// SHOULD FAIL
proof fn test_mutation_container_dom_shrank(
    old_container_dom: Set<ContainerPtr>,
    new_container_dom: Set<ContainerPtr>,
    c: ContainerPtr,
)
    requires
        new_container_dom == old_container_dom,
        old_container_dom.contains(c),
{
    assert(!new_container_dom.contains(c));
}

// Test 6: endpoint_dom must be unchanged after new_thread.
// Mutated: assert endpoint_dom changed.
// Models: ensures self.endpoint_dom() == old(self).endpoint_dom()
// SHOULD FAIL
proof fn test_mutation_endpoint_dom_changed(
    old_endpoint_dom: Set<EndpointPtr>,
    new_endpoint_dom: Set<EndpointPtr>,
    e: EndpointPtr,
)
    requires
        new_endpoint_dom == old_endpoint_dom,
        !old_endpoint_dom.contains(e),
{
    assert(new_endpoint_dom.contains(e));
}

// Test 7: page_closure grows by exactly one page after alloc_page_4k.
// Mutated: assert the page_closure did not change.
// Models: ensures self.page_closure() =~= old(self).page_closure().insert(page_ptr)
// SHOULD FAIL
proof fn test_mutation_page_closure_unchanged(
    old_page_closure: Set<PagePtr>,
    new_page_closure: Set<PagePtr>,
    new_page: PagePtr,
)
    requires
        new_page_closure =~= old_page_closure.insert(new_page),
        !old_page_closure.contains(new_page),
{
    assert(new_page_closure =~= old_page_closure);
}

// Test 8: NoSwitchNew(Error) must set error_code to Error.
// Mutated: claim pcid is Some after NoSwitchNew.
// Models: ensures ret.pcid.is_None()
// SHOULD FAIL
proof fn test_mutation_no_switch_new_pcid_some(
    pcid: Option<usize>,
)
    requires
        pcid.is_None(),
{
    assert(pcid.is_Some());
}

// Test 9: After alloc_page_4k, the allocated page was NOT
// previously in allocated_pages_4k.
// Mutated: assert it WAS already allocated.
// Models: ensures old(self).allocated_pages_4k().contains(ret.0) == false
// SHOULD FAIL
proof fn test_mutation_page_already_allocated(
    allocated: Set<PagePtr>,
    page: PagePtr,
)
    requires
        !allocated.contains(page),
{
    assert(allocated.contains(page));
}

// Test 10: Quota subtraction: after new_thread, mem_4k decreases by 1.
// Mutated: assert mem_4k stays the same.
// Models: ensures old.quota.spec_subtract_mem_4k(new.quota, 1)
// SHOULD FAIL
proof fn test_mutation_quota_unchanged(
    old_mem_4k: usize,
    new_mem_4k: usize,
)
    requires
        old_mem_4k >= 1,
        old_mem_4k - 1 == new_mem_4k,
{
    assert(old_mem_4k == new_mem_4k);
}

}
