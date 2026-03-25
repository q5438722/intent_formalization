use vstd::prelude::*;

fn main() {}

verus!{

// File: spec_t/mmu/defs.rs
pub open spec(checked) fn aligned(addr: nat, size: nat) -> bool {
    addr % size == 0
}


// File: extra.rs
pub proof fn mod_mult_zero_implies_mod_zero(a: nat, b: nat, c: nat) 
    requires aligned(a, b * c), b > 0, c > 0
    ensures aligned(a, b)
{
    broadcast use vstd::arithmetic::div_mod::lemma_mod_mod, vstd::arithmetic::div_mod::lemma_mod_breakdown;
    assert((a % (b * c)) % b == 0) by (nonlinear_arith)
        requires
            aligned(a, b*c), b >0, c > 0;
}




// === Entailment query ===
proof fn phi_4_large_alignment_subsumes_small(a: nat, b: nat, k: nat)
    requires
        aligned(a, b * k),
        b > 0,
        k > 0,
    ensures
        aligned(a, b),
{
    mod_mult_zero_implies_mod_zero(a, b, k);
}

}
