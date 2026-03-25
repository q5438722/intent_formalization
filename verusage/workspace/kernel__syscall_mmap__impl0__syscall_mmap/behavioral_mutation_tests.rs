use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type EndpointPtr = usize;
pub type VAddr = usize;

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid postcondition relationships of
// syscall_mmap and mutates the expected output/behavior.
// Tests whether incorrect behaviors are rejected.
// All tests SHOULD FAIL verification.

// Test 1: When quota is insufficient (ErrorNoQuota), the kernel
// must remain unchanged (new =~= old). Mutated: claim they differ.
// SHOULD FAIL
proof fn test_mutation_error_path_kernel_changed(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
    extra_proc: ProcPtr,
)
    requires
        new_proc_dom =~= old_proc_dom,
        !old_proc_dom.contains(extra_proc),
{
    // On error path, new kernel must equal old. Mutated: extra proc appeared.
    assert(new_proc_dom.contains(extra_proc));
}

// Test 2: On success, thread_dom must be unchanged.
// Mutated: assert a new thread appeared.
// Models: ensures old.thread_dom() =~= new.thread_dom()
// SHOULD FAIL
proof fn test_mutation_thread_dom_changed(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
    new_thread: ThreadPtr,
)
    requires
        new_thread_dom =~= old_thread_dom,
        !old_thread_dom.contains(new_thread),
{
    assert(new_thread_dom.contains(new_thread));
}

// Test 3: On success, proc_dom must be unchanged.
// Mutated: claim proc_dom shrank.
// Models: ensures old.proc_dom() =~= new.proc_dom()
// SHOULD FAIL
proof fn test_mutation_proc_dom_shrank(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
    p: ProcPtr,
)
    requires
        new_proc_dom =~= old_proc_dom,
        old_proc_dom.contains(p),
{
    assert(!new_proc_dom.contains(p));
}

// Test 4: On success, endpoint_dom must be unchanged.
// Mutated: claim a new endpoint appeared.
// Models: ensures old.endpoint_dom() =~= new.endpoint_dom()
// SHOULD FAIL
proof fn test_mutation_endpoint_dom_changed(
    old_endpoint_dom: Set<EndpointPtr>,
    new_endpoint_dom: Set<EndpointPtr>,
    e: EndpointPtr,
)
    requires
        new_endpoint_dom =~= old_endpoint_dom,
        !old_endpoint_dom.contains(e),
{
    assert(new_endpoint_dom.contains(e));
}

// Test 5: On success, address spaces of OTHER processes must be unchanged.
// Mutated: claim another process's address space changed.
// Models: ensures p_ptr != proc_ptr ==> new.get_address_space(p_ptr) =~= old.get_address_space(p_ptr)
// SHOULD FAIL
proof fn test_mutation_other_proc_address_space_changed(
    old_addr_space: Map<VAddr, usize>,
    new_addr_space: Map<VAddr, usize>,
    va: VAddr,
)
    requires
        new_addr_space =~= old_addr_space,
        old_addr_space.dom().contains(va),
{
    // Mutated: claim the address space changed for an unrelated proc
    assert(new_addr_space[va] != old_addr_space[va]);
}

// Test 6: On success, newly mapped VAs must be in the address space domain.
// Mutated: assert a mapped VA is NOT in the new address space domain.
// Models: ensures va_range@.contains(new_va) ==> new.get_address_space(proc_ptr).dom().contains(new_va)
// SHOULD FAIL
proof fn test_mutation_mapped_va_not_in_domain(
    new_addr_dom: Set<VAddr>,
    va_range_set: Set<VAddr>,
    va: VAddr,
)
    requires
        va_range_set.contains(va),
        forall|v: VAddr| va_range_set.contains(v) ==> new_addr_dom.contains(v),
{
    assert(!new_addr_dom.contains(va));
}

// Test 7: On success, mmapped pages must NOT have been in the old
// physical page mapping domain. Mutated: claim they were already mapped.
// Models: ensures mmapped_physcial_pages_seq.contains(page_ptr) ==>
//   old.get_physical_page_mapping().dom().contains(page_ptr) == false
// SHOULD FAIL
proof fn test_mutation_new_pages_were_already_mapped(
    old_page_mapping_dom: Set<PagePtr>,
    new_pages: Seq<PagePtr>,
)
    requires
        new_pages.len() > 0,
        forall|i: int| 0 <= i < new_pages.len() ==> !old_page_mapping_dom.contains(new_pages[i]),
{
    // Mutated: claim the first new page was already mapped
    assert(old_page_mapping_dom.contains(new_pages[0]));
}

// Test 8: On success, VAs outside va_range must have the same mapping.
// Mutated: claim an outside VA's mapping changed.
// Models: ensures va_range@.contains(va) == false ==> new_addr[va] =~= old_addr[va]
// SHOULD FAIL
proof fn test_mutation_outside_va_mapping_changed(
    old_addr_space: Map<VAddr, usize>,
    new_addr_space: Map<VAddr, usize>,
    va_range_seq: Seq<VAddr>,
    outside_va: VAddr,
)
    requires
        !va_range_seq.contains(outside_va),
        old_addr_space.dom().contains(outside_va),
        forall|va: VAddr| !va_range_seq.contains(va) && old_addr_space.dom().contains(va)
            ==> new_addr_space.dom().contains(va) && new_addr_space[va] == old_addr_space[va],
{
    assert(new_addr_space[outside_va] != old_addr_space[outside_va]);
}

// Test 9: On success, the SyscallReturnStruct has switch_decision == NoSwitch.
// Mutated: claim the switch_decision is not NoSwitch (i.e., pcid is Some).
// Models: ensures ret.pcid.is_None() (from NoSwitchNew)
// SHOULD FAIL
proof fn test_mutation_return_has_pcid(
    pcid: Option<usize>,
)
    requires
        pcid.is_None(),
{
    assert(pcid.is_Some());
}

// Test 10: On success, containers other than owning container are unchanged.
// Mutated: claim another container changed.
// Models: ensures c != container_ptr ==> old.get_container(c) =~= new.get_container(c)
// SHOULD FAIL
proof fn test_mutation_other_container_changed(
    old_container_val: usize,
    new_container_val: usize,
)
    requires
        new_container_val == old_container_val,
{
    assert(new_container_val != old_container_val);
}

// Test 11: On success, the new physical page mapping domain is the old
// domain plus the new pages. Mutated: claim domain didn't grow.
// Models: ensures new.get_physical_page_mapping().dom() =~= old.dom() + mmapped_pages.to_set()
// SHOULD FAIL
proof fn test_mutation_page_mapping_dom_unchanged(
    old_dom: Set<PagePtr>,
    new_dom: Set<PagePtr>,
    new_pages: Set<PagePtr>,
)
    requires
        new_dom =~= old_dom + new_pages,
        exists|p: PagePtr| new_pages.contains(p) && !old_dom.contains(p),
{
    assert(new_dom =~= old_dom);
}

// Test 12: On success, each new mapped page maps to a singleton set
// containing (proc_ptr, va). Mutated: claim it maps to empty set.
// Models: ensures new.get_physical_page_mapping()[page] == Set::empty().insert((proc_ptr, va))
// SHOULD FAIL
proof fn test_mutation_page_mapping_is_empty(
    page_mapping: Set<(ProcPtr, VAddr)>,
    proc_ptr: ProcPtr,
    va: VAddr,
)
    requires
        page_mapping =~= Set::empty().insert((proc_ptr, va)),
{
    assert(page_mapping =~= Set::empty());
}

}
