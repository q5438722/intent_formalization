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

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid postconditions of
// helper_kernel_kill_proc_root, then mutates the expected output.
// All tests SHOULD FAIL verification.

// Test 1: The spec ensures proc_dom removes proc_ptr.
// Mutate: claim proc_dom is unchanged after killing.
// SHOULD FAIL
proof fn test_mutation_proc_dom_unchanged(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
    proc_ptr: ProcPtr,
)
    requires
        old_proc_dom.contains(proc_ptr),
        new_proc_dom =~= old_proc_dom.remove(proc_ptr),
{
    assert(new_proc_dom =~= old_proc_dom);
}

// Test 2: The spec ensures thread_dom is unchanged.
// Mutate: claim thread_dom loses a thread.
// SHOULD FAIL
proof fn test_mutation_thread_dom_shrinks(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
    some_thread: ThreadPtr,
)
    requires
        new_thread_dom =~= old_thread_dom,
        old_thread_dom.contains(some_thread),
{
    assert(!new_thread_dom.contains(some_thread));
}

// Test 3: The spec ensures container_dom is unchanged.
// Mutate: claim container_dom changes (loses a container).
// SHOULD FAIL
proof fn test_mutation_container_dom_changed(
    old_container_dom: Set<ContainerPtr>,
    new_container_dom: Set<ContainerPtr>,
    some_container: ContainerPtr,
)
    requires
        new_container_dom =~= old_container_dom,
        old_container_dom.contains(some_container),
{
    assert(!new_container_dom.contains(some_container));
}

// Test 4: The spec ensures processes_fields_unchanged — pcid unchanged.
// Mutate: claim pcid of a remaining process changes.
// SHOULD FAIL
proof fn test_mutation_pcid_changed(
    old_pcid: Pcid,
    new_pcid: Pcid,
)
    requires
        new_pcid =~= old_pcid,
{
    assert(new_pcid != old_pcid);
}

// Test 5: The spec ensures children unchanged for remaining procs.
// Mutate: claim a remaining process's children list gains a child.
// SHOULD FAIL
proof fn test_mutation_children_gains_child(
    old_children: Seq<ProcPtr>,
    new_children: Seq<ProcPtr>,
    extra_child: ProcPtr,
)
    requires
        new_children =~= old_children,
{
    assert(new_children =~= old_children.push(extra_child));
}

// Test 6: The spec ensures uppertree_seq unchanged for remaining procs.
// Mutate: claim uppertree_seq changes for a remaining proc.
// SHOULD FAIL
proof fn test_mutation_uppertree_seq_changed(
    old_seq: Seq<ProcPtr>,
    new_seq: Seq<ProcPtr>,
    extra: ProcPtr,
)
    requires
        new_seq =~= old_seq,
{
    assert(new_seq =~= old_seq.push(extra));
}

// Test 7: The spec ensures subtree_set updated for upper-tree procs:
// subtree_set removes proc_ptr. Mutate: claim subtree_set is unchanged.
// SHOULD FAIL
proof fn test_mutation_subtree_set_unchanged(
    old_subtree: Set<ProcPtr>,
    new_subtree: Set<ProcPtr>,
    proc_ptr: ProcPtr,
)
    requires
        old_subtree.contains(proc_ptr),
        new_subtree =~= old_subtree.remove(proc_ptr),
{
    assert(new_subtree =~= old_subtree);
}

// Test 8: The spec ensures processes_fields_unchanged — depth unchanged.
// Mutate: claim depth of a remaining process changes.
// SHOULD FAIL
proof fn test_mutation_depth_changed(
    old_depth: usize,
    new_depth: usize,
)
    requires
        new_depth =~= old_depth,
{
    assert(new_depth == old_depth + 1);
}

}
