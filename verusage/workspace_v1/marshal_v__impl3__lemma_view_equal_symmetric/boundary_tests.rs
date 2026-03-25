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
// BOUNDARY TESTS
// ═══════════════════════════════════════════════

// SHOULD FAIL: Mismatched Option variants — Some vs None cannot be view_equal
proof fn test_boundary_some_equals_none<T: Marshalable>(a: T) {
    assert(Option::Some(a).view_equal(&Option::None));
}

// SHOULD FAIL: Mismatched Option variants (reversed) — None vs Some cannot be view_equal
proof fn test_boundary_none_equals_some<T: Marshalable>(a: T) {
    assert(Option::None::<T>.view_equal(&Option::Some(a)));
}

// SHOULD FAIL: Tuple view_equal requires BOTH components to match; only first matches
proof fn test_boundary_tuple_partial_first_only<T: Marshalable, U: Marshalable>(
    a: T, b: T, c: U, d: U,
)
    requires
        a.view_equal(&b),
        !c.view_equal(&d),
{
    assert((a, c).view_equal(&(b, d)));
}

// SHOULD FAIL: Tuple view_equal requires BOTH components to match; only second matches
proof fn test_boundary_tuple_partial_second_only<T: Marshalable, U: Marshalable>(
    a: T, b: T, c: U, d: U,
)
    requires
        !a.view_equal(&b),
        c.view_equal(&d),
{
    assert((a, c).view_equal(&(b, d)));
}

// SHOULD FAIL: Nested Option — Some(Some(a)) vs Some(None) should not be view_equal
proof fn test_boundary_nested_option_mismatch<T: Marshalable>(a: T) {
    let x = Option::Some(Option::Some(a));
    let y = Option::Some(Option::None::<T>);
    assert(x.view_equal(&y));
}

}
