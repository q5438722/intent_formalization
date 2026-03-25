use vstd::prelude::*;

fn main() {}

verus! {

// Proof-level abstraction of keys_in_index_range_agree postconditions.
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

// Proof-level abstraction of values_agree postconditions.
// Operates on self.vals@[i]@ directly (not through map lookup).
#[verifier::external_body]
proof fn values_agree_spec(
    direct_vals: Seq<int>,
    lo: int,
    hi: int,
    v: int,
) -> (ret: (bool, bool))
    requires
        0 <= lo <= hi < direct_vals.len(),
    ensures
        ret.0 == (forall |i: int| #![auto] lo <= i <= hi ==> direct_vals[i] == v),
        !ret.0 ==> (ret.1 == (direct_vals[hi] != v && forall |i: int| #![auto] lo <= i < hi ==> direct_vals[i] == v)),
{ unimplemented!() }

// ========== LOGICAL TESTS ==========

// Test 1: Agreement on [lo, hi] does NOT imply agreement on [lo, hi+1].
// The spec constrains each call independently; extending the range may
// introduce a disagreeing element at hi+1. Not entailed by the spec.
// SHOULD FAIL
proof fn test_logical_agreement_extends(
    vals: Seq<int>,
    lo: int,
    hi: int,
    v: int,
)
    requires
        0 <= lo,
        lo <= hi,
        hi + 1 < vals.len(),
{
    let ret1 = keys_in_index_range_agree_spec(vals, lo, hi, v);
    let ret2 = keys_in_index_range_agree_spec(vals, lo, hi + 1, v);
    assert(ret1.0 ==> ret2.0);
}

// Test 2: ret.1 is NOT constrained when ret.0 is true.
// The spec only specifies ret.1 under the guard !ret.0.
// When all values agree (ret.0 == true), ret.1 can be anything.
// SHOULD FAIL
proof fn test_logical_ret1_unspecified_when_agree()
{
    let vals: Seq<int> = seq![1int, 1, 1];
    let ret = keys_in_index_range_agree_spec(vals, 0, 2, 1);
    assert(ret.0 ==> ret.1);
}

// Test 3: values_agree and keys_in_index_range_agree are NOT guaranteed
// to return the same result on different underlying value sequences.
// In the original code, map_valid ensures self@[self.keys@[i]] == self.vals@[i],
// bridging the two. Without that invariant, they are independent.
// SHOULD FAIL
proof fn test_logical_cross_function_agree(
    direct_vals: Seq<int>,
    map_vals: Seq<int>,
    lo: int,
    hi: int,
    v: int,
)
    requires
        0 <= lo <= hi < direct_vals.len(),
        0 <= lo <= hi < map_vals.len(),
{
    let ret1 = values_agree_spec(direct_vals, lo, hi, v);
    let ret2 = keys_in_index_range_agree_spec(map_vals, lo, hi, v);
    assert(ret1.0 == ret2.0);
}

}
