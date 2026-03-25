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

// Proof-level abstraction of erase's postconditions.
#[verifier::external_body]
proof fn erase_spec(s: Seq<int>, start: int, end: int) -> (result: Seq<int>)
    requires
        valid_int(s),
        0 <= start <= end <= s.len(),
    ensures
        valid_int(result),
        result == s.subrange(0, start) + s.subrange(end, s.len() as int),
        s.to_set() == result.to_set() + s.subrange(start, end).to_set(),
{ unimplemented!() }

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Wrong removed element — claims remove at index 1 yields element at index 0.
// s = [1, 3, 5], remove(1) should return 3 (s[1]), not 1 (s[0]).
// SHOULD FAIL
proof fn test_mutation_wrong_removed_element()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = remove_spec(s, 1);
    assert(result.1 == 1int); // mutated: claims element is 1, actual is 3
}

// Test 2: Wrong result length after erase — claims length is unchanged.
// s = [1, 3, 5], erase(1, 2) removes one element, result should have len 2.
// SHOULD FAIL
proof fn test_mutation_erase_length_unchanged()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = erase_spec(s, 1, 2);
    assert(result.len() == 3); // mutated: claims length 3, actual is 2
}

// Test 3: Remove doesn't change the sequence — claims result equals original.
// s = [1, 3, 5], remove(0) should yield [3, 5], not [1, 3, 5].
// SHOULD FAIL
proof fn test_mutation_remove_no_change()
{
    let s: Seq<int> = seq![1int, 3, 5];
    let result = remove_spec(s, 0);
    assert(result.0 =~= s); // mutated: claims result equals original
}

}
