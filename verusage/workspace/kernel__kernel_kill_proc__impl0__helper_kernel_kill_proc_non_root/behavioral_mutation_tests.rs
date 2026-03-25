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
// Each test starts from valid inputs/postconditions of
// helper_kernel_kill_proc_non_root, then mutates expected outputs.
// All tests SHOULD FAIL verification.

// Test 1: The spec ensures proc_dom is reduced by removing proc_ptr.
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
proof fn test_mutation_thread_dom_changed(
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
// Mutate: claim container_dom changes.
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

// Test 4: The spec ensures parent's children list removes proc_ptr.
// Mutate: claim parent's children still contain proc_ptr.
// SHOULD FAIL
proof fn test_mutation_parent_children_still_has_proc(
    old_children: Seq<ProcPtr>,
    new_children: Seq<ProcPtr>,
    proc_ptr: ProcPtr,
)
    requires
        old_children.contains(proc_ptr),
        new_children =~= old_children.remove_value(proc_ptr),
{
    assert(new_children.contains(proc_ptr));
}

// Test 5: The spec ensures parent's children.len() == old.children.len() - 1.
// Mutate: claim children length stays the same.
// SHOULD FAIL
proof fn test_mutation_parent_children_len_unchanged(
    old_len: int,
    new_len: int,
)
    requires
        old_len > 0,
        new_len == old_len - 1,
{
    assert(new_len == old_len);
}

// Test 6: The spec ensures proc_ptr is removed from uppertree proc's subtree_set.
// Mutate: claim subtree_set is unchanged (still contains proc_ptr).
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
    assert(new_subtree.contains(proc_ptr));
}

// Test 7: The spec ensures uppertree_seq is preserved for remaining procs.
// Mutate: claim uppertree_seq changes for a remaining proc.
// SHOULD FAIL
proof fn test_mutation_uppertree_seq_changed(
    old_seq: Seq<ProcPtr>,
    new_seq: Seq<ProcPtr>,
)
    requires
        new_seq =~= old_seq,
        old_seq.len() > 0,
{
    assert(new_seq.len() != old_seq.len());
}

// Test 8: The spec ensures non-parent procs' children are unchanged.
// Mutate: claim a non-parent proc's children changed.
// SHOULD FAIL
proof fn test_mutation_non_parent_children_changed(
    old_children: Seq<ProcPtr>,
    new_children: Seq<ProcPtr>,
    extra: ProcPtr,
)
    requires
        new_children =~= old_children,
{
    assert(new_children =~= old_children.push(extra));
}

// Test 9: The spec ensures wf() holds after kill.
// Mutate: claim wf is false after a valid kill.
// This tests whether invalid states are allowed.
// SHOULD FAIL
proof fn test_mutation_wf_false_after_kill(wf_holds: bool)
    requires
        wf_holds == true,
{
    assert(wf_holds == false);
}

// Test 10: processes_fields_unchanged means pcid/ioid/owned_threads etc. are preserved.
// Mutate: claim pcid changes for a remaining process.
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

}
