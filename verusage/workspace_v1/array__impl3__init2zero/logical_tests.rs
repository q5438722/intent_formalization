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
// LOGICAL TESTS
// These tests check properties NOT explicitly guaranteed by
// the specification: determinism, stronger inequalities,
// structural/global assumptions, cross-function misuse.
// ============================================================

// Test 1: init2zero does NOT preserve old values.
// The spec says all elements become 0. A nonzero old value
// should NOT survive. Asserting preservation should fail.
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
    // init2zero should overwrite all elements to 0
    // Asserting old value is preserved contradicts postcondition
    assert(arr_post@[1] == 42);
}

// Test 2: set is NOT a no-op. After set(i, v) where v != old value,
// the sequence should differ from the original.
// Asserting no change should fail.
// SHOULD FAIL
proof fn test_logical_set_is_noop(arr_pre: Array<usize, 4>, arr_post: Array<usize, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
        arr_pre@[1] == 10,
        arr_post@ =~= arr_pre@.update(1int, 20usize),
{
    // set(1, 20) changes index 1 from 10 to 20
    // Asserting sequence is unchanged should fail
    assert(arr_post@ =~= arr_pre@);
}

// Test 3: The spec does NOT guarantee anything about negative indices.
// init2zero ensures elements in [0, N) are 0.
// Asserting something about index -1 is beyond the spec.
// SHOULD FAIL
proof fn test_logical_init2zero_negative_index(arr: Array<usize, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    // Index -1 is outside the postcondition range [0, N)
    assert(arr@[-1int] == 0);
}

// Test 4: Setting different values at the same index should produce
// different sequences. The spec distinguishes values via update.
// Asserting two distinct updates produce the same result should fail.
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
    // set(0, 1) and set(0, 2) produce different sequences
    // Asserting they're equal should fail
    assert(arr_post1@ =~= arr_post2@);
}

// Test 5: After init2zero then set(0, 5), NOT all elements are 0.
// The set postcondition changes index 0 to 5.
// Claiming all elements are still 0 contradicts the composition.
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
    // After setting index 0 to 5, all-zero invariant is broken
    assert(forall|index:int| 0 <= index < 4 ==> #[trigger] arr_set@[index] == 0);
}

}
