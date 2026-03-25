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
proof fn phi_5_element_outside_set_is_beyond_max(s: Set<nat>)
    requires
        s.finite(),
    ensures
        element_outside_set(s) == max_of_set(s) + 1,
        forall|x: nat| #[trigger] s.contains(x) ==> x < element_outside_set(s),
{
    let _ = element_outside_set(s);
    let _ = max_of_set(s);
}

}
