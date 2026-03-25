use vstd::prelude::*;


fn main() {}

verus! {

pub proof fn sub_distribute(a: int, b: int, c: int)
    ensures a * c - b * c == (a - b) * c,
{
    assert(a * c - b * c == (a - b) * c) by(nonlinear_arith);
}



// === Entailment query ===
proof fn phi_5_sub_distribute_factor_extraction(n: int, c: int)
    ensures
        (n + 1) * c - n * c == c,
{
    sub_distribute(n + 1, n, c);
}

}
