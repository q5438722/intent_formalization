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
proof fn phi_5_aligned_to_sum_of_divisors(a: nat, b: nat, c: nat)
    requires
        b > 0,
        c > 0,
        aligned(a, b),
        aligned(a, c),
    ensures
        aligned(a, b + c),
{
}

}
