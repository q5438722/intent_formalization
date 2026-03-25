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
// LOGICAL TESTS
// ═══════════════════════════════════════════════

// SHOULD FAIL: Reflexivity is NOT guaranteed by the spec
proof fn test_logical_reflexivity<T: Marshalable>(a: T) {
    assert(a.view_equal(&a));
}

// SHOULD FAIL: Transitivity is NOT guaranteed by the spec
proof fn test_logical_transitivity<T: Marshalable>(a: T, b: T, c: T)
    requires
        a.view_equal(&b),
        b.view_equal(&c),
{
    assert(a.view_equal(&c));
}

// SHOULD FAIL: view_equal does NOT imply Rust equality (==)
proof fn test_logical_view_equal_implies_eq<T: Marshalable>(a: T, b: T)
    requires
        a.view_equal(&b),
{
    assert(a == b);
}

// SHOULD FAIL: If a is view_equal to b and a is view_equal to c,
// b and c are NOT necessarily view_equal (not an equivalence relation)
proof fn test_logical_common_neighbor<T: Marshalable>(a: T, b: T, c: T)
    requires
        a.view_equal(&b),
        a.view_equal(&c),
{
    assert(b.view_equal(&c));
}

// SHOULD FAIL: Option reflexivity is NOT guaranteed because inner reflexivity isn't
proof fn test_logical_option_reflexivity<T: Marshalable>(a: T) {
    assert(Option::Some(a).view_equal(&Option::Some(a)));
}

// SHOULD FAIL: Tuple reflexivity is NOT guaranteed
proof fn test_logical_tuple_reflexivity<T: Marshalable, U: Marshalable>(a: T, b: U) {
    assert((a, b).view_equal(&(a, b)));
}

}
