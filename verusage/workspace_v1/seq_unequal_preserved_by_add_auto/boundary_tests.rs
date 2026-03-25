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

// --- Boundary Tests ---

// SHOULD FAIL
// Violate requires: pass two identical empty sequences (s1 == s2)
proof fn test_boundary_equal_empty_sequences() {
    let s: Seq<int> = Seq::empty();
    seq_unequal_preserved_by_add::<int>(s, s, Seq::empty());
}

// SHOULD FAIL
// Violate requires: pass two identical non-empty sequences
proof fn test_boundary_equal_nonempty_sequences() {
    let s: Seq<int> = Seq::empty().push(42);
    seq_unequal_preserved_by_add::<int>(s, s, Seq::empty().push(1));
}

// SHOULD FAIL
// Violate requires: pass structurally equal sequences built independently
proof fn test_boundary_structurally_equal_sequences() {
    let s1: Seq<int> = Seq::empty().push(1).push(2);
    let s2: Seq<int> = Seq::empty().push(1).push(2);
    seq_unequal_preserved_by_add::<int>(s1, s2, Seq::empty());
}

// SHOULD FAIL
// Violate requires: use auto lemma to conclude inequality for equal sequences
proof fn test_boundary_auto_with_equal_sequences() {
    let suffix: Seq<int> = Seq::empty().push(10);
    seq_unequal_preserved_by_add_auto::<int>(suffix);
    let s: Seq<int> = Seq::empty().push(5);
    // The auto lemma says: forall s1, s2: s1 != s2 ==> s1 + suffix != s2 + suffix
    // Try to instantiate with s == s (equal), should not derive s + suffix != s + suffix
    assert(s + suffix != s + suffix);
}

}
