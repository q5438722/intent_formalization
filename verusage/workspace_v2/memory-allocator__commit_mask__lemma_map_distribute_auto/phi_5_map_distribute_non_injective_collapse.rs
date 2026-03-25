use vstd::prelude::*;

fn main() {}

verus! {

	#[verifier::external_body]
proof fn lemma_map_distribute<S,T>(s1: Set<S>, s2: Set<S>, f: spec_fn(S) -> T)
    ensures s1.union(s2).map(f) == s1.map(f).union(s2.map(f))
	{
		unimplemented!()
	}

proof fn lemma_map_distribute_auto<S,T>()
    ensures forall|s1: Set<S>, s2: Set<S>, f: spec_fn(S) -> T| s1.union(s2).map(f) == #[trigger] s1.map(f).union(s2.map(f))
{
    assert forall|s1: Set<S>, s2: Set<S>, f: spec_fn(S) -> T| s1.union(s2).map(f) == #[trigger] s1.map(f).union(s2.map(f)) by {
        lemma_map_distribute(s1, s2, f);
    }
}



// === Entailment query ===
proof fn phi_5_map_distribute_non_injective_collapse(s1: Set<int>, s2: Set<int>)
    requires
        s1 == set![1, 2],
        s2 == set![3, 4],
    ensures
        ({
            let f = |x: int| 0int;
            s1.union(s2).map(f) == s1.map(f).union(s2.map(f))
        }),
{
    let f = |x: int| 0int;
    lemma_map_distribute(s1, s2, f);
}

}
