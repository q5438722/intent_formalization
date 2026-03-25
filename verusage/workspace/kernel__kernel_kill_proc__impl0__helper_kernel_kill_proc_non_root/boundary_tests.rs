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

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition (requires clause) of
// helper_kernel_kill_proc_non_root or uses edge-case values.
// All tests SHOULD FAIL verification.

// Test 1: helper_kernel_kill_proc_non_root requires proc_dom().contains(proc_ptr).
// If proc_ptr is NOT in the domain, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_proc_not_in_domain(
    proc_dom: Set<ProcPtr>,
    proc_ptr: ProcPtr,
)
    requires
        !proc_dom.contains(proc_ptr),
{
    assert(proc_dom.contains(proc_ptr));
}

// Test 2: helper_kernel_kill_proc_non_root requires depth != 0 (non-root).
// If depth == 0, the process is root and should be rejected.
// SHOULD FAIL
proof fn test_boundary_depth_is_zero() {
    let depth: usize = 0;
    assert(depth != 0);
}

// Test 3: helper_kernel_kill_proc_non_root requires children@ == Seq::empty().
// If children is non-empty, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_children_not_empty(
    children: Seq<ProcPtr>,
    child: ProcPtr,
)
    requires
        children == Seq::<ProcPtr>::empty().push(child),
{
    assert(children == Seq::<ProcPtr>::empty());
}

// Test 4: helper_kernel_kill_proc_non_root requires owned_threads@ == Seq::empty().
// If owned_threads is non-empty, precondition is violated.
// SHOULD FAIL
proof fn test_boundary_threads_not_empty(
    owned_threads: Seq<ThreadPtr>,
    thread: ThreadPtr,
)
    requires
        owned_threads == Seq::<ThreadPtr>::empty().push(thread),
{
    assert(owned_threads == Seq::<ThreadPtr>::empty());
}

// Test 5: helper_kernel_kill_proc_non_root requires ioid.is_None().
// If ioid is Some, the precondition is violated.
// SHOULD FAIL
proof fn test_boundary_ioid_is_some() {
    let ioid: Option<IOid> = Some(42usize);
    assert(ioid.is_None());
}

// Test 6: The spec implicitly requires parent.is_Some() (parent.unwrap() used in ensures).
// Depth != 0 implies non-root, so parent must exist.
// Try with parent == None — the unwrap would be invalid.
// SHOULD FAIL
proof fn test_boundary_parent_is_none() {
    let parent: Option<ProcPtr> = None;
    // If depth != 0, parent must be Some for unwrap to succeed
    assert(parent.is_Some());
}

// Test 7: proc_dom must contain the parent for the ensures clause to make sense.
// If parent is not in proc_dom, the postcondition about parent's children is meaningless.
// SHOULD FAIL
proof fn test_boundary_parent_not_in_domain(
    proc_dom: Set<ProcPtr>,
    parent_ptr: ProcPtr,
)
    requires
        !proc_dom.contains(parent_ptr),
{
    // The spec accesses get_proc(parent_ptr), which requires parent in domain
    assert(proc_dom.contains(parent_ptr));
}

// Test 8: page_ptr2page_index requires ptr % 0x1000 == 0.
// Passing ptr = 1 violates the alignment precondition.
// SHOULD FAIL
proof fn test_boundary_page_ptr_not_aligned() {
    let ptr: usize = 1;
    assert(ptr % 0x1000 == 0);
}

}
