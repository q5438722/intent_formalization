use vstd::prelude::*;


fn main() {}

verus! {

pub proof fn sub_distribute(a: int, b: int, c: int)
    ensures a * c - b * c == (a - b) * c,
{
    assert(a * c - b * c == (a - b) * c) by(nonlinear_arith);
}



// === Entailment query ===
proof fn phi_4_sub_distribute_negative(a: int, b: int, c: int)
    requires
        a < b,
        c > 0,
    ensures
        a * c - b * c < 0,
{
    sub_distribute(a, b, c);
    assert((a - b) * c < 0) by(nonlinear_arith)
        requires a < b, c > 0;
}

}
