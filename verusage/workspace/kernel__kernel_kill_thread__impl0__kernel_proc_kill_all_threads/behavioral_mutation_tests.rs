use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs and mutates expected outputs.
// All tests SHOULD FAIL verification.

// --- kernel_proc_kill_all_threads postcondition mutations ---

// Test 1: The spec ensures owned_threads.len() == 0 after killing all threads.
// Mutant: owned_threads.len() == 1 (not all threads killed).
// SHOULD FAIL
proof fn test_mutation_not_all_threads_killed(
    old_len: int,
    new_len: int,
)
    requires
        old_len > 0,
        new_len == 0,
{
    // Mutated: assert one thread survives
    assert(new_len == 1);
}

// Test 2: The spec ensures proc_dom is unchanged after the operation.
// Mutant: proc_dom loses the target process.
// SHOULD FAIL
proof fn test_mutation_proc_removed(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
    proc_ptr: ProcPtr,
)
    requires
        old_proc_dom.contains(proc_ptr),
        new_proc_dom =~= old_proc_dom,
{
    // Mutated: assert proc was removed
    assert(new_proc_dom =~= old_proc_dom.remove(proc_ptr));
}

// Test 3: The spec ensures self.proc_dom().contains(proc_ptr) after.
// Mutant: proc_ptr is NOT in proc_dom after.
// SHOULD FAIL
proof fn test_mutation_proc_not_in_dom_after(
    proc_dom: Set<ProcPtr>,
    proc_ptr: ProcPtr,
)
    requires
        proc_dom.contains(proc_ptr),
{
    // Mutated: proc_ptr removed from domain
    assert(!proc_dom.contains(proc_ptr));
}

// Test 4: The spec ensures container_dom is unchanged.
// Mutant: container_dom is empty after.
// SHOULD FAIL
proof fn test_mutation_container_dom_cleared(
    old_container_dom: Set<ContainerPtr>,
    new_container_dom: Set<ContainerPtr>,
    some_container: ContainerPtr,
)
    requires
        old_container_dom.contains(some_container),
        new_container_dom =~= old_container_dom,
{
    // Mutated: container_dom became empty
    assert(new_container_dom =~= Set::<ContainerPtr>::empty());
}

// Test 5: The spec ensures process_tree_unchanged holds.
// Specifically, parent fields must be preserved.
// Mutant: parent field changed to a different value.
// SHOULD FAIL
proof fn test_mutation_process_parent_changed(
    old_parent: Option<ProcPtr>,
    new_parent: Option<ProcPtr>,
    different_parent: ProcPtr,
)
    requires
        // process_tree_unchanged: parent preserved
        new_parent =~= old_parent,
        old_parent =~= Some(different_parent),
{
    // Mutated: parent changed to None
    assert(new_parent =~= None);
}

// Test 6: The spec ensures process_tree_unchanged.
// Specifically, depth must be preserved.
// Mutant: depth changed.
// SHOULD FAIL
proof fn test_mutation_process_depth_changed(
    old_depth: usize,
    new_depth: usize,
)
    requires
        old_depth > 0,
        new_depth == old_depth,
{
    // Mutated: depth incremented
    assert(new_depth == old_depth + 1);
}

// Test 7: The spec ensures containers_tree_unchanged.
// Mutant: a container's children field changed.
// SHOULD FAIL
proof fn test_mutation_container_children_changed(
    old_children: Seq<ContainerPtr>,
    new_children: Seq<ContainerPtr>,
)
    requires
        new_children =~= old_children,
        old_children.len() > 0,
{
    // Mutated: children list lost an element
    assert(new_children.len() == old_children.len() - 1);
}

// Test 8: The spec ensures self.wf() after the operation.
// Mutant: wf is false after.
// SHOULD FAIL
proof fn test_mutation_wf_false_after(wf_after: bool)
    requires
        wf_after == true,
{
    // Mutated: wf is false after the operation
    assert(wf_after == false);
}

// Test 9: The loop invariant ensures owned_threads.len() decreases
// by exactly 1 per iteration: len == num_threads - i.
// Mutant: len decreases by 2 per iteration.
// SHOULD FAIL
proof fn test_mutation_threads_decrease_by_two(
    num_threads: int,
    i: int,
    actual_len: int,
)
    requires
        num_threads == 5,
        i == 1,
        actual_len == num_threads - i,
{
    // Mutated: len decreases by 2 instead of 1
    assert(actual_len == num_threads - 2 * i);
}

// Test 10: containers_tree_unchanged ensures subtree_set is preserved.
// Mutant: subtree_set gained an extra element.
// SHOULD FAIL
proof fn test_mutation_container_subtree_set_grew(
    old_set: Set<ContainerPtr>,
    new_set: Set<ContainerPtr>,
    extra: ContainerPtr,
)
    requires
        new_set =~= old_set,
        !old_set.contains(extra),
{
    // Mutated: subtree gained an extra element
    assert(new_set.contains(extra));
}

}
