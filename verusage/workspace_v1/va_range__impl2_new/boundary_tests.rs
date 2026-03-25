use vstd::prelude::*;

fn main() {}

pub type VAddr = usize;

verus! {

global size_of usize == 8;

// === Definitions from target ===

pub const KERNEL_MEM_END_L4INDEX: usize = 1;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn spec_va_add_range(va: usize, i: usize) -> usize {
    (va + (i * 4096)) as usize
}

pub open spec fn spec_va_4k_range_valid(va: usize, len: usize) -> bool {
    forall|i: usize|
        #![trigger spec_va_add_range(va, i)]
        0 <= i < len ==> spec_va_4k_valid(spec_va_add_range(va, i))
}

// === Boundary Tests ===

// Test 1: Non-4K-aligned address must not be valid
// SHOULD FAIL
proof fn test_boundary_non_aligned() {
    assert(spec_va_4k_valid(1usize));
}

// Test 2: User-space address (4K-aligned but bits 39-47 = 0) must not be valid
// SHOULD FAIL
proof fn test_boundary_user_space() {
    assert(spec_va_4k_valid(0x1000usize));
}

// Test 3: Address with upper bits set (bits 48-63 != 0) must not be valid
// SHOULD FAIL
proof fn test_boundary_upper_bits_set() {
    assert(spec_va_4k_valid(0xFFFF_0080_0000_0000usize));
}

// Test 4: Zero address must not be valid
// SHOULD FAIL
proof fn test_boundary_zero_address() {
    assert(spec_va_4k_valid(0usize));
}

}
