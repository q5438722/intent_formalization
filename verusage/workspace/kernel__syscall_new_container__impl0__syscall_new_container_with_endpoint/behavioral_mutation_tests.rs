use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type ThreadPtr = usize;
pub type EndpointPtr = usize;
pub type EndpointIdx = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;
pub type CpuId = usize;

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const CONTAINER_CHILD_LIST_LEN: usize = 10;

#[derive(Clone, Copy, Debug)]
pub struct Quota {
    pub mem_4k: usize,
    pub mem_2m: usize,
    pub mem_1g: usize,
    pub pcid: usize,
    pub ioid: usize,
}

impl Quota {
    pub open spec fn spec_subtract_mem_4k(&self, new: Self, k: usize) -> bool {
        &&& self.mem_4k - k == new.mem_4k
        &&& self.mem_2m == new.mem_2m
        &&& self.mem_1g == new.mem_1g
        &&& self.pcid == new.pcid
        &&& self.ioid == new.ioid
    }

    pub open spec fn spec_greater(&self, new: &Quota) -> bool {
        &&& self.mem_4k >= new.mem_4k
        &&& self.mem_2m >= new.mem_2m
        &&& self.mem_1g >= new.mem_1g
        &&& self.pcid >= new.pcid
        &&& self.ioid >= new.ioid
    }
}

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid input conditions and then mutates
// expected outputs/relations. All tests SHOULD FAIL verification.

// Test 1: After new_container_with_endpoint, page_closure should grow by exactly 3 pages.
// Mutate: claim page_closure only grew by 2 pages (missing page_ptr_3).
// SHOULD FAIL
proof fn test_mutation_page_closure_missing_one(
    old_page_closure: Set<PagePtr>,
    page_ptr_1: PagePtr,
    page_ptr_2: PagePtr,
    page_ptr_3: PagePtr,
    new_page_closure: Set<PagePtr>,
)
    requires
        page_ptr_1 != page_ptr_2,
        page_ptr_1 != page_ptr_3,
        page_ptr_2 != page_ptr_3,
        !old_page_closure.contains(page_ptr_1),
        !old_page_closure.contains(page_ptr_2),
        !old_page_closure.contains(page_ptr_3),
        new_page_closure =~= old_page_closure.insert(page_ptr_1).insert(page_ptr_2).insert(page_ptr_3),
{
    // Mutated claim: only two pages added
    assert(new_page_closure =~= old_page_closure.insert(page_ptr_1).insert(page_ptr_2));
}

// Test 2: After syscall, proc_dom should gain exactly one new entry (page_ptr_2).
// Mutate: claim proc_dom gained page_ptr_1 instead.
// SHOULD FAIL
proof fn test_mutation_wrong_proc_ptr_added(
    old_proc_dom: Set<ProcPtr>,
    page_ptr_1: PagePtr,
    page_ptr_2: PagePtr,
    new_proc_dom: Set<ProcPtr>,
)
    requires
        !old_proc_dom.contains(page_ptr_1),
        !old_proc_dom.contains(page_ptr_2),
        page_ptr_1 != page_ptr_2,
        new_proc_dom =~= old_proc_dom.insert(page_ptr_2),
{
    // Mutated claim: page_ptr_1 was added to proc_dom instead
    assert(new_proc_dom =~= old_proc_dom.insert(page_ptr_1));
}

// Test 3: The new container's children should be empty.
// Mutate: claim it has a non-empty children sequence.
// SHOULD FAIL
proof fn test_mutation_new_container_nonempty_children() {
    let new_children: Seq<ContainerPtr> = Seq::empty().push(42);
    assert(new_children =~= Seq::<ContainerPtr>::empty());
}

// Test 4: The quota deduction should subtract exactly 3 mem_4k pages.
// Mutate: claim only 2 were subtracted.
// SHOULD FAIL
proof fn test_mutation_wrong_quota_deduction(
    old_mem_4k: usize,
    new_quota_mem_4k: usize,
    new_container_mem_4k: usize,
)
    requires
        old_mem_4k >= 10,
        new_quota_mem_4k == 3,
        new_container_mem_4k == old_mem_4k - 3 - new_quota_mem_4k,
{
    // Mutated claim: only 2 was deducted (instead of 3 + new_quota)
    assert(new_container_mem_4k == old_mem_4k - 2 - new_quota_mem_4k);
}

// Test 5: The parent container's children list should be appended with page_ptr_1.
// Mutate: claim it was prepended (first element should be page_ptr_1).
// SHOULD FAIL
proof fn test_mutation_children_prepend_instead_of_append(
    old_children: Seq<ContainerPtr>,
    page_ptr_1: ContainerPtr,
    new_children: Seq<ContainerPtr>,
)
    requires
        old_children.len() > 0,
        new_children =~= old_children.push(page_ptr_1),
{
    // Mutated claim: page_ptr_1 is the first element
    assert(new_children[0] == page_ptr_1);
}

// Test 6: The new thread's endpoint_descriptors should have exactly one entry at index 0.
// Mutate: claim the endpoint is at index 1 instead.
// SHOULD FAIL
proof fn test_mutation_endpoint_at_wrong_index(
    endpoint_ptr: EndpointPtr,
) {
    let descriptors: Seq<Option<EndpointPtr>> = Seq::new(
        MAX_NUM_ENDPOINT_DESCRIPTORS as nat,
        |i: int| { None },
    ).update(0, Some(endpoint_ptr));
    // Mutated claim: check index 1 has the endpoint
    assert(descriptors[1] == Some(endpoint_ptr));
}

// Test 7: The new process's owning_container should be page_ptr_1 (new container).
// Mutate: claim owning_container is the parent container instead.
// SHOULD FAIL
proof fn test_mutation_proc_wrong_owning_container(
    new_proc_owning_container: ContainerPtr,
    parent_container: ContainerPtr,
    page_ptr_1: ContainerPtr,
)
    requires
        new_proc_owning_container == page_ptr_1,
        page_ptr_1 != parent_container,
{
    // Mutated claim: the new proc belongs to the parent container
    assert(new_proc_owning_container == parent_container);
}

// Test 8: endpoint_dom should not change after the syscall.
// Mutate: claim endpoint_dom grew by one.
// SHOULD FAIL
proof fn test_mutation_endpoint_dom_changed(
    old_endpoint_dom: Set<EndpointPtr>,
    new_endpoint_dom: Set<EndpointPtr>,
    fake_endpoint: EndpointPtr,
)
    requires
        new_endpoint_dom == old_endpoint_dom,
        !old_endpoint_dom.contains(fake_endpoint),
{
    // Mutated claim: a new endpoint was added
    assert(new_endpoint_dom =~= old_endpoint_dom.insert(fake_endpoint));
}

// Test 9: The new container's depth should be parent.depth + 1.
// Mutate: claim depth is parent.depth + 2.
// SHOULD FAIL
proof fn test_mutation_wrong_depth(
    parent_depth: usize,
    new_depth: usize,
)
    requires
        parent_depth < usize::MAX - 2,
        new_depth as int == parent_depth + 1,
{
    // Mutated claim: depth is parent_depth + 2
    assert(new_depth as int == parent_depth + 2);
}

// Test 10: Old threads should be preserved (unchanged).
// Mutate: claim an old thread's owning_container changed.
// SHOULD FAIL
proof fn test_mutation_old_thread_changed(
    old_owning_container: ContainerPtr,
    new_owning_container: ContainerPtr,
)
    requires
        old_owning_container == new_owning_container,
{
    // Mutated claim: owning_container has changed
    assert(old_owning_container != new_owning_container);
}

}
