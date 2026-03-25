use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

spec fn sorted_int(s: Seq<int>) -> bool {
    forall |i: int, j: int| 0 <= i < j < s.len() ==> s[i] < s[j]
}

spec fn valid_int(s: Seq<int>) -> bool {
    sorted_int(s) && s.no_duplicates()
}

// Proof-level abstraction of remove's postconditions.
#[verifier::external_body]
proof fn remove_spec(s: Seq<int>, i: int) -> (result: (Seq<int>, int))
    requires
        valid_int(s),
        0 <= i < s.len(),
    ensures
        valid_int(result.0),
        result.1 == s[i],
        result.0 == s.remove(i),
        result.0.to_set() == s.to_set().remove(result.1),
{ unimplemented!() }

// ========== BOUNDARY TESTS ==========

// Test 1: Remove with index == len (off-by-one, out of bounds).
// Precondition requires i < s.len(). Using i == s.len() violates this.
// SHOULD FAIL
proof fn test_boundary_remove_index_eq_len()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = remove_spec(s, 3); // i == s.len() == 3, violates i < 3
    assert(result.0.len() >= 0);
}

// Test 2: Remove on an empty sequence.
// Precondition requires 0 <= i < s.len(). For empty seq, s.len() == 0,
// so no valid index exists.
// SHOULD FAIL
proof fn test_boundary_remove_empty_seq()
{
    let s: Seq<int> = Seq::empty();
    let result = remove_spec(s, 0); // violates 0 < s.len() since s.len() == 0
    assert(result.0.len() >= 0);
}

// Test 3: Remove with negative index.
// Precondition requires 0 <= i. Using i = -1 violates this.
// SHOULD FAIL
proof fn test_boundary_remove_negative_index()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = remove_spec(s, -1int); // violates 0 <= i
    assert(result.0.len() >= 0);
}

}
