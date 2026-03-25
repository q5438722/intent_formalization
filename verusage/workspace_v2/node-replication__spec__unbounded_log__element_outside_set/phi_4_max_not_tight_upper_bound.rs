use vstd::prelude::*;

fn main() {}

verus!{

	#[verifier::external_body]
proof fn max_of_set(s: Set<nat>) -> (r: nat)
    requires
        s.finite(),
    ensures
        forall|x: nat| #[trigger] s.contains(x) ==> x <= r,
    decreases s.len(),
	{
		unimplemented!()
	}

proof fn element_outside_set(s: Set<nat>) -> (r: nat)
    requires
        s.finite(),
    ensures
        !s.contains(r),
{
    max_of_set(s) + 1
}



// === Entailment query ===
proof fn phi_4_max_not_tight_upper_bound(s: Set<nat>, x: nat)
    requires
        s.finite(),
        s.contains(x),
    ensures
        x <= max_of_set(s),
{
    let _ = max_of_set(s);
}

}
