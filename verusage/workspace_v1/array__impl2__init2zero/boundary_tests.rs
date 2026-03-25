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

impl<const N: usize> Array<u8, N> {

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
// These tests probe edge cases, invalid inputs, and
// precondition violations to check if the spec rejects them.
// ============================================================

// Test 1: wf should NOT hold when seq length != N.
// wf() is defined as self.seq@.len() == N.
// With seq length 3 and N=4, wf must be false.
// SHOULD FAIL
proof fn test_boundary_wf_wrong_seq_length(arr: Array<u8, 4>)
    requires
        arr.seq@.len() == 3,
{
    assert(arr.wf());
}

// Test 2: init2zero postcondition covers only [0, N).
// Index N is outside the guaranteed range.
// Asserting arr@[4] == 0 when arr@.len() == 4 is unprovable.
// SHOULD FAIL
proof fn test_boundary_init2zero_index_at_N(arr: Array<u8, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(arr@[4int] == 0);
}

// Test 3: set postcondition cannot be derived for index == N.
// set requires 0 <= i < N. For i == N, the postcondition is unwarranted.
// SHOULD FAIL
proof fn test_boundary_set_index_equals_N(arr_pre: Array<u8, 4>, arr_post: Array<u8, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
{
    assert(arr_post@ =~= arr_pre@.update(4int, 42u8));
}

// Test 4: wf guarantees seq length == N.
// After init2zero, asserting length is 5 (not 4) should fail.
// SHOULD FAIL
proof fn test_boundary_init2zero_wrong_length(arr: Array<u8, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(arr@.len() == 5);
}

// Test 5: wf should NOT hold for an empty seq when N > 0.
// N=4 but seq has 0 elements — wf requires len == N.
// SHOULD FAIL
proof fn test_boundary_wf_empty_seq(arr: Array<u8, 4>)
    requires
        arr.seq@.len() == 0,
{
    assert(arr.wf());
}

}
