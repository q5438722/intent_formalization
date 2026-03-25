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

// ========== BOUNDARY TESTS ==========
// These tests assert that invalid inputs satisfy the validity predicates.
// They should all FAIL verification because the inputs violate preconditions.

// SHOULD FAIL: VA = 0 has L4 index 0, which is < KERNEL_MEM_END_L4INDEX (1)
proof fn test_zero_va_is_4k_valid() {
    assert(spec_va_4k_valid(0usize));
}

// SHOULD FAIL: VA with bit 0 set is not 4K-aligned (low 12 bits must be 0)
proof fn test_unaligned_va_is_4k_valid() {
    assert(spec_va_4k_valid(0x0000_0080_0000_0001usize));
}

// SHOULD FAIL: VA with bits 48-63 set fails the mask check
proof fn test_high_bits_va_is_4k_valid() {
    assert(spec_va_4k_valid(0xFFFF_0080_0000_0000usize));
}

// SHOULD FAIL: VA = 0x1000 is 4K aligned but has L4 index = 0 (< 1)
proof fn test_low_aligned_va_is_4k_valid() {
    assert(spec_va_4k_valid(0x0000_0000_0000_1000usize));
}

// SHOULD FAIL: 4K-aligned VA (not 2M-aligned) should not satisfy va_2m_valid
proof fn test_non_2m_aligned_is_2m_valid() {
    assert(spec_va_2m_valid(0x0000_0080_0000_1000usize));
}

// SHOULD FAIL: 2M-aligned VA (not 1G-aligned) should not satisfy va_1g_valid
proof fn test_non_1g_aligned_is_1g_valid() {
    assert(spec_va_1g_valid(0x0000_0080_0020_0000usize));
}

}
