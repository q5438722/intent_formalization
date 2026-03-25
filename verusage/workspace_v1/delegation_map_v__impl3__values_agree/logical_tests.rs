use vstd::prelude::*;

fn main() {}

verus! {

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
}

// ========== LOGICAL TESTS ==========

// Test 1: When ret.0 is true, assert ret.1 must also be true
// The spec does NOT constrain ret.1 when ret.0 is true
// SHOULD FAIL
proof fn test_logical_ret1_when_all_match(
    vals: Seq<AbstractEndPoint>,
    lo: int, hi: int,
    v: AbstractEndPoint,
    ret: (bool, bool),
)
    requires
        0 <= lo <= hi < vals.len(),
        ret.0 == (forall |i: int| #![auto] lo <= i <= hi ==> vals[i] == v),
        !ret.0 ==> (ret.1 == (vals[hi] != v && forall |i: int| #![auto] lo <= i < hi ==> vals[i] == v)),
        ret.0,
{
    assert(ret.1); // SHOULD FAIL: ret.1 is unconstrained when ret.0 is true
}

// Test 2: Assert ret.0 implies ALL values in the sequence match v
// (not just those in [lo, hi])
// SHOULD FAIL
proof fn test_logical_all_values_match(
    vals: Seq<AbstractEndPoint>,
    lo: int, hi: int,
    v: AbstractEndPoint,
    ret: (bool, bool),
)
    requires
        0 <= lo <= hi < vals.len(),
        vals.len() > hi + 1,
        ret.0 == (forall |i: int| #![auto] lo <= i <= hi ==> vals[i] == v),
        !ret.0 ==> (ret.1 == (vals[hi] != v && forall |i: int| #![auto] lo <= i < hi ==> vals[i] == v)),
        ret.0,
{
    assert(forall |i: int| #![auto] 0 <= i < vals.len() ==> vals[i] == v); // SHOULD FAIL
}

// Test 3: Assert ret.1 being true implies ret.0 is true
// SHOULD FAIL — ret.1 can be true when ret.0 is false
proof fn test_logical_ret1_implies_ret0(
    vals: Seq<AbstractEndPoint>,
    lo: int, hi: int,
    v: AbstractEndPoint,
    ret: (bool, bool),
)
    requires
        0 <= lo <= hi < vals.len(),
        ret.0 == (forall |i: int| #![auto] lo <= i <= hi ==> vals[i] == v),
        !ret.0 ==> (ret.1 == (vals[hi] != v && forall |i: int| #![auto] lo <= i < hi ==> vals[i] == v)),
        ret.1,
{
    assert(ret.0); // SHOULD FAIL: ret.1=true is compatible with ret.0=false
}

}
