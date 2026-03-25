use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus!{

// File: set_lib_ext_v.rs
	#[verifier::external_body]
pub proof fn set_map_union<A, B>(s1: Set<A>, s2: Set<A>, f: spec_fn(A) -> B)
    ensures (s1 + s2).map(f) == s1.map(f) + s2.map(f)
	{
		unimplemented!()
	}

pub proof fn set_map_union_auto<A, B>()
    ensures forall |s1: Set<A>, s2: Set<A>, f: spec_fn(A) -> B|
        #[trigger] (s1 + s2).map(f) == s1.map(f) + s2.map(f)
{
    assert forall |s1: Set<A>, s2: Set<A>, f: spec_fn(A) -> B|
        #[trigger] ((s1 + s2).map(f)) == s1.map(f) + s2.map(f) by {
        set_map_union(s1, s2, f);
    }
}




// === Entailment query ===
proof fn phi_2_map_union_empty_left<A, B>(s: Set<A>, f: spec_fn(A) -> B)
    ensures
        (Set::<A>::empty() + s).map(f) == s.map(f),
{
    set_map_union(Set::<A>::empty(), s, f);
    assert(Set::<A>::empty().map(f) =~= Set::<B>::empty());
}

}
