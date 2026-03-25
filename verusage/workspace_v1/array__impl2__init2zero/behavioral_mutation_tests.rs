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
// BEHAVIORAL MUTATION TESTS
// These tests start from valid inputs but mutate expected
// outputs or relations to check if incorrect behaviors are
// rejected by the specification.
// ============================================================

// Test 1: After init2zero, every element is 0.
// Mutated: assert element at index 0 is 1 instead of 0.
// SHOULD FAIL
proof fn test_mutation_init2zero_element_is_one(arr: Array<u8, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(arr@[0] == 1);
}

// Test 2: After set(2, 42), index 2 holds 42.
// Mutated: assert the old value (5) is still at index 2.
// SHOULD FAIL
proof fn test_mutation_set_keeps_old_value(arr_pre: Array<u8, 4>, arr_post: Array<u8, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
        arr_pre@[2] == 5,
        arr_post@ =~= arr_pre@.update(2int, 42u8),
{
    assert(arr_post@[2] == 5);
}

// Test 3: set should NOT corrupt unrelated indices.
// After set(2, 42), index 0 should remain 10.
// Mutated: assert index 0 changed.
// SHOULD FAIL
proof fn test_mutation_set_corrupts_other_index(arr_pre: Array<u8, 4>, arr_post: Array<u8, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
        arr_pre@[0] == 10,
        arr_post@ =~= arr_pre@.update(2int, 42u8),
{
    assert(arr_post@[0] != 10);
}

// Test 4: init2zero ensures wf().
// Mutated: assert wf does NOT hold after init2zero.
// SHOULD FAIL
proof fn test_mutation_init2zero_breaks_wf(arr: Array<u8, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(!arr.wf());
}

// Test 5: set preserves wf, so length stays N.
// Mutated: assert length differs after set.
// SHOULD FAIL
proof fn test_mutation_set_changes_length(arr_pre: Array<u8, 4>, arr_post: Array<u8, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
        arr_post@ =~= arr_pre@.update(1int, 99u8),
{
    assert(arr_post@.len() != arr_pre@.len());
}

}
