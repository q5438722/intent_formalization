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
// BEHAVIORAL MUTATION TESTS
// These tests start from valid inputs but mutate expected
// outputs/relations to check if incorrect behaviors are rejected.
// All tests SHOULD FAIL verification.
// ============================================================

// Test 1: Init ensures post@ == Set::empty().
// Mutate: assert the set is NOT empty after init.
// This directly contradicts the postcondition.
// SHOULD FAIL
proof fn test_mutation_init_not_empty(pre: ArraySet<4>, post: ArraySet<4>)
    requires
        pre.wf(),
        post.wf(),
        post@ == Set::<usize>::empty(),
{
    assert(post@ != Set::<usize>::empty());
}

// Test 2: Init clears the set completely.
// Mutate: claim that an element from the pre-state survives init.
// SHOULD FAIL
proof fn test_mutation_init_preserves_element(pre: ArraySet<4>, post: ArraySet<4>)
    requires
        pre.wf(),
        pre@.contains(2usize),
        post.wf(),
        post@ == Set::<usize>::empty(),
{
    // Element 2 was in pre but should be gone in post
    assert(post@.contains(2usize));
}

// Test 3: Init produces an empty set.
// Mutate: assert it produces a singleton set {0} instead.
// SHOULD FAIL
proof fn test_mutation_init_produces_singleton(pre: ArraySet<4>, post: ArraySet<4>)
    requires
        pre.wf(),
        post.wf(),
        post@ == Set::<usize>::empty(),
{
    assert(post@ == Set::<usize>::empty().insert(0usize));
}

// Test 4: The empty set has length 0.
// Mutate: assert the set length is 1 instead.
// SHOULD FAIL
proof fn test_mutation_init_set_len_one(pre: ArraySet<4>, post: ArraySet<4>)
    requires
        pre.wf(),
        post.wf(),
        post@ == Set::<usize>::empty(),
{
    assert(post@.len() == 1);
}

// Test 5: Init clears the set regardless of pre-state.
// Mutate: assert init acts as identity (preserves the pre-state set).
// SHOULD FAIL
proof fn test_mutation_init_is_identity(pre: ArraySet<4>, post: ArraySet<4>)
    requires
        pre.wf(),
        pre@.contains(1usize),
        pre@.contains(3usize),
        post.wf(),
        post@ == Set::<usize>::empty(),
{
    // Init should clear, not preserve. Asserting equality should fail.
    assert(pre@ == post@);
}


}
