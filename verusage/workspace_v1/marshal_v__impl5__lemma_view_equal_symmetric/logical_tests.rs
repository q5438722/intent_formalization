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
// LOGICAL TESTS
// ═══════════════════════════════════════════════

// SHOULD FAIL: Reflexivity is NOT guaranteed by symmetry alone
proof fn test_logical_reflexivity<T: Marshalable>(a: T) {
    assert(a.view_equal(&a));
}

// SHOULD FAIL: Transitivity is NOT guaranteed by symmetry alone
proof fn test_logical_transitivity<T: Marshalable>(a: T, b: T, c: T)
    requires
        a.view_equal(&b),
        b.view_equal(&c),
{
    assert(a.view_equal(&c));
}

// SHOULD FAIL: view_equal does NOT imply structural equality (==)
proof fn test_logical_view_equal_implies_eq<T: Marshalable>(a: T, b: T)
    requires
        a.view_equal(&b),
{
    assert(a == b);
}

// SHOULD FAIL: Symmetry + common neighbor does NOT yield equivalence
// (a ~= b) && (a ~= c) does NOT imply (b ~= c) without transitivity
proof fn test_logical_common_neighbor<T: Marshalable>(a: T, b: T, c: T)
    requires
        a.view_equal(&b),
        a.view_equal(&c),
{
    assert(b.view_equal(&c));
}

// SHOULD FAIL: Tuple reflexivity is NOT guaranteed (components lack reflexivity)
proof fn test_logical_tuple_reflexivity<T: Marshalable, U: Marshalable>(a: T, b: U) {
    assert((a, b).view_equal(&(a, b)));
}

// SHOULD FAIL: Tuple view_equal does NOT imply component equality (==)
proof fn test_logical_tuple_view_equal_implies_component_eq<T: Marshalable, U: Marshalable>(
    a: T, b: T, c: U, d: U,
)
    requires
        (a, c).view_equal(&(b, d)),
{
    assert(a == b);
}

}
