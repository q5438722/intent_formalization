use vstd::prelude::*;

fn main() {}

verus! {

pub open spec fn pow2(i: int) -> nat
    decreases i
{
    if i <= 0 {
        1
    } else {
        pow2(i - 1) * 2
    }
}

proof fn pow2_positive(e:int)
    ensures pow2(e) > 0,
    decreases e,
{
    if e <= 0 {
    } else {
        pow2_positive(e - 1);
    }
}



// === Entailment query ===
proof fn phi_4_pow2_monotone_strict(a: int, b: int)
    requires
        a > 0,
        b > a,
    ensures
        pow2(b) > pow2(a),
    decreases b - a,
{
    if b == a + 1 {
        pow2_positive(a);
    } else {
        phi_4_pow2_monotone_strict(a, b - 1);
        pow2_positive(b - 1);
    }
}

}
