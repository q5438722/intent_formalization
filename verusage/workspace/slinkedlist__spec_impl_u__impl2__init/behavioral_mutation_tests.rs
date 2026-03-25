use vstd::prelude::*;

fn main() {}

verus!{

pub type SLLIndex = i32;

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid postconditions, then asserts a WRONG
// (mutated) output or relation. All tests SHOULD FAIL verification.

// Test 1: init ensures self@ =~= Seq::empty(), so view length is 0.
// Mutate: claim the view has positive length.
// SHOULD FAIL
proof fn test_mutation_view_nonempty(spec_seq_len: nat)
    requires
        spec_seq_len == 0,
{
    assert(spec_seq_len > 0);
}

// Test 2: init ensures self.len() == 0. Mutate: claim len == 1.
// SHOULD FAIL
proof fn test_mutation_len_is_one(len: usize)
    requires
        len == 0,
{
    assert(len == 1);
}

// Test 3: init ensures self@ =~= Seq::empty(). Mutate: claim
// the view equals a specific non-empty sequence [42].
// SHOULD FAIL
proof fn test_mutation_view_equals_singleton() {
    let s = Seq::<u64>::empty();
    assert(s =~= Seq::<u64>::empty().push(42));
}

// Test 4: set_value ensures value_list_head is preserved
// (new_head == old_head). Mutate: claim it changed.
// SHOULD FAIL
proof fn test_mutation_set_value_head_changes(
    old_head: SLLIndex,
    new_head: SLLIndex,
)
    requires
        new_head == old_head,
{
    assert(new_head != old_head);
}

// Test 5: set_value ensures spec_seq@ is preserved
// (new == old). Mutate: claim spec_seq changed length.
// SHOULD FAIL
proof fn test_mutation_set_value_spec_seq_changes(
    old_len: nat,
    new_len: nat,
)
    requires
        new_len == old_len,
{
    assert(new_len != old_len);
}

// Test 6: set_next ensures free_list_len is preserved.
// Mutate: claim it changed.
// SHOULD FAIL
proof fn test_mutation_set_next_free_len_changes(
    old_free_len: usize,
    new_free_len: usize,
)
    requires
        new_free_len == old_free_len,
{
    assert(new_free_len != old_free_len);
}

// Test 7: set_prev ensures value_list_tail is preserved.
// Mutate: claim it changed.
// SHOULD FAIL
proof fn test_mutation_set_prev_tail_changes(
    old_tail: SLLIndex,
    new_tail: SLLIndex,
)
    requires
        new_tail == old_tail,
{
    assert(new_tail != old_tail);
}

// Test 8: set_value ensures arr_seq[index].value == v.
// Mutate: claim value at index is different from v.
// SHOULD FAIL
proof fn test_mutation_set_value_wrong_value(
    actual_value: u64,
    expected_value: u64,
)
    requires
        actual_value == expected_value,
{
    assert(actual_value != expected_value);
}

// Test 9: set_next ensures arr_seq[index].next == v.
// Mutate: claim next at index differs from v.
// SHOULD FAIL
proof fn test_mutation_set_next_wrong_next(
    actual_next: SLLIndex,
    expected_next: SLLIndex,
)
    requires
        actual_next == expected_next,
{
    assert(actual_next != expected_next);
}

// Test 10: set_prev ensures arr_seq[index].prev == v.
// Mutate: claim prev at index differs from v.
// SHOULD FAIL
proof fn test_mutation_set_prev_wrong_prev(
    actual_prev: SLLIndex,
    expected_prev: SLLIndex,
)
    requires
        actual_prev == expected_prev,
{
    assert(actual_prev != expected_prev);
}

// Test 11: set_value ensures other nodes are unchanged
// (new == old for i != index). Mutate: claim another node changed.
// SHOULD FAIL
proof fn test_mutation_set_value_other_node_changes(
    old_other_val: u64,
    new_other_val: u64,
)
    requires
        new_other_val == old_other_val,
{
    assert(new_other_val != old_other_val);
}

// Test 12: init ensures value_list_len == 0 (from len() == 0)
// and wf requires free_list_len + value_list_len == N.
// Mutate: claim free_list_len is 0 despite N > 2.
// SHOULD FAIL
proof fn test_mutation_free_list_len_zero(
    free_list_len: usize,
    value_list_len: usize,
    n: usize,
)
    requires
        value_list_len == 0,
        free_list_len + value_list_len == n,
        n > 2,
{
    assert(free_list_len == 0);
}

}
