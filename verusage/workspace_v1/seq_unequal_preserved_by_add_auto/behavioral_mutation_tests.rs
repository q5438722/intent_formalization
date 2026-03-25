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

// --- Behavioral Mutation Tests ---

// SHOULD FAIL
// Negate postcondition: assert concatenated sequences ARE equal when inputs differ
proof fn test_mutation_negated_postcondition() {
    let s1: Seq<int> = Seq::empty().push(1int);
    let s2: Seq<int> = Seq::empty().push(2int);
    let suffix: Seq<int> = Seq::empty().push(3int);
    assert(s1[0] != s2[0]);  // hint for solver
    seq_unequal_preserved_by_add::<int>(s1, s2, suffix);
    assert(s1 + suffix == s2 + suffix);
}

// SHOULD FAIL
// Mutated output: assert concatenated sequences have different lengths
// (they actually have the same length since s1 and s2 have the same length)
proof fn test_mutation_wrong_length_relation() {
    let s1: Seq<int> = Seq::empty().push(1int);
    let s2: Seq<int> = Seq::empty().push(2int);
    let suffix: Seq<int> = Seq::empty().push(3int);
    assert(s1[0] != s2[0]);  // hint for solver
    seq_unequal_preserved_by_add::<int>(s1, s2, suffix);
    assert((s1 + suffix).len() != (s2 + suffix).len());
}

// SHOULD FAIL
// Mutated relation: after proving inequality with one suffix, try to conclude
// the concatenated sequences are equal with a DIFFERENT suffix
proof fn test_mutation_equality_different_suffix() {
    let s1: Seq<int> = Seq::empty().push(1int);
    let s2: Seq<int> = Seq::empty().push(2int);
    let suffix1: Seq<int> = Seq::empty().push(10int);
    assert(s1[0] != s2[0]);  // hint for solver
    seq_unequal_preserved_by_add::<int>(s1, s2, suffix1);
    let suffix2: Seq<int> = Seq::empty().push(20int);
    assert(s1 + suffix2 == s2 + suffix2);
}

// SHOULD FAIL
// Mutated output: assert first elements of concatenated sequences differ
// when the original sequences share the same first element
proof fn test_mutation_wrong_element_relation() {
    let s1: Seq<int> = Seq::empty().push(1int).push(2int);
    let s2: Seq<int> = Seq::empty().push(1int).push(3int);
    let suffix: Seq<int> = Seq::empty().push(4int);
    assert(s1[1] != s2[1]);  // hint for solver: they differ at index 1
    seq_unequal_preserved_by_add::<int>(s1, s2, suffix);
    assert((s1 + suffix)[0] != (s2 + suffix)[0]);
}

}
