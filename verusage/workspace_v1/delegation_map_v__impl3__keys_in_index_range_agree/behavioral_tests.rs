use vstd::prelude::*;

fn main() {}

verus! {

// Proof-level abstraction of keys_in_index_range_agree postconditions.
// vals[i] models self@[self.keys@[i]]@ from the original specification.
// v models the comparison endpoint's view v@.
#[verifier::external_body]
proof fn keys_in_index_range_agree_spec(
    vals: Seq<int>,
    lo: int,
    hi: int,
    v: int,
) -> (ret: (bool, bool))
    requires
        0 <= lo <= hi < vals.len(),
    ensures
        ret.0 == (forall |i: int| #![auto] lo <= i <= hi ==> vals[i] == v),
        !ret.0 ==> (ret.1 == (vals[hi] != v && forall |i: int| #![auto] lo <= i < hi ==> vals[i] == v)),
{ unimplemented!() }

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: All values agree with v, but assert ret.0 is false (negate agreement).
// vals = [1, 1, 1], v = 1: all match, so ret.0 must be true.
// Asserting the negation should fail.
// SHOULD FAIL
proof fn test_behavioral_negate_agreement()
{
    let vals: Seq<int> = seq![1int, 1, 1];
    let ret = keys_in_index_range_agree_spec(vals, 0, 2, 1);
    assert(!ret.0);
}

// Test 2: Not all values agree, but assert ret.0 is true (false agreement).
// vals = [1, 1, 2], v = 1: vals[2] != 1, so ret.0 must be false.
// Asserting it is true should fail.
// SHOULD FAIL
proof fn test_behavioral_false_agreement()
{
    let vals: Seq<int> = seq![1int, 1, 2];
    let ret = keys_in_index_range_agree_spec(vals, 0, 2, 1);
    assert(ret.0);
}

// Test 3: "Almost" holds (hi disagrees, rest agree), but assert ret.1 is false.
// vals = [1, 1, 2], v = 1: vals[2] != 1 and vals[0..2) all == 1,
// so ret.1 must be true. Asserting false should fail.
// SHOULD FAIL
proof fn test_behavioral_negate_almost()
{
    let vals: Seq<int> = seq![1int, 1, 2];
    let ret = keys_in_index_range_agree_spec(vals, 0, 2, 1);
    assert(!ret.1);
}

}
