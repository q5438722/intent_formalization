use vstd::prelude::*;

fn main() {}

verus! {

    // ---- Original function under test ----
    pub fn do_vec_u8s_match(e1: &Vec<u8>, e2: &Vec<u8>) -> (eq: bool)
        ensures
            eq == (e1@ == e2@)
    {
        if e1.len() != e2.len() {
            assert (e1@.len() != e2@.len());
            assert (e1@ != e2@);
            return false;
        }

        let mut i: usize = 0;
        while i < e1.len()
            invariant
                0 <= i,
                i <= e1.len(),
                e1.len() == e2.len(),
                forall |j: int| 0 <= j && j < i ==> e1@[j] == e2@[j]
            decreases
                e1.len() - i
        {
            if e1[i] != e2[i] {
                return false;
            }
            i += 1;
        }
        proof {
            assert(e1@=~=e2@);
        }
        return true;
    }

    // ======== BEHAVIORAL MUTATION TESTS ========

    // SHOULD FAIL: two equal multi-element vectors must return true, not false
    fn test_mutation_equal_vecs_return_false() {
        let mut v1: Vec<u8> = Vec::new();
        v1.push(10u8);
        v1.push(20u8);
        let mut v2: Vec<u8> = Vec::new();
        v2.push(10u8);
        v2.push(20u8);
        let eq = do_vec_u8s_match(&v1, &v2);
        assert(!eq); // SHOULD FAIL
    }

    // SHOULD FAIL: two vectors with different elements must return false, not true
    fn test_mutation_different_vecs_return_true() {
        let mut v1: Vec<u8> = Vec::new();
        v1.push(1u8);
        let mut v2: Vec<u8> = Vec::new();
        v2.push(2u8);
        let eq = do_vec_u8s_match(&v1, &v2);
        assert(eq); // SHOULD FAIL
    }

    // SHOULD FAIL: vectors differing at the first element must return false, not true
    fn test_mutation_first_element_differs_still_match() {
        let mut v1: Vec<u8> = Vec::new();
        v1.push(0u8);
        v1.push(5u8);
        v1.push(9u8);
        let mut v2: Vec<u8> = Vec::new();
        v2.push(255u8);
        v2.push(5u8);
        v2.push(9u8);
        let eq = do_vec_u8s_match(&v1, &v2);
        assert(eq); // SHOULD FAIL
    }

}
