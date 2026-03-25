use vstd::prelude::*;
use vstd::assert_by_contradiction;

fn main(){}

verus! {

proof fn singleton_set_unique_elt<T>(s: Set<T>, a:T, b:T)
    requires
        s.finite(),
        s.len() == 1,
        s.contains(a),
        s.contains(b),
    ensures
        a == b,
{
    assert_by_contradiction!(a == b, {
        let empty = s.remove(a);
        assert(empty.len() == 0);
        assert(empty.contains(b));
    });

}



// === Entailment query ===
proof fn phi_2_singleton_element_is_choose(s: Set<nat>, a: nat)
    requires
        s.finite(),
        s.len() == 1,
        s.contains(a),
    ensures
        s.choose() == a,
{
    assert(s.contains(s.choose()));
    singleton_set_unique_elt(s, a, s.choose());
}

}
