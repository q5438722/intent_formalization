use vstd::prelude::*;

fn main() {}

verus! {

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
}

// Proof-level abstraction of endpoints_contain postcondition.
// Body is opaque; only the ensures clause (the spec under test) is trusted.
#[verifier::external_body]
proof fn endpoints_contain_spec(endpoints: Seq<AbstractEndPoint>, endpoint: AbstractEndPoint) -> (present: bool)
    ensures present == endpoints.contains(endpoint)
{ unimplemented!() }

// ========== BOUNDARY TESTS ==========

// Test 1: Empty endpoints list — contains must be false.
// An empty sequence contains nothing, so asserting present should be rejected.
// SHOULD FAIL
proof fn test_boundary_empty_list_claims_present()
{
    let endpoints: Seq<AbstractEndPoint> = Seq::empty();
    let ep = AbstractEndPoint { id: seq![1u8] };
    let present = endpoints_contain_spec(endpoints, ep);
    assert(present);
}

// Test 2: Single-element list where the element does NOT match the target.
// The only element has a different id, so contains must be false.
// SHOULD FAIL
proof fn test_boundary_single_no_match_claims_present()
{
    let ep_in_list = AbstractEndPoint { id: seq![1u8, 2u8] };
    let target = AbstractEndPoint { id: seq![3u8, 4u8] };
    let endpoints: Seq<AbstractEndPoint> = seq![ep_in_list];
    let present = endpoints_contain_spec(endpoints, target);
    assert(present);
}

// Test 3: Multiple elements, none matching the target — contains must be false.
// SHOULD FAIL
proof fn test_boundary_multi_no_match_claims_present()
{
    let ep1 = AbstractEndPoint { id: seq![1u8] };
    let ep2 = AbstractEndPoint { id: seq![2u8] };
    let target = AbstractEndPoint { id: seq![99u8] };
    let endpoints: Seq<AbstractEndPoint> = seq![ep1, ep2];
    let present = endpoints_contain_spec(endpoints, target);
    assert(present);
}

}
