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

    // ======== LOGICAL TESTS ========

    // SHOULD FAIL: matching does NOT imply vectors are non-empty;
    // empty vectors can match
    fn test_logical_match_implies_nonempty() {
        let v1: Vec<u8> = Vec::new();
        let v2: Vec<u8> = Vec::new();
        let eq = do_vec_u8s_match(&v1, &v2);
        assert(eq);             // true: empty vecs match
        assert(v1.len() > 0);   // SHOULD FAIL: they are empty
    }

    // SHOULD FAIL: the function is deterministic — same inputs must yield same output
    fn test_logical_nondeterministic() {
        let v1: Vec<u8> = Vec::new();
        let v2: Vec<u8> = Vec::new();
        let eq1 = do_vec_u8s_match(&v1, &v2);
        let eq2 = do_vec_u8s_match(&v1, &v2);
        assert(eq1 != eq2); // SHOULD FAIL
    }

    // SHOULD FAIL: matching is symmetric — match(a,b) == match(b,a)
    fn test_logical_asymmetric() {
        let mut v1: Vec<u8> = Vec::new();
        v1.push(7u8);
        let mut v2: Vec<u8> = Vec::new();
        v2.push(7u8);
        let eq1 = do_vec_u8s_match(&v1, &v2);
        let eq2 = do_vec_u8s_match(&v2, &v1);
        assert(eq1 != eq2); // SHOULD FAIL
    }

}
