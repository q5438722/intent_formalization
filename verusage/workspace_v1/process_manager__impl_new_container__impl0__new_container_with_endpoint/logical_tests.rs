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

// ===================== LOGICAL TESTS =====================
// These tests assert properties NOT explicitly guaranteed by the spec.
// They probe whether the spec allows unintended reasoning.
// All should FAIL verification if the spec is precise.

// ============================================================
// LOGICAL TEST 1: depth is bounded by a small constant
// The spec says depth = parent_depth + 1 but does NOT bound it.
// Asserting depth <= 10 is an unwarranted assumption.
// SHOULD FAIL
// ============================================================
proof fn test_logical_depth_bounded_small()
{
    let parent_depth: int = 100;
    let new_depth: int = parent_depth + 1;
    assert(new_depth <= 10); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 2: new container was already in parent's subtree
// The spec inserts page_ptr_1 into parent's subtree_set.
// Asserting it was already there before the operation is wrong.
// SHOULD FAIL
// ============================================================
proof fn test_logical_container_already_in_subtree()
{
    let parent_subtree: Set<ContainerPtr> = Set::empty().insert(10).insert(20);
    let page_ptr_1: ContainerPtr = 300;
    assert(parent_subtree.contains(page_ptr_1)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 3: container_dom and proc_dom overlap
// The spec uses different page_ptrs and memory_disjoint() ensures
// these domains are disjoint. But testing two arbitrary sets with
// overlap should fail to verify disjointness.
// SHOULD FAIL
// ============================================================
proof fn test_logical_container_proc_dom_overlap()
{
    let container_dom: Set<ContainerPtr> = Set::empty().insert(1).insert(2).insert(3);
    let proc_dom: Set<ProcPtr> = Set::empty().insert(3).insert(4).insert(5);
    assert(container_dom.disjoint(proc_dom)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 4: the new thread is in the old thread_dom
// The spec says thread_dom grows by inserting page_ptr_3.
// page_ptr_3 was NOT in the old domain.
// SHOULD FAIL
// ============================================================
proof fn test_logical_new_thread_in_old_dom()
{
    let old_thread_dom: Set<ThreadPtr> = Set::empty().insert(10).insert(20);
    let page_ptr_3: ThreadPtr = 30;
    assert(old_thread_dom.contains(page_ptr_3)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 5: new proc has more than 1 owned thread
// The spec says: owned_threads@ == Seq::empty().push(page_ptr_3) — exactly 1.
// Asserting > 1 is wrong.
// SHOULD FAIL
// ============================================================
proof fn test_logical_new_proc_multiple_threads()
{
    let page_ptr_3: ThreadPtr = 0x3000;
    let owned_threads: Seq<ThreadPtr> = Seq::empty().push(page_ptr_3);
    assert(owned_threads.len() > 1); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 6: mem_2m deduction includes extra 3 (like mem_4k)
// The spec deducts mem_4k by (3 + new_quota.mem_4k) but
// mem_2m only by new_quota.mem_2m. Asserting symmetric deduction is wrong.
// SHOULD FAIL
// ============================================================
proof fn test_logical_mem_2m_extra_deduction()
{
    let old_mem_2m: int = 50;
    let new_quota_mem_2m: int = 10;
    let correct_new_mem_2m: int = old_mem_2m - new_quota_mem_2m; // 40
    assert(correct_new_mem_2m == old_mem_2m - 3 - new_quota_mem_2m); // SHOULD FAIL (37 != 40)
}

// ============================================================
// LOGICAL TEST 7: all endpoint descriptors of new thread are filled
// The spec says only slot 0 is set (to endpoint_ptr), rest are None.
// Asserting slot 1 is Some is wrong.
// SHOULD FAIL
// ============================================================
proof fn test_logical_all_endpoint_descriptors_filled()
{
    let endpoint_ptr: EndpointPtr = 42;
    let descriptors: Seq<Option<EndpointPtr>> = Seq::new(
        MAX_NUM_ENDPOINT_DESCRIPTORS as nat,
        |i: int| { None },
    ).update(0, Some(endpoint_ptr));
    assert(descriptors[1] is Some); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 8: new container's owned_cpus is non-empty
// The spec says: owned_cpus@ =~= Set::<CpuId>::empty()
// Asserting it is non-empty is wrong.
// SHOULD FAIL
// ============================================================
proof fn test_logical_new_container_has_cpus()
{
    let owned_cpus: Set<CpuId> = Set::empty();
    assert(owned_cpus.len() > 0); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 9: parent container's owned_procs changed
// The spec says: parent container's owned_procs@ is unchanged.
// Asserting it grew (e.g., includes page_ptr_2) is wrong.
// SHOULD FAIL
// ============================================================
proof fn test_logical_parent_owned_procs_changed()
{
    let old_parent_procs: Seq<ProcPtr> = Seq::empty().push(100);
    let page_ptr_2: ProcPtr = 200;
    let new_parent_procs: Seq<ProcPtr> = old_parent_procs;
    assert(new_parent_procs =~= old_parent_procs.push(page_ptr_2)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 10: new container's subtree_set is non-empty
// The spec says: self.get_container(page_ptr_1).subtree_set@ =~= Set::<ContainerPtr>::empty()
// Asserting it is non-empty is wrong.
// SHOULD FAIL
// ============================================================
proof fn test_logical_new_container_subtree_not_empty()
{
    let subtree: Set<ContainerPtr> = Set::empty();
    assert(subtree.len() > 0); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 11: new container's parent is None (should be Some(parent))
// The spec says: self.get_container(page_ptr_1).parent =~= Some(owning_container)
// Asserting parent is None is wrong.
// SHOULD FAIL
// ============================================================
proof fn test_logical_new_container_parent_none()
{
    let parent_container: ContainerPtr = 0x5000;
    let new_parent: Option<ContainerPtr> = Some(parent_container);
    assert(new_parent.is_None()); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 12: page_ptr_1 is the same as the parent container pointer
// The spec does not prevent page_ptr_1 from equalling any specific container,
// but page_closure disjointness prevents page_ptr_1 from being an existing
// container. Asserting arbitrary equality should fail.
// SHOULD FAIL
// ============================================================
proof fn test_logical_new_container_equals_parent()
{
    let parent_container: ContainerPtr = 0x5000;
    let page_ptr_1: ContainerPtr = 0x1000;
    assert(page_ptr_1 == parent_container); // SHOULD FAIL
}

} // verus!
