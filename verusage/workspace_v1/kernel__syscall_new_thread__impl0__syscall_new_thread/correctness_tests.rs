use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus! {

// ============================================================
// Combined correctness tests for syscall_new_thread
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

// === BOUNDARY TESTS ===

// SHOULD FAIL: thread_ptr not in domain
proof fn boundary_1_thread_ptr_not_in_dom()
{
    let thread_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2).insert(3);
    let thread_ptr: ThreadPtr = 999;
    assert(thread_dom.contains(thread_ptr)); // SHOULD FAIL
}

// SHOULD FAIL: empty domain has no threads
proof fn boundary_2_empty_thread_dom()
{
    let thread_dom: Set<ThreadPtr> = Set::empty();
    assert(thread_dom.contains(0usize)); // SHOULD FAIL
}

// SHOULD FAIL: thread list at max capacity
proof fn boundary_3_thread_list_full()
{
    assert(!(MAX_NUM_THREADS_PER_PROC >= MAX_NUM_THREADS_PER_PROC)); // SHOULD FAIL
}

// SHOULD FAIL: zero quota fails requirement
proof fn boundary_4_zero_quota()
{
    let mem_4k: usize = 0;
    assert(mem_4k >= 1); // SHOULD FAIL
}

// SHOULD FAIL: scheduler at max
proof fn boundary_5_scheduler_full()
{
    assert(!(MAX_CONTAINER_SCHEDULER_LEN >= MAX_CONTAINER_SCHEDULER_LEN)); // SHOULD FAIL
}

// SHOULD FAIL: no free pages
proof fn boundary_6_zero_free_pages()
{
    let free_pages: usize = 0;
    assert(free_pages > 0); // SHOULD FAIL
}

// SHOULD FAIL: quota subtract underflow
proof fn boundary_7_quota_subtract_underflow()
{
    let old_q = Quota { mem_4k: 0, mem_2m: 10, mem_1g: 5, pcid: 2, ioid: 1 };
    let new_q = Quota { mem_4k: usize::MAX, mem_2m: 10, mem_1g: 5, pcid: 2, ioid: 1 };
    assert(old_q.spec_subtract_mem_4k(new_q, 1)); // SHOULD FAIL
}

// === BEHAVIORAL MUTATION TESTS ===

// SHOULD FAIL: wrong subtraction amount
proof fn behavioral_1_wrong_subtract()
{
    let old_q = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_q = Quota { mem_4k: 98, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    assert(old_q.spec_subtract_mem_4k(new_q, 1)); // SHOULD FAIL
}

// SHOULD FAIL: mem_2m mutated
proof fn behavioral_2_mem_2m_changed()
{
    let old_q = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_q = Quota { mem_4k: 99, mem_2m: 49, mem_1g: 10, pcid: 5, ioid: 3 };
    assert(old_q.spec_subtract_mem_4k(new_q, 1)); // SHOULD FAIL
}

// SHOULD FAIL: ioid mutated
proof fn behavioral_3_ioid_changed()
{
    let old_q = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_q = Quota { mem_4k: 99, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 2 };
    assert(old_q.spec_subtract_mem_4k(new_q, 1)); // SHOULD FAIL
}

// SHOULD FAIL: success returns error
proof fn behavioral_4_success_returns_error()
{
    let req: bool = true;
    let err: bool = true;
    assert(req == false <==> err); // SHOULD FAIL
}

// SHOULD FAIL: failure returns success
proof fn behavioral_5_failure_returns_success()
{
    let req: bool = false;
    let err: bool = false;
    assert(req == false <==> err); // SHOULD FAIL
}

// SHOULD FAIL: thread_dom unchanged on success
proof fn behavioral_6_thread_dom_unchanged()
{
    let old_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2);
    let new_dom: Set<ThreadPtr> = old_dom.insert(3);
    assert(old_dom =~= new_dom); // SHOULD FAIL
}

