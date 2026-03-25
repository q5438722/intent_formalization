use vstd::prelude::*;

fn main() {}

verus!{

// File: array.rs
pub struct Array<A, const N: usize>{
    pub seq: Ghost<Seq<A>>,
    pub ar: [A;N]
}

impl<A, const N: usize> Array<A, N> {

    #[verifier(inline)]
    pub open spec fn view(&self) -> Seq<A>{
        self.seq@
    }

    pub open spec fn wf(&self) -> bool{
        self.seq@.len() == N
    }

}

impl<A, const N: usize> Array<A, N> {

    #[verifier(external_body)]
    pub fn set(&mut self, i: usize, out: A)
        requires
            0 <= i < N,
            old(self).wf(),
        ensures
            self.seq@ =~= old(self).seq@.update(i as int, out),
            self.wf(),
    {
        unimplemented!()
    }

}

impl<const N: usize> Array<usize, N> {

    #[verifier(external_body)]
    pub fn init2zero(&mut self)
        requires
            old(self).wf(),
            N <= usize::MAX,
        ensures
            forall|index:int| 0<= index < N ==> #[trigger] self@[index] == 0,
            self.wf(),
    {
        unimplemented!()
    }

}

// ============================================================
// BOUNDARY TESTS
// ============================================================

// Test B1: wf rejects wrong seq length (3 != 4).
// SHOULD FAIL
proof fn test_boundary_wf_wrong_seq_length(arr: Array<usize, 4>)
    requires
        arr.seq@.len() == 3,
{
    assert(arr.wf());
}

// Test B2: init2zero postcondition does not cover index N.
// SHOULD FAIL
proof fn test_boundary_init2zero_index_at_N(arr: Array<usize, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(arr@[4int] == 0);
}

// Test B3: set postcondition not derivable for index == N.
// SHOULD FAIL
proof fn test_boundary_set_index_equals_N(arr_pre: Array<usize, 4>, arr_post: Array<usize, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
{
    assert(arr_post@ =~= arr_pre@.update(4int, 42usize));
}

// Test B4: wf guarantees length == N, not N+1.
// SHOULD FAIL
proof fn test_boundary_init2zero_wrong_length(arr: Array<usize, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(arr@.len() == 5);
}

// Test B5: wf rejects empty seq when N > 0.
// SHOULD FAIL
proof fn test_boundary_wf_empty_seq(arr: Array<usize, 4>)
    requires
        arr.seq@.len() == 0,
{
    assert(arr.wf());
}

// ============================================================
// BEHAVIORAL MUTATION TESTS
// ============================================================

// Test M1: init2zero sets elements to 0, not 1.
// SHOULD FAIL
proof fn test_mutation_init2zero_element_is_one(arr: Array<usize, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(arr@[0] == 1);
}

// Test M2: set(2, 99) puts 99 at index 2, not 100.
// SHOULD FAIL
proof fn test_mutation_set_wrong_value(arr_pre: Array<usize, 4>, arr_post: Array<usize, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
        arr_post@ =~= arr_pre@.update(2int, 99usize),
{
    assert(arr_post@[2] == 100);
}

// Test M3: set(2, 99) does NOT modify index 0.
// SHOULD FAIL
proof fn test_mutation_set_modifies_other_index(arr_pre: Array<usize, 4>, arr_post: Array<usize, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
        arr_post@ =~= arr_pre@.update(2int, 99usize),
        arr_pre@[0] == 42,
{
    assert(arr_post@[0] == 99);
}

// Test M4: init2zero maintains wf(), not breaks it.
// SHOULD FAIL
proof fn test_mutation_init2zero_not_wf(arr: Array<usize, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(!arr.wf());
}

// Test M5: init2zero zeroes ALL elements including the last.
// SHOULD FAIL
proof fn test_mutation_init2zero_last_element_nonzero(arr: Array<usize, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(arr@[3] != 0);
}

// ============================================================
// LOGICAL TESTS
// ============================================================

// Test L1: init2zero does NOT preserve old values.
// SHOULD FAIL
proof fn test_logical_init2zero_preserves_old_value(
    arr_pre: Array<usize, 4>, arr_post: Array<usize, 4>
)
    requires
        arr_pre.wf(),
        arr_pre@[1] == 42,
        arr_post.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr_post@[index] == 0,
{
    assert(arr_post@[1] == 42);
}

// Test L2: set is NOT a no-op when values differ.
// SHOULD FAIL
proof fn test_logical_set_is_noop(arr_pre: Array<usize, 4>, arr_post: Array<usize, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
        arr_pre@[1] == 10,
        arr_post@ =~= arr_pre@.update(1int, 20usize),
{
    assert(arr_post@ =~= arr_pre@);
}

// Test L3: Spec says nothing about negative indices.
// SHOULD FAIL
proof fn test_logical_init2zero_negative_index(arr: Array<usize, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(arr@[-1int] == 0);
}

// Test L4: set with different values produces different results.
// SHOULD FAIL
proof fn test_logical_set_different_values_same_result(
    arr: Array<usize, 4>,
    arr_post1: Array<usize, 4>,
    arr_post2: Array<usize, 4>
)
    requires
        arr.wf(),
        arr_post1.wf(),
        arr_post2.wf(),
        arr_post1@ =~= arr@.update(0int, 1usize),
        arr_post2@ =~= arr@.update(0int, 2usize),
{
    assert(arr_post1@ =~= arr_post2@);
}

// Test L5: set(0, 5) on a zero array breaks the all-zero invariant.
// SHOULD FAIL
proof fn test_logical_set_nonzero_breaks_all_zero(
    arr_init: Array<usize, 4>,
    arr_set: Array<usize, 4>
)
    requires
        arr_init.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr_init@[index] == 0,
        arr_set.wf(),
        arr_set@ =~= arr_init@.update(0int, 5usize),
{
    assert(forall|index:int| 0 <= index < 4 ==> #[trigger] arr_set@[index] == 0);
}

}
