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
proof fn phi_2_element_outside_any_finite_set(s: Set<nat>)
    requires
        s.finite(),
    ensures ({
        let r = element_outside_set(s);
        !s.contains(r)
    }),
{
    let _ = element_outside_set(s);
}

}
