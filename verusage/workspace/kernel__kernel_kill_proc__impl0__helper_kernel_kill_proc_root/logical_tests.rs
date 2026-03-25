use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type ThreadPtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// Tests for unintended reasoning: determinism, stronger inequalities,
// structural/global assumptions, cross-function misuse.
// All tests SHOULD FAIL verification.

// Test 1: The spec does NOT guarantee that killing a root process 
// preserves the page_closure cardinality. Claim: page_closure size 
// is unchanged after removing a page.
// SHOULD FAIL
proof fn test_logical_page_closure_size_unchanged(
    old_closure: Set<PagePtr>,
    new_closure: Set<PagePtr>,
    removed_page: PagePtr,
)
    requires
        old_closure.contains(removed_page),
        new_closure =~= old_closure.remove(removed_page),
        old_closure.finite(),
        old_closure.len() > 0,
{
    assert(new_closure.len() == old_closure.len());
}

// Test 2: The spec does NOT guarantee that the killed proc_ptr 
// can be re-added to proc_dom. Claim: inserting proc_ptr back 
// does not change the set size.
// SHOULD FAIL
proof fn test_logical_proc_reuse_after_kill(
    new_proc_dom: Set<ProcPtr>,
    proc_ptr: ProcPtr,
)
    requires
        !new_proc_dom.contains(proc_ptr),
        new_proc_dom.finite(),
{
    assert(new_proc_dom.insert(proc_ptr).len() == new_proc_dom.len());
}

// Test 3: The spec ensures containers_tree_unchanged, but does NOT
// guarantee that ALL container fields are unchanged (e.g., owned_procs,
// scheduler, owned_threads). Claim: owned_procs is unchanged.
// This is NOT guaranteed by containers_tree_unchanged.
// SHOULD FAIL
proof fn test_logical_container_owned_procs_unchanged(
    old_owned_procs: Seq<ProcPtr>,
    new_owned_procs: Seq<ProcPtr>,
    proc_ptr: ProcPtr,
)
    requires
        old_owned_procs.contains(proc_ptr),
        new_owned_procs =~= old_owned_procs.remove_value(proc_ptr),
{
    assert(new_owned_procs =~= old_owned_procs);
}

// Test 4: The spec does NOT guarantee determinism about which page
// is returned (freed). Two calls with the same proc_ptr could 
// theoretically return different pages. Claim: the returned page 
// is always a specific value (0).
// SHOULD FAIL
proof fn test_logical_deterministic_freed_page(
    freed_page: PagePtr,
)
    requires
        freed_page != 0,
{
    assert(freed_page == 0);
}

// Test 5: The spec guarantees threads_unchanged_except with set![].
// This means ALL threads should be unchanged. But claim a stronger
// property: that the thread content is equal to some arbitrary value.
// SHOULD FAIL
proof fn test_logical_thread_value_determinism(
    thread_val_a: usize,
    thread_val_b: usize,
)
    requires
        thread_val_a != thread_val_b,
{
    assert(thread_val_a == thread_val_b);
}

// Test 6: The spec does NOT say that after killing a root process,
// the remaining proc_dom becomes empty. Even if this was the only 
// process, other processes may exist. Claim: proc_dom is empty.
// SHOULD FAIL
proof fn test_logical_proc_dom_empty_after_kill(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
    proc_ptr: ProcPtr,
    other: ProcPtr,
)
    requires
        old_proc_dom.contains(proc_ptr),
        old_proc_dom.contains(other),
        proc_ptr != other,
        new_proc_dom =~= old_proc_dom.remove(proc_ptr),
{
    assert(new_proc_dom =~= Set::<ProcPtr>::empty());
}

// Test 7: The spec preserves pagetable mappings for remaining procs.
// But it does NOT guarantee the killed proc's pcid becomes free.
// Claim: a pcid stays active even after its proc is removed.
// SHOULD FAIL
proof fn test_logical_killed_pcid_still_active(
    active_pcids: Set<Pcid>,
    killed_pcid: Pcid,
)
    requires
        !active_pcids.contains(killed_pcid),
{
    assert(active_pcids.contains(killed_pcid));
}

// Test 8: The spec does NOT guarantee that the process parent 
// field is None for a root process (depth==0) — only depth==0 is 
// required. Claim: parent must be None simply because depth is 0.
// This is a structural assumption NOT in the spec.
// SHOULD FAIL
proof fn test_logical_root_parent_must_be_none(
    parent: Option<ProcPtr>,
)
    requires
        parent.is_Some(),
{
    assert(parent.is_None());
}

}
