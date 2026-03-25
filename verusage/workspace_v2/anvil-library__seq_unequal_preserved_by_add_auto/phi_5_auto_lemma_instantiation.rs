use vstd::prelude::*;

fn main() {}

verus!{

#[verifier::external_body]
pub proof fn seq_unequal_preserved_by_add<A>(s1: Seq<A>, s2: Seq<A>, suffix: Seq<A>)
    requires s1 != s2
    ensures s1 + suffix != s2 + suffix
{unimplemented!()}

pub proof fn seq_unequal_preserved_by_add_auto<A>(suffix: Seq<A>)
    ensures forall |s1: Seq<A>, s2: Seq<A>| s1 != s2 ==> s1 + suffix != s2 + suffix
{
    assert forall |s1: Seq<A>, s2: Seq<A>| s1 != s2 implies s1 + suffix != s2 + suffix by {
        seq_unequal_preserved_by_add(s1, s2, suffix);
    };
}



// === Entailment query ===
proof fn phi_5_auto_lemma_instantiation(a: int, b: int, suffix: Seq<int>)
    requires
        a != b,
    ensures
        seq![a] + suffix != seq![b] + suffix,
{
    seq_unequal_preserved_by_add_auto(suffix);
}

}
