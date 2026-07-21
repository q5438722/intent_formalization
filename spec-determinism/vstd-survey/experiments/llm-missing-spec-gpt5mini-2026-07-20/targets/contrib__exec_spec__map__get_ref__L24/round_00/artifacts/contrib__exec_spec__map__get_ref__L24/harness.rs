#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::contrib::exec_spec::map::*;


verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_get_ref_equal<K: DeepView + DeepViewClone + std::hash::Hash + std::cmp::Eq, V: DeepView + DeepViewClone>(r1: &HashMap<K, V>, r2: &HashMap<K, V>) -> bool {
    (r1 == r2)
}

proof fn det_get_ref<'a,
    K: DeepView + DeepViewClone + std::hash::Hash + std::cmp::Eq,
    V: DeepView + DeepViewClone,>(g_neq_tuple: bool, self_: &'a HashMap<K, V>, r1: &HashMap<K, V>, r2: &HashMap<K, V>)
    ensures
        ({
            &&& (res.deep_view() == self_.deep_view())
            &&& (res.deep_view() == self_.deep_view())
        }) ==> det_get_ref_equal::<K, V>(r1, r2),
{
    if g_neq_tuple { assume(!det_get_ref_equal::<K, V>(r1, r2)); }
}
}

fn main() {}
