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
proof fn phi_1_lemma_obtain_bit_index_1_aux_unsound_zero_hi()
    ensures
        false,
{
    let a: u64 = 1;
    assert(a != 0);
    assert(a >> 0 == a) by (bit_vector);
    // But the external_body lemma requires a >> hi == 0,
    // which for hi=0 means a == 0, contradicting a != 0.
    // However since it's external_body, we can still "call" it
    // with hi=0 if the requires are vacuously satisfiable.
    // Instead, test that the ensures is consistent:
    // The lemma promises i < hi with hi=64 and is_bit_set.
    // Just verify the lemma's output is usable to derive a concrete bit.
    let b = lemma_obtain_bit_index_1_aux(1u64, 64);
    assert(b < 64);
}

}
