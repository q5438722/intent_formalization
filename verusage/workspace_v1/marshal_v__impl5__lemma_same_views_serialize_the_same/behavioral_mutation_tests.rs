use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

// === Trait and implementation definitions (from source) ===

pub trait Marshalable : Sized {
    spec fn is_marshalable(&self) -> bool;

    #[verifier::external_body]
    spec fn ghost_serialize(&self) -> Seq<u8>
        recommends self.is_marshalable()
    { unimplemented!() }

    spec fn view_equal(&self, other: &Self) -> bool;

    #[verifier::external_body]
    proof fn lemma_same_views_serialize_the_same(&self, other: &Self)
        requires
            self.view_equal(other),
        ensures
            self.is_marshalable() == other.is_marshalable(),
            self.ghost_serialize() == other.ghost_serialize()
    { unimplemented!() }
}

impl Marshalable for u64 {
    open spec fn view_equal(&self, other: &Self) -> bool {
        self@ === other@
    }

    open spec fn is_marshalable(&self) -> bool {
        true
    }

    open spec fn ghost_serialize(&self) -> Seq<u8> {
        spec_u64_to_le_bytes(*self)
    }

    #[verifier::external_body]
    proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
    { unimplemented!() }
}

impl Marshalable for usize {
    open spec fn view_equal(&self, other: &Self) -> bool {
        self@ === other@
    }

    open spec fn is_marshalable(&self) -> bool {
        &&& *self as int <= u64::MAX
    }

    open spec fn ghost_serialize(&self) -> Seq<u8> {
        (*self as u64).ghost_serialize()
    }

    #[verifier::external_body]
    proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
    { unimplemented!() }
}

impl<T: Marshalable, U: Marshalable> Marshalable for (T, U) {
    open spec fn view_equal(&self, other: &Self) -> bool {
        self.0.view_equal(&other.0) && self.1.view_equal(&other.1)
    }

    open spec fn is_marshalable(&self) -> bool {
        &&& self.0.is_marshalable()
        &&& self.1.is_marshalable()
        &&& self.0.ghost_serialize().len() + self.1.ghost_serialize().len() <= usize::MAX
    }

    open spec fn ghost_serialize(&self) -> Seq<u8> {
        self.0.ghost_serialize() + self.1.ghost_serialize()
    }

    #[verifier::spinoff_prover]
    proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
    {
        self.0.lemma_same_views_serialize_the_same(&other.0);
        self.1.lemma_same_views_serialize_the_same(&other.1);
    }
}

// === Behavioral Mutation Tests ===
// Each test starts from valid inputs (view_equal holds), calls the lemma,
// then asserts the NEGATION of the postcondition — a mutated/incorrect behavior.
// These should all FAIL because the assertion contradicts the ensures clause.

// SHOULD FAIL: mutated postcondition — assert serializations DIFFER for view-equal u64
proof fn test_mutation_u64_serialize_differs() {
    let a: u64 = 42u64;
    let b: u64 = 42u64;
    a.lemma_same_views_serialize_the_same(&b);
    assert(!(a.ghost_serialize() =~= b.ghost_serialize()));
}

// SHOULD FAIL: mutated postcondition — assert marshalability DIFFERS for view-equal u64
proof fn test_mutation_u64_marshalability_differs() {
    let a: u64 = 42u64;
    let b: u64 = 42u64;
    a.lemma_same_views_serialize_the_same(&b);
    assert(a.is_marshalable() != b.is_marshalable());
}

// SHOULD FAIL: mutated postcondition — assert serializations DIFFER for view-equal tuples
proof fn test_mutation_tuple_serialize_differs() {
    let a: (u64, u64) = (1u64, 2u64);
    let b: (u64, u64) = (1u64, 2u64);
    a.lemma_same_views_serialize_the_same(&b);
    assert(!(a.ghost_serialize() =~= b.ghost_serialize()));
}

// SHOULD FAIL: mutated postcondition — assert marshalability DIFFERS for view-equal usize
proof fn test_mutation_usize_marshalability_differs() {
    let a: usize = 10usize;
    let b: usize = 10usize;
    a.lemma_same_views_serialize_the_same(&b);
    assert(a.is_marshalable() != b.is_marshalable());
}

// SHOULD FAIL: mutated postcondition — assert serializations DIFFER for view-equal usize
proof fn test_mutation_usize_serialize_differs() {
    let a: usize = 10usize;
    let b: usize = 10usize;
    a.lemma_same_views_serialize_the_same(&b);
    assert(!(a.ghost_serialize() =~= b.ghost_serialize()));
}

}
