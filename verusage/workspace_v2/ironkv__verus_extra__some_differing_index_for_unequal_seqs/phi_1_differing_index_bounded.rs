use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus!{

// File: seq_lib_v.rs
pub proof fn some_differing_index_for_unequal_seqs<A>(s1: Seq<A>, s2: Seq<A>) -> (i: int)
  requires
    s1 != s2,
    s1.len() == s2.len(),
  ensures
    0 <= i < s1.len(),
    s1[i] != s2[i],
{
  if forall |i| 0 <= i < s1.len() ==> s1[i] == s2[i] {
    assert(s1 =~= s2);
  }
  choose |i:int| 0 <= i < s1.len() && s1[i] != s2[i]
}




// === Entailment query ===
proof fn phi_1_differing_index_bounded(s1: Seq<int>, s2: Seq<int>)
    requires
        s1 != s2,
        s1.len() == s2.len(),
        s1.len() > 0,
    ensures
        ({let i = some_differing_index_for_unequal_seqs(s1, s2); i < s1.len() as int}),
{
    let _ = some_differing_index_for_unequal_seqs(s1, s2);
}

}
