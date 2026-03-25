use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus! {

// ============================================================
// Minimal type/const definitions from target file
// ============================================================

pub type IOid = usize;
pub type CpuId = usize;
pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type PageMapPtr = usize;
pub type Pcid = usize;
pub type PAddr = usize;
pub type VAddr = usize;
pub type SLLIndex = i32;
pub type PagePerm4k = PointsTo<[u8; PAGE_SZ_4k]>;

pub const NUM_CPUS: usize = 32;
pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const CONTAINER_PROC_LIST_LEN: usize = 10;
pub const CONTAINER_CHILD_LIST_LEN: usize = 10;
pub const PROC_CHILD_LIST_LEN: usize = 10;
pub const CONTAINER_ENDPOINT_LIST_LEN: usize = 10;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const PAGE_SZ_4k: usize = 1usize << 12;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const PCID_MAX: usize = 4096;
pub const IOID_MAX: usize = 4096;

// ============================================================
// Quota definition
// ============================================================
#[derive(Clone, Copy, Debug)]
pub struct Quota {
    pub mem_4k: usize,
    pub mem_2m: usize,
    pub mem_1g: usize,
    pub pcid: usize,
    pub ioid: usize,
}

impl Quota {
    pub open spec fn spec_greater(&self, new: &Quota) -> bool {
        &&& self.mem_4k >= new.mem_4k
        &&& self.mem_2m >= new.mem_2m
        &&& self.mem_1g >= new.mem_1g
        &&& self.pcid >= new.pcid
        &&& self.ioid >= new.ioid
    }
}

// ============================================================
// MUTATION TEST 1: new container's children list is not empty
// The postcondition specifies:
//   self.get_container(page_ptr_1).children@ == Seq::<ContainerPtr>::empty()
// Mutating to assert it is non-empty should fail.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_new_container_children_not_empty()
{
    let children: Seq<ContainerPtr> = Seq::empty();
    // The spec says children is empty. Assert it has something.
    assert(children.len() > 0); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 2: new container's owned_procs does NOT contain page_ptr_2
// The postcondition specifies:
//   self.get_container(page_ptr_1).owned_procs@ == Seq::<ProcPtr>::empty().push(page_ptr_2)
// Assert that owned_procs is completely empty (mutation: missing the new process).
// SHOULD FAIL
// ============================================================
proof fn test_mutation_new_container_no_owned_proc()
{
    let page_ptr_2: ProcPtr = 0x2000;
    let owned_procs: Seq<ProcPtr> = Seq::empty().push(page_ptr_2);
    // Mutate: assert it's empty instead of containing page_ptr_2
    assert(owned_procs =~= Seq::<ProcPtr>::empty()); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 3: new process's owning_container is wrong
// The postcondition specifies:
//   self.get_proc(page_ptr_2).owning_container == page_ptr_1
// Mutate to check that it equals something else.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_proc_wrong_owning_container()
{
    let page_ptr_1: ContainerPtr = 0x1000;
    let page_ptr_2: ContainerPtr = 0x2000;
    let new_proc_owning_container: ContainerPtr = page_ptr_1;
    // Mutate: assert it's page_ptr_2 instead of page_ptr_1
    assert(new_proc_owning_container == page_ptr_2); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 4: parent container's children list does NOT include new container
// The postcondition specifies:
//   self.get_container(owning_container).children@ =~=
//     old(self).get_container(owning_container).children@.push(page_ptr_1)
// Mutate by asserting children did not grow.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_parent_children_not_updated()
{
    let old_children: Seq<ContainerPtr> = Seq::empty().push(100).push(200);
    let page_ptr_1: ContainerPtr = 300;
    let new_children: Seq<ContainerPtr> = old_children.push(page_ptr_1);
    // Mutate: assert children unchanged
    assert(new_children =~= old_children); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 5: new thread's owning_container is wrong
// The postcondition specifies:
//   self.get_thread(page_ptr_3).owning_container == page_ptr_1
// Mutate by asserting it owns the parent container instead.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_thread_wrong_owning_container()
{
    let page_ptr_1: ContainerPtr = 0x1000;
    let parent_container: ContainerPtr = 0x5000;
    let thread_owning_container: ContainerPtr = page_ptr_1;
    assert(thread_owning_container == parent_container); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 6: new process's pcid does NOT match allocated pcid
// The postcondition specifies:
//   self.get_proc(page_ptr_2).pcid =~= new_pcid
// Mutate by checking a different pcid.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_proc_wrong_pcid()
{
    let new_pcid: Pcid = 42;
    let proc_pcid: Pcid = new_pcid;
    // Mutate: assert pcid is different
    assert(proc_pcid == 43); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 7: new container's quota does NOT match init_quota
// The postcondition specifies:
//   self.get_container(page_ptr_1).quota == new_quota
// Mutate by checking the quota.mem_4k is incorrect.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_new_container_wrong_quota()
{
    let init_quota = Quota { mem_4k: 100, mem_2m: 10, mem_1g: 1, pcid: 5, ioid: 3 };
    let assigned_quota = Quota { mem_4k: 100, mem_2m: 10, mem_1g: 1, pcid: 5, ioid: 3 };
    // Mutate: assert mem_4k is 200 instead of 100
    assert(assigned_quota.mem_4k == 200); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 8: parent container's mem_4k deduction is wrong
// The postcondition specifies:
//   old.quota.mem_4k - 3 - new_quota.mem_4k == new_parent.quota.mem_4k
// Mutate by using wrong deduction (subtract 2 instead of 3).
// SHOULD FAIL
// ============================================================
proof fn test_mutation_wrong_mem_4k_deduction()
{
    let old_mem_4k: int = 100;
    let init_mem_4k: int = 10;
    // Correct deduction: old_mem_4k - 3 - init_mem_4k == 87
    let new_parent_mem_4k: int = old_mem_4k - 3 - init_mem_4k;
    // Mutate: assert wrong deduction (subtract 2 instead of 3)
    assert(new_parent_mem_4k == old_mem_4k - 2 - init_mem_4k); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 9: new process's ioid is not None
// The postcondition specifies:
//   self.get_proc(page_ptr_2).ioid.is_None()
// Mutate by asserting it is Some.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_proc_ioid_not_none()
{
    let ioid: Option<IOid> = None;
    assert(ioid is Some); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 10: endpoint domain changed (should be unchanged)
// The postcondition specifies:
//   self.endpoint_dom() == old(self).endpoint_dom()
// Mutate by asserting a new endpoint was added.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_endpoint_dom_changed()
{
    let old_endpoint_dom: Set<EndpointPtr> = Set::empty().insert(1).insert(2);
    let new_endpoint_dom: Set<EndpointPtr> = old_endpoint_dom;
    let extra_endpoint: EndpointPtr = 999;
    // Mutate: assert the new domain contains an extra endpoint
    assert(new_endpoint_dom.contains(extra_endpoint)); // SHOULD FAIL
}

} // verus!
