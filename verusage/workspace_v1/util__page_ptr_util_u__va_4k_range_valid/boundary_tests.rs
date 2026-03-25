use vstd::prelude::*;

fn main() {}

verus! {

global size_of usize == 8;

// ---- Copied spec definitions from source ----

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

// ---- Boundary Tests ----
// These tests attempt to validate INVALID inputs.
// If the spec is correct, all should FAIL verification.

// SHOULD FAIL: Unaligned address (bit 0 set) cannot be 4K valid
proof fn test_boundary_unaligned_va()
{
    assert(spec_va_4k_valid(1usize));
}

// SHOULD FAIL: Zero address has L4 index = 0 < KERNEL_MEM_END_L4INDEX = 1
proof fn test_boundary_zero_va()
{
    assert(spec_va_4k_valid(0usize));
}

// SHOULD FAIL: Address with bit 48 set violates upper-bits-zero constraint
proof fn test_boundary_upper_bits_set()
{
    assert(spec_va_4k_valid(0x0001_0000_0000_0000u64 as usize));
}

// SHOULD FAIL: Range starting at invalid base (va=0) cannot be valid for len >= 1
proof fn test_boundary_invalid_base_range()
{
    assert(spec_va_4k_range_valid(0usize, 1usize));
}

// SHOULD FAIL: Address with only lower 12 bits set (0xFFF) is not page-aligned
proof fn test_boundary_low_bits_all_set()
{
    assert(spec_va_4k_valid(0xFFFusize));
}

}
