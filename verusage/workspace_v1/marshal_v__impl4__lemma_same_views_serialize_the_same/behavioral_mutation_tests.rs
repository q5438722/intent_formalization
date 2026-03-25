use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus! {

// === Definitions from source ===

#[verifier::external_body]
pub proof fn lemma_fold_left_on_equiv_seqs<A, B>(s1: Seq<A>, s2: Seq<A>, eq: spec_fn(A, A) -> bool, init: B, f: spec_fn(B, A) -> B)
    requires
      s1.len() == s2.len(),
      (forall |i: int| 0 <= i < s1.len() ==> eq(s1[i], s2[i])),
      (forall |b: B, a1: A, a2: A| #[trigger] eq(a1, a2) ==> #[trigger] f(b, a1) == f(b, a2)),
    ensures
      s1.fold_left(init, f) == s2.fold_left(init, f)
    decreases s1.len(),
{ unimplemented!() }

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
    open spec fn is_marshalable(&self) -> bool { true }
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

impl Marshalable for Vec<u8> {
    open spec fn view_equal(&self, other: &Self) -> bool {
        self@ === other@
    }
    open spec fn is_marshalable(&self) -> bool {
        self@.len() <= usize::MAX &&
        (self@.len() as usize).ghost_serialize().len() + self@.len() as int <= usize::MAX
    }
    open spec fn ghost_serialize(&self) -> Seq<u8> {
        (self@.len() as usize).ghost_serialize() + self@
    }
    #[verifier::external_body]
    proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
    { unimplemented!() }
}

impl<T: Marshalable> Marshalable for Vec<T> {
    open spec fn view_equal(&self, other: &Self) -> bool {
        let s = self@;
        let o = other@;
        s.len() == o.len() && (forall |i: int| 0 <= i < s.len() ==> #[trigger] s[i].view_equal(&o[i]))
    }
    open spec fn is_marshalable(&self) -> bool {
        &&& self@.len() <= usize::MAX
        &&& (forall |x: T| self@.contains(x) ==> #[trigger] x.is_marshalable())
        &&& (self@.len() as usize).ghost_serialize().len() +
            self@.fold_left(0, |acc: int, x: T| acc + x.ghost_serialize().len()) <= usize::MAX
    }
    open spec fn ghost_serialize(&self) -> Seq<u8> {
        (self@.len() as usize).ghost_serialize()
          + self@.fold_left(Seq::<u8>::empty(), |acc: Seq<u8>, x: T| acc + x.ghost_serialize())
    }
    #[verifier::external_body]
    proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
    { unimplemented!() }
}

// === Behavioral Mutation Tests ===
// Valid inputs (view_equal holds), but assert mutated (incorrect) postconditions.
// These should FAIL because the assertion contradicts the lemma's ensures clause.

// SHOULD FAIL: assert serializations DIFFER for view-equal u64 values
proof fn test_mutation_u64_serialize_not_equal() {
    let a: u64 = 42u64;
    let b: u64 = 42u64;
    a.lemma_same_views_serialize_the_same(&b);
    assert(a.ghost_serialize() !== b.ghost_serialize());
}

// SHOULD FAIL: assert marshalability DIFFERS for view-equal usize values
proof fn test_mutation_usize_marshalability_differs() {
    let a: usize = 10usize;
    let b: usize = 10usize;
    a.lemma_same_views_serialize_the_same(&b);
    assert(a.is_marshalable() != b.is_marshalable());
}

// SHOULD FAIL: assert serialization of u64 has wrong length (should be 8 bytes)
proof fn test_mutation_u64_serialize_wrong_length() {
    let a: u64 = 1u64;
    let b: u64 = 1u64;
    a.lemma_same_views_serialize_the_same(&b);
    // Postcondition gives equal serialization, but here we assert wrong length
    lemma_auto_spec_u64_to_from_le_bytes();
    assert(a.ghost_serialize().len() == 4);
}

// SHOULD FAIL: assert view-equal u64 values have different marshalability
proof fn test_mutation_u64_marshalability_negated() {
    let a: u64 = 0u64;
    let b: u64 = 0u64;
    a.lemma_same_views_serialize_the_same(&b);
    assert(!a.is_marshalable());
}

// SHOULD FAIL: assert fold_left on equal sequences produces different results
proof fn test_mutation_fold_left_wrong_result() {
    let s1: Seq<u64> = seq![1u64, 2u64];
    let s2: Seq<u64> = seq![1u64, 2u64];
    let eq = |a: u64, b: u64| a@ === b@;
    let f = |acc: int, x: u64| acc + 1int;
    lemma_fold_left_on_equiv_seqs(s1, s2, eq, 0int, f);
    assert(s1.fold_left(0int, f) != s2.fold_left(0int, f));
}

}
