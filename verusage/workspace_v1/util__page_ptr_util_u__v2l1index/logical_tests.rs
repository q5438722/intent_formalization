use vstd::prelude::*;

fn main() {}

verus! {

global size_of usize == 8;

// ===== Definitions from target file =====

pub type L1Index = usize;

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

pub open spec fn spec_v2l1index(va: usize) -> L1Index {
    (va >> 12 & 0x1ff) as usize
}

// ===== Logical Tests =====
// Each test asserts a property NOT explicitly guaranteed by the specification.
// All tests SHOULD FAIL verification.

// Test 1: Assert injectivity — two distinct valid addresses MUST have different L1 indices
// This is FALSE: addresses differing only in higher-order bits share the same L1 index.
// va1 = 0x0000_0080_0000_1000 (L4=1, L1=1)
// va2 = 0x0000_0100_0000_1000 (L4=2, L1=1)
// SHOULD FAIL
proof fn test_logical_injectivity() {
    let va1: usize = 0x0000_0080_0000_1000usize;
    let va2: usize = 0x0000_0100_0000_1000usize;
    // Both have L1 index = 1, asserting they differ should fail
    assert(spec_v2l1index(va1) != spec_v2l1index(va2)) by (bit_vector);
}

// Test 2: Assert stronger upper bound — L1 index < 256 (8 bits instead of 9)
// The spec guarantees <= 0x1ff (512 values). L1 index CAN be >= 256.
// SHOULD FAIL
proof fn test_logical_stronger_bound() {
    // For all va, assert L1 index < 256 — this is too strong
    let va: usize = 0x0000_0080_001f_f000usize; // L1 = 0x1ff = 511 >= 256
    assert(spec_v2l1index(va) < 256usize) by (bit_vector);
}

// Test 3: Assert va_4k_valid implies va_2m_valid — FALSE, 4K addresses may not be 2M aligned
// SHOULD FAIL
proof fn test_logical_4k_implies_2m() {
    let va: usize = 0x0000_0080_0000_1000usize; // 4K aligned, not 2M aligned
    // Assert that 4K validity implies 2M validity
    assert(spec_va_4k_valid(va) ==> spec_va_2m_valid(va)) by (bit_vector);
}

// Test 4: Assert L1 index is always even for valid 4K addresses — FALSE
// SHOULD FAIL
proof fn test_logical_always_even() {
    let va: usize = 0x0000_0080_0000_1000usize; // L1 = 1 (odd)
    assert(spec_v2l1index(va) % 2 == 0usize) by (bit_vector); // wrong: L1 = 1 is odd
}

// Test 5: Assert L1 index uniquely determines the address — FALSE (many addresses map to same L1)
// This tests an unentailed structural claim: the mapping is NOT a bijection.
// SHOULD FAIL
proof fn test_logical_l1_determines_va() {
    let va1: usize = 0x0000_0080_0000_1000usize; // L1 = 1
    let va2: usize = 0x0000_0080_0020_1000usize; // L1 = 1 (differs in L2 index bits)
    // If L1 indices are equal, the addresses must be equal — FALSE
    assert(spec_v2l1index(va1) == spec_v2l1index(va2) ==> va1 == va2) by (bit_vector);
}

// Test 6: Assert v2l1index always returns 0 for all valid addresses — FALSE
// SHOULD FAIL
proof fn test_logical_always_zero() {
    let va: usize = 0x0000_0080_0000_1000usize; // L1 = 1, not 0
    assert(spec_v2l1index(va) == 0usize) by (bit_vector);
}

}
