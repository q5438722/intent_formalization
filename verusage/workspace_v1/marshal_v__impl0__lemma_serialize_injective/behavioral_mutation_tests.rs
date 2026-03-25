use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;
use vstd::bytes::*;
use vstd::slice::*;

fn main() {}

verus! {

// ─── Definitions (from target) ───

pub trait Marshalable : Sized {
    spec fn is_marshalable(&self) -> bool;

    #[verifier::external_body]
    spec fn ghost_serialize(&self) -> Seq<u8>
        recommends self.is_marshalable()
    {unimplemented!()}

    spec fn view_equal(&self, other: &Self) -> bool;

    #[verifier::external_body]
    proof fn lemma_serialize_injective(&self, other: &Self)
        requires
            self.is_marshalable(),
            other.is_marshalable(),
            self.ghost_serialize() == other.ghost_serialize(),
        ensures
            self.view_equal(other),
    {unimplemented!()}
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

    proof fn lemma_serialize_injective(self: &Self, other: &Self) {
        lemma_auto_spec_u64_to_from_le_bytes();
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

    #[verifier::external_body]
    proof fn lemma_serialize_injective(self: &Self, other: &Self) {
        unimplemented!()
    }
}

// ═══════════════════════════════════════════════
// BEHAVIORAL MUTATION TESTS
// ═══════════════════════════════════════════════

// SHOULD FAIL: Negate postcondition — same serialization but assert NOT view_equal
proof fn test_mutation_negate_postcondition(a: u64, b: u64)
    requires a@ == 42, b@ == 42,
{
    a.lemma_serialize_injective(&b);
    assert(!a.view_equal(&b));
}

// SHOULD FAIL: Assert wrong serialization equality — 0 and 1 serialize differently
proof fn test_mutation_wrong_serialize_equality(a: u64, b: u64)
    requires a@ == 0, b@ == 1,
{
    assert(a.ghost_serialize() == b.ghost_serialize());
}

// SHOULD FAIL: Strengthen postcondition — after proving view_equal, assert arithmetic relation
proof fn test_mutation_strengthen_postcondition(a: u64, b: u64)
    requires a@ == 42, b@ == 42,
{
    a.lemma_serialize_injective(&b);
    // The lemma gives view_equal (a@ === b@), but try to derive a@ == b@ + 1
    assert(a@ == b@ + 1);
}

// SHOULD FAIL: Directly assert different u64 values are view_equal (mutated claim)
proof fn test_mutation_different_values_view_equal(a: u64, b: u64)
    requires a@ == 100, b@ == 200,
{
    assert(a.view_equal(&b));
}

// SHOULD FAIL: Mutate postcondition to claim a@ != b@ after valid lemma call
proof fn test_mutation_assert_unequal_after_lemma(a: u64, b: u64)
    requires
        a.is_marshalable(),
        b.is_marshalable(),
        a.ghost_serialize() == b.ghost_serialize(),
{
    a.lemma_serialize_injective(&b);
    // The lemma ensures view_equal, meaning a@ === b@
    // Mutated: assert they are NOT equal
    assert(a@ != b@);
}

}
