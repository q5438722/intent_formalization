use vstd::prelude::*;


fn main() {}

verus! {

pub proof fn sub_distribute(a: int, b: int, c: int)
    ensures a * c - b * c == (a - b) * c,
{
    assert(a * c - b * c == (a - b) * c) by(nonlinear_arith);
}



// === Entailment query ===
proof fn phi_2_sub_distribute_zero_c(a: int, b: int)
    ensures
        a * 0 - b * 0 == 0,
{
    sub_distribute(a, b, 0);
}

}
