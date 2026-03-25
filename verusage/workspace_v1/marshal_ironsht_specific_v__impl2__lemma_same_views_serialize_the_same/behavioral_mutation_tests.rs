extern crate verus_builtin_macros as builtin_macros;
use vstd::prelude::*;
use std::collections;
use vstd::bytes::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus!{

// ============================================================
// Definitions (from source)
// ============================================================

#[verifier(external_body)]
pub struct CKeyHashMap {
  m: collections::HashMap<CKey, Vec<u8>>,
}

impl CKeyHashMap {

    pub uninterp spec fn view(self) -> Map<AbstractKey, Seq<u8>>;

    pub uninterp spec fn spec_to_vec(&self) -> Vec<CKeyKV>;
    #[verifier(external_body)]
    #[verifier(when_used_as_spec(spec_to_vec))]
    pub fn to_vec(&self) -> (res: Vec<CKeyKV>)
    {unimplemented!()}

	#[verifier::external_body]
    pub proof fn lemma_to_vec_view(self, other: Self)
      ensures
        (self@ == other@ <==> self.spec_to_vec()@ == other.spec_to_vec()@)
        && (self@ == other@ <==> (
            self.spec_to_vec().len() == other.spec_to_vec().len() &&
                forall |i: int| #![auto] 0 <= i < self.spec_to_vec().len() ==>
                self.spec_to_vec()[i]@ == other.spec_to_vec()[i]@
        ))
	{
		unimplemented!()
	}

}


pub struct CKeyKV {
    pub k: CKey,
    pub v: Vec<u8>,
}

impl CKeyKV {

    pub open spec fn view(self) -> (AbstractKey, Seq<u8>)
    {
        (self.k, self.v@)
    }

}


pub open spec fn ckeykvlt(a: CKeyKV, b: CKeyKV) -> bool {
    a.k.ukey < b.k.ukey
}

pub open spec fn spec_sorted_keys(v: Vec<CKeyKV>) -> bool {
    forall |i: int, j: int| 0 <= i && i + 1 < v.len() && j == i+1 ==> #[trigger] ckeykvlt(v@[i], v@[j])
}


#[derive(Eq,PartialEq,Hash)]
pub struct SHTKey {
    pub ukey: u64,
}


pub trait Marshalable : Sized {

  spec fn is_marshalable(&self) -> bool;

	#[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable()
  {unimplemented!()}

  spec fn view_equal(&self, other: &Self) -> bool;

	#[verifier::external_body]
  proof fn lemma_same_views_serialize_the_same(&self, other: &Self)
    requires
      self.view_equal(other),
    ensures
      self.is_marshalable() == other.is_marshalable(),
      self.ghost_serialize() == other.ghost_serialize(),
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

	#[verifier::external_body]
  proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
	{
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

	#[verifier::external_body]
  proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
	{
		unimplemented!()
	}

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
    (self@.len() as usize).ghost_serialize()
      + self@
  }

	#[verifier::external_body]
  proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
	{
		unimplemented!()
	}

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
	{
		unimplemented!()
	}

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

	#[verifier::external_body]
  proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
	{
		unimplemented!()
	}

}


macro_rules! derive_marshalable_for_struct {
  {
    $( #[$attr:meta] )*
    $pub:vis
    struct $newstruct:ident $(< $($poly:ident : Marshalable),+ $(,)? >)? {
      $(
        $fieldvis:vis $field:ident : $fieldty:ty
      ),+
      $(,)?
    }
  } => {
    ::builtin_macros::verus! {
      impl $(< $($poly: Marshalable),* >)? Marshalable for $newstruct $(< $($poly),* >)? {

        open spec fn view_equal(&self, other: &Self) -> bool {
          $(
            &&& self.$field.view_equal(&other.$field)
          )*
        }

        open spec fn is_marshalable(&self) -> bool {
          $(
            &&& self.$field.is_marshalable()
          )*
          &&& 0 $(+ self.$field.ghost_serialize().len())* <= usize::MAX
        }

        open spec fn ghost_serialize(&self) -> Seq<u8> {
          Seq::empty() $(+ self.$field.ghost_serialize())*
        }

	#[verifier::external_body]
        proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
	{
		unimplemented!()
	}

}}}}


macro_rules! marshalable_by_bijection {
    {
        [$type:ty] <-> [$marshalable:ty];
        forward ($self:ident) $forward:expr;
        backward ($m:ident) $backward:expr;
    }
    =>
    {
        ::builtin_macros::verus! {
            impl $type {
                 pub open spec fn forward_bijection_for_view_equality_do_not_use_for_anything_else($self: Self) -> $marshalable {
                  $forward
                }
            }
            impl Marshalable for $type {

                open spec fn view_equal(&self, other: &Self) -> bool {
                    self.forward_bijection_for_view_equality_do_not_use_for_anything_else().view_equal(
                      &other.forward_bijection_for_view_equality_do_not_use_for_anything_else())
                }

                open spec fn is_marshalable($self: &Self) -> bool {
                    $forward.is_marshalable()
                }

                open spec fn ghost_serialize($self: &Self) -> Seq<u8>
                {
                    $forward.ghost_serialize()
                }

	#[verifier::external_body]
                proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
	{
		unimplemented!()
	}

}}}}



	#[verifier::opaque]
    pub open spec fn ckeyhashmap_max_serialized_size() -> usize {
        0x100000
    }


    impl Marshalable for CKeyHashMap {

        open spec fn view_equal(&self, other: &Self) -> bool {
            self@ === other@
        }

        open spec fn is_marshalable(&self) -> bool {
            self.to_vec().is_marshalable()
                && spec_sorted_keys(self.to_vec())
                && self.to_vec().ghost_serialize().len() <= (ckeyhashmap_max_serialized_size() as int)
        }

        open spec fn ghost_serialize(&self) -> Seq<u8>
        {
            self.to_vec().ghost_serialize()
        }

        proof fn lemma_same_views_serialize_the_same(self: &Self, other: &Self)
        {
            self.lemma_to_vec_view(*other);
            self.to_vec().lemma_same_views_serialize_the_same(&other.to_vec());
        }

}


pub type AbstractKey = SHTKey;
pub type CKey = SHTKey;

    marshalable_by_bijection! {
        [SHTKey] <-> [u64];
        forward(self) self.ukey;
        backward(x) SHTKey { ukey: x };
    }


    derive_marshalable_for_struct! {
        pub struct CKeyKV {
            pub k: CKey,
            pub v: Vec::<u8>,
        }
    }


// ============================================================
// BEHAVIORAL MUTATION TESTS
// Start from valid inputs, mutate expected outputs/relations.
// Each test SHOULD FAIL verification.
// ============================================================

// Test 1: view_equal CKeyHashMaps, assert serializations are DIFFERENT (negate ensures)
// SHOULD FAIL — the lemma guarantees they are equal
proof fn test_mutation_ckh_negate_serialize(a: CKeyHashMap, b: CKeyHashMap)
    requires
        a.view_equal(&b),
{
    a.lemma_same_views_serialize_the_same(&b);
    assert(a.ghost_serialize() != b.ghost_serialize()); // SHOULD FAIL
}

// Test 2: view_equal CKeyHashMaps, assert marshalable status DIFFERS (negate ensures)
// SHOULD FAIL — the lemma guarantees marshalable status is the same
proof fn test_mutation_ckh_negate_marshalable(a: CKeyHashMap, b: CKeyHashMap)
    requires
        a.view_equal(&b),
{
    a.lemma_same_views_serialize_the_same(&b);
    assert(a.is_marshalable() != b.is_marshalable()); // SHOULD FAIL
}

// Test 3: view_equal u64s, assert serializations are DIFFERENT
// SHOULD FAIL — u64 lemma guarantees equal serialization
proof fn test_mutation_u64_negate_serialize(a: u64, b: u64)
    requires
        a.view_equal(&b),
{
    a.lemma_same_views_serialize_the_same(&b);
    assert(a.ghost_serialize() != b.ghost_serialize()); // SHOULD FAIL
}

// Test 4: view_equal CKeyHashMaps, assert first byte of serialization differs
// SHOULD FAIL — if serializations are equal, all bytes must be equal
proof fn test_mutation_ckh_negate_first_byte(a: CKeyHashMap, b: CKeyHashMap)
    requires
        a.view_equal(&b),
        a.is_marshalable(),
        a.ghost_serialize().len() > 0,
{
    a.lemma_same_views_serialize_the_same(&b);
    assert(a.ghost_serialize()[0] != b.ghost_serialize()[0]); // SHOULD FAIL
}

}
