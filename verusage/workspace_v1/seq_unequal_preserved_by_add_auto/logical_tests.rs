use vstd::prelude::*;

fn main() {}

verus! {

// --- Original spec definitions ---

#[verifier::external_body]
pub proof fn seq_unequal_preserved_by_add<A>(s1: Seq<A>, s2: Seq<A>, suffix: Seq<A>)
    requires s1 != s2
    ensures s1 + suffix != s2 + suffix
{ unimplemented!() }

pub proof fn seq_unequal_preserved_by_add_auto<A>(suffix: Seq<A>)
    ensures forall |s1: Seq<A>, s2: Seq<A>| s1 != s2 ==> s1 + suffix != s2 + suffix
{
    assert forall |s1: Seq<A>, s2: Seq<A>| s1 != s2 implies s1 + suffix != s2 + suffix by {
        seq_unequal_preserved_by_add(s1, s2, suffix);
    };
}

// --- Logical Tests ---

// SHOULD FAIL
// Unintended stronger property: sequence inequality does NOT imply length inequality
// Sequences [1] and [2] are unequal but have the same length
proof fn test_logical_length_inequality_from_seq_inequality() {
    let suffix: Seq<int> = Seq::empty();
    seq_unequal_preserved_by_add_auto::<int>(suffix);
    assert(
        forall |s1: Seq<int>, s2: Seq<int>| s1 != s2 ==> s1.len() != s2.len()
    );
}

// SHOULD FAIL
// Unintended structural assumption: sequence inequality does NOT imply
// first-element inequality (sequences can share prefix and differ later)
proof fn test_logical_first_element_inequality() {
    let s1: Seq<int> = Seq::empty().push(1int).push(2int);
    let s2: Seq<int> = Seq::empty().push(1int).push(3int);
    let suffix: Seq<int> = Seq::empty();
    assert(s1[1] != s2[1]);  // hint for solver: they differ at index 1
    seq_unequal_preserved_by_add::<int>(s1, s2, suffix);
    // s1 != s2 but s1[0] == s2[0] == 1
    assert(s1[0] != s2[0]);
}

// SHOULD FAIL
// Cross-function misuse: the spec says appending preserves inequality,
// but try to derive that inequality implies specific element-wise differences
// at the position AFTER the original sequences (i.e., in the suffix portion)
proof fn test_logical_suffix_elements_differ() {
    let s1: Seq<int> = Seq::empty().push(1int);
    let s2: Seq<int> = Seq::empty().push(2int);
    let suffix: Seq<int> = Seq::empty().push(99int);
    assert(s1[0] != s2[0]);  // hint for solver
    seq_unequal_preserved_by_add::<int>(s1, s2, suffix);
    // s1 + suffix = [1, 99], s2 + suffix = [2, 99]
    // They differ at index 0, not at the suffix position (index 1)
    assert((s1 + suffix)[1] != (s2 + suffix)[1]);
}

// SHOULD FAIL
// Unintended global property: try to derive that the auto lemma implies
// ALL pairs of sequences with the same length are unequal (absurd)
proof fn test_logical_all_same_length_unequal() {
    let suffix: Seq<int> = Seq::empty();
    seq_unequal_preserved_by_add_auto::<int>(suffix);
    assert(
        forall |s1: Seq<int>, s2: Seq<int>| s1.len() == s2.len() && s1.len() > 0 ==> s1 != s2
    );
}

}
