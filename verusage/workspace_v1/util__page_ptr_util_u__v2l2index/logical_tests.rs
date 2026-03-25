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

// ========== LOGICAL TESTS ==========
// These tests assert properties NOT explicitly guaranteed by the specification.
// They should all FAIL verification.

// SHOULD FAIL: L2 index is not always 0 — many valid VAs have nonzero L2 indices
proof fn test_l2_index_always_zero() {
    assert(forall|va: usize| spec_va_4k_valid(va) ==> spec_v2l2index(va) == 0usize);
}

// SHOULD FAIL: v2l2index is not injective — different VAs can share the same L2 index
// VA1 and VA2 differ only in L4 index (bits 47:39) so they share the same L2 index
proof fn test_l2_index_injective() {
    let va1: usize = 0x0000_0080_0020_0000usize; // L4=1, L2=1
    let va2: usize = 0x0000_0100_0020_0000usize; // L4=2, L2=1
    assert(spec_v2l2index(va1) != spec_v2l2index(va2));
}

// SHOULD FAIL: The upper bound is 0x1ff (511), not 255. Stronger bound does not hold.
proof fn test_stronger_upper_bound() {
    assert(forall|va: usize| (spec_va_4k_valid(va) || spec_va_2m_valid(va))
        ==> spec_v2l2index(va) < 256usize);
}

// SHOULD FAIL: 4K validity does NOT imply 2M validity (4K-aligned but not 2M-aligned)
proof fn test_4k_valid_implies_2m_valid() {
    let va: usize = 0x0000_0080_0000_1000usize;
    assert(spec_va_4k_valid(va) ==> spec_va_2m_valid(va));
}

// SHOULD FAIL: L2 index is not always nonzero for valid VAs
// VA 0x0000_0080_0000_0000 is valid and has L2 index = 0
proof fn test_l2_index_always_nonzero() {
    assert(forall|va: usize| spec_va_4k_valid(va) ==> spec_v2l2index(va) > 0usize);
}

// SHOULD FAIL: 2M validity does NOT imply 1G validity
proof fn test_2m_valid_implies_1g_valid() {
    let va: usize = 0x0000_0080_0020_0000usize;
    assert(spec_va_2m_valid(va) ==> spec_va_1g_valid(va));
}

}
