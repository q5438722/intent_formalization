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
proof fn phi_4_witness_bit_position_unique_power_of_two() -> (i: usize)
    ensures
        i < 64,
        !is_bit_set(0usize, i),
        is_bit_set(1usize, i),
        i == 0,
{
    assert(0u64 & 1u64 != 1u64) by (bit_vector);
    assert(0u64 >> 64 == 0) by (bit_vector);
    assert(1u64 >> 64 == 0) by (bit_vector);
    reveal(is_bit_set);
    let i = lemma_obtain_bit_index_3_aux(0, 1, 64);
    assert(i == 0) by (bit_vector)
        requires !is_bit_set!(0u64, i), is_bit_set!(1u64, i), i < 64;
    i as usize
}

}
