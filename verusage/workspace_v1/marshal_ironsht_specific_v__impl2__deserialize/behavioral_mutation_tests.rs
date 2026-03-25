// BEHAVIORAL MUTATION TESTS for marshal_ironsht_specific_v__impl2__deserialize
// Tests that mutate expected outputs or relations.
// All tests SHOULD FAIL verification.

extern crate verus_builtin_macros as builtin_macros;
use vstd::prelude::*;
use std::collections;
use vstd::bytes::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus!{

// ==================== Source Definitions ====================

// File: hashmap_t.rs
#[verifier(external_body)]
pub struct CKeyHashMap {
  m: collections::HashMap<CKey, Vec<u8>>,
}

impl CKeyHashMap {

    pub uninterp spec fn spec_to_vec(&self) -> Vec<CKeyKV>;
    #[verifier(external_body)]
    #[verifier(when_used_as_spec(spec_to_vec))]
    pub fn to_vec(&self) -> (res: Vec<CKeyKV>)
    {unimplemented!()}

    pub uninterp spec fn spec_from_vec(v: Vec<CKeyKV>) -> Self;

	#[verifier::external_body]
    #[verifier(when_used_as_spec(spec_from_vec))]
    #[verifier(external_body)]
    pub fn from_vec(v: Vec<CKeyKV>) -> (res: Self)
      ensures res == Self::spec_from_vec(v)
	{
		unimplemented!()
	}

	#[verifier::external_body]
    pub axiom fn lemma_from_vec(v: Vec<CKeyKV>)
      ensures
        #![trigger Self::spec_from_vec(v)]
        spec_sorted_keys(v) ==> Self::spec_from_vec(v).spec_to_vec() == v;

}


pub struct CKeyKV {
    pub k: CKey,
    pub v: Vec<u8>,
}

pub open spec fn ckeykvlt(a: CKeyKV, b: CKeyKV) -> bool {
    a.k.ukey < b.k.ukey
}

pub open spec fn spec_sorted_keys(v: Vec<CKeyKV>) -> bool {
    forall |i: int, j: int| 0 <= i && i + 1 < v.len() && j == i+1 ==> #[trigger] ckeykvlt(v@[i], v@[j])
}


// File: keys_t.rs
#[derive(Eq,PartialEq,Hash)]
pub struct SHTKey {
    pub ukey: u64,
}


// File: marshal_v.rs
pub trait Marshalable : Sized {

  spec fn is_marshalable(&self) -> bool;

	#[verifier::external_body]
  spec fn ghost_serialize(&self) -> Seq<u8>
    recommends self.is_marshalable()
  {unimplemented!()}

	#[verifier::external_body]
  exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
    ensures match res {
      Some((x, end)) => {
        &&& x.is_marshalable()
        &&& start <= end <= data.len()
        &&& data@.subrange(start as int, end as int) == x.ghost_serialize()
      }
      None => true,
  }
	{
		unimplemented!()
	}

}


impl Marshalable for u64 {

  open spec fn is_marshalable(&self) -> bool {
    true
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    spec_u64_to_le_bytes(*self)
  }

	#[verifier::external_body]
  exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
	{
		unimplemented!()
	}

}


impl Marshalable for usize {

  open spec fn is_marshalable(&self) -> bool {
    &&& *self as int <= u64::MAX
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    (*self as u64).ghost_serialize()
  }

	#[verifier::external_body]
  exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
	{
		unimplemented!()
	}

}


impl Marshalable for Vec<u8> {

  open spec fn is_marshalable(&self) -> bool {
    self@.len() <= usize::MAX &&
    (self@.len() as usize).ghost_serialize().len() + self@.len() as int <= usize::MAX
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    (self@.len() as usize).ghost_serialize()
      + self@
  }

	#[verifier::external_body]
  exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
	{
		unimplemented!()
	}

}


impl<T: Marshalable> Marshalable for Option<T> {

  open spec fn is_marshalable(&self) -> bool {
    match self {
      None => true,
      Some(x) => x.is_marshalable() && 1 + x.ghost_serialize().len() <= usize::MAX,
    }
  }

  open spec fn ghost_serialize(&self) -> Seq<u8>
  {
    match self {
      None => seq![0],
      Some(x) => seq![1] + x.ghost_serialize(),
    }
  }

	#[verifier::external_body]
  exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
	{
		unimplemented!()
	}

}


impl<T: Marshalable> Marshalable for Vec<T> {

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
  exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
	{
		unimplemented!()
	}

}


impl<T: Marshalable, U: Marshalable> Marshalable for (T, U) {

  open spec fn is_marshalable(&self) -> bool {
    &&& self.0.is_marshalable()
    &&& self.1.is_marshalable()
    &&& self.0.ghost_serialize().len() + self.1.ghost_serialize().len() <= usize::MAX
  }

  open spec fn ghost_serialize(&self) -> Seq<u8> {
    self.0.ghost_serialize() + self.1.ghost_serialize()
  }

	#[verifier::external_body]
  exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
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
        exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
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

                open spec fn is_marshalable($self: &Self) -> bool {
                    $forward.is_marshalable()
                }

                open spec fn ghost_serialize($self: &Self) -> Seq<u8>
                {
                    $forward.ghost_serialize()
                }

	#[verifier::external_body]
                exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
	{
		unimplemented!()
	}

}}}}



