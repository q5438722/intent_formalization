use vstd::prelude::*;

fn main() {}

verus!{

// File: spec/utils.rs
    #[verifier::spinoff_prover]
pub proof fn int_mod_less_than_same(i: int, len: int)
    requires
        0 <= i < len,
        len > 0,
    ensures
        (i % len) == i,
{
    assert ( (i % len) == i ) by (nonlinear_arith)
        requires
            0 <= i < len,
            len > 0;
}




// === Entailment query ===
proof fn phi_1_mod_identity_small(i: int, len: int)
    requires
        0 <= i < len,
        len > 0,
    ensures
        (i % len) == i,
{
    int_mod_less_than_same(i, len);
}

}
