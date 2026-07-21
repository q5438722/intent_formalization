#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::thread::*;


verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_clone_equal(r1: IsThread, r2: IsThread) -> bool {
    (r1 == r2)
}

proof fn det_clone(g_neq_tuple: bool, self_: &IsThread, r1: IsThread, r2: IsThread)
    ensures
        ({
            &&& (r1@ == self_@)
            &&& (r2@ == self_@)
        }) ==> det_clone_equal(r1, r2),
{
    if g_neq_tuple { assume(!det_clone_equal(r1, r2)); }
}
}

fn main() {}
