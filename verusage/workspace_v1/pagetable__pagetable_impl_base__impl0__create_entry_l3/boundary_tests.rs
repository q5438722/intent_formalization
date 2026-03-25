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
pub const MEM_2m_MASK: u64 = 0x0000_ffff_ffe0_0000;
pub const MEM_1g_MASK: u64 = 0x0000_fffc_0000_0000;
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


// ===================== BOUNDARY TESTS =====================
// Each test encodes a property that is likely NOT entailed by the specification.
// These tests are intended to FAIL verification if the specification is correct.

// Test 1: page_ptr_valid should reject unaligned pointers.
// ptr=1 fails the alignment requirement (1 % 0x1000 != 0).
// SHOULD FAIL
proof fn test_boundary_unaligned_page_ptr() {
    assert(page_ptr_valid(1usize));
}

// Test 2: page_ptr_valid should reject pointer at the upper boundary.
// When ptr / 0x1000 == NUM_PAGES, the strict < check fails.
// SHOULD FAIL
proof fn test_boundary_page_ptr_upper_limit(ptr: usize)
    requires
        ptr % 0x1000 == 0,
        ptr / 0x1000 == NUM_PAGES,
{
    assert(page_ptr_valid(ptr));
}

// Test 3: PageEntry with present=true should NOT satisfy is_empty().
// is_empty() requires perm.present == false.
// SHOULD FAIL
proof fn test_boundary_present_entry_not_empty() {
    let pe = PageEntry {
        addr: 0usize,
        perm: PageEntryPerm {
            present: true,
            ps: false,
            write: false,
            execute_disable: false,
            user: false,
        },
    };
    assert(pe.is_empty());
}

// Test 4: page_ptr_1g_valid should reject 2M-aligned but not 1G-aligned pointers.
// 0x200000 (2MB) is not a multiple of 0x40000000 (1GB).
// SHOULD FAIL
proof fn test_boundary_2m_not_1g_valid() {
    assert(page_ptr_1g_valid(0x200000usize));
}

// Test 5: MEM_valid should reject addresses with low bits set.
// v=1 has bit 0 set, which falls outside MEM_MASK, so 1 & (!MEM_MASK) != 0.
// SHOULD FAIL
proof fn test_boundary_mem_valid_low_bits() {
    assert(MEM_valid(1usize));
}

}
