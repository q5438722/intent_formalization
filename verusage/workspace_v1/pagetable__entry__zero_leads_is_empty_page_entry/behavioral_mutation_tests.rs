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
// BEHAVIORAL MUTATION TESTS — Mutate expected outputs/relations
// Each test asserts wrong field values for valid inputs.
// =====================================================================

// Test 1: Claim present=false when bit 0 is set (v=1)
// SHOULD FAIL
pub proof fn test_present_extraction_mutated()
    ensures usize2present(1) == false,
{
    assert(1usize & 0x1u64 as usize != 0 == true) by (bit_vector);
}

// Test 2: Claim write=false when bit 1 is set (v=2)
// SHOULD FAIL
pub proof fn test_write_extraction_mutated()
    ensures usize2write(2) == false,
{
    assert(2usize & (0x1u64 << 0x1u64) as usize != 0 == true) by (bit_vector);
}

// Test 3: Claim user=false when bit 2 is set (v=4)
// SHOULD FAIL
pub proof fn test_user_extraction_mutated()
    ensures usize2user(4) == false,
{
    assert(4usize & (0x1u64 << 0x2u64) as usize != 0 == true) by (bit_vector);
}

// Test 4: Claim ps=false when bit 7 is set (v=0x80)
// SHOULD FAIL
pub proof fn test_ps_extraction_mutated()
    ensures usize2ps(0x80) == false,
{
    assert(0x80usize & (0x1u64 << 0x7u64) as usize != 0 == true) by (bit_vector);
}

// Test 5: Claim physical address of 0x2000 is zero (it should be 0x2000)
// SHOULD FAIL
pub proof fn test_pa_extraction_mutated()
    ensures spec_usize2pa(0x2000) == 0usize,
{
    assert(0x2000usize & 0x0000_ffff_ffff_f000u64 as usize == 0x2000usize) by (bit_vector);
}

// Test 6: Claim execute_disable=false when bit 63 is set
// SHOULD FAIL
pub proof fn test_execute_disable_extraction_mutated()
    ensures usize2execute_disable(0x8000_0000_0000_0000u64 as usize) == false,
{
    assert(0x8000_0000_0000_0000usize & (0x1u64 << 63u64) as usize != 0 == true) by (bit_vector);
}

}
