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
proof fn phi_2_map_singleton_collapses_distinct<A, B>(x: A, y: A, f: spec_fn(A) -> B)
    requires
        f(x) == f(y),
    ensures
        set![x].map(f) == set![y].map(f),
{
    lemma_map_set_singleton_auto::<A, B>();
}

}
