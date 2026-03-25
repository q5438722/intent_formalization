use vstd::prelude::*;

fn main() {}

verus!{

// File: spec_t/mmu/defs.rs
pub open spec(checked) fn aligned(addr: nat, size: nat) -> bool {
    addr % size == 0
}


// File: extra.rs
pub proof fn subtract_mod_eq_zero(a: nat, b: nat, c: nat)
    requires aligned(a, c), aligned(b, c), a <= b, c > 0
    ensures aligned((b - a) as nat, c)
{
    let a = a as int; let b = b as int; let c = c as int;
    vstd::arithmetic::div_mod::lemma_sub_mod_noop(b, a, c);
    assert(((b % c) - (a % c)) % c == (b - a) % c);
    assert(0int % c == (b - a) % c);
}




// === Entailment query ===
proof fn phi_3_aligned_difference_no_ordering_on_result(a: nat, b: nat, c: nat)
    requires
        aligned(a, c),
        aligned(b, c),
        a <= b,
        c > 0,
        b - a > 0,
    ensures
        aligned((b - a) as nat, c) && (b - a) as nat >= c,
{
    subtract_mod_eq_zero(a, b, c);
    assert((b - a) as nat % c == 0);
    assert((b - a) as nat >= 1);
    assert((b - a) as nat >= c) by {
        assert(((b - a) as nat) % c == 0 && (b - a) as nat >= 1 ==> (b - a) as nat >= c) by (nonlinear_arith) requires c > 0;
    };
}

}
