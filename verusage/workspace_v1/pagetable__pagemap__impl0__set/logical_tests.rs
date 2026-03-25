use vstd::prelude::*;

fn main() {}

verus! {

global size_of usize == 8;

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

// Logical Test 1: MEM_valid does NOT imply the address is nonzero
// (MEM_valid(0) is true since 0 & anything == 0)
// SHOULD FAIL
proof fn test_mem_valid_implies_nonzero() {
    assert(forall|v: PAddr| MEM_valid(v) ==> v != 0usize);
}

// Logical Test 2: An empty page entry does NOT imply the usize representation was 0
// (bits not covered by any mask, e.g. bit 3, can be set without affecting the PageEntry)
// SHOULD FAIL
proof fn test_empty_implies_zero_usize() {
    assert(forall|v: usize| spec_usize2page_entry(v).is_empty() ==> v == 0usize);
}

// Logical Test 3: usize2page_entry is NOT injective
// (different usize values can produce the same PageEntry due to bit masking)
// Concrete: 0 and 8 both map to the same empty PageEntry
// SHOULD FAIL
proof fn test_usize2page_entry_injective() {
    assert(forall|a: usize, b: usize|
        spec_usize2page_entry(a) =~= spec_usize2page_entry(b) ==> a == b
    );
}

// Logical Test 4: NOT all addresses are MEM_valid
// (addresses with bits above MEM_MASK range or in low 12 bits are invalid)
// SHOULD FAIL
proof fn test_all_addresses_mem_valid() {
    assert(forall|v: PAddr| MEM_valid(v));
}

// Logical Test 5: A present PageEntry is NOT required to have a nonzero address
// (the spec allows present entries with addr=0 as long as MEM_valid(addr) holds,
//  and MEM_valid(0) is true — this is a spec weakness for page table semantics)
// SHOULD FAIL
proof fn test_present_entry_must_have_nonzero_addr() {
    assert(forall|pe: PageEntry| pe.perm.present ==> pe.addr != 0usize);
}

}
