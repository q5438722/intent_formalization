use vstd::prelude::*;

fn main() {}

verus!{

proof fn max_of_set(s: Set<nat>) -> (r: nat)
    requires
        s.finite(),
    ensures
        forall|x: nat| #[trigger] s.contains(x) ==> x <= r,
    decreases s.len(),
{
    if s.is_empty() {
        0
    } else {
        let v1 = s.choose();
        let v2 = max_of_set(s.remove(v1));
        assert(forall|x: nat| #[trigger] s.contains(x) && x != v1 ==> s.remove(v1).contains(x));
        if v1 >= v2 {
            v1
        } else {
            v2
        }
    }
}


// === Entailment query ===
proof fn phi_5_max_monotone_subset(s1: Set<nat>, s2: Set<nat>)
    requires
        s1.finite(),
        s2.finite(),
        s1.subset_of(s2),
    ensures
        max_of_set(s1) <= max_of_set(s2),
{
    let r1 = max_of_set(s1);
    let r2 = max_of_set(s2);
    // r1 is the return value, and for all x in s1, x <= r1
    // but r1 itself might not be in s1, so we can't directly conclude r1 <= r2
    // However, r1 is constructed as either v1 or v2 in the recursion...
    // This might NOT be provable — the max could exceed actual elements
}

}
