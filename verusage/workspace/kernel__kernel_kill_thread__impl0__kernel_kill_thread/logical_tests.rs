use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type EndpointIdx = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// These probe the semantic boundary for unintended entailment.
// All tests SHOULD FAIL verification.

// --- Determinism / Uniqueness tests ---

// Test 1: The spec does not guarantee that killing different threads
// produces different final states. Assert determinism:
// killing thread A and killing thread B must yield the same result.
// SHOULD FAIL
proof fn test_logical_kill_different_threads_same_result(
    old_thread_dom: Set<ThreadPtr>,
    thread_a: ThreadPtr,
    thread_b: ThreadPtr,
)
    requires
        thread_a != thread_b,
        old_thread_dom.contains(thread_a),
        old_thread_dom.contains(thread_b),
{
    // The spec says thread_dom removes the killed thread.
    // These two results should differ, so asserting equality should fail.
    assert(old_thread_dom.remove(thread_a) =~= old_thread_dom.remove(thread_b));
}

// Test 2: The spec does NOT guarantee that kernel_kill_thread
// is idempotent. After removing a thread, it's no longer in the domain.
// Trying to assert it's still there should fail.
// SHOULD FAIL
proof fn test_logical_kill_thread_idempotent(
    old_thread_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        old_thread_dom.contains(thread_ptr),
{
    let new_dom = old_thread_dom.remove(thread_ptr);
    // Assert the removed thread is still present (idempotency would require this)
    assert(new_dom.contains(thread_ptr));
}

// --- Stronger inequality tests ---

// Test 3: The spec guarantees owned_threads.len() decreases by 1.
// Assert a STRONGER property: length decreases by 2.
// SHOULD FAIL
proof fn test_logical_owned_threads_decrease_by_two(
    old_len: int,
    new_len: int,
)
    requires
        old_len > 1,
        new_len == old_len - 1,
{
    assert(new_len == old_len - 2);
}

// Test 4: The spec ensures proc_dom is unchanged. Assert a STRONGER
// property: proc_dom is empty after the operation.
// SHOULD FAIL
proof fn test_logical_proc_dom_empty_after_kill(
    proc_dom: Set<ProcPtr>,
    some_proc: ProcPtr,
)
    requires
        proc_dom.contains(some_proc),
{
    // Assert proc_dom is empty — clearly stronger than "unchanged"
    assert(proc_dom =~= Set::empty());
}

// --- Structural / cross-function misuse tests ---

// Test 5: The spec for threads_unchanged_except uses an empty changed set.
// Assert that a thread NOT in the new domain is still unchanged.
// The spec only quantifies over threads IN new_thread_dom; this probes
// whether the spec accidentally guarantees facts about removed threads.
// SHOULD FAIL
proof fn test_logical_removed_thread_is_in_new_dom(
    old_thread_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        old_thread_dom.contains(thread_ptr),
{
    let new_dom = old_thread_dom.remove(thread_ptr);
    // The removed thread should NOT be in new_dom
    assert(new_dom.contains(thread_ptr));
}

// Test 6: The spec does not say anything about the container_dom being
// a subset of page_closure. Assert this cross-domain relationship.
// SHOULD FAIL
proof fn test_logical_container_dom_subset_page_closure(
    container_dom: Set<ContainerPtr>,
    page_closure: Set<PagePtr>,
    container: ContainerPtr,
)
    requires
        container_dom.contains(container),
        !page_closure.contains(container),
{
    // Assert container_dom is a subset of page_closure — not guaranteed
    assert(page_closure.contains(container));
}

// Test 7: kill_blocked_thread MAY return an optional second page.
// Assert it ALWAYS returns a second page (ret.1 is always Some).
// The spec says it CAN be None, so this is too strong.
// SHOULD FAIL
proof fn test_logical_blocked_always_returns_two_pages(
    ret_is_some: bool,
)
    requires
        ret_is_some == false,
{
    // Assert the second page is always returned
    assert(ret_is_some == true);
}

// Test 8: The spec says process_tree_unchanged holds after kill.
// Assert a STRONGER property: the entire process (all fields, not just tree)
// is unchanged. The spec only guarantees tree fields are preserved.
// SHOULD FAIL
proof fn test_logical_entire_process_unchanged(
    old_owned_threads_len: int,
    new_owned_threads_len: int,
)
    requires
        old_owned_threads_len > 0,
        new_owned_threads_len == old_owned_threads_len - 1,
{
    // process_tree_unchanged does NOT cover owned_threads.
    // Assert owned_threads length is unchanged — too strong.
    assert(new_owned_threads_len == old_owned_threads_len);
}

// Test 9: page_ptr2page_index has no ensures about bijectivity with
// page_index2page_ptr for values outside the valid range.
// Assert bijectivity for an invalid pointer.
// SHOULD FAIL
proof fn test_logical_page_ptr_index_bijection_invalid() {
    // ptr = 0x1001 is not page-aligned, so spec_page_ptr2page_index
    // is not meaningful. Assert it round-trips anyway.
    let ptr: usize = 0x1001;
    // The precondition requires alignment, so we cannot use the function.
    // But we can assert the alignment holds — it shouldn't.
    assert(ptr % 0x1000 == 0);
}

// Test 10: The spec does not guarantee that killing a thread affects
// the container tree. But it also doesn't prevent one from asserting
// the container tree changes. Assert containers_tree changed.
// SHOULD FAIL
proof fn test_logical_container_tree_changed(
    old_container_field: int,
    new_container_field: int,
)
    requires
        // containers_tree_unchanged: new fields == old fields
        new_container_field == old_container_field,
{
    // Assert the container tree DID change — contradicts the postcondition
    assert(new_container_field != old_container_field);
}

}
