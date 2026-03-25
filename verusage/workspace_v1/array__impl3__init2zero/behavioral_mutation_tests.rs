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
// BEHAVIORAL MUTATION TESTS
// These tests start from valid inputs but mutate expected
// outputs/relations to check if incorrect behaviors are rejected.
// ============================================================

// Test 1: init2zero should set all elements to 0, not 1.
// Mutate: assert arr@[0] == 1 instead of 0.
// SHOULD FAIL
proof fn test_mutation_init2zero_element_is_one(arr: Array<usize, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(arr@[0] == 1);
}

// Test 2: After set(2, 99), the value at index 2 should be 99, not 100.
// Mutate: assert wrong value at updated index.
// SHOULD FAIL
proof fn test_mutation_set_wrong_value(arr_pre: Array<usize, 4>, arr_post: Array<usize, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
        arr_post@ =~= arr_pre@.update(2int, 99usize),
{
    assert(arr_post@[2] == 100);
}

// Test 3: set should NOT modify elements at other indices.
// After set(2, 99), index 0 should be unchanged.
// Mutate: assert index 0 was also changed to 99.
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

// Test 4: init2zero should maintain wf().
// Mutate: assert NOT wf after init2zero.
// SHOULD FAIL
proof fn test_mutation_init2zero_not_wf(arr: Array<usize, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(!arr.wf());
}

// Test 5: init2zero guarantees ALL elements are 0, including the last.
// Mutate: assert last element is nonzero.
// SHOULD FAIL
proof fn test_mutation_init2zero_last_element_nonzero(arr: Array<usize, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(arr@[3] != 0);
}

}
