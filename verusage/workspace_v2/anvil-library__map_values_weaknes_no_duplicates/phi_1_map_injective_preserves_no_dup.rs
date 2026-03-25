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
proof fn phi_1_map_injective_preserves_no_dup(s: Seq<int>, map: spec_fn(int) -> int)
    requires
        s.no_duplicates(),
        forall |x: int, y: int| x != y ==> map(x) != map(y),
    ensures
        s.map_values(map).no_duplicates(),
{
}

}
