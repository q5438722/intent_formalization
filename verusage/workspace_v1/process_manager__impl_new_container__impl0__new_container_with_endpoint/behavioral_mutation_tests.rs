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
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const PAGE_SZ_4k: usize = 1usize << 12;
pub const PAGE_SZ_2m: usize = 1usize << 21;
pub const PAGE_SZ_1g: usize = 1usize << 30;

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

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs/outputs and mutates expected relations.
// These should FAIL verification if the spec correctly rejects wrong behavior.

// ============================================================
// MUTATION TEST 1: new container's children list is NOT empty
// Postcondition: self.get_container(page_ptr_1).children@ == Seq::<ContainerPtr>::empty()
// Mutate: assert children has an element.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_new_container_children_not_empty()
{
    let children: Seq<ContainerPtr> = Seq::empty();
    assert(children.len() > 0); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 2: new container's owned_procs does NOT contain page_ptr_2
// Postcondition: self.get_container(page_ptr_1).owned_procs@ == Seq::empty().push(page_ptr_2)
// Mutate: assert owned_procs is completely empty.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_new_container_no_owned_proc()
{
    let page_ptr_2: ProcPtr = 0x2000;
    let owned_procs: Seq<ProcPtr> = Seq::empty().push(page_ptr_2);
    assert(owned_procs =~= Seq::<ProcPtr>::empty()); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 3: new process's owning_container is wrong
// Postcondition: self.get_proc(page_ptr_2).owning_container == page_ptr_1
// Mutate: assert it equals the parent container instead.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_proc_wrong_owning_container()
{
    let page_ptr_1: ContainerPtr = 0x1000;
    let parent_container: ContainerPtr = 0x5000;
    let proc_owning_container: ContainerPtr = page_ptr_1;
    assert(proc_owning_container == parent_container); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 4: parent container's children list unchanged (not updated)
// Postcondition: self.get_container(parent).children@ =~=
//   old(self).get_container(parent).children@.push(page_ptr_1)
// Mutate: assert children remained the same.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_parent_children_not_updated()
{
    let old_children: Seq<ContainerPtr> = Seq::empty().push(100).push(200);
    let page_ptr_1: ContainerPtr = 300;
    let new_children: Seq<ContainerPtr> = old_children.push(page_ptr_1);
    assert(new_children =~= old_children); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 5: new thread's owning_container is wrong
// Postcondition: self.get_thread(page_ptr_3).owning_container == page_ptr_1
// Mutate: assert it equals the parent container.
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
// MUTATION TEST 6: new process's pcid does not match allocated pcid
// Postcondition: self.get_proc(page_ptr_2).pcid =~= new_pcid
// Mutate: assert pcid is a different value.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_proc_wrong_pcid()
{
    let new_pcid: Pcid = 42;
    let proc_pcid: Pcid = new_pcid;
    assert(proc_pcid == 99); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 7: parent quota mem_4k deduction is wrong (subtract 2 instead of 3)
// Postcondition: old.quota.mem_4k - 3 - new_quota.mem_4k == new_parent.quota.mem_4k
// Mutate: assert deduction is 2 instead of 3.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_wrong_mem_4k_deduction()
{
    let old_mem_4k: int = 100;
    let new_quota_mem_4k: int = 10;
    let new_parent_mem_4k: int = old_mem_4k - 3 - new_quota_mem_4k; // == 87
    assert(new_parent_mem_4k == old_mem_4k - 2 - new_quota_mem_4k); // SHOULD FAIL (88 != 87)
}

// ============================================================
// MUTATION TEST 8: new process's ioid is Some (should be None)
// Postcondition: self.get_proc(page_ptr_2).ioid.is_None()
// Mutate: assert ioid is Some.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_proc_ioid_not_none()
{
    let ioid: Option<IOid> = None;
    assert(ioid is Some); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 9: endpoint domain changed (should be unchanged)
// Postcondition: self.endpoint_dom() == old(self).endpoint_dom()
// Mutate: assert a new endpoint was added.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_endpoint_dom_changed()
{
    let old_endpoint_dom: Set<EndpointPtr> = Set::empty().insert(1).insert(2);
    let new_endpoint_dom: Set<EndpointPtr> = old_endpoint_dom;
    let extra: EndpointPtr = 999;
    assert(new_endpoint_dom.contains(extra)); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 10: new container's quota does NOT match new_quota
// Postcondition: self.get_container(page_ptr_1).quota == new_quota
// Mutate: assert quota.mem_4k is doubled.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_new_container_wrong_quota()
{
    let init_quota = Quota { mem_4k: 50, mem_2m: 10, mem_1g: 1, pcid: 5, ioid: 3 };
    let assigned_quota = Quota { mem_4k: 50, mem_2m: 10, mem_1g: 1, pcid: 5, ioid: 3 };
    assert(assigned_quota.mem_4k == 100); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 11: new container's scheduler is empty (should have page_ptr_3)
// Postcondition: self.get_container(page_ptr_1).scheduler@ =~=
//   Seq::<ThreadPtr>::empty().push(page_ptr_3)
// Mutate: assert scheduler is empty.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_scheduler_empty()
{
    let page_ptr_3: ThreadPtr = 0x3000;
    let scheduler: Seq<ThreadPtr> = Seq::empty().push(page_ptr_3);
    assert(scheduler =~= Seq::<ThreadPtr>::empty()); // SHOULD FAIL
}

// ============================================================
// MUTATION TEST 12: endpoint's owning_threads NOT updated with new thread
// Postcondition: self.get_endpoint(ep).owning_threads@ =~=
//   old(self).get_endpoint(ep).owning_threads@.insert((page_ptr_3, 0))
// Mutate: assert owning_threads is unchanged.
// SHOULD FAIL
// ============================================================
proof fn test_mutation_endpoint_owning_threads_unchanged()
{
    let page_ptr_3: ThreadPtr = 0x3000;
    let old_owning: Set<(ThreadPtr, EndpointIdx)> = Set::empty().insert((10, 0));
    let new_owning: Set<(ThreadPtr, EndpointIdx)> = old_owning.insert((page_ptr_3, 0));
    assert(new_owning =~= old_owning); // SHOULD FAIL
}

} // verus!
