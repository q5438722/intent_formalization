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
// LOGICAL TEST 1: determinism — same inputs must produce same outputs
// The spec does not guarantee that page_ptr_1, page_ptr_2, page_ptr_3
// are deterministic (they are allocated). Two calls could produce
// different page pointers. Asserting determinism should fail.
// SHOULD FAIL
// ============================================================
proof fn test_logical_determinism_of_allocation()
{
    // Simulate two allocation results
    let page_ptr_1a: PagePtr = 0x1000;
    let page_ptr_1b: PagePtr = 0x2000;
    // The spec doesn't guarantee allocation is deterministic
    assert(page_ptr_1a == page_ptr_1b); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 2: new container depth is bounded by a small constant
// The spec says depth = parent_depth + 1. It does NOT bound depth
// to be small. Asserting depth <= 10 is an unwarranted assumption.
// SHOULD FAIL
// ============================================================
proof fn test_logical_depth_bounded_small()
{
    let parent_depth: int = 100;
    let new_depth: int = parent_depth + 1;
    assert(new_depth <= 10); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 3: new container is in parent's subtree_set
// The spec says the parent's subtree_set is updated to include
// page_ptr_1, but does NOT say page_ptr_1 is already in the
// parent's subtree before the operation. Asserting it was already
// there should fail.
// SHOULD FAIL
// ============================================================
proof fn test_logical_container_already_in_subtree()
{
    let parent_subtree: Set<ContainerPtr> = Set::empty().insert(10).insert(20);
    let page_ptr_1: ContainerPtr = 300;
    // page_ptr_1 was NOT in parent subtree before the operation
    assert(parent_subtree.contains(page_ptr_1)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 4: container_dom and proc_dom are always disjoint
// The spec uses page_ptr_1 for new container and page_ptr_2 for new process,
// and requires page_ptr_1 != page_ptr_2. But the spec does NOT
// guarantee that the old container_dom and proc_dom are disjoint
// in general — only that these specific new pointers differ.
// Asserting arbitrary disjointness should fail.
// SHOULD FAIL
// ============================================================
proof fn test_logical_container_proc_dom_disjoint()
{
    let container_dom: Set<ContainerPtr> = Set::empty().insert(1).insert(2).insert(3);
    let proc_dom: Set<ProcPtr> = Set::empty().insert(3).insert(4).insert(5);
    // These share element 3, but spec doesn't guarantee disjointness
    assert(container_dom.disjoint(proc_dom)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 5: all existing procs are unchanged implies
// all existing containers are unchanged (incorrect inference)
// The spec guarantees existing procs are unchanged, and existing
// containers (other than parent) are unchanged. But asserting
// the parent container is also unchanged is wrong — its children,
// quota, and subtree_set are updated.
// SHOULD FAIL
// ============================================================
proof fn test_logical_parent_container_unchanged()
{
    let old_children: Seq<ContainerPtr> = Seq::empty().push(100);
    let page_ptr_1: ContainerPtr = 200;
    let new_children: Seq<ContainerPtr> = old_children.push(page_ptr_1);
    // The parent container IS changed (children updated)
    assert(new_children =~= old_children); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 6: thread_dom grows by exactly 1 implies
// total thread count across all containers grows by exactly 1
// The spec says thread_dom grows by inserting page_ptr_3.
// But it does NOT explicitly guarantee the new thread is owned
// by exactly one container. Asserting a stronger counting property
// is not directly entailed.
// However, the spec DOES say the new container owns {page_ptr_3},
// so the owned set of new container has exactly 1 element.
// Instead, test: asserting the new thread is also in the old
// thread_dom is wrong.
// SHOULD FAIL
// ============================================================
proof fn test_logical_new_thread_in_old_dom()
{
    let old_thread_dom: Set<ThreadPtr> = Set::empty().insert(10).insert(20);
    let page_ptr_3: ThreadPtr = 30;
    let new_thread_dom: Set<ThreadPtr> = old_thread_dom.insert(page_ptr_3);
    // Assert the new thread was already in old domain (wrong)
    assert(old_thread_dom.contains(page_ptr_3)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 7: new process has children (stronger than spec)
// The spec does NOT mention the new process's children list.
// This is a gap — one cannot assume it is empty or non-empty.
// Here we test an unjustified property: process has no children.
// Actually the spec doesn't specify owned_threads length bound.
// Instead, test: the new proc has more than 1 thread.
// The spec says: owned_threads@ == Seq::empty().push(page_ptr_3)
// — exactly 1 thread. Asserting > 1 should fail.
// SHOULD FAIL
// ============================================================
proof fn test_logical_new_proc_multiple_threads()
{
    let page_ptr_3: ThreadPtr = 0x3000;
    let owned_threads: Seq<ThreadPtr> = Seq::empty().push(page_ptr_3);
    assert(owned_threads.len() > 1); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 8: quota deduction is symmetric across mem types
// The spec deducts mem_4k by (3 + init_quota.mem_4k) but
// mem_2m only by init_quota.mem_2m (no extra 3).
// Asserting that mem_2m also loses 3 extra is wrong.
// SHOULD FAIL
// ============================================================
proof fn test_logical_mem_2m_extra_deduction()
{
    let old_mem_2m: int = 50;
    let init_mem_2m: int = 10;
    // Correct: new_parent_mem_2m == old_mem_2m - init_mem_2m == 40
    // Wrong claim: new_parent_mem_2m == old_mem_2m - 3 - init_mem_2m == 37
    let new_parent_mem_2m: int = old_mem_2m - init_mem_2m;
    assert(new_parent_mem_2m == old_mem_2m - 3 - init_mem_2m); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 9: endpoint_descriptors of new thread has
// ALL slots filled (stronger than spec)
// The spec says only slot 0 is set, rest are None.
// Asserting all slots are Some is wrong.
// SHOULD FAIL
// ============================================================
proof fn test_logical_all_endpoint_descriptors_filled()
{
    let endpoint_ptr: EndpointPtr = 42;
    let descriptors: Seq<Option<EndpointPtr>> = Seq::new(
        MAX_NUM_ENDPOINT_DESCRIPTORS as nat,
        |i: int| { None },
    ).update(0, Some(endpoint_ptr));
    // Slot 1 should be None per spec
    assert(descriptors[1] is Some); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 10: new container's owned_cpus is non-empty
// The spec says:
//   self.get_container(page_ptr_1).owned_cpus@ =~= Set::<CpuId>::empty()
// Asserting it is non-empty should fail.
// SHOULD FAIL
// ============================================================
proof fn test_logical_new_container_has_cpus()
{
    let owned_cpus: Set<CpuId> = Set::empty();
    assert(owned_cpus.len() > 0); // SHOULD FAIL
}

} // verus!
