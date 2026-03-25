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
proof fn phi_1_max_of_set_external_body(s: Set<nat>)
    requires
        s.finite(),
    ensures ({
        let r = max_of_set(s);
        forall|x: nat| #[trigger] s.contains(x) ==> x <= r
    }),
{
    let _ = max_of_set(s);
}

}
