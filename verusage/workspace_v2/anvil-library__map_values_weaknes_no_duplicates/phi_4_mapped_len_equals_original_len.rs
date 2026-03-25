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
proof fn phi_4_mapped_len_equals_original_len(s: Seq<int>, map: spec_fn(int) -> int)
    ensures
        s.map_values(map).len() == s.len(),
{
}

}
