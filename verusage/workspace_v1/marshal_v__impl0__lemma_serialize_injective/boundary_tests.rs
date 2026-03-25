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

}
