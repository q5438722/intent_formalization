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



// File: array_set.rs
pub struct ArraySet<const N: usize> {
    pub data: Array<bool, N>,
    pub len: usize,

    pub set: Ghost<Set<usize>>,
}

impl <const N: usize> ArraySet<N> {

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

    pub fn init(&mut self)
        requires
            old(self).wf(),
        ensures
            self.wf(),
            self@ == Set::<usize>::empty(),
    {
            self.len = 0;
            self.set = Ghost(Set::<usize>::empty());
        for i in 0..N
            invariant
                0<=i<=N,
                self.data.wf(),
                self.len == 0,
                self.set@ == Set::<usize>::empty(),
                forall|j:int|
                    0<=j<i ==> self.data@[j] == false,
        {
            self.data.set(i,false);
        }
    }


}


// ============================================================
// LOGICAL TESTS
// These tests check properties NOT explicitly guaranteed by
// the specification: determinism, stronger inequalities,
// structural/global assumptions, cross-function misuse.
// All tests SHOULD FAIL verification.
// ============================================================

// Test 1: wf() does not guarantee the set is empty.
// A well-formed ArraySet could have any valid elements.
// Asserting emptiness from wf() alone is unwarranted.
// SHOULD FAIL
proof fn test_logical_wf_implies_empty(s: ArraySet<4>)
    requires
        s.wf(),
{
    assert(s@ == Set::<usize>::empty());
}

// Test 2: wf() does not guarantee the set is non-empty either.
// A well-formed ArraySet could be empty or non-empty.
// SHOULD FAIL
proof fn test_logical_wf_implies_nonempty(s: ArraySet<4>)
    requires
        s.wf(),
{
    assert(s@.len() > 0);
}

// Test 3: Two well-formed ArraySets are not necessarily equal.
// The wf() predicate does not constrain the set contents uniquely.
// SHOULD FAIL
proof fn test_logical_two_wf_sets_equal(s1: ArraySet<4>, s2: ArraySet<4>)
    requires
        s1.wf(),
        s2.wf(),
{
    assert(s1@ == s2@);
}

// Test 4: Init does NOT preserve the data array from pre-state.
// The spec never claims init is a no-op on internal data.
// Asserting data is preserved is an unwarranted structural assumption.
// SHOULD FAIL
proof fn test_logical_init_preserves_data(pre: ArraySet<4>, post: ArraySet<4>)
    requires
        pre.wf(),
        post.wf(),
        post@ == Set::<usize>::empty(),
{
    // Init overwrites all data to false; pre-state data may differ
    assert(post.data@ =~= pre.data@);
}

// Test 5: Init does NOT preserve the len field from pre-state.
// A wf pre-state can have len > 0, but post always has len == 0.
// Asserting preservation is an unwarranted assumption.
// SHOULD FAIL
proof fn test_logical_init_preserves_len(pre: ArraySet<4>, post: ArraySet<4>)
    requires
        pre.wf(),
        post.wf(),
        post@ == Set::<usize>::empty(),
{
    // post.len is 0 (derivable from post.wf() + empty set)
    // but pre.len could be > 0, so equality fails
    assert(post.len == pre.len);
}


}
