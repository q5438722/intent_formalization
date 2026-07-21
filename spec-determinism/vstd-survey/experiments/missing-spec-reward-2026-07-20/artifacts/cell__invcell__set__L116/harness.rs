#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::cell::invcell::*;

use vstd::predicate::Predicate;

verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_set_equal(r1: (), r2: ()) -> bool {
    (r1 == r2)
}

proof fn det_set<T, Pred: Predicate<T>>(g_neq_tuple: bool, self_: &InvCell<T, Pred>, val: T, r1: (), r2: ())
    requires (self_.inv(val)),
    ensures
        ({
            &&& (self_.inv(val))
            &&& (self_.inv(val))
        }) ==> det_set_equal(r1, r2),
{
    if g_neq_tuple { assume(!det_set_equal(r1, r2)); }
}
}

fn main() {}
