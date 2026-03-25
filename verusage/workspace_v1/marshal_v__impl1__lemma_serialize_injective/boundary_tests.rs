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

// ========== BOUNDARY TESTS ==========

// Test 1: Violate precondition — call lemma with values that serialize differently
// SHOULD FAIL
proof fn test_boundary_different_serializations() {
    let a: u64 = 0u64;
    let b: u64 = 1u64;
    // Precondition violated: ghost_serialize(0) != ghost_serialize(1)
    a.lemma_serialize_injective(&b);
}

// Test 2: Assert view_equal for distinct values without calling lemma
// SHOULD FAIL
proof fn test_boundary_assert_view_equal_no_proof() {
    let a: u64 = 100u64;
    let b: u64 = 200u64;
    // No lemma call, no basis for view_equal
    assert(a.view_equal(&b));
}

// Test 3: Violate precondition with extreme edge cases (MAX vs 0)
// SHOULD FAIL
proof fn test_boundary_max_vs_zero() {
    let a: u64 = u64::MAX;
    let b: u64 = 0u64;
    // Precondition violated: MAX and 0 serialize differently
    a.lemma_serialize_injective(&b);
}

// Test 4: Call lemma with adjacent u64 values (off-by-one edge case)
// SHOULD FAIL
proof fn test_boundary_adjacent_values() {
    let a: u64 = 255u64;
    let b: u64 = 256u64;
    // Adjacent values at byte boundary — still serialize differently
    a.lemma_serialize_injective(&b);
}

// Test 5: Assert view_equal between a usize and deduce a wrong conclusion
// SHOULD FAIL
proof fn test_boundary_usize_wrong_view_equal() {
    let a: usize = 0usize;
    let b: usize = 1usize;
    assert(a.view_equal(&b));
}

}
