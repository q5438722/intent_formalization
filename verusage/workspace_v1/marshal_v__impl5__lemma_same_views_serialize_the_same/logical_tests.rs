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

// === Logical Tests ===
// Each test asserts a property NOT explicitly guaranteed by the specification.
// These probe the semantic boundary of the spec for unintended entailments.

// SHOULD FAIL: tuple serialization is NOT commutative — (a,b) != (b,a) when a != b
// The spec concatenates component serializations, so order matters.
// This tests whether the spec incorrectly entails commutativity.
proof fn test_logical_tuple_serialize_commutative() {
    let a: (u64, u64) = (0u64, 1u64);
    let b: (u64, u64) = (1u64, 0u64);
    // Assert they serialize the same (they shouldn't — order matters)
    assert(a.ghost_serialize() =~= b.ghost_serialize());
}

// NOTE: `forall |x: usize| x.is_marshalable()` PASSES verification, revealing that
// the is_marshalable guard for usize (<= u64::MAX) is vacuous on 64-bit architectures.
// This is a spec weakness: the guard appears restrictive but restricts nothing.

// SHOULD FAIL: the spec does NOT entail that all u64 values serialize identically.
// ghost_serialize is injective (distinct values → distinct bytes), so this is false.
proof fn test_logical_all_u64_serialize_same() {
    assert(forall |x: u64, y: u64| x.ghost_serialize() =~= y.ghost_serialize());
}

// SHOULD FAIL: tuple serialize length is NOT equal to just one component's length
// ghost_serialize for (T,U) is T.serialize() ++ U.serialize(), so length is the SUM.
// This tests whether the spec allows confusing tuple length with component length.
proof fn test_logical_tuple_length_equals_component() {
    let t: (u64, u64) = (1u64, 2u64);
    assert(t.ghost_serialize().len() == t.0.ghost_serialize().len());
}

// SHOULD FAIL: the spec does NOT guarantee that non-view-equal values are distinguishable
// by marshalability. Two non-view-equal u64 values are both marshalable.
// Trying to prove one is NOT marshalable should fail.
proof fn test_logical_non_view_equal_implies_not_marshalable() {
    let a: u64 = 0u64;
    let b: u64 = 1u64;
    assert(!a.view_equal(&b));
    assert(!b.is_marshalable());
}

// SHOULD FAIL: the spec does NOT guarantee that view_equal can be derived from
// equal serialization (converse of the lemma). This property is not entailed.
proof fn test_logical_equal_serialize_implies_view_equal() {
    let a: u64 = 0u64;
    let b: u64 = 0u64;
    // Even though a and b happen to be the same, try to prove the general converse:
    // assert the converse as a universal property — NOT guaranteed by the spec
    assert(forall |x: u64, y: u64|
        x.ghost_serialize() =~= y.ghost_serialize() ==> x.view_equal(&y));
}

}
