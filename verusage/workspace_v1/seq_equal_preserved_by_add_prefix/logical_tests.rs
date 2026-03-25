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

// === LOGICAL TESTS ===
// These test properties NOT explicitly guaranteed by the specification.

// SHOULD FAIL
// Logical Test 1: Different prefixes — the lemma only applies when SAME prefix is used.
// With different prefixes, p1 + s == p2 + s does NOT imply p1 == p2 in general,
// and p1 + s1 == p2 + s2 does NOT imply s1 == s2.
// Here p1=[1], p2=[2], s1=[3], s2=[3]. p1+s1=[1,3] != p2+s2=[2,3], but we assert equality.
proof fn test_logical_different_prefixes_imply_concat_equal()
{
    let p1: Seq<int> = seq![1];
    let p2: Seq<int> = seq![2];
    let s: Seq<int> = seq![3];
    // Call lemma with p1 (establishes s==s <==> p1+s == p1+s, trivially true)
    seq_equal_preserved_by_add_prefix(p1, s, s);
    // Try to over-generalize: assert p1 + s == p2 + s (different prefixes)
    assert(p1 + s == p2 + s); // SHOULD FAIL
}

// SHOULD FAIL
// Logical Test 2: Commutativity of concatenation — the spec says nothing about
// prefix + s == s + prefix. Concatenation is NOT commutative in general.
proof fn test_logical_concat_commutativity()
{
    let prefix: Seq<int> = seq![1, 2];
    let s: Seq<int> = seq![3, 4];
    seq_equal_preserved_by_add_prefix(prefix, s, s);
    // The lemma tells us prefix + s == prefix + s (trivially).
    // Try to derive commutativity — this is NOT a consequence of the spec.
    assert(prefix + s == s + prefix); // SHOULD FAIL
}

// SHOULD FAIL
// Logical Test 3: Stronger inequality — try to derive a structural property not
// guaranteed by the spec. The spec only relates equality/inequality of s1,s2 to
// equality/inequality of prefix+s1, prefix+s2. It does NOT say anything about
// the length of prefix relative to s1.
// Here we try to assert prefix.len() <= s1.len(), which is not entailed.
proof fn test_logical_unentailed_length_relationship()
{
    let prefix: Seq<int> = seq![1, 2, 3, 4, 5];
    let s1: Seq<int> = seq![10];
    let s2: Seq<int> = seq![10];
    seq_equal_preserved_by_add_prefix(prefix, s1, s2);
    // The spec says nothing about length relationships between prefix and s1/s2.
    // prefix.len() = 5, s1.len() = 1, so prefix.len() <= s1.len() is FALSE.
    assert(prefix.len() <= s1.len()); // SHOULD FAIL
}

}
