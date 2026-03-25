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

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Start from valid inputs, mutate expected outputs or relations.
// Each test asserts something the postcondition should NOT guarantee.
// All tests SHOULD FAIL verification.

// Test 1: share_mapping ensures free page count is unchanged.
// Mutation: claim free pages decreased by 1 (sharing doesn't consume free pages).
// SHOULD FAIL
proof fn test_mutation_free_pages_decreased(
    old_free_pages: usize,
    new_free_pages: usize,
)
    requires
        old_free_pages == new_free_pages,  // postcondition: unchanged
{
    // Mutate: claim free pages decreased
    assert(new_free_pages == old_free_pages - 1);
}

// Test 2: share_mapping ensures ref counter increments by exactly 1.
// Mutation: claim ref counter increments by 2.
// SHOULD FAIL
proof fn test_mutation_ref_counter_incremented_by_2(
    old_ref_counter: nat,
    new_ref_counter: nat,
)
    requires
        old_ref_counter + 1 == new_ref_counter,  // postcondition
{
    // Mutate: claim it incremented by 2
    assert(old_ref_counter + 2 == new_ref_counter);
}

// Test 3: share_mapping ensures target address space is old + insert(target_va, entry).
// Mutation: claim target address space is unchanged (no insert happened).
// SHOULD FAIL
proof fn test_mutation_address_space_unchanged(
    old_addr_space: Map<VAddr, MapEntry>,
    new_addr_space: Map<VAddr, MapEntry>,
    target_va: VAddr,
    entry: MapEntry,
)
    requires
        new_addr_space =~= old_addr_space.insert(target_va, entry),
        !old_addr_space.dom().contains(target_va),
{
    // Mutate: claim address space didn't change
    assert(new_addr_space =~= old_addr_space);
}

// Test 4: share_mapping ensures other procs' address spaces are unchanged.
// Mutation: claim the other proc's address space changed.
// SHOULD FAIL
proof fn test_mutation_other_proc_space_changed(
    old_space: Map<VAddr, MapEntry>,
    new_space: Map<VAddr, MapEntry>,
    some_va: VAddr,
    some_entry: MapEntry,
)
    requires
        new_space =~= old_space,  // postcondition: unchanged for other procs
        !old_space.dom().contains(some_va),
{
    // Mutate: claim it was modified
    assert(new_space =~= old_space.insert(some_va, some_entry));
}

// Test 5: share_mapping ensures proc_dom() is preserved.
// Mutation: claim a new proc was added to the domain.
// SHOULD FAIL
proof fn test_mutation_proc_dom_grew(
    old_dom: Set<ProcPtr>,
    new_dom: Set<ProcPtr>,
    new_proc: ProcPtr,
)
    requires
        new_dom == old_dom,  // postcondition: unchanged
        !old_dom.contains(new_proc),
{
    // Mutate: claim a new proc was added
    assert(new_dom.contains(new_proc));
}

// Test 6: share_mapping ensures page_mapping domain is preserved.
// Mutation: claim the domain grew (a new page was added).
// SHOULD FAIL
proof fn test_mutation_page_mapping_dom_grew(
    old_dom: Set<PagePtr>,
    new_dom: Set<PagePtr>,
    new_page: PagePtr,
)
    requires
        new_dom == old_dom,  // postcondition: unchanged
        !old_dom.contains(new_page),
{
    // Mutate: claim domain grew
    assert(new_dom.contains(new_page));
}

// Test 7: share_mapping ensures ref counter of OTHER pages is unchanged.
// Mutation: claim another page's ref counter also incremented.
// SHOULD FAIL
proof fn test_mutation_other_page_ref_changed(
    old_ref: nat,
    new_ref: nat,
)
    requires
        old_ref == new_ref,  // postcondition: unchanged for other pages
{
    // Mutate: claim it changed
    assert(new_ref == old_ref + 1);
}

// Test 8: share_mapping ensures page_mapping for OTHER pages is unchanged.
// Mutation: claim another page's mapping set changed.
// SHOULD FAIL
proof fn test_mutation_other_page_mapping_changed(
    old_mapping: Set<(ProcPtr, VAddr)>,
    new_mapping: Set<(ProcPtr, VAddr)>,
    extra: (ProcPtr, VAddr),
)
    requires
        old_mapping == new_mapping,  // postcondition: unchanged for other pages
        !old_mapping.contains(extra),
{
    // Mutate: claim it changed
    assert(new_mapping.contains(extra));
}

// Test 9: share_mapping ensures the target page's mapping set gains exactly (target_proc_ptr, target_va).
// Mutation: claim the mapping set is unchanged.
// SHOULD FAIL
proof fn test_mutation_target_page_mapping_unchanged(
    old_mapping: Set<(ProcPtr, VAddr)>,
    new_mapping: Set<(ProcPtr, VAddr)>,
    target_proc_ptr: ProcPtr,
    target_va: VAddr,
)
    requires
        new_mapping == old_mapping.insert((target_proc_ptr, target_va)),
        !old_mapping.contains((target_proc_ptr, target_va)),
{
    // Mutate: claim no change
    assert(new_mapping == old_mapping);
}

// Test 10: share_mapping ensures container_dom is preserved.
// Mutation: claim the container domain shrank.
// SHOULD FAIL
proof fn test_mutation_container_dom_shrank(
    old_dom: Set<ContainerPtr>,
    new_dom: Set<ContainerPtr>,
    c_ptr: ContainerPtr,
)
    requires
        new_dom == old_dom,  // postcondition: unchanged
        old_dom.contains(c_ptr),
{
    // Mutate: claim c_ptr was removed
    assert(!new_dom.contains(c_ptr));
}

}
