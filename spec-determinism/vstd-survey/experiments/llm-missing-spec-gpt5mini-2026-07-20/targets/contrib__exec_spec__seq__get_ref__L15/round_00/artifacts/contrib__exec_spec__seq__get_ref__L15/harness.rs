#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::contrib::exec_spec::seq::*;


verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_get_ref_equal<T: DeepView>(r1: &[T], r2: &[T]) -> bool {
    (r1 =~= r2)
}

proof fn det_get_ref<'a, T: DeepView>(g_r1_leneq: bool, k_r1_leneq: nat, g_r1_lenrng: bool, k_r1_lenrng_lo: nat, k_r1_lenrng_hi: nat, g_r2_leneq: bool, k_r2_leneq: nat, g_r2_lenrng: bool, k_r2_lenrng_lo: nat, k_r2_lenrng_hi: nat, g_neq_tuple: bool, self_: &'a Vec<T>, r1: &[T], r2: &[T])
    ensures
        ({
            &&& (res.deep_view() == self_.deep_view())
            &&& (res.deep_view() == self_.deep_view())
        }) ==> det_get_ref_equal::<T>(r1, r2),
{
    if g_r1_leneq { assume(r1.len() == k_r1_leneq); }
    if g_r1_lenrng { assume(r1.len() >= k_r1_lenrng_lo && r1.len() <= k_r1_lenrng_hi); }
    if g_r2_leneq { assume(r2.len() == k_r2_leneq); }
    if g_r2_lenrng { assume(r2.len() >= k_r2_lenrng_lo && r2.len() <= k_r2_lenrng_hi); }
    if g_neq_tuple { assume(!det_get_ref_equal::<T>(r1, r2)); }
}
}

fn main() {}
