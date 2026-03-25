use vstd::prelude::*;

fn main() {}

verus! {

global size_of usize == 8;

pub type L3Index = usize;

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

pub open spec fn spec_v2l3index(va: usize) -> L3Index {
    (va >> 30 & 0x1ff) as usize
}

// SHOULD FAIL
// For va = 0x0000_0080_4000_0000 (valid, L3 index = 1), assert wrong value 2
proof fn test_behavioral_wrong_value() {
    let va: usize = 0x0000_0080_4000_0000usize;
    assert(spec_v2l3index(va) == 2usize) by(bit_vector);
}

// SHOULD FAIL
// For va = 0x0000_00ff_c000_0000 (valid, L3 index = 0x1ff), assert exceeds bound
proof fn test_behavioral_exceeds_bound() {
    let va: usize = 0x0000_00ff_c000_0000usize;
    assert(spec_v2l3index(va) > 0x1ffusize) by(bit_vector);
}

// SHOULD FAIL
// For va = 0x0000_0080_4000_0000 (valid, L3 index = 1), assert off-by-one value 0
proof fn test_behavioral_off_by_one() {
    let va: usize = 0x0000_0080_4000_0000usize;
    assert(spec_v2l3index(va) == 0usize) by(bit_vector);
}

// SHOULD FAIL
// For va = 0x0000_0080_0000_0000 (valid, L3 index = 0), assert it's non-zero
proof fn test_behavioral_negate_result() {
    let va: usize = 0x0000_0080_0000_0000usize;
    assert(spec_v2l3index(va) != 0usize) by(bit_vector);
}

}
