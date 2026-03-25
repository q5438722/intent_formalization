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
proof fn phi_3_map_self_union_idempotent<S, T>(s: Set<S>, f: spec_fn(S) -> T)
    ensures
        s.union(s).map(f) == s.map(f),
{
    lemma_map_distribute(s, s, f);
    // s.map(f).union(s.map(f)) should equal s.map(f) by set idempotence
}

}
