use vstd::prelude::*;
use vstd::bytes::*;

fn main() {}

verus!{

// File: marshal_v.rs
pub trait Marshalable : Sized {

  spec fn view_equal(&self, other: &Self) -> bool;

	#[verifier::external_body]
  proof fn lemma_view_equal_symmetric(&self, other: &Self)
    ensures self.view_equal(other) == other.view_equal(self)
  {unimplemented!()}

}


impl<T: Marshalable, U: Marshalable> Marshalable for (T, U) {

  open spec fn view_equal(&self, other: &Self) -> bool {
    self.0.view_equal(&other.0) && self.1.view_equal(&other.1)
  }

  #[verifier::spinoff_prover]
  proof fn lemma_view_equal_symmetric(&self, other: &Self)
    // req, ens from trait
  {
    self.0.lemma_view_equal_symmetric(&other.0);
    self.1.lemma_view_equal_symmetric(&other.1);
  }

}





// === Entailment query ===
proof fn phi_4_view_equal_implies_actual_equality_u64(a: u64, b: u64)
    requires
        a.view_equal(&b),
    ensures
        a == b,
{
}

}
