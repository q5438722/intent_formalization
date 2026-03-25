use vstd::prelude::*;

fn main() {}

verus! {

proof fn two_mul_with_bit0(x1: int, y1: int)
    requires y1 != 0,
    ensures (2 * x1) / (2 * y1) == x1 / y1
{
    assert(
        (2 * x1) / (2 * y1) == ((2 * x1) / 2) / y1) by(nonlinear_arith)
        requires y1 != 0;
    assert((2 * x1) / 2 == x1);
}



// === Entailment query ===
proof fn phi_4_two_mul_bit0_chain(x1: int, y1: int)
    requires
        y1 != 0,
    ensures
        (4 * x1) / (4 * y1) == x1 / y1,
{
    two_mul_with_bit0(2 * x1, 2 * y1);
    assert(2 * (2 * x1) == 4 * x1) by(nonlinear_arith);
    assert(2 * (2 * y1) == 4 * y1) by(nonlinear_arith);
    two_mul_with_bit0(x1, y1);
}

}
