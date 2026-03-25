use vstd::prelude::*;

fn main() {}

verus!{

// File: spec_t/mmu/defs.rs
pub open spec(checked) fn aligned(addr: nat, size: nat) -> bool {
    addr % size == 0
}


// File: extra.rs
pub proof fn leq_add_aligned_less(a: nat, b: nat, c: nat) 
    requires 0 < b, a < c, aligned(a, b), aligned(c, b),
    ensures a + b <= c,
{
    assert(a == b * (a / b) + a % b)by (nonlinear_arith)
        requires 0 < b, a < c, aligned(a, b), aligned(c, b);
    assert(c == b * (c / b) + c % b) by (nonlinear_arith)
        requires 0 < b, a < c, aligned(a, b), aligned(c, b);
    assert( a + b <= c) by (nonlinear_arith)
        requires 0 < b, a < c, aligned(a, b), aligned(c, b);

}




// === Entailment query ===
proof fn phi_1_aligned_zero_to_any(b: nat)
    requires
        b > 0,
    ensures
        aligned(0, b),
{
}

}
