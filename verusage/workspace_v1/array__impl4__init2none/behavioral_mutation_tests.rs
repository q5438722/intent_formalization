use vstd::prelude::*;

fn main() {}

verus! {

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

impl<T: Copy, const N: usize> Array<Option<T>, N> {
    #[verifier(external_body)]
    pub fn init2none(&mut self)
        requires
            old(self).wf(),
            N <= usize::MAX,
        ensures
            forall|index:int| 0<= index < N ==> #[trigger] self@[index].is_None(),
            self.wf(),
    {
        unimplemented!()
    }
}

// === BEHAVIORAL MUTATION TESTS ===

// Test 1: After init2none, assert element 0 is Some (mutated expected output)
// SHOULD FAIL: postcondition guarantees all elements are None
fn test_init2none_element_is_some(arr: &mut Array<Option<u32>, 5>)
    requires old(arr).wf(),
    ensures arr@[0int] == Some(42u32), // SHOULD FAIL
{
    arr.init2none();
}

// Test 2: After init2none, assert wrong length
// SHOULD FAIL: postcondition guarantees self.wf() i.e. len == N
fn test_init2none_wrong_length(arr: &mut Array<Option<u32>, 5>)
    requires old(arr).wf(),
    ensures arr@.len() != 5, // SHOULD FAIL
{
    arr.init2none();
}

// Test 3: After set(0, Some(42)), assert element 0 is None (mutated relation)
// SHOULD FAIL: set postcondition guarantees seq@[0] == Some(42)
fn test_set_value_mutated(arr: &mut Array<Option<u32>, 5>)
    requires old(arr).wf(),
    ensures arr@[0int].is_None(), // SHOULD FAIL
{
    arr.set(0, Some(42u32));
}

// Test 4: After set(2, Some(99)), assert element 0 became None
// SHOULD FAIL: set only changes the target index, not other indices
fn test_set_changes_other_index(arr: &mut Array<Option<u32>, 5>)
    requires
        old(arr).wf(),
        old(arr)@[0int] == Some(10u32),
    ensures
        arr@[0int].is_None(), // SHOULD FAIL
{
    arr.set(2, Some(99u32));
}

}
