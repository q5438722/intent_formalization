#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::contrib::exec_spec::map::*;


verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_deep_clone_equal<K: DeepView + DeepViewClone + std::hash::Hash + std::cmp::Eq, V: DeepView + DeepViewClone>(r1: HashMap<K, V>, r2: HashMap<K, V>) -> bool {
    (((r1).view() =~= (r2).view()))
}

proof fn det_deep_clone<K: DeepView + DeepViewClone + std::hash::Hash + std::cmp::Eq,
    V: DeepView + DeepViewClone,>(g_neq_tuple: bool, self_: &HashMap<K, V>, r1: HashMap<K, V>, r2: HashMap<K, V>)
    ensures
        ({
            &&& (res.len() <= self_.len())
            &&& (res.len() <= self_.len())
        }) ==> det_deep_clone_equal::<K, V>(r1, r2),
{
    if g_neq_tuple { assume(!det_deep_clone_equal::<K, V>(r1, r2)); }
}
}

fn main() {}
