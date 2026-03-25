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
proof fn phi_4_mod_one_is_identity(i: int)
    requires
        0 <= i < 1,
    ensures
        (i % 1) == i,
{
    int_mod_less_than_same(i, 1);
}

}
