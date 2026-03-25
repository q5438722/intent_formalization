use vstd::prelude::*;


fn main() {}

verus! {

pub proof fn sub_distribute(a: int, b: int, c: int)
    ensures a * c - b * c == (a - b) * c,
{
    assert(a * c - b * c == (a - b) * c) by(nonlinear_arith);
}



// === Entailment query ===
proof fn phi_3_sub_distribute_self(a: int, c: int)
    ensures
        a * c - a * c == 0,
{
    sub_distribute(a, a, c);
}

}
