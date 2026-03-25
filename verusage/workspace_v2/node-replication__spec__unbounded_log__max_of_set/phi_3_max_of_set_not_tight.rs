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
proof fn phi_3_max_of_set_not_tight(s: Set<nat>)
    requires
        s.finite(),
        s.contains(5),
        forall|x: nat| s.contains(x) ==> x <= 5,
    ensures
        max_of_set(s) > 5,
{
    let _ = max_of_set(s);
}

}
