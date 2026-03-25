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

// === Boundary Tests ===
// Each test violates the precondition (view_equal) of lemma_same_views_serialize_the_same.
// These should all FAIL verification because the requires clause is not satisfied.

// SHOULD FAIL: precondition violated — two distinct u64 values are not view_equal
proof fn test_boundary_u64_non_view_equal() {
    let a: u64 = 0u64;
    let b: u64 = 1u64;
    a.lemma_same_views_serialize_the_same(&b);
}

// SHOULD FAIL: precondition violated — two distinct usize values are not view_equal
proof fn test_boundary_usize_non_view_equal() {
    let a: usize = 0usize;
    let b: usize = 42usize;
    a.lemma_same_views_serialize_the_same(&b);
}

// SHOULD FAIL: precondition violated — tuples differ in first component
proof fn test_boundary_tuple_non_view_equal_first() {
    let a: (u64, u64) = (0u64, 5u64);
    let b: (u64, u64) = (1u64, 5u64);
    a.lemma_same_views_serialize_the_same(&b);
}

// SHOULD FAIL: precondition violated — tuples differ in second component
proof fn test_boundary_tuple_non_view_equal_second() {
    let a: (u64, u64) = (5u64, 0u64);
    let b: (u64, u64) = (5u64, 1u64);
    a.lemma_same_views_serialize_the_same(&b);
}

// SHOULD FAIL: precondition violated — edge case with max u64 value
proof fn test_boundary_u64_zero_vs_max() {
    let a: u64 = 0u64;
    let b: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;
    a.lemma_same_views_serialize_the_same(&b);
}

}
