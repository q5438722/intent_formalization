use vstd::prelude::*;

fn main() {}

verus!{

// File: spec_t/mmu/defs.rs
pub open spec(checked) fn aligned(addr: nat, size: nat) -> bool {
    addr % size == 0
}


// File: extra.rs
pub proof fn aligned_transitive_auto()
    ensures forall|a: nat, b: nat, c: nat| 0 < b && 0 < c && aligned(a, b) && aligned(b, c) ==> aligned(a, c),
{
    assert forall|a: nat, b: nat, c: nat| 0 < b && 0 < c && aligned(a, b) && aligned(b, c) implies aligned(a, c) by {
        aligned_transitive(a, b, c);
    }
}

	#[verifier::external_body]
pub proof fn aligned_transitive(a: nat, b: nat, c: nat)
    requires
        0 < b,
        0 < c,
        aligned(a, b),
        aligned(b, c),
    ensures aligned(a, c)
	{
		unimplemented!()
	}




// === Entailment query ===
proof fn phi_3_aligned_chain_three(a: nat, b: nat, c: nat, d: nat)
    requires
        0 < b, 0 < c, 0 < d,
        aligned(a, b),
        aligned(b, c),
        aligned(c, d),
    ensures
        aligned(a, d),
{
    aligned_transitive(a, b, c);
    aligned_transitive(a, c, d);
}

}
