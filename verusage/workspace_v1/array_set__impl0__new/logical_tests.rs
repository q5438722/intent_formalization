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
// LOGICAL TESTS
// These tests check properties NOT explicitly guaranteed by
// the specification: determinism, stronger inequalities,
// structural/global assumptions, cross-function misuse.
// All tests SHOULD FAIL verification.
// ============================================================

// Test 1: wf() does not guarantee the set is empty.
// A well-formed ArraySet could have any valid subset of 0..N.
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
// The wf() predicate allows any valid set, not a unique one.
// SHOULD FAIL
proof fn test_logical_two_wf_sets_equal(s1: ArraySet<4>, s2: ArraySet<4>)
    requires
        s1.wf(),
        s2.wf(),
{
    assert(s1@ == s2@);
}

// Test 4: Array::new() only guarantees length == N (wf).
// It does NOT guarantee determinism of element values.
// Two Array::new() calls may produce different sequences.
// SHOULD FAIL
proof fn test_logical_array_new_determinism(a1: Array<bool, 4>, a2: Array<bool, 4>)
    requires
        a1.wf(),
        a2.wf(),
{
    assert(a1@ =~= a2@);
}

// Test 5: new() does not imply N > 0.
// ArraySet<0>::new() is valid and produces an empty set.
// Asserting N > 0 from the postcondition is unwarranted.
// SHOULD FAIL
proof fn test_logical_new_implies_positive_n(post: ArraySet<4>)
    requires
        post.wf(),
        post@ == Set::<usize>::empty(),
{
    // Try to assert a stronger bound: len must be at least 1
    // This is false because len == 0 for an empty set
    assert(post.len >= 1);
}


}
