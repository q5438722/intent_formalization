#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::pervasive::*;


verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_unreached_equal<A>(r1: A, r2: A) -> bool {
    (r1 == r2)
}

proof fn det_unreached<A>(g_neq_tuple: bool, r1: A, r2: A)
    requires (false),
    ensures
        ({
            &&& (false)
            &&& (false)
        }) ==> det_unreached_equal::<A>(r1, r2),
{
    if g_neq_tuple { assume(!det_unreached_equal::<A>(r1, r2)); }
}
}

fn main() {}
