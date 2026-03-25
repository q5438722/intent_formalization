use vstd::prelude::*;

fn main() {}

verus! {

// ─── Definitions (from target) ───

pub trait Marshalable : Sized {
    spec fn view_equal(&self, other: &Self) -> bool;

    #[verifier::external_body]
    proof fn lemma_view_equal_symmetric(&self, other: &Self)
        ensures self.view_equal(other) == other.view_equal(self)
    { unimplemented!() }
}

impl<T: Marshalable> Marshalable for Option<T> {
    open spec fn view_equal(&self, other: &Self) -> bool {
        match (self, other) {
            (None, None) => true,
            (Some(s), Some(o)) => s.view_equal(o),
            _ => false,
        }
    }

    #[verifier::spinoff_prover]
    proof fn lemma_view_equal_symmetric(&self, other: &Self) {
        match (self, other) {
            (None, None) => (),
            (Some(s), Some(o)) => s.lemma_view_equal_symmetric(o),
            _ => (),
        }
    }
}

impl<T: Marshalable, U: Marshalable> Marshalable for (T, U) {
    open spec fn view_equal(&self, other: &Self) -> bool {
        self.0.view_equal(&other.0) && self.1.view_equal(&other.1)
    }

    #[verifier::external_body]
    #[verifier::spinoff_prover]
    proof fn lemma_view_equal_symmetric(&self, other: &Self) {
        unimplemented!()
    }
}

// ═══════════════════════════════════════════════
// BEHAVIORAL MUTATION TESTS
// ═══════════════════════════════════════════════

// SHOULD FAIL: Mutated output — None-None view_equal is true, not false
proof fn test_mutation_none_none_is_false<T: Marshalable>() {
    assert(!Option::None::<T>.view_equal(&Option::None::<T>));
}

// SHOULD FAIL: Mutated output — after proving symmetry, assert the negation
proof fn test_mutation_symmetry_negated<T: Marshalable>(a: T, b: T)
    requires
        a.view_equal(&b),
{
    a.lemma_view_equal_symmetric(&b);
    assert(!b.view_equal(&a));
}

// SHOULD FAIL: Mutated relation — assert view_equal(a,b) implies NOT view_equal(b,a)
proof fn test_mutation_anti_symmetric<T: Marshalable>(a: T, b: T)
    requires
        a.view_equal(&b),
{
    a.lemma_view_equal_symmetric(&b);
    assert(a.view_equal(&b) && !b.view_equal(&a));
}

// SHOULD FAIL: Mutated output — tuple equality should hold, assert it doesn't
proof fn test_mutation_tuple_equal_negated<T: Marshalable, U: Marshalable>(
    a: T, b: T, c: U, d: U,
)
    requires
        a.view_equal(&b),
        c.view_equal(&d),
{
    assert(!(a, c).view_equal(&(b, d)));
}

// SHOULD FAIL: Mutated output — Option::Some equality is delegated to inner;
// if inner matches, assert outer doesn't
proof fn test_mutation_some_equal_negated<T: Marshalable>(a: T, b: T)
    requires
        a.view_equal(&b),
{
    assert(!Option::Some(a).view_equal(&Option::Some(b)));
}

}
