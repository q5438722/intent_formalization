use vstd::prelude::*;

fn main() {}

verus!{

pub type ThreadPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type ProcPtr = usize;
pub type PagePtr = usize;

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by
// the kernel_drop_endpoint specification. These probe for
// unintended reasoning: stronger invariants, determinism,
// cross-function misuse, and structural assumptions.
// All tests SHOULD FAIL verification.

// Test 1: The spec does NOT guarantee that endpoint_dom shrinks after
// dropping an endpoint descriptor. endpoint_dom is about which
// EndpointPtr are in the process manager, NOT about descriptors.
// Claim: endpoint_dom shrank by one element.
// SHOULD FAIL
proof fn test_logical_endpoint_dom_shrinks(
    old_endpoint_dom: Set<EndpointPtr>,
    new_endpoint_dom: Set<EndpointPtr>,
    dropped_ep: EndpointPtr,
)
    requires
        old_endpoint_dom.contains(dropped_ep),
        // We only know thread_dom/proc_dom/container_dom are preserved,
        // NOT that endpoint_dom is unchanged or shrinks.
        new_endpoint_dom.len() <= old_endpoint_dom.len(),
        old_endpoint_dom.finite(),
        new_endpoint_dom.finite(),
{
    // Claim the dropped endpoint is no longer in the domain.
    // The spec doesn't guarantee this—the Endpoint object may still exist.
    assert(!new_endpoint_dom.contains(dropped_ep));
}

// Test 2: The spec does NOT guarantee that ALL endpoint descriptors
// OTHER than edp_idx remain unchanged. The spec only says the
// full array equals old.update(edp_idx, None). Actually that DOES
// imply others are unchanged. So this test: claim a DIFFERENT
// descriptor also became None (even if it was Some before).
// We set up: old[other_idx] was Some, new = old.update(edp_idx, None),
// then claim new[other_idx] is None. Since update only changes edp_idx,
// this should fail.
// SHOULD FAIL
proof fn test_logical_other_descriptor_also_cleared(
    old_descriptors: Seq<Option<EndpointPtr>>,
    new_descriptors: Seq<Option<EndpointPtr>>,
    edp_idx: int,
    other_idx: int,
    some_ep: EndpointPtr,
)
    requires
        0 <= edp_idx < MAX_NUM_ENDPOINT_DESCRIPTORS as int,
        0 <= other_idx < MAX_NUM_ENDPOINT_DESCRIPTORS as int,
        edp_idx != other_idx,
        old_descriptors.len() == MAX_NUM_ENDPOINT_DESCRIPTORS as int,
        old_descriptors[other_idx] == Some(some_ep),
        new_descriptors =~= old_descriptors.update(edp_idx, None),
{
    // Claim: the other descriptor also became None.
    assert(new_descriptors[other_idx].is_None());
}

// Test 3: The spec does NOT say dropping endpoint is idempotent.
// If the descriptor at edp_idx is already None, the spec still
// requires the function to be callable but doesn't say the result
// is identical. Claim: dropping a None descriptor produces exactly
// the same state (which IS trivially true for the descriptor array,
// but NOT necessarily for page_closure or other state).
// We test: if old descriptor is None, new page_closure == old page_closure.
// This is NOT guaranteed—drop_endpoint may still return Some page.
// SHOULD FAIL
proof fn test_logical_idempotent_drop(
    old_page_closure: Set<PagePtr>,
    new_page_closure: Set<PagePtr>,
    old_descriptor: Option<EndpointPtr>,
)
    requires
        old_descriptor.is_None(),
        // We don't constrain new_page_closure to equal old_page_closure
        // because the spec doesn't guarantee this when descriptor is None.
        old_page_closure.finite(),
        new_page_closure.finite(),
        old_page_closure.len() > 0,
        new_page_closure.len() > 0,
{
    // Claim page_closure is strictly identical—not guaranteed by the spec.
    assert(old_page_closure =~= new_page_closure);
}

// Test 4: The spec does NOT guarantee that kernel_drop_endpoint is
// deterministic. Two calls with identical states should produce
// identical results—but the spec doesn't explicitly state this.
// SHOULD FAIL
proof fn test_logical_determinism(
    ret1_page: Option<PagePtr>,
    ret2_page: Option<PagePtr>,
)
    requires
        ret1_page.is_Some(),
        ret2_page.is_Some(),
{
    // Claim: the two results are identical page pointers.
    assert(ret1_page.unwrap() == ret2_page.unwrap());
}

// Test 5: The spec does NOT say that the thread_dom size equals a
// specific value. It only says thread_dom is preserved.
// Claim: thread_dom always has at least 2 elements.
// SHOULD FAIL
proof fn test_logical_thread_dom_min_size(
    thread_dom: Set<ThreadPtr>,
    thread_ptr: ThreadPtr,
)
    requires
        thread_dom.contains(thread_ptr),
        thread_dom.finite(),
{
    // Claim: there are at least 2 threads. Not guaranteed.
    assert(thread_dom.len() >= 2);
}

// Test 6: The spec says containers_tree_unchanged and
// containers_owned_proc_unchanged, but does NOT say that
// ALL container fields are unchanged. Endpoint-related
// container fields (owned_endpoints, quota, scheduler, etc.)
// may change. Claim: all container data is identical.
// Actually, kernel_drop_endpoint's ensures don't guarantee
// containers_unchanged unconditionally—only except for the
// owning container when ret is Some.
// SHOULD FAIL
proof fn test_logical_all_containers_fully_identical(
    c1_owned_endpoints: Set<EndpointPtr>,
    c2_owned_endpoints: Set<EndpointPtr>,
    some_ep: EndpointPtr,
)
    requires
        c1_owned_endpoints.contains(some_ep),
        c2_owned_endpoints.finite(),
        c1_owned_endpoints.finite(),
{
    // Claim: owned_endpoints is unchanged for all containers.
    // The spec allows the owning container to change.
    assert(c1_owned_endpoints =~= c2_owned_endpoints);
}

// Test 7: The spec does NOT guarantee that edp_idx == 0 is treated
// specially. Claim: dropping descriptor 0 always returns None
// (no page freed). This is not guaranteed.
// SHOULD FAIL
proof fn test_logical_idx_zero_always_none(
    ret: Option<PagePtr>,
    edp_idx: EndpointIdx,
)
    requires
        edp_idx == 0,
        ret.is_Some(),
{
    // Claim: dropping index 0 always returns None.
    assert(ret.is_None());
}

}
