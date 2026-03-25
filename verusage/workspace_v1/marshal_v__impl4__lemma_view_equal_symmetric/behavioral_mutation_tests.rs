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

// ===== BEHAVIORAL MUTATION TESTS =====

// Test 1: Calling symmetry lemma on different values doesn't create equality
// Valid call, mutated assertion: assert equal when values differ
// SHOULD FAIL: symmetry preserves the truth value, doesn't change it
proof fn test_mutation_symmetry_no_false_equality(a: usize, b: usize)
    requires a@ != b@
    ensures a.view_equal(&b) // SHOULD FAIL
{
    a.lemma_view_equal_symmetric(&b);
}

// Test 2: Calling symmetry lemma on equal values, then negate equality
// Valid call, mutated result: negate true → false
// SHOULD FAIL: a@ === a@ is true, negation is false
proof fn test_mutation_negate_true_equality(a: usize)
    ensures !a.view_equal(&a) // SHOULD FAIL
{
    a.lemma_view_equal_symmetric(&a);
}

// Test 3: Symmetry contradiction — assert reverse direction is false
// Given a.view_equal(b), mutate to assert !b.view_equal(a)
// SHOULD FAIL: directly contradicts symmetry ensures clause
proof fn test_mutation_symmetry_reverse_negated(a: usize, b: usize)
    requires a.view_equal(&b)
    ensures !b.view_equal(&a) // SHOULD FAIL
{
    a.lemma_view_equal_symmetric(&b);
}

// Test 4: Vec with different lengths — symmetry doesn't fix length mismatch
// Valid symmetry call, mutated assertion: equal despite length difference
// SHOULD FAIL: lengths differ, view_equal is false regardless of symmetry
proof fn test_mutation_vec_length_mismatch_after_symmetry(a: Vec<u8>, b: Vec<u8>)
    requires a@.len() != b@.len()
    ensures a.view_equal(&b) // SHOULD FAIL
{
    a.lemma_view_equal_symmetric(&b);
}

}
