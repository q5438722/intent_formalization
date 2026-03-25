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
// Boundary Tests: Violate preconditions with invalid inputs
// Each test asserts that an INVALID input satisfies the
// precondition — these should all FAIL verification.
// ============================================================

// SHOULD FAIL: va=0 has L4 index 0, below KERNEL_MEM_END_L4INDEX=1
proof fn test_zero_address_valid() {
    assert(spec_va_4k_valid(0usize) || spec_va_2m_valid(0usize) || spec_va_1g_valid(0usize));
}

// SHOULD FAIL: va=1 is not aligned to any page size (bit 0 set)
proof fn test_unaligned_address_4k_valid() {
    assert(spec_va_4k_valid(1usize));
}

// SHOULD FAIL: bits 48-63 are set, violating canonical address constraint
proof fn test_high_bits_set_address_valid() {
    assert(spec_va_4k_valid(0xFFFF_0080_0000_0000usize));
}

// SHOULD FAIL: 4K-aligned but L4 index = 0 (bits 39-47 all zero)
proof fn test_l4index_zero_4k_aligned_valid() {
    assert(spec_va_4k_valid(0x0000_0000_0000_1000usize));
}

// SHOULD FAIL: max usize has high bits set and is unaligned
proof fn test_max_usize_valid() {
    assert(spec_va_4k_valid(0xFFFF_FFFF_FFFF_FFFFusize) ||
           spec_va_2m_valid(0xFFFF_FFFF_FFFF_FFFFusize) ||
           spec_va_1g_valid(0xFFFF_FFFF_FFFF_FFFFusize));
}

}
