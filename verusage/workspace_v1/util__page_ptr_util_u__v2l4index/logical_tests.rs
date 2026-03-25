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
// Logical Tests: Assert properties NOT guaranteed by the spec.
// These probe unintended reasoning paths.
// These should all FAIL verification.
// ============================================================

// SHOULD FAIL: L4 index is not injective — different addresses can share the same index
proof fn test_non_injectivity() {
    let va1: usize = 0x0000_0080_0000_0000usize;
    let va2: usize = 0x0000_0100_0000_0000usize;
    // va1 has L4 index 1, va2 has L4 index 2 — they ARE different
    // But asserting they're equal should fail
    assert(spec_v2l4index(va1) == spec_v2l4index(va2)) by(bit_vector);
}

// SHOULD FAIL: L4 index can equal KERNEL_MEM_END_L4INDEX, not strictly greater
proof fn test_strictly_greater_lower_bound() {
    let va: usize = 0x0000_0080_0000_0000usize;
    // spec_v2l4index returns 1 == KERNEL_MEM_END_L4INDEX, so > 1 is false
    assert(spec_v2l4index(va) > KERNEL_MEM_END_L4INDEX) by(bit_vector);
}

// SHOULD FAIL: 4K validity does NOT imply 2M validity
proof fn test_4k_implies_2m() {
    let va: usize = 0x0000_0080_0000_1000usize;
    // This address is 4K-aligned (bit 12+) but NOT 2M-aligned (bits 0-20 not all zero)
    assert(
        spec_va_4k_valid(va) ==> spec_va_2m_valid(va)
    ) by(bit_vector);
}

// SHOULD FAIL: monotonicity does not hold — close addresses can have the same L4 index
proof fn test_monotonicity() {
    let va1: usize = 0x0000_0080_0000_0000usize;
    let va2: usize = 0x0000_0080_0001_0000usize;
    // va1 < va2, but both have L4 index 1, so strict < does not hold
    assert(spec_v2l4index(va1) < spec_v2l4index(va2)) by(bit_vector);
}

// SHOULD FAIL: the three validity predicates are NOT equivalent
proof fn test_validity_equivalence() {
    let va: usize = 0x0000_0080_0000_1000usize;
    // 4K-valid but not 2M-valid — asserting equivalence should fail
    assert(
        spec_va_4k_valid(va) == spec_va_2m_valid(va)
    ) by(bit_vector);
}

}
