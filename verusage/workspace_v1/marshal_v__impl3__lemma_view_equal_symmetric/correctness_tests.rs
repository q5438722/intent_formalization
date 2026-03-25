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

// SHOULD FAIL: Some vs None cannot be view_equal
proof fn test_boundary_some_equals_none<T: Marshalable>(a: T) {
    assert(Option::Some(a).view_equal(&Option::None));
}

// SHOULD FAIL: None vs Some cannot be view_equal
proof fn test_boundary_none_equals_some<T: Marshalable>(a: T) {
    assert(Option::None::<T>.view_equal(&Option::Some(a)));
}

// SHOULD FAIL: Tuple requires BOTH components; only first matches
proof fn test_boundary_tuple_partial_first_only<T: Marshalable, U: Marshalable>(
    a: T, b: T, c: U, d: U,
)
    requires a.view_equal(&b), !c.view_equal(&d),
{
    assert((a, c).view_equal(&(b, d)));
}

// SHOULD FAIL: Tuple requires BOTH components; only second matches
proof fn test_boundary_tuple_partial_second_only<T: Marshalable, U: Marshalable>(
    a: T, b: T, c: U, d: U,
)
    requires !a.view_equal(&b), c.view_equal(&d),
{
    assert((a, c).view_equal(&(b, d)));
}

// SHOULD FAIL: Nested Option mismatch — Some(Some(a)) vs Some(None)
proof fn test_boundary_nested_option_mismatch<T: Marshalable>(a: T) {
    let x = Option::Some(Option::Some(a));
    let y = Option::Some(Option::None::<T>);
    assert(x.view_equal(&y));
}

// ═══════════════════════════════════════════════
// BEHAVIORAL MUTATION TESTS
// ═══════════════════════════════════════════════

// SHOULD FAIL: Mutated output — None-None is true, assert it's false
proof fn test_mutation_none_none_is_false<T: Marshalable>() {
    assert(!Option::None::<T>.view_equal(&Option::None::<T>));
}

// SHOULD FAIL: After proving symmetry, assert the negation
proof fn test_mutation_symmetry_negated<T: Marshalable>(a: T, b: T)
    requires a.view_equal(&b),
{
    a.lemma_view_equal_symmetric(&b);
    assert(!b.view_equal(&a));
}

// SHOULD FAIL: Assert view_equal(a,b) AND NOT view_equal(b,a) — contradicts symmetry
proof fn test_mutation_anti_symmetric<T: Marshalable>(a: T, b: T)
    requires a.view_equal(&b),
{
    a.lemma_view_equal_symmetric(&b);
    assert(a.view_equal(&b) && !b.view_equal(&a));
}

// SHOULD FAIL: Both components match but assert tuple NOT equal
proof fn test_mutation_tuple_equal_negated<T: Marshalable, U: Marshalable>(
    a: T, b: T, c: U, d: U,
)
    requires a.view_equal(&b), c.view_equal(&d),
{
    assert(!(a, c).view_equal(&(b, d)));
}

// SHOULD FAIL: Inner elements match but assert Option::Some NOT equal
proof fn test_mutation_some_equal_negated<T: Marshalable>(a: T, b: T)
    requires a.view_equal(&b),
{
    assert(!Option::Some(a).view_equal(&Option::Some(b)));
}

// ═══════════════════════════════════════════════
// LOGICAL TESTS
// ═══════════════════════════════════════════════

// SHOULD FAIL: Reflexivity NOT guaranteed by the spec
proof fn test_logical_reflexivity<T: Marshalable>(a: T) {
    assert(a.view_equal(&a));
}

// SHOULD FAIL: Transitivity NOT guaranteed by the spec
proof fn test_logical_transitivity<T: Marshalable>(a: T, b: T, c: T)
    requires a.view_equal(&b), b.view_equal(&c),
{
    assert(a.view_equal(&c));
}

// SHOULD FAIL: view_equal does NOT imply Rust structural equality
proof fn test_logical_view_equal_implies_eq<T: Marshalable>(a: T, b: T)
    requires a.view_equal(&b),
{
    assert(a == b);
}

// SHOULD FAIL: Common neighbor — NOT an equivalence relation necessarily
proof fn test_logical_common_neighbor<T: Marshalable>(a: T, b: T, c: T)
    requires a.view_equal(&b), a.view_equal(&c),
{
    assert(b.view_equal(&c));
}

// SHOULD FAIL: Option reflexivity NOT guaranteed (inner reflexivity isn't)
proof fn test_logical_option_reflexivity<T: Marshalable>(a: T) {
    assert(Option::Some(a).view_equal(&Option::Some(a)));
}

// SHOULD FAIL: Tuple reflexivity NOT guaranteed
proof fn test_logical_tuple_reflexivity<T: Marshalable, U: Marshalable>(a: T, b: U) {
    assert((a, b).view_equal(&(a, b)));
}

}
