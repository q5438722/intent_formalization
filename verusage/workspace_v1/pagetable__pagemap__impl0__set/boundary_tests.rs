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
// BOUNDARY TESTS: Violate preconditions and use edge cases
// ============================================================

// Boundary Test 1: Address with ALL bits set violates MEM_valid
// (bits above MEM_MASK range and below bit 12 are set)
// SHOULD FAIL
proof fn test_all_bits_set_mem_valid() {
    assert(MEM_valid(0xFFFF_FFFF_FFFF_FFFFusize));
}

// Boundary Test 2: Address with only low 12 bits set violates MEM_valid
// (MEM_MASK starts at bit 12, so low bits are outside the valid mask)
// SHOULD FAIL
proof fn test_low_12_bits_mem_valid() {
    assert(MEM_valid(0xFFFusize));
}

// Boundary Test 3: Entry with write=true but present=false is NOT empty
// (violates the precondition: present==false ==> is_empty())
// SHOULD FAIL
proof fn test_entry_write_true_is_empty() {
    let entry = PageEntry {
        addr: 0,
        perm: PageEntryPerm {
            present: false,
            ps: false,
            write: true,
            execute_disable: false,
            user: false,
        },
    };
    assert(entry.is_empty());
}

// Boundary Test 4: Entry with present=true is NOT empty
// (the present field directly contradicts is_empty)
// SHOULD FAIL
proof fn test_entry_present_true_is_empty() {
    let entry = PageEntry {
        addr: 0,
        perm: PageEntryPerm {
            present: true,
            ps: false,
            write: false,
            execute_disable: false,
            user: false,
        },
    };
    assert(entry.is_empty());
}

// Boundary Test 5: Address at bit 48 (just above MEM_MASK range 12-47)
// should NOT be MEM_valid
// SHOULD FAIL
proof fn test_bit_48_address_mem_valid() {
    assert(MEM_valid(0x0001_0000_0000_0000usize));
}

}
