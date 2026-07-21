#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::invariant::*;


verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_spend_open_invariant_credit_equal(r1: (), r2: ()) -> bool {
    (r1 == r2)
}

proof fn det_spend_open_invariant_credit(g_neq_tuple: bool, credit: Tracked<OpenInvariantCredit>, r1: (), r2: ())
    ensures
        ({
            &&& (The supplied Tracked<OpenInvariantCredit> is consumed: on return the caller no longer owns or may use that tracked credit.)
            &&& (The supplied Tracked<OpenInvariantCredit> is consumed: on return the caller no longer owns or may use that tracked credit.)
        }) ==> det_spend_open_invariant_credit_equal(r1, r2),
{
    if g_neq_tuple { assume(!det_spend_open_invariant_credit_equal(r1, r2)); }
}
}

fn main() {}
