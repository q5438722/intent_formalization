use vstd::prelude::*;

fn main() {}

verus!{


pub proof fn map_values_weakens_no_duplicates<A, B>(s: Seq<A>, map: spec_fn(A) -> B)
    requires s.map_values(map).no_duplicates()
    ensures s.no_duplicates()
{
    assert forall |i, j| 0 <= i < s.len() && 0 <= j < s.len() && i != j implies s[i] != s[j] by {
        if s[i] == s[j] {
            assert(s.map_values(map)[i] == s.map_values(map)[j]);
            assert(false);
        }
    }
}



// === Entailment query ===
proof fn phi_5_non_injective_map_loses_no_dup(v1: int, v2: int)
    requires
        v1 != v2,
    ensures
        !seq![v1, v2].map_values(|x: int| 0int).no_duplicates(),
{
}

}
