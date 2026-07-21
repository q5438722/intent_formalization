#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::contrib::exec_spec::option::*;


verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_get_ref_equal<T: Sized + DeepView>(r1: &Option<T>, r2: &Option<T>) -> bool {
    (((r1 is Some) == (r2 is Some)) && ((r1 is Some) ==> (r1->Some_0 == r2->Some_0)))
}

proof fn det_get_ref<'a, T: Sized + DeepView>(g_r1_is_Some: bool, g_r1_is_None: bool, g_r2_is_Some: bool, g_r2_is_None: bool, g_neq_tuple: bool, self_: &'a Option<T>, r1: &Option<T>, r2: &Option<T>)
    ensures
        ({
            &&& (res.deep_view() == self_.deep_view())
            &&& (res.deep_view() == self_.deep_view())
        }) ==> det_get_ref_equal::<T>(r1, r2),
{
    if g_r1_is_Some { assume(r1 is Some); }
    if g_r1_is_None { assume(r1 is None); }
    if g_r2_is_Some { assume(r2 is Some); }
    if g_r2_is_None { assume(r2 is None); }
    if g_neq_tuple { assume(!det_get_ref_equal::<T>(r1, r2)); }
}
}

fn main() {}
