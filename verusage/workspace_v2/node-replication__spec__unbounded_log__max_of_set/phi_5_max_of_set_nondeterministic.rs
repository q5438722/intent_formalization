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
proof fn phi_5_max_of_set_nondeterministic()
    ensures
        ({
            let s = set![1nat, 3nat, 5nat];
            max_of_set(s) != max_of_set(s)
        }),
{
}

}
