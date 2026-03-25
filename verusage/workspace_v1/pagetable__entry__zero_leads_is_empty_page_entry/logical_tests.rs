use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus!{

global size_of usize==8;

pub type PAddr = usize;

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
}

#[derive(Clone,Debug)]
pub struct PageEntryPerm {
    pub present: bool,
    pub ps: bool,
    pub write: bool,
    pub execute_disable: bool,
    pub user: bool,
}

#[derive(Clone,Debug)]
pub struct PageEntry {
    pub addr: PAddr,
    pub perm: PageEntryPerm,
}

impl PageEntry {
    pub open spec fn is_empty(&self) -> bool {
        &&& self.addr == 0
        &&& self.perm.present == false
        &&& self.perm.ps == false
        &&& self.perm.write == false
        &&& self.perm.execute_disable == false
        &&& self.perm.user == false
    }
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

pub open spec fn spec_usize2page_entry_perm(v: usize) -> PageEntryPerm {
    PageEntryPerm {
        present: usize2present(v),
        ps: usize2ps(v),
        write: usize2write(v),
        execute_disable: usize2execute_disable(v),
        user: usize2user(v),
    }
}

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

pub open spec fn spec_usize2page_entry(v: usize) -> PageEntry {
    PageEntry { addr: spec_usize2pa(v), perm: spec_usize2page_entry_perm(v) }
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

// =====================================================================
// LOGICAL TESTS — Properties NOT explicitly guaranteed by the spec
// Tests unintended inferences, stronger claims, structural assumptions
// =====================================================================

// Test 1: MEM_valid does NOT hold for arbitrary values (e.g., 0x1 has low bits set)
// This tests whether the spec allows claiming validity for invalid addresses
// SHOULD FAIL
pub proof fn test_mem_valid_for_arbitrary_value()
    ensures MEM_valid(0x1usize),
{
    assert(0x1usize & (!0x0000_ffff_ffff_f000u64) as usize == 0x1usize) by (bit_vector);
}

// Test 2: Injectivity — different inputs can map to the same physical address
// 0x1001 and 0x1002 both mask to 0x1000; asserting they differ SHOULD FAIL
// SHOULD FAIL
pub proof fn test_pa_injectivity_false()
    ensures spec_usize2pa(0x1001) != spec_usize2pa(0x1002),
{
    assert(0x1001usize & 0x0000_ffff_ffff_f000u64 as usize == 0x1000usize) by (bit_vector);
    assert(0x1002usize & 0x0000_ffff_ffff_f000u64 as usize == 0x1000usize) by (bit_vector);
}

// Test 3: Claim addr equals raw input (identity) — masking strips low bits
// spec_usize2pa(0x1FFF) == 0x1FFF is false; actual result is 0x1000
// SHOULD FAIL
pub proof fn test_pa_is_identity()
    ensures spec_usize2pa(0x1FFF) == 0x1FFFusize,
{
    assert(0x1FFFusize & 0x0000_ffff_ffff_f000u64 as usize == 0x1000usize) by (bit_vector);
}

// Test 4: Claim that is_empty implies the raw value was 0 (converse of zero_leads_is_empty)
// Value 0x800 has addr=0 and all perm bits false (bit 11 not in any mask),
// so is_empty holds but input != 0. This tests whether the spec wrongly assumes bijectivity.
// SHOULD FAIL
pub proof fn test_is_empty_implies_zero_input()
    ensures
        spec_usize2page_entry(0x800).is_empty() ==> 0x800usize == 0usize,
{
    // 0x800 & MEM_MASK = 0 (bit 11 is below the MEM_MASK range? Let's check: MEM_MASK starts at bit 12)
    // Actually 0x800 = bit 11, which is NOT in MEM_MASK (bits 12-51), so addr=0
    // And 0x800 has no perm bits set (present=bit0, write=bit1, user=bit2, ps=bit7, exec=bit63)
    // So is_empty() is true for 0x800, but 0x800 != 0
    assert(0x800usize & 0x0000_ffff_ffff_f000u64 as usize == 0usize) by (bit_vector);
    assert(0x800usize & 0x1u64 as usize != 0 == false) by (bit_vector);
    assert(0x800usize & (0x1u64 << 0x7u64) as usize != 0 == false) by (bit_vector);
    assert(0x800usize & (0x1u64 << 0x1u64) as usize != 0 == false) by (bit_vector);
    assert(0x800usize & (0x1u64 << 63u64) as usize != 0 == false) by (bit_vector);
    assert(0x800usize & (0x1u64 << 0x2u64) as usize != 0 == false) by (bit_vector);
}

// Test 5: Claim a stronger inequality — addr is always page-aligned (4K = 0x1000 multiple)
// This IS actually true due to MEM_MASK zeroing bits 0-11, but the spec never states it.
// We test the NEGATION: claim addr can be non-page-aligned. This SHOULD FAIL because
// the masking guarantees alignment (the spec actually entails this property).
// If this test fails, the spec correctly rejects the false claim. If it passes, spec is too weak.
// SHOULD FAIL
pub proof fn test_addr_not_page_aligned()
    ensures spec_usize2pa(0x12345678) % 0x1000usize != 0usize,
{
}

}