// File: marshal_ironsht_specific_v.rs
	#[verifier::external_body]
    pub exec fn sorted_keys(v: &Vec<CKeyKV>) -> (res: bool)
        ensures res == spec_sorted_keys(*v),
	{
		unimplemented!()
	}

	#[verifier::opaque]
    pub open spec fn ckeyhashmap_max_serialized_size() -> usize {
        0x100000
    }


	#[verifier::external_body]
    pub fn ckeyhashmap_max_serialized_size_exec() -> (r: usize)
        ensures r == ckeyhashmap_max_serialized_size()
	{
		unimplemented!()
	}

    impl Marshalable for CKeyHashMap {

        open spec fn is_marshalable(&self) -> bool {
            self.to_vec().is_marshalable()
                && spec_sorted_keys(self.to_vec())
                && self.to_vec().ghost_serialize().len() <= (ckeyhashmap_max_serialized_size() as int)
        }

        open spec fn ghost_serialize(&self) -> Seq<u8>
        {
            self.to_vec().ghost_serialize()
        }

        exec fn deserialize(data: &Vec<u8>, start: usize) -> (res: Option<(Self, usize)>)
        {
            match <Vec<CKeyKV>>::deserialize(data, start) {
                None => {
                    None
                },
                Some((x, end)) => {
                    if !sorted_keys(&x) {
                        None
                    } else {
                        let res = CKeyHashMap::from_vec(x);
                        if end - start > ckeyhashmap_max_serialized_size_exec() {
                            None
                        } else {
                            proof {
                                CKeyHashMap::lemma_from_vec(x);
                            }
                            Some((res, end))
                        }
                    }
                }
            }
        }

}

//////
pub type CKey=SHTKey;

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


// ==================== BEHAVIORAL MUTATION TESTS ====================

// Test 1: Different sorted vectors map to the same hashmap // SHOULD FAIL
// spec_from_vec is uninterpreted; nothing guarantees two distinct inputs collide.
proof fn test_from_vec_conflates_different_inputs(v1: Vec<CKeyKV>, v2: Vec<CKeyKV>)
    requires
        spec_sorted_keys(v1),
        spec_sorted_keys(v2),
        v1 != v2,
{
    assert(CKeyHashMap::spec_from_vec(v1) == CKeyHashMap::spec_from_vec(v2)); // SHOULD FAIL
}

// Test 2: Serialization mismatch after round-trip // SHOULD FAIL
// By the axiom, from_vec(v).to_vec() == v when sorted, so serializations must match.
// Asserting they DIFFER is a mutated behavior that should be rejected.
proof fn test_roundtrip_serialization_mismatch(v: Vec<CKeyKV>)
    requires
        spec_sorted_keys(v),
        v.is_marshalable(),
{
    CKeyHashMap::lemma_from_vec(v);
    assert(CKeyHashMap::spec_from_vec(v).ghost_serialize() !== v.ghost_serialize()); // SHOULD FAIL
}

// Test 3: Non-marshalable input produces marshalable hashmap // SHOULD FAIL
// By the axiom, from_vec(v).to_vec() == v when sorted.
// So from_vec(v).is_marshalable() requires v.is_marshalable(), which contradicts requires.
proof fn test_non_marshalable_produces_marshalable(v: Vec<CKeyKV>)
    requires
        spec_sorted_keys(v),
        !v.is_marshalable(),
{
    CKeyHashMap::lemma_from_vec(v);
    assert(CKeyHashMap::spec_from_vec(v).is_marshalable()); // SHOULD FAIL
}

}
