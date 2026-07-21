#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::contrib::exec_spec::option::*;


verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_deep_clone_equal<T: DeepViewClone>(r1: Option<T>, r2: Option<T>) -> bool {
    (((r1)@ =~= (r2)@))
}

proof fn det_deep_clone<T: DeepViewClone>(g_neq_tuple: bool, self_: &Option<T>, r1: Option<T>, r2: Option<T>)
    ensures
        ({
            &&& (res.deep_view() == self_.deep_view())
            &&& (res.is_some() == self_.is_some())
            &&& (res.deep_view() == self_.deep_view())
            &&& (res.is_some() == self_.is_some())
        }) ==> det_deep_clone_equal::<T>(r1, r2),
{
    if g_neq_tuple { assume(!det_deep_clone_equal::<T>(r1, r2)); }
}
}

fn main() {}
