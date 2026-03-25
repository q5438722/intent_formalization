use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::set_lib::*;

fn main() {}

verus! {

// ===== Original function (copied from target) =====

#[verifier::spinoff_prover]
pub proof fn lemma_to_set_distributes_over_addition<A>(s: Seq<A>, t: Seq<A>)
    ensures (s+t).to_set() == s.to_set() + t.to_set()
{
    let left = (s+t).to_set();
    let right = s.to_set() + t.to_set();
    assert forall |x| right.contains(x) implies left.contains(x) by {
        assert(s.to_set()+t.to_set() == s.to_set().union(t.to_set()));
        if s.to_set().contains(x) {
            let si = choose |si| 0<=si<s.len() && s[si] == x;
            assert((s+t)[si] == x);
        } else {
            let ti = choose |ti| 0<=ti<t.len() && t[ti] == x;
            assert((s+t)[s.len() + ti] == x);
        }
    }
    assert_sets_equal!(left, right);
}

// ========== LOGICAL TESTS ==========

// Test 1: Soundness — derive `false` from the spec.
// If the specification is consistent, no valid call should allow proving `false`.
// SHOULD FAIL
proof fn test_logical_derive_false(s: Seq<int>, t: Seq<int>) {
    lemma_to_set_distributes_over_addition(s, t);
    assert(false);
}

// Test 2: Set equality does NOT imply sequence equality.
// Set union is commutative: s.to_set() + t.to_set() == t.to_set() + s.to_set().
// By the lemma, (s+t).to_set() == (t+s).to_set(). But this does NOT mean s+t == t+s
// as sequences — concatenation is not commutative.
// SHOULD FAIL
proof fn test_logical_seq_commutativity() {
    let s = seq![1int, 2int];
    let t = seq![3int, 4int];
    lemma_to_set_distributes_over_addition(s, t);
    lemma_to_set_distributes_over_addition(t, s);
    // The sets are equal, but the sequences are not: [1,2,3,4] != [3,4,1,2]
    assert(s + t =~= t + s);
}

// Test 3: Union equals only one operand's set — ignoring t's contribution.
// The spec says (s+t).to_set() == s.to_set() ∪ t.to_set(). Claiming the result
// equals just s.to_set() would mean t contributes nothing, which is false when
// t has elements not in s.
// SHOULD FAIL
proof fn test_logical_union_collapses_to_one_operand() {
    let s = seq![1int, 2int];
    let t = seq![3int, 4int];
    lemma_to_set_distributes_over_addition(s, t);
    // t has elements {3, 4} not in s. The union cannot equal s.to_set() alone.
    assert((s + t).to_set() =~= s.to_set());
}

}
