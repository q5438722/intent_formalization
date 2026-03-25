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
proof fn phi_5_map_singleton_subset_of_any_containing_image<A, B>(x: A, f: spec_fn(A) -> B, s: Set<B>)
    requires
        s.contains(f(x)),
    ensures
        set![x].map(f).subset_of(s),
{
    lemma_map_set_singleton_auto::<A, B>();
    assert(set![x].map(f) == set![f(x)]);
    assert(set![f(x)].subset_of(s));
}

}
