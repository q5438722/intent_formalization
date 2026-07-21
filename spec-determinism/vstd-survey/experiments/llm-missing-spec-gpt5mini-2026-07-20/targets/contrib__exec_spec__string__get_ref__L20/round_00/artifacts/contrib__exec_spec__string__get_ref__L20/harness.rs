#![allow(unused_imports)]
extern crate alloc;
use vstd::prelude::*;
use vstd::contrib::exec_spec::string::*;


verus! {
// Generated equal-fn for determinism check.
// Policy: errs_equivalent=True, opaque_ok=False
spec fn det_get_ref_equal(r1: &str, r2: &str) -> bool {
    ((r1)@ == (r2)@)
}

proof fn det_get_ref<'a>(g_r1_eq_empty: bool, g_r1_eq_string_1: bool, g_r1_eq_string_2: bool, g_r2_eq_empty: bool, g_r2_eq_string_1: bool, g_r2_eq_string_2: bool, g_neq_tuple: bool, self_: &'a String, r1: &str, r2: &str)
    ensures
        ({
            &&& (res.deep_view() == self_.deep_view())
            &&& (res.deep_view() == self_.deep_view())
        }) ==> det_get_ref_equal(r1, r2),
{
    if g_r1_eq_empty { assume(r1@ == ""@); }
    if g_r1_eq_string_1 { assume(r1@ == "string 1"@); }
    if g_r1_eq_string_2 { assume(r1@ == "string 2"@); }
    if g_r2_eq_empty { assume(r2@ == ""@); }
    if g_r2_eq_string_1 { assume(r2@ == "string 1"@); }
    if g_r2_eq_string_2 { assume(r2@ == "string 2"@); }
    if g_neq_tuple { assume(!det_get_ref_equal(r1, r2)); }
}
}

fn main() {}
