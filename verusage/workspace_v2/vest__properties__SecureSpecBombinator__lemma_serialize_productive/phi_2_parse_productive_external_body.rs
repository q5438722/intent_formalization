use vstd::prelude::*;

fn main() {}

verus!{

// File: src/properties.rs
pub trait SpecCombinator {
    type Type;

    open spec fn wf(&self, v: Self::Type) -> bool {
        true
    }

    open spec fn requires(&self) -> bool {
        true
    }

    spec fn spec_parse(&self, s: Seq<u8>) -> Option<(int, Self::Type)>;

    spec fn spec_serialize(&self, v: Self::Type) -> Seq<u8>;

}

pub trait SecureSpecCombinator: SpecCombinator {

	#[verifier::external_body]
    proof fn theorem_serialize_parse_roundtrip(&self, v: Self::Type)
        requires
            self.requires(),
        ensures
            self.wf(v) ==> self.spec_parse(self.spec_serialize(v)) == Some(
                (self.spec_serialize(v).len() as int, v),
            ),
	{
		unimplemented!()
	}

    spec fn is_prefix_secure() -> bool;

    spec fn is_productive(&self) -> bool;

	#[verifier::external_body]
    proof fn lemma_parse_productive(&self, s: Seq<u8>)
        requires
            self.requires(),
        ensures
            self.is_productive() ==> (self.spec_parse(s) matches Some((n, _)) ==> n > 0),
	{
		unimplemented!()
	}

    proof fn lemma_serialize_productive(&self, v: Self::Type)
        requires
            self.requires(),
            self.wf(v),
        ensures
            self.is_productive() ==> self.spec_serialize(v).len() > 0,
    {
        self.theorem_serialize_parse_roundtrip(v);
        self.lemma_parse_productive(self.spec_serialize(v));
    }

}






// === Entailment query ===
proof fn phi_2_parse_productive_external_body<C: SecureSpecCombinator>(c: &C, s: Seq<u8>)
    requires
        c.requires(),
        c.is_productive(),
        c.spec_parse(s) matches Some((n, _)) && true,
    ensures
        ({
            if let Some((n, _)) = c.spec_parse(s) { n > 0 } else { true }
        }),
{
    c.lemma_parse_productive(s);
}

}
