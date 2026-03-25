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
proof fn phi_3_zero_aligned_to_anything(b: nat, c: nat)
    requires
        b > 0,
        c > 0,
    ensures
        aligned(0, b * c) && aligned(0, b),
{
    mod_mult_zero_implies_mod_zero(0, b, c);
}

}
