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
proof fn phi_2_mod_zero_is_zero(len: int)
    requires
        len > 0,
    ensures
        (0int % len) == 0,
{
    int_mod_less_than_same(0, len);
}

}
