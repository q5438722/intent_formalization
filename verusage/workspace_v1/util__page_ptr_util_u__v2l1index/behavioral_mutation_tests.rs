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

// ===== Behavioral Mutation Tests =====
// Each test asserts an incorrect input-output relationship.
// All tests SHOULD FAIL verification.

// Test 1: Assert wrong concrete value — L1 index of valid address is NOT 0 when it should be 1
// va = 0x0000_0080_0000_1000: L4=1, 4K-aligned, L1 index = 1
// SHOULD FAIL
proof fn test_mutation_wrong_l1_index_value() {
    let va: usize = 0x0000_0080_0000_1000usize;
    assert(spec_v2l1index(va) == 0usize) by (bit_vector); // wrong: actual L1 = 1
}

// Test 2: Assert result exceeds upper bound (> 0x1ff)
// va = 0x0000_0080_001f_f000: L4=1, 4K-aligned, L1 index = 0x1ff
// SHOULD FAIL
proof fn test_mutation_exceeds_upper_bound() {
    let va: usize = 0x0000_0080_001f_f000usize;
    assert(spec_v2l1index(va) > 0x1ffusize) by (bit_vector); // postcondition says <= 0x1ff
}

// Test 3: Assert off-by-one — claim L1 index is 2 when it's actually 1
// va = 0x0000_0080_0000_1000: L1 index = 1
// SHOULD FAIL
proof fn test_mutation_off_by_one() {
    let va: usize = 0x0000_0080_0000_1000usize;
    assert(spec_v2l1index(va) == 2usize) by (bit_vector); // wrong: actual is 1
}

// Test 4: Assert negated correctness — result != spec for a valid address
// va = 0x0000_0080_0010_0000: L4=1, L1 index should be (0x100 & 0x1ff) = 0x100
// SHOULD FAIL
proof fn test_mutation_negated_correctness() {
    let va: usize = 0x0000_0080_0010_0000usize;
    let expected: usize = (va >> 12usize & 0x1ffusize) as usize;
    assert(spec_v2l1index(va) != expected) by (bit_vector); // contradicts definition
}

// Test 5: Assert max L1 index for a 2M-aligned address (lower 21 bits are 0, so L1 = 0)
// va = 0x0000_0080_0020_0000: 2M aligned, L4=1, L1 index = 0
// SHOULD FAIL
proof fn test_mutation_wrong_2m_l1_index() {
    let va: usize = 0x0000_0080_0020_0000usize;
    assert(spec_v2l1index(va) == 0x1ffusize) by (bit_vector); // wrong: actual L1 = 0 for 2M
}

// Test 6: Assert L1 index is negative equivalent (wraps around)
// va = 0x0000_0080_0000_1000: L1 = 1
// SHOULD FAIL
proof fn test_mutation_impossible_large_value() {
    let va: usize = 0x0000_0080_0000_1000usize;
    assert(spec_v2l1index(va) == 0x200usize) by (bit_vector); // 0x200 > 0x1ff, impossible
}

}
