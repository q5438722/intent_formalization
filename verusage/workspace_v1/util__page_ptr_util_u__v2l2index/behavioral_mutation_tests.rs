use vstd::prelude::*;

fn main() {}

verus! {

global size_of usize == 8;

pub type L2Index = usize;

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

pub open spec fn spec_v2l2index(va: usize) -> L2Index {
    (va >> 21 & 0x1ff) as usize
}

// ========== BEHAVIORAL MUTATION TESTS ==========
// These tests assert incorrect input-output relationships for valid inputs.
// They should all FAIL verification because the outputs are wrong.

// SHOULD FAIL: L2 index of 0x0000_0080_0020_0000 is 1, not 0
// (bits 29:21 = 1 for this VA)
proof fn test_wrong_l2_index_value() {
    let va: usize = 0x0000_0080_0020_0000usize;
    assert(spec_v2l2index(va) == 0usize);
}

// SHOULD FAIL: L2 index of 0x0000_0080_3FE0_0000 is 0x1ff (511), not 0x1fe (510)
proof fn test_off_by_one_l2_index() {
    let va: usize = 0x0000_0080_3FE0_0000usize;
    assert(spec_v2l2index(va) == 0x1feusize);
}

// SHOULD FAIL: Using wrong shift amount (12 instead of 21) gives wrong result
// spec_v2l2index uses >> 21, but if we assert the >> 12 result, it should differ
proof fn test_wrong_shift_amount() {
    let va: usize = 0x0000_0080_0020_0000usize;
    // (va >> 12) & 0x1ff != (va >> 21) & 0x1ff for this VA
    assert(spec_v2l2index(va) == ((va >> 12usize) & 0x1ffusize));
}

// SHOULD FAIL: Using wrong mask (0xff instead of 0x1ff) gives wrong result
// For VA with L2 index > 0xff, the narrower mask truncates
proof fn test_wrong_mask_width() {
    let va: usize = 0x0000_0080_3FE0_0000usize;
    // spec_v2l2index gives 0x1ff, but (va >> 21) & 0xff gives 0xff
    assert(spec_v2l2index(va) == ((va >> 21usize) & 0xffusize));
}

// SHOULD FAIL: L2 index cannot exceed 0x1ff due to the 0x1ff mask
proof fn test_l2_index_exceeds_upper_bound() {
    let va: usize = 0x0000_0080_3FE0_0000usize;
    assert(spec_v2l2index(va) > 0x1ffusize);
}

}
