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
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const PROC_CHILD_LIST_LEN: usize = 10;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;

// ===================== LOGICAL TESTS =====================
// Each test asserts a property NOT explicitly guaranteed by the spec.
// These probe whether the spec allows unintended reasoning.
// All tests SHOULD FAIL verification.

// Test 1: Determinism — the spec does not guarantee that new_proc_ptr
// is deterministic given the same inputs. Assert two calls yield same ptr.
// SHOULD FAIL
proof fn test_logical_deterministic_new_proc_ptr(
    new_proc_ptr_1: ProcPtr,
    new_proc_ptr_2: ProcPtr,
)
{
    // No spec guarantees the same new_proc_ptr for different calls
    assert(new_proc_ptr_1 == new_proc_ptr_2);
}

// Test 2: The spec says new_proc_ptr is NOT in old.proc_dom().
// But it does not guarantee that new_proc_ptr != new_thread_ptr.
// Assert they must be equal (they shouldn't be, but spec may not say so explicitly).
// SHOULD FAIL
proof fn test_logical_proc_ptr_equals_thread_ptr(
    new_proc_ptr: ProcPtr,
    new_thread_ptr: ThreadPtr,
)
    requires
        new_proc_ptr != new_thread_ptr,
{
    assert(new_proc_ptr == new_thread_ptr);
}

// Test 3: The spec doesn't guarantee an upper bound on how many pages the
// new process can eventually consume. Assert it is bounded by 1.
// SHOULD FAIL
proof fn test_logical_new_proc_bounded_by_one_page(
    va_range_len: usize,
)
    requires
        va_range_len > 1,
{
    assert(va_range_len <= 1);
}

// Test 4: The spec does not explicitly state that the new thread's
// endpoint_descriptors[1..] are all None. Assert slot 1 has something.
// Actually it does: Seq::new(MAX, |i| None).update(0, Some(ep)).
// So slot 1 IS None. This verifies the spec covers this.
// Mutate: assert slot 1 is Some.
// SHOULD FAIL
proof fn test_logical_endpoint_descriptors_slot1_nonempty(
    descriptors: Seq<Option<EndpointPtr>>,
    endpoint_ptr: EndpointPtr,
)
    requires
        descriptors =~= Seq::new(MAX_NUM_ENDPOINT_DESCRIPTORS as nat, |i: int| { None::<EndpointPtr> }).update(0, Some(endpoint_ptr)),
{
    assert(descriptors[1].is_Some());
}

// Test 5: The spec does not guarantee that the new proc is the only proc in
// the container. Assert the container has exactly one proc after the operation.
// SHOULD FAIL
proof fn test_logical_container_has_exactly_one_proc(
    old_procs: Seq<ProcPtr>,
    new_procs: Seq<ProcPtr>,
    new_proc_ptr: ProcPtr,
)
    requires
        new_procs =~= old_procs.push(new_proc_ptr),
        old_procs.len() >= 1,
{
    assert(new_procs.len() == 1);
}

// Test 6: The spec says old containers not equal to container_ptr are unchanged.
// But it does NOT explicitly say the container_ptr container's depth is unchanged.
// Assert the depth changed (unintended reasoning).
// SHOULD FAIL
proof fn test_logical_container_depth_changed(
    old_depth: usize,
    new_depth: usize,
)
    requires
        old_depth == new_depth,
{
    assert(old_depth != new_depth);
}

// Test 7: The spec says endpoint's owning_threads gains (new_thread_ptr, 0).
// It does NOT say other entries are removed. Assert the old entries are gone.
// SHOULD FAIL
proof fn test_logical_endpoint_old_owners_removed(
    old_owners: Set<(ThreadPtr, EndpointIdx)>,
    new_owners: Set<(ThreadPtr, EndpointIdx)>,
    old_entry: (ThreadPtr, EndpointIdx),
    new_thread_ptr: ThreadPtr,
)
    requires
        new_owners =~= old_owners.insert((new_thread_ptr, 0)),
        old_owners.contains(old_entry),
        old_entry != (new_thread_ptr, 0usize),
{
    assert(!new_owners.contains(old_entry));
}

// Test 8: Cross-function misuse — the spec for alloc_page_4k says the returned
// page was in free_pages. The spec does NOT say the page was not previously mapped.
// Assert the allocated page was previously mapped.
// SHOULD FAIL
proof fn test_logical_alloc_page_was_mapped(
    allocated_pages: Set<PagePtr>,
    page_ptr: PagePtr,
)
    requires
        !allocated_pages.contains(page_ptr),
{
    assert(allocated_pages.contains(page_ptr));
}

// Test 9: The spec says address spaces of existing procs are unchanged.
// It does NOT guarantee that the new proc's address space is non-empty
// when va_range.len == 0 (though wf() likely prevents len==0).
// Assert empty va_range still yields non-empty address space.
// SHOULD FAIL
proof fn test_logical_empty_va_range_nonempty_address_space(
    va_range_len: usize,
    addr_space_dom: Set<VAddr>,
)
    requires
        va_range_len == 0,
        addr_space_dom =~= Set::<VAddr>::empty(),
{
    assert(addr_space_dom.len() > 0);
}

// Test 10: Stronger inequality — assert the number of free pages after
// the operation is strictly greater than before (should be less).
// SHOULD FAIL
proof fn test_logical_free_pages_increase(
    old_free: usize,
    new_free: usize,
)
    requires
        new_free <= old_free,
        old_free > 0,
{
    assert(new_free > old_free);
}

// Test 11: The spec says physical page mapping domain is unchanged.
// Assert a new page appeared in the mapping domain.
// SHOULD FAIL
proof fn test_logical_page_mapping_dom_grows(
    old_dom: Set<PagePtr>,
    new_dom: Set<PagePtr>,
    extra: PagePtr,
)
    requires
        old_dom =~= new_dom,
        !old_dom.contains(extra),
{
    assert(new_dom.contains(extra));
}

// Test 12: The spec allows the new proc to share pages with the parent.
// But it does NOT say the new proc can map pages to VAs not in va_range.
// Assert a mapping exists outside va_range.
// SHOULD FAIL
proof fn test_logical_mapping_outside_va_range(
    va_range: Set<VAddr>,
    new_addr_space_dom: Set<VAddr>,
    outside_va: VAddr,
)
    requires
        !va_range.contains(outside_va),
        forall|va: VAddr| !va_range.contains(va) ==> !new_addr_space_dom.contains(va),
{
    assert(new_addr_space_dom.contains(outside_va));
}

}
