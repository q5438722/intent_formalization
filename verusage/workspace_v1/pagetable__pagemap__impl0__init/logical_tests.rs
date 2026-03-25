use vstd::prelude::*;

fn main() {}

verus! {

pub type PAddr = usize;

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
}

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


// ============================================================
// LOGICAL TESTS: Properties NOT explicitly guaranteed by spec
// ============================================================

// Logical Test 1: Spec does NOT guarantee all MEM_valid addrs are >= page size
// (0 is MEM_valid but is not >= 0x1000)
// SHOULD FAIL
proof fn test_all_mem_valid_above_page_size() {
    assert(forall|v: PAddr| MEM_valid(v) ==> v >= 0x1000usize);
}

// Logical Test 2: Spec does NOT guarantee usize2pa is the identity function
// (masking removes bits outside MEM_MASK)
// SHOULD FAIL
proof fn test_usize2pa_is_identity() {
    assert(forall|v: usize| spec_usize2pa(v) == v);
}

// Logical Test 3: Spec does NOT guarantee only zero maps to empty
// (e.g., value 8 also maps to empty since bit 3 is not covered by any mask)
// SHOULD FAIL
proof fn test_empty_implies_zero_input() {
    assert(forall|v: usize| spec_usize2page_entry(v).is_empty() ==> v == 0usize);
}

// Logical Test 4: Spec does NOT guarantee address field equals the input value
// (address is masked through MEM_MASK)
// SHOULD FAIL
proof fn test_address_preserves_full_input() {
    assert(forall|v: usize| spec_usize2page_entry(v).addr == v);
}

// Logical Test 5: Spec does NOT guarantee that MEM_valid addresses are non-zero
// (MEM_valid(0) is true since 0 & anything == 0)
// SHOULD FAIL
proof fn test_mem_valid_implies_nonzero() {
    assert(forall|v: PAddr| MEM_valid(v) ==> v != 0usize);
}

}
