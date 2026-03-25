use vstd::prelude::*;
use vstd::seq_lib::*;

fn main() {}

verus! {

// Proof-level abstraction of vec_erase's postcondition.
#[verifier::external_body]
proof fn vec_erase_spec(v: Seq<int>, start: int, end: int) -> (r: Seq<int>)
    requires
        0 <= start <= end <= v.len(),
    ensures
        r == v.subrange(0, start) + v.subrange(end, v.len() as int),
{ unimplemented!() }

// ========== BEHAVIORAL MUTATION TESTS ==========

// Test 1: Mutated length — claim length is unchanged after erasing 2 elements.
// v = [10, 20, 30, 40, 50], erase [1, 3) removes indices 1 and 2.
// Correct r.len() == 3, not 5.
// SHOULD FAIL
proof fn test_mutation_length_unchanged()
{
    let v: Seq<int> = seq![10int, 20, 30, 40, 50];
    let r = vec_erase_spec(v, 1, 3);
    assert(r.len() == 5);
}

// Test 2: Mutated element — claim erased element is still at index 1.
// v = [10, 20, 30, 40, 50], erase [1, 3) → r = [10, 40, 50].
// r[1] should be 40, not 20 (which was erased).
// SHOULD FAIL
proof fn test_mutation_erased_element_present()
{
    let v: Seq<int> = seq![10int, 20, 30, 40, 50];
    let r = vec_erase_spec(v, 1, 3);
    assert(r[1] == 20int);
}

// Test 3: Mutated result — claim result equals the full original after erasure.
// v = [10, 20, 30], erase [0, 2) → r = [30].
// r cannot equal v.
// SHOULD FAIL
proof fn test_mutation_result_equals_original()
{
    let v: Seq<int> = seq![10int, 20, 30];
    let r = vec_erase_spec(v, 0, 2);
    assert(r =~= v);
}

}
