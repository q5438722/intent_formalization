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

// ===== LOGICAL TESTS =====

// Test 1: Reflexivity is NOT entailed by symmetry alone
// Symmetry gives a.view_equal(a) == a.view_equal(a) — a tautology, proves nothing
// SHOULD FAIL: uninterpreted view_equal at trait level has no reflexivity guarantee
proof fn test_logical_reflexivity_not_entailed<T: Marshalable>(a: &T)
    ensures a.view_equal(a) // SHOULD FAIL
{
    a.lemma_view_equal_symmetric(a);
}

// Test 2: Transitivity is NOT entailed by symmetry alone
// Given a.view_equal(b) and b.view_equal(c), symmetry cannot derive a.view_equal(c)
// SHOULD FAIL: symmetry is strictly weaker than an equivalence relation
proof fn test_logical_transitivity_not_entailed<T: Marshalable>(a: &T, b: &T, c: &T)
    requires a.view_equal(b), b.view_equal(c)
    ensures a.view_equal(c) // SHOULD FAIL
{
    a.lemma_view_equal_symmetric(b);
    b.lemma_view_equal_symmetric(c);
    a.lemma_view_equal_symmetric(c);
}

// Test 3: Anti-reflexivity is NOT entailed by symmetry
// Cannot prove a value is NOT view_equal to itself from symmetry alone
// SHOULD FAIL: symmetry says nothing about the truth value of view_equal(a, a)
proof fn test_logical_anti_reflexivity_not_entailed<T: Marshalable>(a: &T)
    ensures !a.view_equal(a) // SHOULD FAIL
{
    a.lemma_view_equal_symmetric(a);
}

// Test 4: Cross-argument misuse — symmetry of (a,b) leaks no info about (a,c)
// Knowing a.view_equal(b) and using symmetry does not imply a.view_equal(c)
// SHOULD FAIL: unrelated pair (a,c) is unconstrained by symmetry of (a,b)
proof fn test_logical_cross_argument_no_leakage<T: Marshalable>(a: &T, b: &T, c: &T)
    requires a.view_equal(b)
    ensures a.view_equal(c) // SHOULD FAIL
{
    a.lemma_view_equal_symmetric(b);
}

}
