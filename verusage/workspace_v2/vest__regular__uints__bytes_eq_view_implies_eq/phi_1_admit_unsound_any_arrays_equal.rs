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
proof fn phi_1_admit_unsound_any_arrays_equal()
    ensures
        [0u8, 1u8] == [0u8, 0u8],
{
    bytes_eq_view_implies_eq([0u8, 1u8], [0u8, 0u8]);
}

}
