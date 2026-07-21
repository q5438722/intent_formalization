#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::contrib::exec_spec::string::*;


verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_deep_clone_equal(r1: String, r2: String) -> bool {
    (((r1).view() =~= (r2).view()))
}

proof fn det_deep_clone(g_neq_tuple: bool, self_: &String, r1: String, r2: String)
    ensures
        ({
            &&& (res.deep_view() == self_.deep_view())
            &&& (res.deep_view() == self_.deep_view())
        }) ==> det_deep_clone_equal(r1, r2),
{
    if g_neq_tuple { assume(!det_deep_clone_equal(r1, r2)); }
}
}

fn main() {}
