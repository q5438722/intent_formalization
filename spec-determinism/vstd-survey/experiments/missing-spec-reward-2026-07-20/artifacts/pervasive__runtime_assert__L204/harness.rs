#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::pervasive::*;


verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_runtime_assert_equal(r1: (), r2: ()) -> bool {
    (r1 == r2)
}

proof fn det_runtime_assert(g_b_is_true: bool, g_b_is_false: bool, g_neq_tuple: bool, b: bool, r1: (), r2: ())
    requires (b),
    ensures
        ({
            &&& (b)
            &&& (b)
        }) ==> det_runtime_assert_equal(r1, r2),
{
    if g_b_is_true { assume(b == true); }
    if g_b_is_false { assume(b == false); }
    if g_neq_tuple { assume(!det_runtime_assert_equal(r1, r2)); }
}
}

fn main() {}
