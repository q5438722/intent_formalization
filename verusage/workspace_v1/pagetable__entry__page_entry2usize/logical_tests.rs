use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus! {

global size_of usize == 8;

pub type PAddr = usize;

#[derive(Clone, Debug)]
pub struct PageEntryPerm {
    pub present: bool,
    pub ps: bool,
    pub write: bool,
    pub execute_disable: bool,
    pub user: bool,
}

#[derive(Clone, Debug)]
pub struct PageEntry {
    pub addr: PAddr,
    pub perm: PageEntryPerm,
}

pub open spec fn usize2present(v: usize) -> bool {
    (v & PAGE_ENTRY_PRESENT_MASK as usize) != 0
}

pub open spec fn usize2ps(v: usize) -> bool {
    (v & PAGE_ENTRY_PS_MASK as usize) != 0
}

pub open spec fn usize2write(v: usize) -> bool {
    (v & PAGE_ENTRY_WRITE_MASK as usize) != 0
}

pub open spec fn usize2execute_disable(v: usize) -> bool {
    (v & PAGE_ENTRY_EXECUTE_MASK as usize) != 0
}

pub open spec fn usize2user(v: usize) -> bool {
    (v & PAGE_ENTRY_USER_MASK as usize) != 0
}

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
}

pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;
pub const PAGE_ENTRY_USER_SHIFT: u64 = 2;
pub const PAGE_ENTRY_PS_SHIFT: u64 = 7;
pub const PAGE_ENTRY_EXECUTE_SHIFT: u64 = 63;
pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;
pub const PAGE_ENTRY_USER_MASK: u64 = 0x1u64 << PAGE_ENTRY_USER_SHIFT;
pub const PAGE_ENTRY_PS_MASK: u64 = 0x1u64 << PAGE_ENTRY_PS_SHIFT;
pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x1u64 << PAGE_ENTRY_EXECUTE_SHIFT;

// ============================================================
// LOGICAL TESTS — properties NOT guaranteed by the spec
// The spec constrains bits 0,1,2,7,12-47,63 but leaves
// bits 3-6, 8-11, 48-62 unconstrained. These tests exploit
// that gap to assert properties the spec does NOT entail.
// ============================================================

// SHOULD FAIL — The spec does not guarantee all unconstrained bits are zero.
// Bits 3-6, 8-11, 48-62 are not covered by any postcondition,
// so a value satisfying all postconditions could still have those bits set.
proof fn test_logical_no_extra_bits(v: usize) {
    assume(usize2present(v) == false);
    assume(usize2ps(v) == false);
    assume(usize2write(v) == false);
    assume(usize2execute_disable(v) == false);
    assume(usize2user(v) == false);
    assume(spec_usize2pa(v) == 0usize);
    assert(v == 0usize);
}

// SHOULD FAIL — The spec does not guarantee determinism.
// Two different values could satisfy the same postconditions
// if they differ only in the unconstrained bits (3-6, 8-11, 48-62).
proof fn test_logical_determinism(v1: usize, v2: usize) {
    assume(usize2present(v1) == true);
    assume(usize2ps(v1) == false);
    assume(usize2write(v1) == true);
    assume(usize2execute_disable(v1) == false);
    assume(usize2user(v1) == false);
    assume(spec_usize2pa(v1) == 0x1000usize);

    assume(usize2present(v2) == true);
    assume(usize2ps(v2) == false);
    assume(usize2write(v2) == true);
    assume(usize2execute_disable(v2) == false);
    assume(usize2user(v2) == false);
    assume(spec_usize2pa(v2) == 0x1000usize);

    assert(v1 == v2);
}

// SHOULD FAIL — The spec does not bound the output to 48 bits.
// Bits 48-62 are unconstrained, so the output could exceed 2^48
// even when execute_disable (bit 63) is false.
proof fn test_logical_output_bounded(v: usize) {
    assume(usize2present(v) == true);
    assume(usize2ps(v) == true);
    assume(usize2write(v) == true);
    assume(usize2execute_disable(v) == false);
    assume(usize2user(v) == true);
    assume(spec_usize2pa(v) == 0x0000_ffff_ffff_f000usize);
    assert(v < 0x0001_0000_0000_0000usize);
}

}
