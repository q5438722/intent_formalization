use vstd::prelude::*;

fn main() {}

verus! {

pub struct AbstractEndPoint {
    pub id: Seq<u8>,
}

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Assert ret.0 is always true regardless of val contents
// SHOULD FAIL — ret.0 depends on whether all vals[lo..=hi] match v
proof fn test_behavioral_ret0_always_true(
    vals: Seq<AbstractEndPoint>,
    lo: int, hi: int,
    v: AbstractEndPoint,
    ret: (bool, bool),
)
    requires
        0 <= lo <= hi < vals.len(),
        ret.0 == (forall |i: int| #![auto] lo <= i <= hi ==> vals[i] == v),
        !ret.0 ==> (ret.1 == (vals[hi] != v && forall |i: int| #![auto] lo <= i < hi ==> vals[i] == v)),
{
    assert(ret.0); // SHOULD FAIL
}

// Test 2: Assert ret.0 tracks the weakened range [lo, hi) instead of [lo, hi]
// SHOULD FAIL — spec uses <= hi, not < hi
proof fn test_behavioral_weaken_range(
    vals: Seq<AbstractEndPoint>,
    lo: int, hi: int,
    v: AbstractEndPoint,
    ret: (bool, bool),
)
    requires
        0 <= lo < hi < vals.len(),
        ret.0 == (forall |i: int| #![auto] lo <= i <= hi ==> vals[i] == v),
        !ret.0 ==> (ret.1 == (vals[hi] != v && forall |i: int| #![auto] lo <= i < hi ==> vals[i] == v)),
{
    assert(ret.0 == (forall |i: int| #![auto] lo <= i < hi ==> vals[i] == v)); // SHOULD FAIL
}

// Test 3: When !ret.0, assert ret.1 means vals[hi] MATCHES v (flipped semantics)
// SHOULD FAIL — actual spec says vals[hi] != v
proof fn test_behavioral_flip_ret1(
    vals: Seq<AbstractEndPoint>,
    lo: int, hi: int,
    v: AbstractEndPoint,
    ret: (bool, bool),
)
    requires
        0 <= lo <= hi < vals.len(),
        ret.0 == (forall |i: int| #![auto] lo <= i <= hi ==> vals[i] == v),
        !ret.0 ==> (ret.1 == (vals[hi] != v && forall |i: int| #![auto] lo <= i < hi ==> vals[i] == v)),
        !ret.0,
{
    assert(ret.1 == (vals[hi] == v && forall |i: int| #![auto] lo <= i < hi ==> vals[i] == v)); // SHOULD FAIL
}

}
