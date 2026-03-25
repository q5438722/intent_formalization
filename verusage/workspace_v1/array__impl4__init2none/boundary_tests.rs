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

// === BOUNDARY TESTS ===

// Test 1: Call init2none without wf() precondition
// SHOULD FAIL: precondition old(self).wf() is not satisfied
fn test_init2none_missing_wf(arr: &mut Array<Option<u32>, 5>)
{
    arr.init2none(); // SHOULD FAIL
}

// Test 2: Call set with out-of-bounds index (i = 5, N = 5)
// SHOULD FAIL: precondition 0 <= i < N is violated (5 < 5 is false)
fn test_set_index_equals_n(arr: &mut Array<Option<u32>, 5>)
    requires old(arr).wf(),
{
    arr.set(5, None); // SHOULD FAIL
}

// Test 3: Call set without wf() precondition
// SHOULD FAIL: precondition old(self).wf() is not satisfied
fn test_set_missing_wf(arr: &mut Array<Option<u32>, 5>)
{
    arr.set(0, None); // SHOULD FAIL
}

// Test 4: Call set on empty array with index 0 (N = 0)
// SHOULD FAIL: 0 <= 0 < 0 is false, so precondition violated
fn test_set_on_empty_array(arr: &mut Array<Option<u32>, 0>)
    requires old(arr).wf(),
{
    arr.set(0, None); // SHOULD FAIL
}

}
