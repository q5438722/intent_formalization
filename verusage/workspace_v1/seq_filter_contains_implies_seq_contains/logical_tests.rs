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

// ---- Logical Tests: Properties NOT explicitly guaranteed ----

// SHOULD FAIL: Derive false from the axiom — axiom should not be unsound
proof fn test_logical_derive_false() 
    ensures false,
{
    let s: Seq<int> = Seq::empty().push(1);
    let pred = |x: int| x > 0;
    seq_filter_is_a_subset_of_original_seq::<int>(s, pred);
}

// SHOULD FAIL: Stronger postcondition — filter containment does NOT guarantee specific index
proof fn test_logical_specific_index(s: Seq<int>, pred: spec_fn(int) -> bool, elt: int)
    requires
        s.filter(pred).contains(elt),
        s.len() > 0,
    ensures s[0] == elt,
{
    seq_filter_contains_implies_seq_contains(s, pred, elt);
}

// SHOULD FAIL: Uniqueness — spec does not guarantee filter returns at most one match
proof fn test_logical_uniqueness()
    ensures forall |s: Seq<int>, pred: spec_fn(int) -> bool|
        s.filter(pred).len() <= 1,
{
}

// SHOULD FAIL: Filter idempotence is NOT derivable from the axiom alone
// (s.filter(pred).filter(pred) == s.filter(pred) is true, but try asserting
//  the wrong direction: filter(pred) == s)
proof fn test_logical_filter_is_identity()
    ensures forall |s: Seq<int>, pred: spec_fn(int) -> bool|
        #[trigger] s.filter(pred) =~= s,
{
}

// SHOULD FAIL: Monotonicity — filtering with different predicates gives same length (wrong)
proof fn test_logical_pred_irrelevant_to_length()
    ensures forall |s: Seq<int>, pred1: spec_fn(int) -> bool, pred2: spec_fn(int) -> bool|
        #[trigger] s.filter(pred1).len() == #[trigger] s.filter(pred2).len(),
{
}

}
