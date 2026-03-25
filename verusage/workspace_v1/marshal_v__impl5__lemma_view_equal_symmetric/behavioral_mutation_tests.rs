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
// BEHAVIORAL MUTATION TESTS
// ═══════════════════════════════════════════════

// SHOULD FAIL: Symmetry is guaranteed — asserting its negation must fail
proof fn test_mutation_symmetry_negated<T: Marshalable>(a: T, b: T)
    requires
        a.view_equal(&b),
{
    a.lemma_view_equal_symmetric(&b);
    assert(!b.view_equal(&a));
}

// SHOULD FAIL: Anti-symmetry — view_equal(a,b) does NOT imply NOT view_equal(b,a)
proof fn test_mutation_anti_symmetric<T: Marshalable>(a: T, b: T)
    requires
        a.view_equal(&b),
{
    a.lemma_view_equal_symmetric(&b);
    assert(a.view_equal(&b) && !b.view_equal(&a));
}

// SHOULD FAIL: Tuple equality holds when both components match — negate the result
proof fn test_mutation_tuple_equal_negated<T: Marshalable, U: Marshalable>(
    a: T, b: T, c: U, d: U,
)
    requires
        a.view_equal(&b),
        c.view_equal(&d),
{
    assert(!(a, c).view_equal(&(b, d)));
}

// SHOULD FAIL: Tuple symmetry negated — if (a,c).view_equal((b,d)) then
// (b,d).view_equal((a,c)) should hold, not fail
proof fn test_mutation_tuple_symmetry_negated<T: Marshalable, U: Marshalable>(
    a: T, b: T, c: U, d: U,
)
    requires
        a.view_equal(&b),
        c.view_equal(&d),
{
    (a, c).lemma_view_equal_symmetric(&(b, d));
    assert(!(b, d).view_equal(&(a, c)));
}

// SHOULD FAIL: Mutated claim — component view_equal being true implies tuple is false
proof fn test_mutation_components_true_tuple_false<T: Marshalable, U: Marshalable>(
    a: T, b: T, c: U, d: U,
)
    requires
        a.view_equal(&b),
        c.view_equal(&d),
{
    assert(!((a, c).view_equal(&(b, d))) || !(a.view_equal(&b)));
}

}
