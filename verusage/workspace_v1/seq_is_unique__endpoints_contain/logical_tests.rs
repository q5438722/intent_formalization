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

// ========== LOGICAL TESTS ==========

// Test 1: Contains does NOT guarantee element is at index 0.
// ep2 is at index 1, not 0. Claiming it must be at index 0 is too strong.
// SHOULD FAIL
proof fn test_logical_must_be_at_index_zero()
{
    let ep1 = AbstractEndPoint { id: seq![1u8] };
    let ep2 = AbstractEndPoint { id: seq![2u8] };
    let endpoints: Seq<AbstractEndPoint> = seq![ep1, ep2];
    let present = endpoints_contain_spec(endpoints, ep2);
    assert(endpoints[1] == ep2); // trigger: establish contains is true
    assert(present ==> endpoints[0] == ep2);
}

// Test 2: Two different lists both containing the same element does NOT
// imply the lists are equal. Membership is not injective on containers.
// SHOULD FAIL
proof fn test_logical_contains_implies_list_equality()
{
    let ep = AbstractEndPoint { id: seq![1u8] };
    let extra = AbstractEndPoint { id: seq![2u8] };
    let list_a: Seq<AbstractEndPoint> = seq![ep];
    let list_b: Seq<AbstractEndPoint> = seq![ep, extra];
    let pa = endpoints_contain_spec(list_a, ep);
    let pb = endpoints_contain_spec(list_b, ep);
    assert(list_a[0] == ep);  // trigger for pa
    assert(list_b[0] == ep);  // trigger for pb
    assert(pa && pb ==> list_a =~= list_b);
}

// Test 3: Contains does NOT imply ALL elements equal the target.
// Only one element needs to match for contains to be true.
// SHOULD FAIL
proof fn test_logical_contains_implies_all_equal()
{
    let ep1 = AbstractEndPoint { id: seq![1u8] };
    let ep2 = AbstractEndPoint { id: seq![2u8] };
    let endpoints: Seq<AbstractEndPoint> = seq![ep1, ep2];
    let present = endpoints_contain_spec(endpoints, ep1);
    assert(endpoints[0] == ep1); // trigger: establish contains is true
    assert(present ==> (forall |i: int| 0 <= i < endpoints.len() ==> endpoints[i] == ep1));
}

}
