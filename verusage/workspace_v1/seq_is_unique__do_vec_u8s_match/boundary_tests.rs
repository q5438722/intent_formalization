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

    // ======== BOUNDARY TESTS ========

    // SHOULD FAIL: two empty vectors ARE equal, so result should be true
    fn test_boundary_empty_vecs_not_equal() {
        let v1: Vec<u8> = Vec::new();
        let v2: Vec<u8> = Vec::new();
        let eq = do_vec_u8s_match(&v1, &v2);
        assert(!eq); // SHOULD FAIL
    }

    // SHOULD FAIL: two identical single-element vectors ARE equal
    fn test_boundary_same_single_element_not_equal() {
        let mut v1: Vec<u8> = Vec::new();
        v1.push(42u8);
        let mut v2: Vec<u8> = Vec::new();
        v2.push(42u8);
        let eq = do_vec_u8s_match(&v1, &v2);
        assert(!eq); // SHOULD FAIL
    }

    // SHOULD FAIL: sequences of different lengths cannot be equal
    proof fn test_boundary_different_length_seqs_equal() {
        let s1: Seq<u8> = Seq::empty();
        let s2: Seq<u8> = Seq::empty().push(1u8);
        assert(s1 =~= s2); // SHOULD FAIL
    }

}
