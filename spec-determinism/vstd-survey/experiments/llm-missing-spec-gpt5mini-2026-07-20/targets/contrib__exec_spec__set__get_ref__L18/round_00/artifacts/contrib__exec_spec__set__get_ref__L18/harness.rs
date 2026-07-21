#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::contrib::exec_spec::set::*;


verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_get_ref_equal<K: DeepView + DeepViewClone + std::hash::Hash + std::cmp::Eq>(r1: &HashSet<K>, r2: &HashSet<K>) -> bool {
    (r1 == r2)
}

proof fn det_get_ref<'a, K: DeepView + DeepViewClone + std::hash::Hash + std::cmp::Eq>(g_neq_tuple: bool, self_: &'a HashSet<K>, r1: &HashSet<K>, r2: &HashSet<K>)
    ensures
        ({
            &&& (res.deep_view() == self_.deep_view())
            &&& (res.deep_view() == self_.deep_view())
        }) ==> det_get_ref_equal::<K>(r1, r2),
{
    if g_neq_tuple { assume(!det_get_ref_equal::<K>(r1, r2)); }
}
}

fn main() {}
