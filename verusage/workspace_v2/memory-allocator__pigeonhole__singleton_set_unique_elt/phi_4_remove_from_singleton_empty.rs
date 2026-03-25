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
proof fn phi_4_remove_from_singleton_empty(s: Set<nat>, a: nat)
    requires
        s.finite(),
        s.len() == 1,
        s.contains(a),
    ensures
        s.remove(a) =~= Set::<nat>::empty(),
{
    let r = s.remove(a);
    assert(r.len() == 0);
    assert forall |x: nat| !r.contains(x) by {}
}

}
