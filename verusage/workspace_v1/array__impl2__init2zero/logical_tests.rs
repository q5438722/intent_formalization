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
// LOGICAL TESTS
// These tests check properties NOT explicitly guaranteed by
// the specification: determinism, stronger inequalities,
// structural/global assumptions, cross-function reasoning.
// ============================================================

// Test 1: Two independently zero-initialized arrays should not differ.
// The spec says each element in [0, N) is 0 and wf holds for both.
// If Verus can prove they are extensionally equal, asserting they
// differ should fail. If it cannot, the spec is too weak.
// SHOULD FAIL
proof fn test_logical_two_zero_arrays_differ(arr1: Array<u8, 4>, arr2: Array<u8, 4>)
    requires
        arr1.wf(),
        arr2.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr1@[index] == 0,
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr2@[index] == 0,
{
    assert(!(arr1@ =~= arr2@));
}

// Test 2: init2zero overwrites all elements; old values are NOT preserved.
// If arr_pre@[0] was 5, after init2zero, arr_post@[0] must be 0, not 5.
// SHOULD FAIL
proof fn test_logical_init2zero_preserves_old_value(arr_pre: Array<u8, 4>, arr_post: Array<u8, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
        arr_pre@[0] == 5,
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr_post@[index] == 0,
{
    assert(arr_post@[0] == 5);
}

// Test 3: set does NOT imply anything about the original value.
// After set(1, 42), we cannot derive what arr_pre@[1] was before.
// Asserting a specific original value is unwarranted.
// SHOULD FAIL
proof fn test_logical_set_implies_original_value(arr_pre: Array<u8, 4>, arr_post: Array<u8, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
        arr_post@ =~= arr_pre@.update(1int, 42u8),
{
    assert(arr_pre@[1] == 0);
}

// Test 4: After a non-trivial set, the sequence differs from original.
// If arr_pre@[1] == 0 and we set index 1 to 42, the sequences should differ.
// Asserting the entire sequence is unchanged should fail.
// SHOULD FAIL
proof fn test_logical_set_is_identity(arr_pre: Array<u8, 4>, arr_post: Array<u8, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
        arr_pre@[1] == 0,
        arr_post@ =~= arr_pre@.update(1int, 42u8),
{
    assert(arr_pre@ =~= arr_post@);
}

// Test 5: set is invertible: set(1, 42) then set(1, original) restores state.
// The spec doesn't explicitly state invertibility, but Seq::update semantics
// should make this true. Assert they are NOT equal — should fail.
// SHOULD FAIL
proof fn test_logical_set_not_invertible(arr: Array<u8, 4>, arr_mid: Array<u8, 4>, arr_end: Array<u8, 4>)
    requires
        arr.wf(),
        arr_mid.wf(),
        arr_end.wf(),
        arr@[1] == 10,
        arr_mid@ =~= arr@.update(1int, 42u8),
        arr_end@ =~= arr_mid@.update(1int, 10u8),
{
    assert(!(arr@ =~= arr_end@));
}

}
