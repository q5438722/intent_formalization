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
proof fn phi_4_map_singleton_idempotent<A>(x: A, f: spec_fn(A) -> A, g: spec_fn(A) -> A)
    requires
        g(f(x)) == f(x),
    ensures
        set![x].map(f).map(g) == set![x].map(f),
{
    lemma_map_set_singleton_auto::<A, A>();
    assert(set![x].map(f) == set![f(x)]);
    assert(set![f(x)].map(g) == set![g(f(x))]);
}

}
