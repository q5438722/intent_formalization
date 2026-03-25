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
// BOUNDARY TESTS — Violate preconditions / use edge case inputs
// Each test asserts a property that SHOULD NOT hold.
// =====================================================================

// Test 1: Present bit set (bit 0) — entry should NOT be empty
// SHOULD FAIL
pub proof fn test_present_bit_set_is_empty()
    ensures spec_usize2page_entry(1).is_empty(),
{
    assert(1usize & 0x0000_ffff_ffff_f000u64 as usize == 0) by (bit_vector);
    assert(1usize & 0x1 as usize != 0 == true) by (bit_vector);
}

// Test 2: Address bits set (0x1000 = bit 12) — addr != 0, entry not empty
// SHOULD FAIL
pub proof fn test_addr_bit_set_is_empty()
    ensures spec_usize2page_entry(0x1000).is_empty(),
{
    assert(0x1000usize & 0x0000_ffff_ffff_f000u64 as usize == 0x1000usize) by (bit_vector);
}

// Test 3: Maximum usize — all bits set, entry definitely not empty
// SHOULD FAIL
pub proof fn test_max_usize_is_empty()
    ensures spec_usize2page_entry(0xFFFF_FFFF_FFFF_FFFFu64 as usize).is_empty(),
{
}

// Test 4: Execute-disable bit set (bit 63) — entry not empty
// SHOULD FAIL
pub proof fn test_execute_disable_bit_is_empty()
    ensures spec_usize2page_entry(0x8000_0000_0000_0000u64 as usize).is_empty(),
{
}

// Test 5: PS bit set (bit 7 = 0x80) — entry not empty
// SHOULD FAIL
pub proof fn test_ps_bit_set_is_empty()
    ensures spec_usize2page_entry(0x80).is_empty(),
{
}

}
