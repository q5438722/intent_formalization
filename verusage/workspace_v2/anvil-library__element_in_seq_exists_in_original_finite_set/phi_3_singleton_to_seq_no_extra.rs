#![allow(unused_imports)]
use vstd::prelude::*;
use vstd::set::*;
use vstd::set_lib::*;

fn main() {}

verus! {

proof fn element_in_seq_exists_in_original_finite_set<A>(s: Set<A>, e: A)
    requires s.finite(), s.to_seq().contains(e),
    ensures s.contains(e),
    decreases s.len()
{
    if s.len() != 0 {
        // need choose() to be not-random
        let x = s.choose();
        if x != e {
            element_in_seq_exists_in_original_finite_set(s.remove(x), e);
        }
    }
}


// === Entailment query ===
proof fn phi_3_singleton_to_seq_no_extra(v: int, e: int)
    requires
        v != e,
    ensures
        !Set::empty().insert(v).to_seq().contains(e),
{
    if Set::empty().insert(v).to_seq().contains(e) {
        element_in_seq_exists_in_original_finite_set(Set::empty().insert(v), e);
    }
}

}
