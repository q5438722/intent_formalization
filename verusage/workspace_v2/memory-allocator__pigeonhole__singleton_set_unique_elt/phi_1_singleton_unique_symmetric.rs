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
proof fn phi_1_singleton_unique_symmetric(s: Set<nat>, a: nat, b: nat)
    requires
        s.finite(),
        s.len() == 1,
        s.contains(a),
        s.contains(b),
    ensures
        b == a,
{
    singleton_set_unique_elt(s, a, b);
}

}
