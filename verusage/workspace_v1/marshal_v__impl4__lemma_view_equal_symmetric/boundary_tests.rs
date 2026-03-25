use vstd::prelude::*;

fn main() {}

verus! {

// ===== Trait and implementations (from source) =====

pub trait Marshalable : Sized {
    spec fn view_equal(&self, other: &Self) -> bool;

    #[verifier::external_body]
    proof fn lemma_view_equal_symmetric(&self, other: &Self)
        ensures self.view_equal(other) == other.view_equal(self)
    { unimplemented!() }
}

impl Marshalable for usize {
    open spec fn view_equal(&self, other: &Self) -> bool {
        self@ === other@
    }

    #[verifier::external_body]
    proof fn lemma_view_equal_symmetric(&self, other: &Self)
    { unimplemented!() }
}

impl Marshalable for Vec<u8> {
    open spec fn view_equal(&self, other: &Self) -> bool {
        self@ === other@
    }

    #[verifier::external_body]
    proof fn lemma_view_equal_symmetric(&self, other: &Self)
    { unimplemented!() }
}

impl<T: Marshalable> Marshalable for Vec<T> {
    open spec fn view_equal(&self, other: &Self) -> bool {
        let s = self@;
        let o = other@;
        s.len() == o.len() && (forall |i: int| 0 <= i < s.len() ==> #[trigger] s[i].view_equal(&o[i]))
    }

    #[verifier::spinoff_prover]
    proof fn lemma_view_equal_symmetric(&self, other: &Self)
    {
        let s = self@;
        let o = other@;
        if self.view_equal(other) {
            assert forall |i: int| 0 <= i < o.len() implies #[trigger] o[i].view_equal(&s[i]) by {
                s[i].lemma_view_equal_symmetric(&o[i]);
            }
        } else {
            if s.len() != o.len() {
            } else {
                let i = choose |i: int| 0 <= i < s.len() && ! #[trigger] s[i].view_equal(&o[i]);
                s[i].lemma_view_equal_symmetric(&o[i]);
            }
        }
    }
}

// ===== BOUNDARY TESTS =====

// Test 1: Different usize values (0 vs 1) asserted as view_equal
// Edge case: adjacent integer values
// SHOULD FAIL: 0 != 1, so view_equal returns false
proof fn test_boundary_different_usize_are_equal(a: usize, b: usize)
    requires a@ == 0int, b@ == 1int
    ensures a.view_equal(&b) // SHOULD FAIL
{
}

// Test 2: Same usize value asserted as NOT view_equal
// Edge case: self-comparison with zero
// SHOULD FAIL: 0 == 0, so view_equal returns true
proof fn test_boundary_same_usize_not_equal(a: usize, b: usize)
    requires a@ == 0int, b@ == 0int
    ensures !a.view_equal(&b) // SHOULD FAIL
{
}

// Test 3: Empty Vec<u8> vs non-empty Vec<u8> asserted as view_equal
// Edge case: length 0 vs length > 0
// SHOULD FAIL: different lengths imply different sequences
proof fn test_boundary_empty_vs_nonempty_vec(a: Vec<u8>, b: Vec<u8>)
    requires a@.len() == 0, b@.len() > 0
    ensures a.view_equal(&b) // SHOULD FAIL
{
}

// Test 4: usize::MAX vs 0 asserted as view_equal
// Edge case: extreme value boundary
// SHOULD FAIL: MAX != 0
proof fn test_boundary_max_vs_zero_usize(a: usize, b: usize)
    requires a@ == usize::MAX as int, b@ == 0int
    ensures a.view_equal(&b) // SHOULD FAIL
{
}

}
