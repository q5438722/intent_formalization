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

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Start from valid preconditions but assert mutated/incorrect postconditions.
// All tests SHOULD FAIL verification.

// Test 1: range_create_and_share_mapping ensures proc_dom unchanged.
// Mutate: assert proc_dom shrinks after the call.
// SHOULD FAIL
proof fn test_mutation_proc_dom_shrinks(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
    removed_ptr: ProcPtr,
)
    requires
        new_proc_dom == old_proc_dom,
        old_proc_dom.contains(removed_ptr),
{
    assert(!new_proc_dom.contains(removed_ptr));
}

// Test 2: range_create_and_share_mapping ensures thread_dom unchanged.
// Mutate: assert thread_dom gains a new element.
// SHOULD FAIL
proof fn test_mutation_thread_dom_changes(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
    new_thread: ThreadPtr,
)
    requires
        new_thread_dom == old_thread_dom,
        !old_thread_dom.contains(new_thread),
{
    assert(new_thread_dom.contains(new_thread));
}

// Test 3: range_create_and_share_mapping ensures
// get_num_of_free_pages() == old(...) - ret.
// Mutate: assert free pages increased instead.
// SHOULD FAIL
proof fn test_mutation_free_pages_increase(
    old_free: usize,
    new_free: usize,
    ret: usize,
)
    requires
        new_free == old_free - ret,
        ret > 0,
        old_free >= ret,
{
    assert(new_free > old_free);
}

// Test 4: create_entry_and_share ensures ret <= 3.
// Mutate: assert ret == 4.
// SHOULD FAIL
proof fn test_mutation_ret_exceeds_bound(
    ret: usize,
)
    requires
        ret <= 3,
{
    assert(ret == 4);
}

// Test 5: create_entry_and_share ensures the target address space grows by exactly one entry.
// Mutate: assert old entries disappeared.
// SHOULD FAIL
proof fn test_mutation_target_addr_space_loses_old_entry(
    old_addr_space: Map<VAddr, MapEntry>,
    new_addr_space: Map<VAddr, MapEntry>,
    target_va: VAddr,
    existing_va: VAddr,
)
    requires
        old_addr_space.dom().contains(existing_va),
        existing_va != target_va,
        !old_addr_space.dom().contains(target_va),
        new_addr_space.dom() == old_addr_space.dom().insert(target_va),
{
    assert(!new_addr_space.dom().contains(existing_va));
}

// Test 6: create_entry_and_share ensures that for procs other than target,
// address space is unchanged.
// Mutate: assert some other proc's address space changed.
// SHOULD FAIL
proof fn test_mutation_other_proc_addr_space_changed(
    old_other_space: Map<VAddr, MapEntry>,
    new_other_space: Map<VAddr, MapEntry>,
)
    requires
        new_other_space =~= old_other_space,
{
    assert(new_other_space.dom() != old_other_space.dom());
}

// Test 7: range_create_and_share_mapping ensures endpoint_dom unchanged.
// Mutate: assert endpoint_dom changed.
// SHOULD FAIL
proof fn test_mutation_endpoint_dom_changes(
    old_endpoint_dom: Set<EndpointPtr>,
    new_endpoint_dom: Set<EndpointPtr>,
    ep: EndpointPtr,
)
    requires
        new_endpoint_dom == old_endpoint_dom,
        !old_endpoint_dom.contains(ep),
{
    assert(new_endpoint_dom.contains(ep));
}

// Test 8: create_entry_and_share ensures physical page reference counter
// increases by exactly 1.
// Mutate: assert it increased by 2.
// SHOULD FAIL
proof fn test_mutation_ref_counter_wrong_increment(
    old_ref: nat,
    new_ref: nat,
)
    requires
        old_ref + 1 == new_ref,
{
    assert(old_ref + 2 == new_ref);
}

// Test 9: create_entry_and_share ensures target address space maps target_va
// to the same entry as src_va in src's space.
// Mutate: assert the mapped address is different.
// SHOULD FAIL
proof fn test_mutation_shared_entry_addr_differs(
    src_addr: PAddr,
    target_mapped_addr: PAddr,
)
    requires
        target_mapped_addr == src_addr,
{
    assert(target_mapped_addr != src_addr);
}

// Test 10: range_create_and_share_mapping ensures container_dom unchanged.
// Mutate: assert container_dom gained an element.
// SHOULD FAIL
proof fn test_mutation_container_dom_changes(
    old_container_dom: Set<ContainerPtr>,
    new_container_dom: Set<ContainerPtr>,
    new_c: ContainerPtr,
)
    requires
        new_container_dom == old_container_dom,
        !old_container_dom.contains(new_c),
{
    assert(new_container_dom.contains(new_c));
}

// Test 11: range_create_and_share_mapping ensures page_mapping domain unchanged.
// Mutate: assert domain grew.
// SHOULD FAIL
proof fn test_mutation_page_mapping_dom_changes(
    old_dom: Set<PagePtr>,
    new_dom: Set<PagePtr>,
    new_page: PagePtr,
)
    requires
        new_dom == old_dom,
        !old_dom.contains(new_page),
{
    assert(new_dom.contains(new_page));
}

// Test 12: range_create_and_share_mapping ensures quota is subtracted by ret.
// Mutate: assert quota was not changed at all.
// SHOULD FAIL
proof fn test_mutation_quota_unchanged(
    old_mem_4k: usize,
    new_mem_4k: usize,
    ret: usize,
)
    requires
        old_mem_4k - ret == new_mem_4k,
        ret > 0,
        old_mem_4k >= ret,
{
    assert(new_mem_4k == old_mem_4k);
}

}
