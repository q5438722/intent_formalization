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
proof fn phi_5_is_bit_set_contradicts_for_wrapping_shifts()
    ensures
        ({
            // 1usize << 64 wraps to 0, so is_bit_set(x, 64) is always false
            // But 1usize << 0 wraps to 1, so is_bit_set(1, 0) is true
            // Yet phi_3 says they're equal — both must be false
            !is_bit_set(1usize, 0),
        }) || ({
            // Or alternatively, is_bit_set(1, 0) is true as expected
            is_bit_set(1usize, 0)
        }),
{
    reveal(is_bit_set);
    assert(1usize & 1usize == 1usize) by (bit_vector);
    assert(1usize << 0 == 1usize) by (bit_vector);
}

}
