use vstd::prelude::*;

fn main() {}

verus!{

// Original lemma under test
pub proof fn seq_equal_preserved_by_add_prefix<A>(prefix: Seq<A>, s1: Seq<A>, s2: Seq<A>)
    ensures s1 == s2 <==> prefix + s1 == prefix + s2
{
    assert_by(
        s1 == s2 ==> prefix + s1 == prefix + s2,
        {
            if s1 == s2 {
                let len = prefix.len();
                assert forall |i| 0<= i < (prefix + s1).len() implies (#[trigger] (prefix + s1)[i]) == (prefix + s2)[i] by {
                    if i < len {
                        assert((prefix + s1)[i] == prefix[i]);
                        assert((prefix + s2)[i] == prefix[i]);
                    } else {
                        assert((prefix + s1)[i] == s1[i - len]);
                        assert((prefix + s2)[i] == s2[i - len]);
                    }
                }
            }
        }
    );
    assert_by(
        prefix + s1 == prefix + s2 ==> s1 == s2,
        {
            if prefix + s1 == prefix + s2 {
                assert((prefix + s1).len() == (prefix + s2).len());
                assert(s1.len() == s2.len());
                let len = prefix.len();
                assert forall |i| 0<= i < s1.len() implies (#[trigger] s1[i]) == s2[i] by {
                    assert(s1[i] == (prefix + s1)[i + len]);
                    assert(s2[i] == (prefix + s2)[i + len]);
                }
                assert(s1 =~= s2);
            }
        }
    )
}

// === BEHAVIORAL MUTATION TESTS ===
// These start from valid inputs but mutate expected output relations.

// SHOULD FAIL
// Mutation Test 1: Negate forward direction — equal sequences but claim concatenations differ.
// s1 == s2, so prefix + s1 == prefix + s2 by the lemma. Asserting != is a mutation.
proof fn test_mutation_negate_forward()
{
    let s1: Seq<int> = seq![1, 2, 3];
    let s2: Seq<int> = seq![1, 2, 3];
    let prefix: Seq<int> = seq![0];
    seq_equal_preserved_by_add_prefix(prefix, s1, s2);
    assert(prefix + s1 != prefix + s2); // SHOULD FAIL
}

// SHOULD FAIL
// Mutation Test 2: Negate backward direction — unequal sequences but claim they are equal.
// s1 != s2 (differ at index 0), so the lemma guarantees prefix + s1 != prefix + s2.
// Mutated: assert s1 == s2 despite them being different.
proof fn test_mutation_negate_backward()
{
    let s1: Seq<int> = seq![1];
    let s2: Seq<int> = seq![2];
    let prefix: Seq<int> = seq![0];
    seq_equal_preserved_by_add_prefix(prefix, s1, s2);
    assert(s1 == s2); // SHOULD FAIL
}

// SHOULD FAIL
// Mutation Test 3: Wrong element relationship — after establishing equality, claim a wrong
// element value in the concatenation.
// prefix = [10], s1 = s2 = [20]. prefix + s1 = [10, 20].
// Mutated: assert (prefix + s1)[1] == 99 (should be 20).
proof fn test_mutation_wrong_element()
{
    let s1: Seq<int> = seq![20int];
    let s2: Seq<int> = seq![20int];
    let prefix: Seq<int> = seq![10int];
    seq_equal_preserved_by_add_prefix(prefix, s1, s2);
    assert((prefix + s1)[1] == 99int); // SHOULD FAIL
}

}
