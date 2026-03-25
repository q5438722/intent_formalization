use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

// ========== SPEC (copied from source) ==========

pub trait Marshalable : Sized {
    spec fn is_marshalable(&self) -> bool;

    #[verifier::external_body]
    spec fn ghost_serialize(&self) -> Seq<u8>
        recommends self.is_marshalable()
    { unimplemented!() }

    spec fn view_equal(&self, other: &Self) -> bool;

    #[verifier::external_body]
    proof fn lemma_serialize_injective(&self, other: &Self)
        requires
            self.is_marshalable(),
            other.is_marshalable(),
            self.ghost_serialize() == other.ghost_serialize(),
        ensures
            self.view_equal(other),
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
    proof fn lemma_serialize_injective(self: &Self, other: &Self) {
        unimplemented!()
    }
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

    proof fn lemma_serialize_injective(self: &Self, other: &Self) {
        (*self as u64).lemma_serialize_injective(&(*other as u64));
    }
}

// ========== LOGICAL TESTS ==========

// Test 1: Assert all u64 values serialize to the same bytes (universally false)
// This tests whether the spec improperly collapses the serialization function
// SHOULD FAIL
proof fn test_logical_all_serialize_same() {
    let a: u64 = 0u64;
    let b: u64 = 1u64;
    // Different values must produce different serializations
    assert(a.ghost_serialize() == b.ghost_serialize());
}

// Test 2: Assert view_equal is NOT reflexive (there exists a value not equal to itself)
// The spec doesn't explicitly state reflexivity, but for u64 it follows from === semantics
// SHOULD FAIL
proof fn test_logical_view_equal_not_reflexive() {
    let a: u64 = 42u64;
    // view_equal(a, a) is a@ === a@ which is trivially true
    assert(!a.view_equal(&a));
}

// Test 3: Assert there exist two distinct u64 values with identical serialization
// This would violate injectivity of spec_u64_to_le_bytes
// SHOULD FAIL
proof fn test_logical_exists_collision() {
    assert(exists|a: u64, b: u64| a@ != b@ && spec_u64_to_le_bytes(a) == spec_u64_to_le_bytes(b));
}

// Test 4: Assert symmetry of view_equal follows from one-directional lemma application
// The lemma ensures self.view_equal(other), NOT other.view_equal(self)
// For u64 this holds due to === being symmetric, but it's NOT guaranteed by the trait spec
// SHOULD FAIL (from trait-spec perspective, may pass due to open spec fn)
proof fn test_logical_symmetry_beyond_spec() {
    let a: u64 = 10u64;
    let b: u64 = 10u64;
    a.lemma_serialize_injective(&b);
    // Lemma gives a.view_equal(&b). Try to also get the converse:
    assert(b.view_equal(&a));
}

// Test 5: Assert the converse of the lemma: view_equal implies serialize_equal
// The spec ONLY states: serialize_equal => view_equal (injectivity)
// The converse (view_equal => serialize_equal) is NOT part of the spec
// SHOULD FAIL (from spec perspective, may pass for u64 due to open spec)
proof fn test_logical_converse_of_lemma() {
    let a: u64 = 42u64;
    let b: u64 = 42u64;
    // Without calling the lemma, assert view_equal => same serialization
    assert(a.view_equal(&b) ==> a.ghost_serialize() == b.ghost_serialize());
}

// Test 6: Assert a global structural property — serialization length is always 8 for u64
// This is a property of spec_u64_to_le_bytes, NOT stated in the Marshalable trait
// SHOULD FAIL (from trait perspective, may pass due to vstd axioms)
proof fn test_logical_fixed_serialize_length() {
    let a: u64 = 0u64;
    assert(a.ghost_serialize().len() == 8);
}

}
