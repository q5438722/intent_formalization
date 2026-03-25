use vstd::prelude::*;

fn main() {}

verus!{

// File: spec_t/mmu/defs.rs
pub open spec(checked) fn aligned(addr: nat, size: nat) -> bool {
    addr % size == 0
}


// File: extra.rs
pub proof fn lemma_aligned_iff_eq_mul_div(a: nat, b: nat)
    requires b > 0
    ensures aligned(a, b) <==> a == b * (a / b)
{
    assert(a % b == 0 ==> a == b * (a / b)) by (nonlinear_arith)
        requires b > 0;
    assert(a == b * (a / b) ==> a % b == 0) by (nonlinear_arith)
        requires b > 0;
}




// === Entailment query ===
proof fn phi_4_aligned_mul_any(k: nat, size: nat)
    requires
        size > 0,
    ensures
        aligned(k * size, size),
{
    assert(k * size % size == 0) by (nonlinear_arith)
        requires size > 0;
}

}
