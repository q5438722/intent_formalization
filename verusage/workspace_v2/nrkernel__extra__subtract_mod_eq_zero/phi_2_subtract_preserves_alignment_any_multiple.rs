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
proof fn phi_2_subtract_preserves_alignment_any_multiple(c: nat, k1: nat, k2: nat)
    requires
        c > 0,
        k2 >= k1,
    ensures
        aligned((k2 * c - k1 * c) as nat, c),
{
    assert(aligned(k1 * c, c)) by {
        assert(k1 * c % c == 0) by (nonlinear_arith) requires c > 0;
    };
    assert(aligned(k2 * c, c)) by {
        assert(k2 * c % c == 0) by (nonlinear_arith) requires c > 0;
    };
    subtract_mod_eq_zero(k1 * c, k2 * c, c);
}

}
