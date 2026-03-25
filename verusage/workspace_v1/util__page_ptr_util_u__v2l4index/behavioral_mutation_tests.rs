use vstd::prelude::*;

fn main() {}

verus!{

global size_of usize == 8;

pub type L4Index = usize;

pub const KERNEL_MEM_END_L4INDEX: usize = 1;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_2m_MASK: u64 = 0x0000_ffff_ffe0_0000;
pub const MEM_1g_MASK: u64 = 0x0000_fffc_0000_0000;

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn spec_va_2m_valid(va: usize) -> bool {
    (va & (!MEM_2m_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn spec_va_1g_valid(va: usize) -> bool {
    (va & (!MEM_1g_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn spec_v2l4index(va: usize) -> L4Index {
    (va >> 39 & 0x1ff) as usize
}

// ============================================================
// Behavioral Mutation Tests: Assert WRONG output values
// for valid inputs. Each test mutates the expected result.
// These should all FAIL verification.
// ============================================================

// SHOULD FAIL: off-by-one — result should be 1, not 2
proof fn test_result_off_by_one() {
    let va: usize = 0x0000_0080_0000_0000usize;
    assert(spec_v2l4index(va) == 2usize) by(bit_vector);
}

// SHOULD FAIL: result should be 1, not 0 (below KERNEL_MEM_END_L4INDEX)
proof fn test_result_zero_for_valid_input() {
    let va: usize = 0x0000_0080_0000_0000usize;
    assert(spec_v2l4index(va) == 0usize) by(bit_vector);
}

// SHOULD FAIL: result should be 0x1FF (511), not 0x100 (256)
proof fn test_wrong_result_max_index() {
    let va: usize = 0x0000_FF80_0000_0000usize;
    assert(spec_v2l4index(va) == 0x100usize) by(bit_vector);
}

// SHOULD FAIL: result should be <= 0x1FF, not greater
proof fn test_result_exceeds_upper_bound() {
    let va: usize = 0x0000_FF80_0000_0000usize;
    assert(spec_v2l4index(va) > 0x1FFusize) by(bit_vector);
}

// SHOULD FAIL: result IS 1 for this input, asserting != 1 is false
proof fn test_negated_correct_result() {
    let va: usize = 0x0000_0080_0000_0000usize;
    assert(spec_v2l4index(va) != 1usize) by(bit_vector);
}

}
