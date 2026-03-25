use vstd::prelude::*;

fn main() {}


verus! {

global size_of usize == 8;

spec fn is_bit_set(a: usize, b: usize) -> bool {
    a & (1usize << b) == (1usize << b)
}
#[allow(unused_macros)]
macro_rules! is_bit_set {
    ($a:expr, $b:expr) => {
        $a & (1u64 << $b) == (1u64 << $b)
    }
}
	#[verifier::external_body]
proof fn lemma_obtain_bit_index_1_aux(a: u64, hi: u64) -> (i: u64)
    requires
        a != 0,
        hi <= 64,
        a >> hi == 0,
    ensures
        i < hi,
        is_bit_set!(a, i),
    decreases hi
	{
		unimplemented!()
	}

proof fn lemma_obtain_bit_index_1(a: usize) -> (b: usize)
    requires a != 0
    ensures
        b < 64,
        is_bit_set(a, b)
{
    reveal(is_bit_set);
    assert(a as u64 >> 64 == 0) by (bit_vector);
    lemma_obtain_bit_index_1_aux(a as u64, 64) as usize
}

pub struct CommitMask {
    mask: [usize; 8],     // size = COMMIT_MASK_FIELD_COUNT
}



// === Entailment query ===
proof fn phi_5_obtain_bit_aux_hi_zero_vacuous(a: u64)
    requires
        a != 0,
        0u64 <= 64u64,
        a >> 0 == 0,
    ensures
        false,
{
    let i = lemma_obtain_bit_index_1_aux(a, 0);
    assert(i < 0) by { }
}

}
