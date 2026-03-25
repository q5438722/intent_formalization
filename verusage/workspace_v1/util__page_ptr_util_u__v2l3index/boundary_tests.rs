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
// va = 0 has L4 index = 0 < KERNEL_MEM_END_L4INDEX, so no validity check passes
proof fn test_boundary_zero_satisfies_precond() {
    assert(spec_va_4k_valid(0usize) || spec_va_2m_valid(0usize) || spec_va_1g_valid(0usize));
}

// SHOULD FAIL
// va = 0x0000_0080_0000_0001 has bit 0 set, violating alignment for all page sizes
proof fn test_boundary_unaligned_address() {
    let va: usize = 0x0000_0080_0000_0001usize;
    assert(spec_va_4k_valid(va) || spec_va_2m_valid(va) || spec_va_1g_valid(va));
}

// SHOULD FAIL
// va = 0x1000 is 4k-aligned but L4 index = 0, which is below KERNEL_MEM_END_L4INDEX
proof fn test_boundary_low_l4_index() {
    let va: usize = 0x0000_0000_0000_1000usize;
    assert(spec_va_4k_valid(va) || spec_va_2m_valid(va) || spec_va_1g_valid(va));
}

// SHOULD FAIL
// va with upper 16 bits set fails the alignment mask check for all page sizes
proof fn test_boundary_upper_bits_set() {
    let va: usize = 0xffff_0080_0000_0000usize;
    assert(spec_va_4k_valid(va) || spec_va_2m_valid(va) || spec_va_1g_valid(va));
}

}
