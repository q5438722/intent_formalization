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
// BEHAVIORAL MUTATION TESTS: Mutate expected outputs/relations
// ============================================================

// Mutation Test 1: Mutate is_empty result — zero entry IS empty, assert it's not
// SHOULD FAIL
proof fn test_zero_entry_not_empty() {
    assert(!spec_usize2page_entry(0usize).is_empty());
}

// Mutation Test 2: Seq::update at index 0 should NOT change element at index 1
// (tests the frame property of the set postcondition)
// SHOULD FAIL
proof fn test_seq_update_changes_other_index() {
    let val = PageEntry {
        addr: 0x1000,
        perm: PageEntryPerm {
            present: true,
            ps: false,
            write: true,
            execute_disable: false,
            user: false,
        },
    };
    let s: Seq<PageEntry> = Seq::new(512, |i: int| spec_usize2page_entry(0));
    let s2 = s.update(0int, val);
    assert(s2[1] !== s[1]);
}

// Mutation Test 3: Mutate present detection — value 1 HAS present bit, assert it doesn't
// SHOULD FAIL
proof fn test_present_bit_not_detected_for_one() {
    assert(!usize2present(1usize));
}

// Mutation Test 4: Mutate addr result — spec_usize2pa(0) IS 0, assert it's nonzero
// SHOULD FAIL
proof fn test_usize2pa_zero_is_nonzero() {
    assert(spec_usize2pa(0usize) != 0usize);
}

// Mutation Test 5: Seq::update at index 0 sets value to val, assert it's different
// (directly tests the update postcondition of set)
// SHOULD FAIL
proof fn test_seq_update_wrong_value_at_index() {
    let val = PageEntry {
        addr: 0x2000,
        perm: PageEntryPerm {
            present: true,
            ps: false,
            write: false,
            execute_disable: false,
            user: false,
        },
    };
    let s: Seq<PageEntry> = Seq::new(512, |i: int| spec_usize2page_entry(0));
    let s2 = s.update(0int, val);
    assert(s2[0] !== val);
}

}
