use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type ContainerPtr = usize;

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// These probe the semantic boundary for unintended entailment.
// All tests SHOULD FAIL verification.

// --- Determinism tests ---

// Test 1: The spec does not guarantee that killing all threads of
// two different procs produces the same final thread_dom.
// SHOULD FAIL
proof fn test_logical_kill_all_threads_different_procs_same_dom(
    thread_dom: Set<ThreadPtr>,
    threads_of_a: Set<ThreadPtr>,
    threads_of_b: Set<ThreadPtr>,
    proc_a: ProcPtr,
    proc_b: ProcPtr,
)
    requires
        proc_a != proc_b,
        threads_of_a.len() > 0,
        threads_of_b.len() > 0,
        threads_of_a.disjoint(threads_of_b),
        thread_dom.subset_of(threads_of_a + threads_of_b),
{
    // After killing all threads of proc_a, threads_of_a are gone.
    // After killing all threads of proc_b, threads_of_b are gone.
    // Asserting they produce the same result is incorrect.
    let dom_after_a = thread_dom.difference(threads_of_a);
    let dom_after_b = thread_dom.difference(threads_of_b);
    assert(dom_after_a =~= dom_after_b);
}

// Test 2: The spec does NOT guarantee that the thread_dom is empty
// after killing all threads of ONE process. Other processes' threads
// may still exist.
// SHOULD FAIL
proof fn test_logical_thread_dom_empty_after(
    thread_dom: Set<ThreadPtr>,
    proc_threads: Set<ThreadPtr>,
    other_thread: ThreadPtr,
)
    requires
        proc_threads.len() > 0,
        !proc_threads.contains(other_thread),
        thread_dom.contains(other_thread),
{
    // The spec only kills threads owned by the proc, not all threads
    let dom_after = thread_dom.difference(proc_threads);
    assert(dom_after =~= Set::<ThreadPtr>::empty());
}

// --- Stronger inequality tests ---

// Test 3: The spec guarantees owned_threads.len() == 0 after.
// Assert a STRONGER property: the process has no resources at all
// (e.g., no children procs). This is not guaranteed.
// SHOULD FAIL
proof fn test_logical_process_has_no_children_after(
    children_len: int,
)
    requires
        children_len > 0,
{
    // Killing all threads does NOT remove child processes
    assert(children_len == 0);
}

// Test 4: The spec guarantees proc_dom is unchanged.
// Assert a STRONGER property: proc_dom is a singleton.
// SHOULD FAIL
proof fn test_logical_proc_dom_singleton_after(
    proc_dom: Set<ProcPtr>,
    proc_a: ProcPtr,
    proc_b: ProcPtr,
)
    requires
        proc_a != proc_b,
        proc_dom.contains(proc_a),
        proc_dom.contains(proc_b),
{
    // proc_dom has at least 2 elements; assert it's a singleton
    assert(proc_dom =~= set![proc_a]);
}

// --- Cross-function misuse tests ---

// Test 5: kernel_proc_kill_all_threads does not specify anything
// about page_closure or memory state. Assert that page_closure
// is empty after (unwarranted cross-domain claim).
// SHOULD FAIL
proof fn test_logical_page_closure_empty_after(
    page_closure: Set<usize>,
    some_page: usize,
)
    requires
        page_closure.contains(some_page),
{
    assert(page_closure =~= Set::<usize>::empty());
}

// Test 6: The spec does not guarantee that the process's pcid
// changes or is invalidated after killing all threads.
// Assert pcid becomes 0 (unwarranted).
// SHOULD FAIL
proof fn test_logical_pcid_reset_after(
    old_pcid: usize,
    new_pcid: usize,
)
    requires
        old_pcid > 0,
        new_pcid == old_pcid,
{
    // pcid should NOT change — but asserting it resets to 0 is unwarranted
    assert(new_pcid == 0);
}

// Test 7: The spec ensures process_tree_unchanged. This covers
// parent, children, uppertree_seq, subtree_set, depth.
// It does NOT cover owned_threads or pcid.
// Assert that owned_threads is also unchanged (too strong).
// SHOULD FAIL
proof fn test_logical_owned_threads_unchanged_too_strong(
    old_owned_threads_len: int,
    new_owned_threads_len: int,
)
    requires
        old_owned_threads_len > 0,
        new_owned_threads_len == 0,
{
    // process_tree_unchanged does NOT include owned_threads
    assert(new_owned_threads_len == old_owned_threads_len);
}

// --- Global assumption tests ---

// Test 8: The spec does not say the loop terminates in bounded time
// or that the number of iterations is bounded by MAX_NUM_THREADS_PER_PROC.
// Assert num_threads <= some small bound.
// SHOULD FAIL
proof fn test_logical_thread_count_bounded_small(
    num_threads: int,
)
    requires
        num_threads == 100,
{
    // Spec allows up to MAX_NUM_THREADS_PER_PROC (128) threads
    // Asserting a tighter bound is unwarranted
    assert(num_threads <= 10);
}

// Test 9: The spec ensures containers_tree_unchanged, which
// preserves container fields. Assert that container_dom GROWS
// (i.e., a new container appears) — not guaranteed and contradicts
// containers_tree_unchanged + container_dom equality.
// SHOULD FAIL
proof fn test_logical_container_dom_grows(
    old_container_dom: Set<ContainerPtr>,
    new_container_dom: Set<ContainerPtr>,
    new_container: ContainerPtr,
)
    requires
        !old_container_dom.contains(new_container),
        new_container_dom =~= old_container_dom,
{
    // container_dom is unchanged, so new containers cannot appear
    assert(new_container_dom.contains(new_container));
}

// Test 10: The spec does not order the thread killing.
// Assert that the first killed thread is always the smallest pointer.
// The spec uses get_head() which returns the head of the linked list,
// not necessarily the smallest value.
// SHOULD FAIL
proof fn test_logical_threads_killed_in_order(
    thread_a: ThreadPtr,
    thread_b: ThreadPtr,
    head: ThreadPtr,
)
    requires
        thread_a < thread_b,
        head == thread_b,
{
    // get_head returns the list head, not the min pointer
    assert(head == thread_a);
}

}
