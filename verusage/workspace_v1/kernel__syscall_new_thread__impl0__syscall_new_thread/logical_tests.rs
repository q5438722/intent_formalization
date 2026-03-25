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

pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;

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
// LOGICAL TEST 1: Determinism of the returned thread pointer
// The spec does NOT guarantee that new_thread_ptr is deterministic.
// Given the same old state, two different new_thread_ptrs could be
// returned. Asserting they must be equal should fail.
// SHOULD FAIL: new_thread_ptr is not deterministic
// ============================================================
proof fn test_logical_determinism_new_thread_ptr()
{
    let old_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2);
    let new_ptr_a: ThreadPtr = 3;
    let new_ptr_b: ThreadPtr = 4;
    let dom_a: Set<ThreadPtr> = old_dom.insert(new_ptr_a);
    let dom_b: Set<ThreadPtr> = old_dom.insert(new_ptr_b);
    // Both are valid post-states, but they differ
    assert(dom_a =~= dom_b); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 2: Stronger inequality — quota stays positive after subtract
// The spec only requires mem_4k - 1 == new.mem_4k.
// When old.mem_4k == 1, new.mem_4k == 0. Claiming > 0 fails.
// SHOULD FAIL: quota can reach zero
// ============================================================
proof fn test_logical_stronger_quota_stays_positive()
{
    let old_quota = Quota { mem_4k: 1, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 0, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    assume(old_quota.spec_subtract_mem_4k(new_quota, 1));
    assert(new_quota.mem_4k > 0); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 3: Thread domain grows by exactly 2
// The spec inserts exactly ONE new thread into thread_dom.
// Claiming thread_dom grew by 2 should fail.
// SHOULD FAIL: only one thread is inserted
// ============================================================
proof fn test_logical_thread_dom_grows_by_two()
{
    let old_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2);
    let new_dom_two: Set<ThreadPtr> = old_dom.insert(3).insert(4);
    let new_dom_one: Set<ThreadPtr> = old_dom.insert(3);
    assert(new_dom_two =~= new_dom_one); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 4: Cross-function misuse — other procs change
// The spec guarantees: for all p_ptr != proc_ptr,
//   new.get_proc(p_ptr) =~= old.get_proc(p_ptr).
// Claiming a different proc's pcid changed should fail.
// SHOULD FAIL: unrelated proc's pcid must be preserved
// ============================================================
proof fn test_logical_other_proc_pcid_changes()
{
    let old_pcid: Pcid = 10;
    let new_pcid: Pcid = 20; // mutated for an unrelated proc
    // For p_ptr != proc_ptr, old.get_proc(p_ptr) =~= new.get_proc(p_ptr)
    // Means pcid must stay the same
    assert(old_pcid =~= new_pcid); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 5: Structural assumption — free pages after alloc
// After allocating one 4k page, free_pages_4k decreases by exactly 1.
// Claiming it decreased by 2 should fail.
// SHOULD FAIL: free pages decrease by exactly 1, not 2
// ============================================================
proof fn test_logical_free_pages_decrease_by_two()
{
    let old_free: int = 100;
    let new_free: int = old_free - 2; // wrong: should be old_free - 1
    let expected_free: int = old_free - 1;
    assert(new_free == expected_free); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 6: Global assumption — allocated page was previously free
// alloc_page_4k ensures old.free_pages_4k().contains(ret.0).
// Claiming the returned page was NOT in the old free set should fail.
// We model this with set membership.
// SHOULD FAIL: the returned page must have been free
// ============================================================
proof fn test_logical_allocated_page_not_free()
{
    let free_set: Set<PagePtr> = Set::empty().insert(100).insert(200).insert(300);
    let allocated_page: PagePtr = 100;
    // Spec says: old.free_pages_4k().contains(ret.0)
    // Asserting it was NOT in the free set should fail
    assert(!free_set.contains(allocated_page)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 7: Stronger property — page cannot be simultaneously
// free and allocated. The spec ensures the returned page was in
// free_pages and NOT in allocated_pages. Asserting both contain
// the same page should fail.
// SHOULD FAIL: a page cannot be both free and allocated
// ============================================================
proof fn test_logical_page_both_free_and_allocated()
{
    let free_set: Set<PagePtr> = Set::empty().insert(100).insert(200);
    let alloc_set: Set<PagePtr> = Set::empty().insert(100).insert(300);
    // The spec requires free and allocated sets to be disjoint
    // Claiming they are disjoint when they share element 100 should fail
    assert(free_set.disjoint(alloc_set)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 8: Set insert of existing element grows the set
// If new_thread_ptr is already in old_dom, inserting it is a no-op.
// Claiming the set strictly grew should fail.
// SHOULD FAIL: inserting existing element doesn't change the set
// ============================================================
proof fn test_logical_insert_existing_grows()
{
    let s: Set<ThreadPtr> = Set::empty().insert(1).insert(2).insert(3);
    let s2 = s.insert(2); // 2 already present
    // s2 should be identical to s
    assert(!(s =~= s2)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 9: Stronger invariant — wf preserved implies quota > 0
// The spec ensures self.wf() after the call. But wf() does NOT
// require that every container has quota > 0. After a successful
// new_thread call with mem_4k == 1, the new quota is 0.
// Claiming wf implies quota > 0 is unwarranted.
// SHOULD FAIL: wf does not imply quota is positive
// ============================================================
proof fn test_logical_wf_implies_positive_quota()
{
    let mem_4k: usize = 0; // valid state after subtracting 1 from 1
    // The spec allows mem_4k == 0 in a wf kernel
    // Asserting wf implies mem_4k > 0 should fail
    assume(mem_4k == 0); // post-state has zero quota
    assert(mem_4k > 0); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 10: Cross-function — container_owned_pages changes
// The spec says: forall c, old.get_container_owned_pages(c)
//   =~= new.get_container_owned_pages(c).
// Claiming container owned pages changed should fail.
// SHOULD FAIL: container owned pages must be preserved
// ============================================================
proof fn test_logical_container_owned_pages_change()
{
    let old_pages: Set<PagePtr> = Set::empty().insert(10).insert(20);
    let new_pages: Set<PagePtr> = Set::empty().insert(10).insert(20).insert(30);
    // The spec guarantees these are equal
    assert(old_pages =~= new_pages); // SHOULD FAIL
}

} // verus!
