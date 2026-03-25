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

// ========== BOUNDARY TESTS ==========

// Test 1: lo > hi (reversed range).
// Precondition requires lo <= hi. Using lo=2, hi=1 violates this.
// SHOULD FAIL
proof fn test_boundary_lo_gt_hi()
{
    let vals: Seq<int> = seq![1int, 2, 3];
    let ret = keys_in_index_range_agree_spec(vals, 2, 1, 1);
}

// Test 2: hi == vals.len() (off-by-one, out of bounds).
// Precondition requires hi < vals.len(). Using hi == 3 when len == 3 violates this.
// SHOULD FAIL
proof fn test_boundary_hi_eq_len()
{
    let vals: Seq<int> = seq![1int, 2, 3];
    let ret = keys_in_index_range_agree_spec(vals, 0, 3, 1);
}

// Test 3: Empty sequence — no valid index pair exists.
// Precondition requires hi < vals.len(). For empty seq, len == 0,
// so 0 < 0 is false.
// SHOULD FAIL
proof fn test_boundary_empty_seq()
{
    let vals: Seq<int> = Seq::empty();
    let ret = keys_in_index_range_agree_spec(vals, 0, 0, 1);
}

}
