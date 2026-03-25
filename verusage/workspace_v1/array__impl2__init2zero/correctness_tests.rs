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
// ============================================================

// Test B1: wf rejects wrong seq length
// SHOULD FAIL
proof fn test_boundary_wf_wrong_seq_length(arr: Array<u8, 4>)
    requires
        arr.seq@.len() == 3,
{
    assert(arr.wf());
}

// Test B2: init2zero postcondition does not cover index N
// SHOULD FAIL
proof fn test_boundary_init2zero_index_at_N(arr: Array<u8, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(arr@[4int] == 0);
}

// Test B3: set postcondition unprovable for index == N
// SHOULD FAIL
proof fn test_boundary_set_index_equals_N(arr_pre: Array<u8, 4>, arr_post: Array<u8, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
{
    assert(arr_post@ =~= arr_pre@.update(4int, 42u8));
}

// Test B4: wf-ensured length cannot be wrong
// SHOULD FAIL
proof fn test_boundary_init2zero_wrong_length(arr: Array<u8, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(arr@.len() == 5);
}

// Test B5: wf rejects empty seq when N > 0
// SHOULD FAIL
proof fn test_boundary_wf_empty_seq(arr: Array<u8, 4>)
    requires
        arr.seq@.len() == 0,
{
    assert(arr.wf());
}

// ============================================================
// BEHAVIORAL MUTATION TESTS
// ============================================================

// Test M1: After init2zero, element is 0, not 1
// SHOULD FAIL
proof fn test_mutation_init2zero_element_is_one(arr: Array<u8, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(arr@[0] == 1);
}

// Test M2: After set(2, 42), value is 42, not old value 5
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

// Test M3: set does not corrupt unrelated index
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

// Test M4: init2zero ensures wf, asserting !wf fails
// SHOULD FAIL
proof fn test_mutation_init2zero_breaks_wf(arr: Array<u8, 4>)
    requires
        arr.wf(),
        forall|index:int| 0 <= index < 4 ==> #[trigger] arr@[index] == 0,
{
    assert(!arr.wf());
}

// Test M5: set preserves length, asserting length change fails
// SHOULD FAIL
proof fn test_mutation_set_changes_length(arr_pre: Array<u8, 4>, arr_post: Array<u8, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
        arr_post@ =~= arr_pre@.update(1int, 99u8),
{
    assert(arr_post@.len() != arr_pre@.len());
}

// ============================================================
// LOGICAL TESTS
// ============================================================

// Test L1: Two zero-initialized arrays cannot differ
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

// Test L2: init2zero does NOT preserve old values
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

// Test L3: set does NOT imply original value
// SHOULD FAIL
proof fn test_logical_set_implies_original_value(arr_pre: Array<u8, 4>, arr_post: Array<u8, 4>)
    requires
        arr_pre.wf(),
        arr_post.wf(),
        arr_post@ =~= arr_pre@.update(1int, 42u8),
{
    assert(arr_pre@[1] == 0);
}

// Test L4: Non-trivial set is NOT identity
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

// Test L5: set IS invertible — asserting non-invertibility should fail
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
