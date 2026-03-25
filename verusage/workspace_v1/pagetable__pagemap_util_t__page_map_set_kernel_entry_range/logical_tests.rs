use vstd::prelude::*;

fn main() {}

verus! {

// === Type Definitions ===
pub type PAddr = usize;

// === Constants ===
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;
pub const PAGE_ENTRY_USER_SHIFT: u64 = 2;
pub const PAGE_ENTRY_USER_MASK: u64 = 0x1u64 << PAGE_ENTRY_USER_SHIFT;
pub const PAGE_ENTRY_PS_SHIFT: u64 = 7;
pub const PAGE_ENTRY_PS_MASK: u64 = 0x1u64 << PAGE_ENTRY_PS_SHIFT;
pub const PAGE_ENTRY_EXECUTE_SHIFT: u64 = 63;
pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x1u64 << PAGE_ENTRY_EXECUTE_SHIFT;

// === Structs ===
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

// === Spec Functions ===
pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
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

pub open spec fn spec_usize2page_entry_perm(v: usize) -> PageEntryPerm {
    PageEntryPerm {
        present: usize2present(v),
        ps: usize2ps(v),
        write: usize2write(v),
        execute_disable: usize2execute_disable(v),
        user: usize2user(v),
    }
}

pub open spec fn spec_usize2page_entry(v: usize) -> PageEntry {
    PageEntry { addr: spec_usize2pa(v), perm: spec_usize2page_entry_perm(v) }
}

// ============================================================
// LOGICAL TESTS — Properties NOT explicitly guaranteed
// ============================================================

// SHOULD FAIL
// Test: usize2pa is NOT identity — it masks off bits outside MEM_MASK
// 0xFFFF has low 16 bits set; only bits 12-15 survive the mask → result is 0xF000
proof fn test_usize2pa_is_identity() {
    assert(spec_usize2pa(0xFFFFusize) == 0xFFFFusize);
}

// SHOULD FAIL
// Test: MEM_valid does NOT imply the address is zero.
// 0x1000 is a valid address (bit 12 is in MEM_MASK range), but it's nonzero.
proof fn test_mem_valid_implies_zero() {
    assert(MEM_valid(0x1000usize) ==> (0x1000usize == 0usize));
}

// SHOULD FAIL
// Test: usize2pa is NOT injective on values differing in low bits.
// 0x1001 and 0x1002 both map to PA 0x1000 (low bits masked off).
proof fn test_usize2pa_injective_on_low_bits() {
    assert(spec_usize2pa(0x1001usize) != spec_usize2pa(0x1002usize));
}

// SHOULD FAIL
// Test: Two distinct addresses do NOT produce equal page entries.
// 0x1000 and 0x2000 have different PAs (0x1000 vs 0x2000).
proof fn test_different_entries_equal() {
    assert(spec_usize2page_entry(0x1000usize) =~= spec_usize2page_entry(0x2000usize));
}

}
