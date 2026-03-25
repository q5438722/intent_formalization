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
// 4k-valid does NOT imply 1g-valid; va = 0x0000_0080_4000_0000 is 4k-valid but not 1g-aligned
proof fn test_logical_4k_implies_1g() {
    let va: usize = 0x0000_0080_4000_0000usize;
    assume(spec_va_4k_valid(va));
    assert(spec_va_1g_valid(va));
}

// SHOULD FAIL
// L3 index is NOT always > 0; va = 0x0000_0080_0000_0000 has L3 index = 0
proof fn test_logical_always_nonzero() {
    let va: usize = 0x0000_0080_0000_0000usize;
    assume(spec_va_1g_valid(va));
    assert(spec_v2l3index(va) > 0usize);
}

// SHOULD FAIL
// The spec guarantees ret <= 0x1ff, NOT ret <= 0xff; va with L3 index = 0x1ff violates this
proof fn test_logical_stronger_bound() {
    let va: usize = 0x0000_00ff_c000_0000usize;
    assume(spec_va_4k_valid(va));
    assert(spec_v2l3index(va) <= 0xffusize);
}

// SHOULD FAIL
// spec_v2l3index is NOT injective: different VAs can map to the same L3 index
proof fn test_logical_injective() {
    let va1: usize = 0x0000_0080_4000_0000usize;
    let va2: usize = 0x0000_0080_4000_1000usize;
    assume(spec_va_4k_valid(va1));
    assume(spec_va_4k_valid(va2));
    assert(va1 == va2 || spec_v2l3index(va1) != spec_v2l3index(va2));
}

}
