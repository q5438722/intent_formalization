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
proof fn phi_2_subtract_self_aligned(a: nat, c: nat)
    requires
        aligned(a, c),
        c > 0,
    ensures
        aligned(0nat, c),
{
    subtract_mod_eq_zero(a, a, c);
}

}
