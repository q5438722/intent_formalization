use vstd::prelude::*;

fn main() {}

verus! {

// ===== Target function (copied from source) =====

pub proof fn map_values_weakens_no_duplicates<A, B>(s: Seq<A>, map: spec_fn(A) -> B)
    requires s.map_values(map).no_duplicates()
    ensures s.no_duplicates()
{
    assert forall |i, j| 0 <= i < s.len() && 0 <= j < s.len() && i != j implies s[i] != s[j] by {
        if s[i] == s[j] {
            assert(s.map_values(map)[i] == s.map_values(map)[j]);
            assert(false);
        }
    }
}

// ===== BOUNDARY TESTS =====

// Test 1: Constant map on a 2-element sequence.
// s.map_values(|x| 0) = [0, 0] has duplicates, violating requires.
// SHOULD FAIL
proof fn test_boundary_constant_map() {
    let s: Seq<int> = Seq::empty().push(1int).push(2int);
    // map sends everything to 0, so mapped seq has duplicates
    map_values_weakens_no_duplicates::<int, int>(s, |x: int| 0int);
}

// Test 2: Identity map on a sequence with duplicate elements.
// s = [1, 1], map = id. Mapped = [1, 1] has duplicates.
// SHOULD FAIL
proof fn test_boundary_duplicate_elements_identity_map() {
    let s: Seq<int> = Seq::empty().push(1int).push(1int);
    map_values_weakens_no_duplicates::<int, int>(s, |x: int| x);
}

// Test 3: Non-injective map on 3-element sequence.
// s = [0, 1, 2], map = |x| x % 2. Mapped = [0, 1, 0] has duplicates.
// SHOULD FAIL
proof fn test_boundary_non_injective_map() {
    let s: Seq<int> = Seq::empty().push(0int).push(1int).push(2int);
    map_values_weakens_no_duplicates::<int, int>(s, |x: int| x % 2);
}

}
