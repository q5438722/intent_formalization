use vstd::prelude::*;

fn main() {}

verus!{

// ---- Original specification under test ----

#[verifier::external_body]
pub proof fn seq_filter_is_a_subset_of_original_seq<A>(s: Seq<A>, pred: spec_fn(A) -> bool)
    ensures
        forall |e: A| s.filter(pred).contains(e) ==> #[trigger] s.contains(e),
        forall |i: int| 0 <= i < s.filter(pred).len() ==> s.contains(#[trigger] s.filter(pred)[i]),
{ unimplemented!() }

pub proof fn seq_filter_contains_implies_seq_contains<A>(s: Seq<A>, pred: spec_fn(A) -> bool, elt: A)
    requires s.filter(pred).contains(elt),
    ensures s.contains(elt)
{
    seq_filter_is_a_subset_of_original_seq(s, pred);
}

// ---- Behavioral Mutation Tests: Mutate outputs/relations ----

// SHOULD FAIL: Negated postcondition — filter containment should NOT imply absence from original
proof fn test_mutation_negated_postcondition(s: Seq<int>, pred: spec_fn(int) -> bool, elt: int)
    requires s.filter(pred).contains(elt),
    ensures !s.contains(elt),
{
    seq_filter_contains_implies_seq_contains(s, pred, elt);
}

// SHOULD FAIL: General converse — s.contains(elt) does NOT imply s.filter(pred).contains(elt)
proof fn test_mutation_converse_direction()
    ensures forall |s: Seq<int>, pred: spec_fn(int) -> bool, e: int|
        s.contains(e) ==> s.filter(pred).contains(e),
{
}

// SHOULD FAIL: Filter preserves length — filtering generally removes elements
proof fn test_mutation_filter_preserves_length()
    ensures forall |s: Seq<int>, pred: spec_fn(int) -> bool|
        s.filter(pred).len() == s.len(),
{
}

// SHOULD FAIL: Wrong element transfer — filter containing elt1 does NOT prove elt2 is in original
proof fn test_mutation_wrong_element(s: Seq<int>, pred: spec_fn(int) -> bool, elt1: int, elt2: int)
    requires
        s.filter(pred).contains(elt1),
        elt1 != elt2,
    ensures s.contains(elt2),
{
    seq_filter_contains_implies_seq_contains(s, pred, elt1);
}

// SHOULD FAIL: Cross-predicate transfer — filter(pred1).contains(e) does NOT imply filter(pred2).contains(e)
proof fn test_mutation_cross_predicate()
    ensures forall |s: Seq<int>, pred1: spec_fn(int) -> bool, pred2: spec_fn(int) -> bool, e: int|
        s.filter(pred1).contains(e) ==> s.filter(pred2).contains(e),
{
}

}
