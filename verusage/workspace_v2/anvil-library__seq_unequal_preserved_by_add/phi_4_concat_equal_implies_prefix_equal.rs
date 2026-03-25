#![allow(unused_imports)]
use vstd::prelude::*;
use vstd::seq::*;
use vstd::seq_lib::*;

fn main() {}

verus! {

pub proof fn seq_unequal_preserved_by_add<A>(s1: Seq<A>, s2: Seq<A>, suffix: Seq<A>)
    requires s1 != s2
    ensures s1 + suffix != s2 + suffix
{
    assert(!(s1 =~= s2));
    if s1.len() == s2.len() {
        let witness_idx = choose |i: int| 0 <= i < s1.len() && s1[i] != s2[i];
        assert((s1 + suffix)[witness_idx] != (s2 + suffix)[witness_idx]);
    } else {
        assert((s1 + suffix).len() != (s2 + suffix).len());
    }
}



// === Entailment query ===
proof fn phi_4_concat_equal_implies_prefix_equal(s1: Seq<int>, s2: Seq<int>, suffix: Seq<int>)
    requires
        s1 + suffix == s2 + suffix,
    ensures
        s1 == s2,
{
    if s1 != s2 {
        seq_unequal_preserved_by_add(s1, s2, suffix);
    }
}

}
