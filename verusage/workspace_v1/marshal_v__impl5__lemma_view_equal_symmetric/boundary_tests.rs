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

impl<T: Marshalable, U: Marshalable> Marshalable for (T, U) {
    open spec fn view_equal(&self, other: &Self) -> bool {
        self.0.view_equal(&other.0) && self.1.view_equal(&other.1)
    }

    #[verifier::spinoff_prover]
    proof fn lemma_view_equal_symmetric(&self, other: &Self) {
        self.0.lemma_view_equal_symmetric(&other.0);
        self.1.lemma_view_equal_symmetric(&other.1);
    }
}

// ═══════════════════════════════════════════════
// BOUNDARY TESTS
// ═══════════════════════════════════════════════

// SHOULD FAIL: Tuple view_equal requires BOTH components; only first matches
proof fn test_boundary_tuple_partial_first_only<T: Marshalable, U: Marshalable>(
    a: T, b: T, c: U, d: U,
)
    requires
        a.view_equal(&b),
        !c.view_equal(&d),
{
    assert((a, c).view_equal(&(b, d)));
}

// SHOULD FAIL: Tuple view_equal requires BOTH components; only second matches
proof fn test_boundary_tuple_partial_second_only<T: Marshalable, U: Marshalable>(
    a: T, b: T, c: U, d: U,
)
    requires
        !a.view_equal(&b),
        c.view_equal(&d),
{
    assert((a, c).view_equal(&(b, d)));
}

// SHOULD FAIL: Neither component matches, yet asserting tuple view_equal
proof fn test_boundary_tuple_neither_component<T: Marshalable, U: Marshalable>(
    a: T, b: T, c: U, d: U,
)
    requires
        !a.view_equal(&b),
        !c.view_equal(&d),
{
    assert((a, c).view_equal(&(b, d)));
}

// SHOULD FAIL: Asserting arbitrary distinct values are always view_equal
proof fn test_boundary_arbitrary_always_equal<T: Marshalable>(a: T, b: T) {
    assert(a.view_equal(&b));
}

// SHOULD FAIL: Asserting arbitrary values are always NOT view_equal
proof fn test_boundary_arbitrary_always_unequal<T: Marshalable>(a: T, b: T) {
    assert(!a.view_equal(&b));
}

}
