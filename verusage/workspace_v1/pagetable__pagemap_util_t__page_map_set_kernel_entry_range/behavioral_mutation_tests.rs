use vstd::prelude::*;

fn main() {}

verus! {

// === Type Definitions ===
pub type PAddr = usize;

// === Constants ===
pub const KERNEL_MEM_END_L4INDEX: usize = 1;
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
// BEHAVIORAL MUTATION TESTS — Mutate expected outputs/relations
// ============================================================

// SHOULD FAIL
// Test: Value 1 has bit 0 set, so present should be TRUE; asserting false is wrong
proof fn test_present_bit_negated() {
    let entry = spec_usize2page_entry(1usize);
    assert(!entry.perm.present);
}

// SHOULD FAIL
// Test: Value 2 has bit 1 set, so write should be TRUE; asserting false is wrong
proof fn test_write_bit_negated() {
    let entry = spec_usize2page_entry(2usize);
    assert(!entry.perm.write);
}

// SHOULD FAIL
// Test: Value 4 has bit 2 set, so user should be TRUE; asserting false is wrong
proof fn test_user_bit_negated() {
    let entry = spec_usize2page_entry(4usize);
    assert(!entry.perm.user);
}

// SHOULD FAIL
// Test: Address 0x1000 has bit 12 set (in MEM_MASK), so PA should be 0x1000, not 0
proof fn test_address_mutated_to_zero() {
    let entry = spec_usize2page_entry(0x1000usize);
    assert(entry.addr == 0usize);
}

// SHOULD FAIL
// Test: After page_map_set_kernel_entry_range, entry 0 is SET to new value.
//       Asserting it equals the old value is wrong.
proof fn test_kernel_entry0_wrongly_preserved(
    old_entries: Seq<PageEntry>,
    new_entries: Seq<PageEntry>,
    kernel_val: usize,
)
    requires
        old_entries.len() == 512,
        new_entries.len() == 512,
        // Postcondition: entry 0 is set to usize2page_entry(kernel_entries[0])
        new_entries[0] =~= spec_usize2page_entry(kernel_val),
        // Postcondition: entries 1-511 preserved
        forall|i: int| #![trigger new_entries[i]] 1 <= i < 512 ==> new_entries[i] =~= old_entries[i],
        // The old entry was different from the new value
        !(old_entries[0] =~= spec_usize2page_entry(kernel_val)),
{
    // Incorrectly assert entry 0 is unchanged
    assert(new_entries[0] =~= old_entries[0]);
}

// SHOULD FAIL
// Test: After page_map_set_kernel_entry_range, entry 1 is PRESERVED.
//       Asserting it was changed is wrong.
proof fn test_kernel_entry1_wrongly_changed(
    old_entries: Seq<PageEntry>,
    new_entries: Seq<PageEntry>,
    kernel_val: usize,
)
    requires
        old_entries.len() == 512,
        new_entries.len() == 512,
        // Postcondition: entry 0 is set
        new_entries[0] =~= spec_usize2page_entry(kernel_val),
        // Postcondition: entries 1-511 preserved
        forall|i: int| #![trigger new_entries[i]] 1 <= i < 512 ==> new_entries[i] =~= old_entries[i],
{
    // Incorrectly assert entry 1 was changed
    assert(!(new_entries[1] =~= old_entries[1]));
}

}
