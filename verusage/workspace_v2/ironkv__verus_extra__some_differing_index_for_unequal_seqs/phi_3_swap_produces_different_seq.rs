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
proof fn phi_3_swap_produces_different_seq(s: Seq<int>, i: int, j: int)
    requires
        0 <= i < s.len(),
        0 <= j < s.len(),
        i != j,
        s[i] != s[j],
    ensures
        s != s.update(i, s[j]).update(j, s[i]) || s[i] == s[j],
{
    let s2 = s.update(i, s[j]).update(j, s[i]);
    if s == s2 {
        assert(s[i] == s2[i]);
    }
}

}
