use vstd::prelude::*;

fn main() {}

verus! {

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
}

// Proof-level abstraction of endpoints_contain postcondition.
#[verifier::external_body]
proof fn endpoints_contain_spec(endpoints: Seq<AbstractEndPoint>, endpoint: AbstractEndPoint) -> (present: bool)
    ensures present == endpoints.contains(endpoint)
{ unimplemented!() }

// Proof-level abstraction of do_end_points_match postcondition.
#[verifier::external_body]
proof fn do_end_points_match_spec(e1: AbstractEndPoint, e2: AbstractEndPoint) -> (eq: bool)
    ensures eq == (e1 == e2)
{ unimplemented!() }

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Endpoint IS the sole element in the list — mutate result to absent.
// contains must return true for an element that is in the list.
// SHOULD FAIL
proof fn test_mutation_present_claims_absent()
{
    let ep = AbstractEndPoint { id: seq![1u8, 2u8] };
    let endpoints: Seq<AbstractEndPoint> = seq![ep];
    let present = endpoints_contain_spec(endpoints, ep);
    assert(endpoints[0] == ep); // trigger: ep is at index 0
    assert(!present);
}

// Test 2: Endpoint at last position in multi-element list — mutate to absent.
// SHOULD FAIL
proof fn test_mutation_last_position_claims_absent()
{
    let ep1 = AbstractEndPoint { id: seq![1u8] };
    let ep2 = AbstractEndPoint { id: seq![2u8] };
    let endpoints: Seq<AbstractEndPoint> = seq![ep1, ep2];
    let present = endpoints_contain_spec(endpoints, ep2);
    assert(endpoints[1] == ep2); // trigger: ep2 is at index 1
    assert(!present);
}

// Test 3: Two distinct endpoints — mutate match result to claim they are equal.
// do_end_points_match must return false for endpoints with different ids.
// SHOULD FAIL
proof fn test_mutation_distinct_claim_match()
{
    let e1 = AbstractEndPoint { id: seq![1u8] };
    let e2 = AbstractEndPoint { id: seq![2u8] };
    let eq = do_end_points_match_spec(e1, e2);
    assert(eq);
}

}
