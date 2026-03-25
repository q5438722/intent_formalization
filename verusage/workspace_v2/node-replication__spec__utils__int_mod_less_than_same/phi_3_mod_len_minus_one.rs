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
proof fn phi_3_mod_len_minus_one(len: int)
    requires
        len > 1,
    ensures
        ((len - 1) % len) == len - 1,
{
    int_mod_less_than_same(len - 1, len);
}

}
