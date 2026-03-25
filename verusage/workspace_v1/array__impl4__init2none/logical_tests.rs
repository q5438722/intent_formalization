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

// === LOGICAL TESTS ===

// Test 1: Assert init2none preserves the original sequence (structural preservation)
// SHOULD FAIL: init2none sets all elements to None, it does NOT preserve old values
fn test_init2none_preserves_seq(arr: &mut Array<Option<u32>, 5>)
    requires
        old(arr).wf(),
        old(arr)@[0int] == Some(42u32),
    ensures
        arr@ =~= old(arr)@, // SHOULD FAIL
{
    arr.init2none();
}

// Test 2: Assert stronger length guarantee (len > N instead of len == N)
// SHOULD FAIL: spec guarantees len == N (via wf), not len > N
fn test_init2none_stronger_length(arr: &mut Array<Option<u32>, 5>)
    requires old(arr).wf(),
    ensures arr@.len() > 5, // SHOULD FAIL
{
    arr.init2none();
}

// Test 3: Cross-function: set before init2none, assert set value preserved
// SHOULD FAIL: init2none overrides all values to None regardless of prior set
fn test_set_then_init2none_preserves(arr: &mut Array<Option<u32>, 5>)
    requires old(arr).wf(),
    ensures arr@[0int] == Some(42u32), // SHOULD FAIL
{
    arr.set(0, Some(42u32));
    arr.init2none();
}

// Test 4: Cross-function: init2none then set, assert element still None
// SHOULD FAIL: set(0, Some(42)) overrides the None value placed by init2none
fn test_init2none_then_set_still_none(arr: &mut Array<Option<u32>, 5>)
    requires old(arr).wf(),
    ensures arr@[0int].is_None(), // SHOULD FAIL
{
    arr.init2none();
    arr.set(0, Some(42u32));
}

}
