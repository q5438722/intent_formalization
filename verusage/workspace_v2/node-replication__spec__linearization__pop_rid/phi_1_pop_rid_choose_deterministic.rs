use vstd::prelude::*;

fn main() {}

pub type ReqId = nat;

verus!{

// File: spec/linearization.rs

proof fn pop_rid(t: Set<ReqId>) -> (res: (Set<ReqId>, ReqId))
    requires
        !t.is_empty(),
        t.finite(),
    ensures
        res.0.len() < t.len(),
        t.contains(res.1),
        res.0 =~= t.remove(res.1),
        res.0.finite(),
{
    let r = t.choose();
    (t.remove(r), r)
}




// === Entailment query ===
proof fn phi_1_pop_rid_choose_deterministic(t: Set<ReqId>)
    requires
        !t.is_empty(),
        t.finite(),
    ensures ({
        let (s1, r1) = pop_rid(t);
        let (s2, r2) = pop_rid(t);
        r1 == r2 && s1 =~= s2
    }),
{
    let _ = pop_rid(t);
    let _ = pop_rid(t);
}

}
