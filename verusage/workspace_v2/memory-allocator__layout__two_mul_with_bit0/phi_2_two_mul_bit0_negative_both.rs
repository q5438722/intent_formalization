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
proof fn phi_2_two_mul_bit0_negative_both(x1: int, y1: int)
    requires
        y1 < 0,
        x1 < 0,
    ensures
        (2 * x1) / (2 * y1) == x1 / y1,
{
    two_mul_with_bit0(x1, y1);
}

}
