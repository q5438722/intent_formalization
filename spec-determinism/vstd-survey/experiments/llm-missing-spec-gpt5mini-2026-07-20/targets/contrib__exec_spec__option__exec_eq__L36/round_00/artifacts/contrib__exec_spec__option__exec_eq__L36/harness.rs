#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::contrib::exec_spec::option::*;


verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_exec_eq_equal(r1: bool, r2: bool) -> bool {
    (r1 == r2)
}

proof fn det_exec_eq<'a, T: DeepView>(g_r1_is_true: bool, g_r1_is_false: bool, g_r2_is_true: bool, g_r2_is_false: bool, g_neq_tuple: bool, this: &'a Option<T>, other: &'a Option<T>::Other, r1: bool, r2: bool)
    where &'a T: ExecSpecEq<'a, Other = &'a T>
    ensures
        ({
            &&& (res == match (this, other) { (Some(t1), Some(t2)) => <&'a T>::exec_eq(t1, t2), _ => this.deep_view() =~~= other.deep_view(), })
            &&& (res == match (this, other) { (Some(t1), Some(t2)) => <&'a T>::exec_eq(t1, t2), _ => this.deep_view() =~~= other.deep_view(), })
        }) ==> det_exec_eq_equal(r1, r2),
{
    if g_r1_is_true { assume(r1 == true); }
    if g_r1_is_false { assume(r1 == false); }
    if g_r2_is_true { assume(r2 == true); }
    if g_r2_is_false { assume(r2 == false); }
    if g_neq_tuple { assume(!det_exec_eq_equal(r1, r2)); }
}
}

fn main() {}
