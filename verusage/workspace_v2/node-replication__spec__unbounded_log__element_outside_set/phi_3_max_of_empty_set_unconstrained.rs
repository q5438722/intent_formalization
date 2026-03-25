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
proof fn phi_3_max_of_empty_set_unconstrained(r1: nat, r2: nat)
    requires
        r1 != r2,
    ensures ({
        let m = max_of_set(Set::empty());
        m <= m  // vacuously satisfies ensures, but the return value is unconstrained
    }),
{
    let _ = max_of_set(Set::<nat>::empty());
}

}
