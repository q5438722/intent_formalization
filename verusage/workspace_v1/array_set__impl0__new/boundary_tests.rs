use vstd::prelude::*;

fn main() {}

verus!{

// File: array.rs
pub struct Array<A, const N: usize>{
    pub seq: Ghost<Seq<A>>,
    pub ar: [A;N]
}

impl<A, const N: usize> Array<A, N> {

	#[verifier::external_body]
    #[verifier(external_body)]
    pub const fn new() -> (ret: Self)
        ensures
            ret.wf(),
	{
		unimplemented!()
	}

    #[verifier(inline)]
    pub open spec fn view(&self) -> Seq<A>{
        self.seq@
    }

    pub open spec fn wf(&self) -> bool{
        self.seq@.len() == N
    }

}


impl<A, const N: usize> Array<A, N> {

	#[verifier::external_body]
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



// File: array_set.rs
pub struct ArraySet<const N: usize> {
    pub data: Array<bool, N>,
    pub len: usize,

    pub set: Ghost<Set<usize>>,
}

impl <const N: usize> ArraySet<N> {

    pub fn new() -> (ret:Self)
        ensures
            ret.wf(),
            ret@ == Set::<usize>::empty(),
    {
        let mut ret = Self{
            data: Array::new(),
            len: 0,
            set:Ghost(Set::<usize>::empty()),
        };
        for i in 0..N
            invariant
                0<=i<=N,
                ret.data.wf(),
                ret.len == 0,
                ret.set@ == Set::<usize>::empty(),
                forall|j:int|
                    0<=j<i ==> ret.data@[j] == false,
        {
            ret.data.set(i,false);
        }
        ret
    }

    pub closed spec fn view(&self) -> Set<usize>{
        self.set@
    }

    pub closed spec fn wf(&self) -> bool{
        &&&
        self.data.wf()
        &&&
        self.set@.finite()
        &&&
        0 <= self.len <= N
        &&&
        forall|i:usize| 
            #![trigger self.data@[i as int]]
            #![trigger self.set@.contains(i)]
            0 <= i < N && self.data@[i as int] ==> self.set@.contains(i)
        &&&
        forall|i:usize| 
            #![trigger self.data@[i as int]]
            #![trigger self.set@.contains(i)]
            self.set@.contains(i) ==> 0 <= i < N && self.data@[i as int]     
        &&&
        self.len == self.set@.len() 
    }

}


// ============================================================
// BOUNDARY TESTS
// These tests probe the semantic boundary of ArraySet::new()
// by asserting properties about edge cases and invalid states.
// All tests SHOULD FAIL verification.
// ============================================================

// Test 1: After new(), the result set is empty.
// Asserting it contains element 0 should fail.
// SHOULD FAIL
proof fn test_boundary_new_set_contains_zero(post: ArraySet<4>)
    requires
        post.wf(),
        post@ == Set::<usize>::empty(),
{
    assert(post@.contains(0usize));
}

// Test 2: After new(), the empty set has length 0.
// Asserting length > 0 should fail.
// SHOULD FAIL
proof fn test_boundary_new_set_len_positive(post: ArraySet<4>)
    requires
        post.wf(),
        post@ == Set::<usize>::empty(),
{
    assert(post@.len() > 0);
}

// Test 3: After new(), the set is empty.
// Asserting it contains the maximum valid index (N-1 = 3) should fail.
// SHOULD FAIL
proof fn test_boundary_new_set_contains_max_index(post: ArraySet<4>)
    requires
        post.wf(),
        post@ == Set::<usize>::empty(),
{
    assert(post@.contains(3usize));
}

// Test 4: After new(), the set is empty.
// Asserting it contains an out-of-range index (N = 4) should fail.
// SHOULD FAIL
proof fn test_boundary_new_set_contains_out_of_range(post: ArraySet<4>)
    requires
        post.wf(),
        post@ == Set::<usize>::empty(),
{
    assert(post@.contains(4usize));
}

// Test 5: Array::new() only guarantees wf() (length == N).
// It does NOT guarantee any specific element value.
// Asserting a specific value at index 0 should fail.
// SHOULD FAIL
proof fn test_boundary_array_new_element_value(a: Array<bool, 4>)
    requires
        a.wf(),
{
    assert(a@[0] == false);
}


}
