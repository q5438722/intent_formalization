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
    pub open spec fn spec_subtract_mem_4k(&self, new: Self, k: usize) -> bool {
        &&& self.mem_4k - k == new.mem_4k
        &&& self.mem_2m == new.mem_2m
        &&& self.mem_1g == new.mem_1g
        &&& self.pcid == new.pcid
        &&& self.ioid == new.ioid
    }
}

// ============================================================
// LOGICAL TEST 1: Determinism of new_proc_ptr
// The spec does NOT guarantee that new_proc_ptr is deterministic.
// Given the same old state, two different new_proc_ptrs could be
// valid. Asserting they must be equal should fail.
// SHOULD FAIL: new_proc_ptr is not deterministic
// ============================================================
proof fn test_logical_determinism_new_proc_ptr()
{
    let old_dom: Set<ProcPtr> = Set::empty().insert(1).insert(2);
    let new_ptr_a: ProcPtr = 3;
    let new_ptr_b: ProcPtr = 4;
    let dom_a: Set<ProcPtr> = old_dom.insert(new_ptr_a);
    let dom_b: Set<ProcPtr> = old_dom.insert(new_ptr_b);
    // Both are valid post-states (either could be chosen), but differ
    assert(dom_a =~= dom_b); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 2: Stronger inequality — quota stays positive after subtract
// The spec only requires mem_4k - 2 == new.mem_4k.
// When old.mem_4k == 2, new.mem_4k == 0. Claiming > 0 fails.
// SHOULD FAIL: quota can reach zero
// ============================================================
proof fn test_logical_stronger_quota_stays_positive()
{
    let old_quota = Quota { mem_4k: 2, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 0, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    assume(old_quota.spec_subtract_mem_4k(new_quota, 2));
    assert(new_quota.mem_4k > 0); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 3: proc_dom and thread_dom grow by same element
// The spec inserts page_ptr_1 into proc_dom and page_ptr_2 into
// thread_dom. Assuming they are the same pointer should fail
// unless forced, because they are allocated from separate calls.
// SHOULD FAIL: page_ptr_1 and page_ptr_2 are distinct allocations
// ============================================================
proof fn test_logical_proc_thread_same_ptr()
{
    let old_proc_dom: Set<ProcPtr> = Set::empty().insert(1);
    let old_thread_dom: Set<ThreadPtr> = Set::empty().insert(10);
    let page_ptr_1: usize = 100;
    let page_ptr_2: usize = 200;
    // The spec adds page_ptr_1 to proc_dom, page_ptr_2 to thread_dom
    // Asserting they must be the same should fail
    assert(page_ptr_1 == page_ptr_2); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 4: page_closure grows by exactly 2
// The spec ensures page_closure =~= old.page_closure.insert(p1).insert(p2).
// Claiming it grows by 3 should fail.
// SHOULD FAIL: only two pages are added
// ============================================================
proof fn test_logical_page_closure_grows_by_three()
{
    let old_closure: Set<PagePtr> = Set::empty().insert(1).insert(2);
    let p1: PagePtr = 100;
    let p2: PagePtr = 200;
    let p3: PagePtr = 300;
    let correct = old_closure.insert(p1).insert(p2);
    let wrong = old_closure.insert(p1).insert(p2).insert(p3);
    assert(correct =~= wrong); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 5: new_proc.owned_threads has length > 1
// The spec ensures new_proc.owned_threads@ ==
//   Seq::<ThreadPtr>::empty().push(page_ptr_2) — length is exactly 1.
// Claiming length >= 2 should fail.
// SHOULD FAIL: owned_threads has exactly one element
// ============================================================
proof fn test_logical_owned_threads_length_two()
{
    let page_ptr_2: ThreadPtr = 300;
    let owned_threads: Seq<ThreadPtr> = Seq::<ThreadPtr>::empty().push(page_ptr_2);
    assert(owned_threads.len() >= 2); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 6: old threads are preserved in thread_dom (inverse direction)
// The spec ensures old.thread_dom().insert(page_ptr_2) =~= new.thread_dom().
// This does NOT mean old threads are removed. But asserting that
// an old thread is NOT in new_dom should fail.
// SHOULD FAIL: old threads must still be in new thread_dom
// ============================================================
proof fn test_logical_old_thread_removed()
{
    let old_thread: ThreadPtr = 10;
    let old_dom: Set<ThreadPtr> = Set::empty().insert(old_thread).insert(20);
    let page_ptr_2: ThreadPtr = 300;
    let new_dom = old_dom.insert(page_ptr_2);
    assert(new_dom.contains(old_thread) == false); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 7: owned_endpoints unchanged after syscall
// The spec ensures owned_endpoints@ =~= old.owned_endpoints@.
// Claiming they differ should fail.
// SHOULD FAIL: owned_endpoints must not change
// ============================================================
proof fn test_logical_owned_endpoints_differ()
{
    let old_endpoints: Set<EndpointPtr> = Set::empty().insert(5).insert(6);
    let new_endpoints: Set<EndpointPtr> = Set::empty().insert(5).insert(7);
    assert(old_endpoints =~= new_endpoints); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 8: Subtree set must include new_proc_ptr
// The spec ensures for procs in uppertree_seq of new_proc:
//   self.get_proc(p_ptr).subtree_set@ =~=
//     old.get_proc(p_ptr).subtree_set@.insert(page_ptr_1).
// So the parent's subtree should grow. Asserting the subtree
// does not grow should fail.
// SHOULD FAIL: subtree must grow by new_proc_ptr
// ============================================================
proof fn test_logical_subtree_unchanged()
{
    let old_subtree: Set<ProcPtr> = Set::empty().insert(1).insert(2);
    let page_ptr_1: ProcPtr = 100;
    let new_subtree = old_subtree.insert(page_ptr_1);
    assert(old_subtree =~= new_subtree); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 9: Cross-function — assuming endpoint_dom grows
// The spec guarantees endpoint_dom() == old.endpoint_dom().
// Claiming a new endpoint was created should fail.
// SHOULD FAIL: no new endpoint is created
// ============================================================
proof fn test_logical_cross_fn_endpoint_dom_grows()
{
    let old_endpoint_dom: Set<EndpointPtr> = Set::empty().insert(10).insert(20);
    let new_endpoint: EndpointPtr = 99;
    let new_endpoint_dom = old_endpoint_dom.insert(new_endpoint);
    // The syscall should NOT create new endpoints
    assert(new_endpoint_dom =~= old_endpoint_dom); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 10: New thread endpoint_descriptors slot 0 is None
// The spec ensures slot 0 of the new thread's endpoint_descriptors
// is Some(endpoint_ptr). Asserting it is None should fail.
// SHOULD FAIL: slot 0 must be Some
// ============================================================
proof fn test_logical_new_thread_slot_0_is_none()
{
    let endpoint_ptr: EndpointPtr = 50;
    let descriptors: Seq<Option<EndpointPtr>> = Seq::new(
        MAX_NUM_ENDPOINT_DESCRIPTORS as nat,
        |i: int| { None },
    ).update(0, Some(endpoint_ptr));
    assert(descriptors[0].is_None()); // SHOULD FAIL
}

} // verus!
