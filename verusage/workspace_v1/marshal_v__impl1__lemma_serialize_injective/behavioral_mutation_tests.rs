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

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Mutate output — claim distinct u64 values are view_equal
// SHOULD FAIL
proof fn test_mutation_distinct_values_view_equal() {
    let a: u64 = 1u64;
    let b: u64 = 2u64;
    // 1 != 2, so view_equal (which checks self@ === other@) must be false
    assert(a.view_equal(&b));
}

// Test 2: Mutate output — claim identical values have different serializations
// SHOULD FAIL
proof fn test_mutation_same_value_different_serialize() {
    let a: u64 = 42u64;
    let b: u64 = 42u64;
    // Same value must serialize identically
    assert(a.ghost_serialize() != b.ghost_serialize());
}

// Test 3: Negate ensures — call lemma correctly then assert the NEGATION of the postcondition
// SHOULD FAIL
proof fn test_mutation_negate_ensures_clause() {
    let a: u64 = 42u64;
    let b: u64 = 42u64;
    a.lemma_serialize_injective(&b);
    // Lemma guarantees view_equal; asserting its negation must fail
    assert(!a.view_equal(&b));
}

// Test 4: Assert serialization produces wrong length (4 bytes instead of 8)
// SHOULD FAIL
proof fn test_mutation_wrong_serialize_length() {
    let a: u64 = 42u64;
    assert(a.ghost_serialize().len() == 4);
}

// Test 5: Assert usize 0 and usize 1 have the same serialization
// SHOULD FAIL
proof fn test_mutation_usize_same_serialize() {
    let a: usize = 0usize;
    let b: usize = 1usize;
    assert(a.ghost_serialize() == b.ghost_serialize());
}

}