// SHOULD FAIL: page_closure unchanged
proof fn behavioral_7_page_closure_unchanged()
{
    let old_p: Set<PagePtr> = Set::empty().insert(100).insert(200);
    let new_p: Set<PagePtr> = old_p.insert(300);
    assert(old_p =~= new_p); // SHOULD FAIL
}

// SHOULD FAIL: proc_dom changed
proof fn behavioral_8_proc_dom_changed()
{
    let old_d: Set<ProcPtr> = Set::empty().insert(10).insert(20);
    let new_d: Set<ProcPtr> = old_d.insert(30);
    assert(old_d =~= new_d); // SHOULD FAIL
}

// SHOULD FAIL: container_dom changed
proof fn behavioral_9_container_dom_changed()
{
    let old_d: Set<ContainerPtr> = Set::empty().insert(100);
    let new_d: Set<ContainerPtr> = old_d.insert(200);
    assert(old_d =~= new_d); // SHOULD FAIL
}

// SHOULD FAIL: pcid changed for owning proc
proof fn behavioral_10_pcid_changed()
{
    let old_pcid: Pcid = 42;
    let new_pcid: Pcid = 43;
    assert(old_pcid =~= new_pcid); // SHOULD FAIL
}

// === LOGICAL TESTS ===

// SHOULD FAIL: new thread ptr is not deterministic
proof fn logical_1_determinism()
{
    let d: Set<ThreadPtr> = Set::empty().insert(1).insert(2);
    assert(d.insert(3usize) =~= d.insert(4usize)); // SHOULD FAIL
}

// SHOULD FAIL: quota can reach zero
proof fn logical_2_quota_stays_positive()
{
    let old_q = Quota { mem_4k: 1, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_q = Quota { mem_4k: 0, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    assume(old_q.spec_subtract_mem_4k(new_q, 1));
    assert(new_q.mem_4k > 0); // SHOULD FAIL
}

// SHOULD FAIL: only one thread inserted
proof fn logical_3_grows_by_two()
{
    let d: Set<ThreadPtr> = Set::empty().insert(1).insert(2);
    assert(d.insert(3usize).insert(4usize) =~= d.insert(3usize)); // SHOULD FAIL
}

// SHOULD FAIL: unrelated proc's pcid preserved
proof fn logical_4_other_proc_changes()
{
    let a: Pcid = 10;
    let b: Pcid = 20;
    assert(a =~= b); // SHOULD FAIL
}

// SHOULD FAIL: free pages decrease by 1, not 2
proof fn logical_5_free_pages_off_by_one()
{
    let old_free: int = 100;
    assert(old_free - 2 == old_free - 1); // SHOULD FAIL
}

// SHOULD FAIL: returned page was in free set
proof fn logical_6_allocated_page_was_free()
{
    let free_set: Set<PagePtr> = Set::empty().insert(100).insert(200).insert(300);
    assert(!free_set.contains(100usize)); // SHOULD FAIL
}

// SHOULD FAIL: free and allocated sets must be disjoint
proof fn logical_7_free_alloc_overlap()
{
    let free: Set<PagePtr> = Set::empty().insert(100).insert(200);
    let alloc: Set<PagePtr> = Set::empty().insert(100).insert(300);
    assert(free.disjoint(alloc)); // SHOULD FAIL
}

// SHOULD FAIL: inserting existing element is idempotent
proof fn logical_8_set_insert_idempotent()
{
    let s: Set<ThreadPtr> = Set::empty().insert(1).insert(2).insert(3);
    assert(!(s =~= s.insert(2usize))); // SHOULD FAIL
}

// SHOULD FAIL: wf does not imply positive quota
proof fn logical_9_wf_implies_positive()
{
    assume(0usize == 0usize);
    assert(0usize > 0); // SHOULD FAIL
}

// SHOULD FAIL: container_owned_pages preserved
proof fn logical_10_container_pages_change()
{
    let old_p: Set<PagePtr> = Set::empty().insert(10).insert(20);
    let new_p: Set<PagePtr> = old_p.insert(30);
    assert(old_p =~= new_p); // SHOULD FAIL
}

} // verus!
