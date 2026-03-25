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
// BOUNDARY TESTS
// ═══════════════════════════════════════════════

// SHOULD FAIL: Violate equal-serialization precondition for u64
// 0 and 1 have different serializations, so lemma precondition is not met
proof fn test_boundary_violate_serialize_eq(a: u64, b: u64)
    requires a@ == 0, b@ == 1,
{
    a.lemma_serialize_injective(&b);
}

// SHOULD FAIL: Edge case — assert 0 and u64::MAX are view_equal without proof
proof fn test_boundary_zero_vs_max(a: u64, b: u64)
    requires a@ == 0, b@ == 0xFFFF_FFFF_FFFF_FFFF,
{
    assert(a.view_equal(&b));
}

// SHOULD FAIL: Adjacent values at max boundary — different serializations
proof fn test_boundary_max_adjacent(a: u64, b: u64)
    requires a@ == 0xFFFF_FFFF_FFFF_FFFF, b@ == 0xFFFF_FFFF_FFFF_FFFE,
{
    a.lemma_serialize_injective(&b);
}

// SHOULD FAIL: Violate serialization precondition for usize
// 10 and 20 have different serializations via the u64 delegation
proof fn test_boundary_usize_distinct(a: usize, b: usize)
    requires a@ == 10, b@ == 20,
        a.is_marshalable(), b.is_marshalable(),
{
    a.lemma_serialize_injective(&b);
}

// SHOULD FAIL: Only first argument is marshalable — violate second precondition
// Uses generic type where is_marshalable can be false
proof fn test_boundary_only_self_marshalable<T: Marshalable>(a: T, b: T)
    requires
        a.is_marshalable(),
        !b.is_marshalable(),
{
    a.lemma_serialize_injective(&b);
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
    assert(a@ != b@);
}

// ═══════════════════════════════════════════════
// LOGICAL TESTS
// ═══════════════════════════════════════════════

// SHOULD FAIL: Converse of injectivity at trait level is NOT guaranteed
// Spec says: same serialization → view_equal
// Converse (view_equal → same serialization) is NOT stated
proof fn test_logical_converse_injectivity<T: Marshalable>(a: T, b: T)
    requires
        a.is_marshalable(),
        b.is_marshalable(),
        a.view_equal(&b),
{
    assert(a.ghost_serialize() == b.ghost_serialize());
}

// SHOULD FAIL: Contrapositive without calling lemma at trait level
// NOT view_equal should imply different serializations, but lemma is not called
proof fn test_logical_contrapositive_without_lemma<T: Marshalable>(a: T, b: T)
    requires
        a.is_marshalable(),
        b.is_marshalable(),
        !a.view_equal(&b),
{
    assert(a.ghost_serialize() != b.ghost_serialize());
}

// SHOULD FAIL: view_equal reflexivity is NOT guaranteed by the spec at trait level
proof fn test_logical_reflexivity<T: Marshalable>(a: T)
    requires a.is_marshalable(),
{
    assert(a.view_equal(&a));
}

// SHOULD FAIL: Wrong serialization length — u64 serializes to 8 bytes, not 4
proof fn test_logical_wrong_serialize_length(a: u64) {
    assert(a.ghost_serialize().len() == 4);
}

// SHOULD FAIL: Transitivity of view_equal is NOT guaranteed by the spec
proof fn test_logical_transitivity<T: Marshalable>(a: T, b: T, c: T)
    requires
        a.is_marshalable(),
        b.is_marshalable(),
        c.is_marshalable(),
        a.view_equal(&b),
        b.view_equal(&c),
{
    assert(a.view_equal(&c));
}

}
