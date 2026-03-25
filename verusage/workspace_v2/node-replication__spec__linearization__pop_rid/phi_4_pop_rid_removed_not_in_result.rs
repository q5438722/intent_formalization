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
proof fn phi_4_pop_rid_removed_not_in_result(t: Set<ReqId>)
    requires
        !t.is_empty(),
        t.finite(),
    ensures ({
        let (s, r) = pop_rid(t);
        !s.contains(r)
    }),
{
    let _ = pop_rid(t);
}

}
