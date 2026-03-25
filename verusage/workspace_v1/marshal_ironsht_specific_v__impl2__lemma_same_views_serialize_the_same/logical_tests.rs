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
// LOGICAL TESTS
// Assert properties NOT explicitly guaranteed by the spec.
// Each test SHOULD FAIL verification.
// ============================================================

// Test 1: Converse — same serialization does NOT imply same view
// The spec only guarantees: view_equal → same serialization.
// The converse (same serialization → view_equal) is NOT guaranteed.
// SHOULD FAIL
proof fn test_logical_converse_ckh(a: CKeyHashMap, b: CKeyHashMap)
    requires
        a.ghost_serialize() == b.ghost_serialize(),
        a.is_marshalable(),
        b.is_marshalable(),
{
    assert(a.view_equal(&b)); // SHOULD FAIL
}

// Test 2: Injectivity — different views do NOT imply different serializations
// The spec does NOT guarantee that serialization is injective.
// SHOULD FAIL
proof fn test_logical_injectivity_ckh(a: CKeyHashMap, b: CKeyHashMap)
    requires
        !(a.view_equal(&b)),
        a.is_marshalable(),
        b.is_marshalable(),
{
    assert(a.ghost_serialize() != b.ghost_serialize()); // SHOULD FAIL
}

// Test 3: Opaque constant — cannot access value of opaque spec without reveal
// ckeyhashmap_max_serialized_size() is marked #[verifier::opaque], so its value
// is hidden from proofs unless revealed.
// SHOULD FAIL
proof fn test_logical_opaque_constant()
{
    assert(ckeyhashmap_max_serialized_size() == 0x100000usize); // SHOULD FAIL
}

// Test 4: Empty map view does NOT imply marshalable
// Even if a CKeyHashMap has an empty view, we cannot derive is_marshalable
// because is_marshalable depends on uninterpreted spec_to_vec().
// SHOULD FAIL
proof fn test_logical_empty_map_marshalable(a: CKeyHashMap)
    requires
        a@ === Map::<AbstractKey, Seq<u8>>::empty(),
{
    assert(a.is_marshalable()); // SHOULD FAIL
}

// Test 5: view_equal does NOT imply to_vec length equality without lemma
// Without calling lemma_to_vec_view, Verus cannot relate @ to spec_to_vec.
// SHOULD FAIL
proof fn test_logical_no_lemma_to_vec_len(a: CKeyHashMap, b: CKeyHashMap)
    requires
        a.view_equal(&b),
{
    // Do NOT call lemma_to_vec_view — try to derive to_vec relationship directly
    assert(a.to_vec().len() == b.to_vec().len()); // SHOULD FAIL
}

}
