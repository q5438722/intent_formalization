use vstd::prelude::*;

fn main() {}

verus!{

pub type SLLIndex = i32;

// ===================== BOUNDARY TESTS =====================
// Each test violates a precondition of init or a helper function,
// or uses edge-case values. All tests SHOULD FAIL verification.

// Test 1: init requires N > 2. N = 2 is at the strict boundary.
// SHOULD FAIL
proof fn test_boundary_n_equals_2() {
    assert(2usize > 2);
}

// Test 2: init requires N > 2. N = 0 is well below the minimum.
// SHOULD FAIL
proof fn test_boundary_n_equals_0() {
    assert(0usize > 2);
}

// Test 3: init requires N > 2. N = 1 violates the lower bound.
// SHOULD FAIL
proof fn test_boundary_n_equals_1() {
    assert(1usize > 2);
}

// Test 4: init requires N < SLLIndex::MAX (i32::MAX).
// N = i32::MAX violates the strict inequality.
// SHOULD FAIL
proof fn test_boundary_n_at_i32_max() {
    let n: usize = i32::MAX as usize;
    assert(n < i32::MAX as usize);
}

// Test 5: init ensures self.len() == 0. Claiming len > 0
// after init violates the postcondition.
// SHOULD FAIL
proof fn test_boundary_init_len_positive(len: usize)
    requires
        len == 0,
{
    assert(len > 0);
}

// Test 6: init ensures self@ =~= Seq::empty(). The empty
// sequence has length 0. Claiming length > 0 contradicts this.
// SHOULD FAIL
proof fn test_boundary_init_seq_nonempty() {
    let s = Seq::<u64>::empty();
    assert(s.len() > 0);
}

// Test 7: wf requires all free_list indices in [0, N).
// An index equal to N is out of bounds.
// SHOULD FAIL
proof fn test_boundary_free_index_at_n() {
    let n: usize = 4;
    let index: SLLIndex = 4;
    assert(0 <= index && (index as usize) < n);
}

// Test 8: SLLIndex is i32. The value -2 is below the sentinel
// value -1 used for list terminators. It should never be valid.
// SHOULD FAIL
proof fn test_boundary_index_below_sentinel() {
    let index: SLLIndex = -2i32;
    assert(index >= -1i32);
}

// Test 9: wf requires free_list_len + value_list_len == N.
// Mismatched sums violate the partition invariant.
// SHOULD FAIL
proof fn test_boundary_partition_mismatch() {
    let n: usize = 5;
    let free_list_len: usize = 2;
    let value_list_len: usize = 2;
    assert(free_list_len + value_list_len == n);
}

// Test 10: array_wf requires arr_seq@.len() == N and size == N.
// A mismatch between array length and N violates this.
// SHOULD FAIL
proof fn test_boundary_array_size_mismatch() {
    let arr_len: usize = 3;
    let n: usize = 5;
    assert(arr_len == n);
}

// Test 11: init ensures len() == 0. For any valid N > 2,
// claiming value_list_len == N contradicts the postcondition.
// SHOULD FAIL
proof fn test_boundary_len_equals_n(value_list_len: usize, n: usize)
    requires
        value_list_len == 0,
        n > 2,
{
    assert(value_list_len == n);
}

// Test 12: set_value/set_next/set_prev operate on arr_seq by index.
// Claim: index 5 is valid for an array of length 3.
// SHOULD FAIL
proof fn test_boundary_set_index_exceeds_arr_len() {
    let arr_len: usize = 3;
    let index: SLLIndex = 5;
    assert(0 <= index && (index as usize) < arr_len);
}

}
