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
