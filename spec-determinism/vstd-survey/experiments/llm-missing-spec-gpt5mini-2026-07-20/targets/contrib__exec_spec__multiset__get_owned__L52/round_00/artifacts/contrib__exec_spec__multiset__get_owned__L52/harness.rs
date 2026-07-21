#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::contrib::exec_spec::multiset::*;


verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_get_owned_equal<T: DeepView + DeepViewClone + std::hash::Hash + std::cmp::Eq>(r1: ExecMultiset<T>, r2: ExecMultiset<T>) -> bool {
    (r1 == r2)
}

proof fn det_get_owned<'a, T: DeepView + DeepViewClone + std::hash::Hash + std::cmp::Eq>(g_neq_tuple: bool, self_: &'a ExecMultiset<T>, r1: ExecMultiset<T>, r2: ExecMultiset<T>)
    ensures
        ({
            &&& (res.deep_view() == self_.deep_view())
            &&& (res.deep_view() == self_.deep_view())
        }) ==> det_get_owned_equal::<T>(r1, r2),
{
    if g_neq_tuple { assume(!det_get_owned_equal::<T>(r1, r2)); }
}
}

fn main() {}
