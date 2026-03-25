use vstd::prelude::*;

fn main() {}

verus!{

// File: src/regular/uints.rs
proof fn bytes_eq_view_implies_eq<const N: usize>(a: [u8; N], b: [u8; N])
    ensures
        a@ =~= b@ <==> a == b,
{
    if a@ == b@ {
        assert(a.len() == N);
        assert(a.len() == b.len());
        assert forall|i: int| 0 <= i < N implies a[i] == b[i] by {}
        admit();
    }
}




// === Entailment query ===
proof fn phi_5_admit_derives_false()
    ensures
        false,
{
    let a: [u8; 2] = [0u8, 0u8];
    let b: [u8; 2] = [0u8, 1u8];
    bytes_eq_view_implies_eq(a, b);
    assert(a@ =~= b@ <==> a == b);
    // If the biconditional is unsound due to admit(), we might derive false
    // from a != b but the lemma claiming a@ =~= b@ <==> a == b
}

}
