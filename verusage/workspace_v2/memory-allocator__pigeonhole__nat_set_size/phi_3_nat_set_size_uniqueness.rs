use vstd::prelude::*;

fn main() {}

verus! {

pub open spec fn set_nat_range(lo: nat, hi: nat) -> Set<nat> {
    Set::new(|i: nat| lo <= i && i < hi)
}

	#[verifier::external_body]
pub proof fn lemma_nat_range(lo: nat, hi: nat)
    requires
        lo <= hi,
    ensures
        set_nat_range(lo, hi).finite(),
        set_nat_range(lo, hi).len() == hi - lo,
    decreases
        hi - lo,
	{
		unimplemented!()
	}

proof fn nat_set_size(s:Set<nat>, bound:nat)
    requires
        forall |i: nat| (0 <= i < bound <==> s.contains(i)),
    ensures
        s.finite(),
        s.len() == bound,
{
    let nats = set_nat_range(0, bound);
    lemma_nat_range(0, bound);
    assert(s =~= nats);
}



// === Entailment query ===
proof fn phi_3_nat_set_size_uniqueness(s1: Set<nat>, s2: Set<nat>, bound: nat)
    requires
        forall |i: nat| (0 <= i < bound <==> s1.contains(i)),
        forall |i: nat| (0 <= i < bound <==> s2.contains(i)),
    ensures
        s1 =~= s2,
{
    nat_set_size(s1, bound);
    nat_set_size(s2, bound);
}

}
