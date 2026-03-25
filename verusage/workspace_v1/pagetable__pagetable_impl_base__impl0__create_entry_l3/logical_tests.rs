use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus!{

pub type PAddr = usize;
pub type VAddr = usize;
pub type PageMapPtr = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

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

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_ptr_2m_valid(ptr: usize) -> bool {
    ((ptr % (0x200000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn page_ptr_1g_valid(ptr: usize) -> bool {
    ((ptr % (0x40000000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn spec_index2va(i: (L4Index, L3Index, L2Index, L1Index)) -> usize
    recommends
        i.0 <= 0x1ff,
        i.1 <= 0x1ff,
        i.2 <= 0x1ff,
        i.3 <= 0x1ff,
{
    (i.0 as usize) << 39 & (i.1 as usize) << 30 & (i.2 as usize) << 21 & (i.3 as usize) << 12
}


// ===================== LOGICAL TESTS =====================
// Each test asserts a property NOT explicitly guaranteed by the specification,
// testing whether the spec allows unintended reasoning.

// Test 1: page_ptr_valid does NOT imply page_ptr_2m_valid.
// A 4K-aligned pointer (e.g., 0x1000) is page_ptr_valid but NOT page_ptr_2m_valid
// since it's not 2MB-aligned. Asserting the stronger property should fail.
// SHOULD FAIL
proof fn test_logical_valid_not_2m_valid(ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(page_ptr_2m_valid(ptr));
}

// Test 2: MEM_valid does NOT imply page_ptr_valid.
// MEM_valid allows addresses up to ~2^48 while page_ptr_valid requires
// ptr / 0x1000 < NUM_PAGES (about 8GB). Large MEM_valid addresses are
// not page_ptr_valid.
// SHOULD FAIL
proof fn test_logical_mem_valid_not_page_valid(v: usize)
    requires
        MEM_valid(v),
{
    assert(page_ptr_valid(v));
}

// Test 3: page_ptr_valid does NOT imply ptr > 0.
// page_ptr_valid(0) is true: 0 % 0x1000 == 0 and 0 / 0x1000 == 0 < NUM_PAGES.
// So the spec allows ptr == 0 as a valid page pointer. Asserting ptr > 0 fails.
// SHOULD FAIL
proof fn test_logical_valid_ptr_nonzero(ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(ptr > 0usize);
}

// Test 4: spec_index2va is NOT injective due to using & (AND) instead of | (OR).
// Both (1,0,0,0) and (2,0,0,0) map to the same value because the AND
// of any shifted index with 0 yields 0. Asserting they differ should fail.
// SHOULD FAIL
proof fn test_logical_index2va_not_injective() {
    assert(
        spec_index2va((1usize, 0usize, 0usize, 0usize))
        != spec_index2va((2usize, 0usize, 0usize, 0usize))
    );
}

// Test 5: page_ptr_valid does NOT uniquely determine a single pointer.
// Many valid pointers exist (0x0, 0x1000, 0x2000, ...).
// Asserting ptr must be a specific value should fail.
// SHOULD FAIL
proof fn test_logical_valid_ptr_not_unique(ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(ptr == 0usize);
}

}
