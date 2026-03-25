use vstd::prelude::*;

fn main() {}

verus! {

    global size_of usize==8;

#[allow(unused_macros)]
macro_rules! is_bit_set {
    ($a:expr, $b:expr) => {
        $a & (1u64 << $b) == (1u64 << $b)
    }
}

spec fn is_bit_set(a: usize, b: usize) -> bool {
    a & (1usize << b) == (1usize << b)
}

	#[verifier::external_body]
proof fn lemma_obtain_bit_index_3_aux(a: u64, b: u64, hi: u64) -> (i: u64)
    requires
        a & b != b,
        hi <= 64,
        a >> hi == 0,
        b >> hi == 0,
    ensures
        i < hi,
        !is_bit_set!(a, i),
        is_bit_set!(b, i),
    decreases hi
	{
		unimplemented!()
	}

proof fn lemma_obtain_bit_index_3(a: usize, b: usize) -> (i: usize)
    requires a & b != b
    ensures
        i < 64,
        !is_bit_set(a, i),
        is_bit_set(b, i),
{
    reveal(is_bit_set);
    assert(a as u64 >> 64 == 0) by (bit_vector);
    assert(b as u64 >> 64 == 0) by (bit_vector);
    lemma_obtain_bit_index_3_aux(a as u64, b as u64, 64) as usize
}

pub struct CommitMask {
    mask: [usize; 8],     // size = COMMIT_MASK_FIELD_COUNT
}



// === Entailment query ===
proof fn phi_5_is_bit_set_wrap_at_64(a: usize)
    requires
        is_bit_set(a, 0),
    ensures
        is_bit_set(a, 64),
{
    reveal(is_bit_set);
    assert(1usize << 64 == 1usize << 0) by (bit_vector);
}

}
