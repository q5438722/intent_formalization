use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus!{

// File: set_lib_ext_v.rs
#[verifier::spinoff_prover]
pub proof fn lemma_map_set_singleton_auto<A, B>()
ensures
    forall |x: A, f: spec_fn(A) -> B| #[trigger] set![x].map(f) == set![f(x)],
{
    assert forall |x: A, f: spec_fn(A) -> B| #[trigger] set![x].map(f) =~= set![f(x)] by {
        assert(set![x].contains(x));
    }
}




// === Entailment query ===
proof fn phi_1_map_singleton_len_one<A, B>(x: A, f: spec_fn(A) -> B)
    ensures
        set![x].map(f).len() == 1,
{
    lemma_map_set_singleton_auto::<A, B>();
    assert(set![x].map(f) == set![f(x)]);
}

}
