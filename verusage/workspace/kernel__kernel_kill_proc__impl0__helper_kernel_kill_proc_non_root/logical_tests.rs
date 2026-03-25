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

pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// Tests for unintended reasoning: determinism, stronger inequalities,
// structural assumptions, cross-function misuse.
// All tests SHOULD FAIL verification.

// Test 1: The spec does NOT guarantee that killing a process preserves
// the page_closure size (it removes one page). Claim: page_closure
// size is unchanged after removing a page.
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
// can be reused (re-added to proc_dom). Claim: proc_ptr stays valid
// for insertion back.
// SHOULD FAIL
proof fn test_logical_proc_reuse_after_kill(
    new_proc_dom: Set<ProcPtr>,
    proc_ptr: ProcPtr,
)
    requires
        !new_proc_dom.contains(proc_ptr),
{
    // The spec doesn't say proc_ptr can be re-inserted
    assert(new_proc_dom.insert(proc_ptr).len() == new_proc_dom.len());
}

// Test 3: The spec ensures containers_tree_unchanged, but does NOT
// guarantee that ALL container fields are unchanged (only tree fields).
// Claim: owned_procs is also unchanged (not guaranteed by containers_tree_unchanged).
// SHOULD FAIL
proof fn test_logical_container_owned_procs_unchanged(
    old_owned_procs: Seq<ProcPtr>,
    new_owned_procs: Seq<ProcPtr>,
    proc_ptr: ProcPtr,
)
    requires
        old_owned_procs.contains(proc_ptr),
        // containers_tree_unchanged does NOT cover owned_procs
        new_owned_procs =~= old_owned_procs.remove_value(proc_ptr),
{
    // Incorrectly assume owned_procs unchanged
    assert(new_owned_procs =~= old_owned_procs);
}

// Test 4: The spec does NOT guarantee determinism about which page is freed.
// Two calls with same proc_ptr should NOT necessarily return same page_ptr.
// SHOULD FAIL
proof fn test_logical_free_page_determinism(
    page_ptr1: PagePtr,
    page_ptr2: PagePtr,
    proc_closure: Set<PagePtr>,
)
    requires
        proc_closure.contains(page_ptr1),
        proc_closure.contains(page_ptr2),
        proc_closure.len() > 1,
{
    assert(page_ptr1 == page_ptr2);
}

// Test 5: The spec does NOT guarantee that proc_dom becomes empty
// after killing one process. Claim: proc_dom is empty.
// SHOULD FAIL
proof fn test_logical_proc_dom_empty_after_kill(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
    proc_ptr: ProcPtr,
)
    requires
        old_proc_dom.contains(proc_ptr),
        old_proc_dom.len() > 1,
        old_proc_dom.finite(),
        new_proc_dom =~= old_proc_dom.remove(proc_ptr),
{
    assert(new_proc_dom =~= Set::<ProcPtr>::empty());
}

// Test 6: The spec preserves pagetable mappings for remaining procs.
// But it does NOT guarantee that the killed proc's pagetable data
// is completely erased. Claim: all pagetables are None.
// SHOULD FAIL
proof fn test_logical_all_pagetables_none(
    pcid_active: bool,
    other_pcid_active: bool,
)
    requires
        // After kill, the killed proc's pcid becomes inactive
        pcid_active == false,
        // But other procs' pcids remain active
        other_pcid_active == true,
{
    // Incorrectly claim all pcids are inactive
    assert(other_pcid_active == false);
}

// Test 7: The spec does NOT guarantee ordering of proc_dom elements.
// Claim: removing a proc preserves some ordering property.
// SHOULD FAIL
proof fn test_logical_proc_dom_ordering(
    old_dom: Set<ProcPtr>,
    new_dom: Set<ProcPtr>,
    proc_ptr: ProcPtr,
    other_ptr: ProcPtr,
)
    requires
        old_dom.contains(proc_ptr),
        old_dom.contains(other_ptr),
        proc_ptr != other_ptr,
        new_dom =~= old_dom.remove(proc_ptr),
        proc_ptr > other_ptr,
{
    // No ordering guarantee — claim other_ptr > proc_ptr
    assert(other_ptr > proc_ptr);
}

// Test 8: Cross-function: kill_process_none_root returns a page and perm.
// The spec says ret.0 == ret.1@.addr() and ret.1@.is_init().
// But it does NOT say the page is zero-initialized or has specific content.
// Claim: the freed page is always at address 0.
// SHOULD FAIL
proof fn test_logical_freed_page_always_zero(
    ret_page: PagePtr,
    old_closure: Set<PagePtr>,
)
    requires
        old_closure.contains(ret_page),
        page_ptr_valid(ret_page),
{
    assert(ret_page == 0);
}

// Test 9: The spec ensures threads_unchanged_except with empty changed set,
// meaning ALL threads unchanged. But this does NOT mean thread endpoints
// or scheduling state is preserved across different kernel operations.
// Claim: thread count == 0 after kill (not guaranteed).
// SHOULD FAIL
proof fn test_logical_thread_count_zero(
    thread_dom: Set<ThreadPtr>,
)
    requires
        thread_dom.len() > 0,
        thread_dom.finite(),
{
    assert(thread_dom.len() == 0);
}

// Test 10: The spec says depth != 0 is required but does NOT bound
// depth to any maximum. Claim: depth must be <= 1.
// SHOULD FAIL
proof fn test_logical_depth_bounded() {
    let depth: usize = 5;
    assert(depth != 0);  // passes: depth is non-zero
    assert(depth <= 1);  // should fail: no upper bound on depth
}

}
