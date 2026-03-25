use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;

pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;
pub const PAGE_ENTRY_USER_SHIFT: u64 = 2;
pub const PAGE_ENTRY_PS_SHIFT: u64 = 7;
pub const PAGE_ENTRY_EXECUTE_SHIFT: u64 = 63;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;
pub const PAGE_ENTRY_USER_MASK: u64 = 0x1u64 << PAGE_ENTRY_USER_SHIFT;
pub const PAGE_ENTRY_PS_MASK: u64 = 0x1u64 << PAGE_ENTRY_PS_SHIFT;
pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x1u64 << PAGE_ENTRY_EXECUTE_SHIFT;

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

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize {
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize {
    (i * 4096) as usize
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

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Quota {
    pub mem_4k: usize,
    pub mem_2m: usize,
    pub mem_1g: usize,
    pub pcid: usize,
    pub ioid: usize,
}

impl Quota {
    pub open spec fn spec_subtract_mem_4k(&self, new: Self, k: usize) -> bool {
        &&& self.mem_4k - k == new.mem_4k
        &&& self.mem_2m == new.mem_2m
        &&& self.mem_1g == new.mem_1g
        &&& self.pcid == new.pcid
        &&& self.ioid == new.ioid
    }
}

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs, then mutates expected outputs
// or relations. All tests SHOULD FAIL verification.

// Test 1: usize2page_entry_perm(0) should have all fields false.
// Mutate: assert present == true for input 0.
// SHOULD FAIL
proof fn test_mutation_zero_entry_perm_present_true() {
    let perm = spec_usize2page_entry_perm(0usize);
    assert(perm.present == true);
}

// Test 2: usize2page_entry(0) should have addr == 0.
// Mutate: assert addr != 0.
// SHOULD FAIL
proof fn test_mutation_zero_entry_addr_nonzero() {
    let entry = spec_usize2page_entry(0usize);
    assert(entry.addr != 0);
}

// Test 3: usize2pa should give MEM_valid result.
// Mutate: assert usize2pa(0x1000) == 0 (wrong value; 0x1000 & MEM_MASK = 0x1000).
// SHOULD FAIL
proof fn test_mutation_usize2pa_wrong_value() {
    assert(spec_usize2pa(0x1000usize) == 0usize);
}

// Test 4: page_ptr2page_index(0x2000) should be 2.
// Mutate: assert it equals 3.
// SHOULD FAIL
proof fn test_mutation_page_ptr2index_wrong() {
    assert(spec_page_ptr2page_index(0x2000usize) == 3usize);
}

// Test 5: page_index2page_ptr(2) should be 0x2000.
// Mutate: assert it equals 0x1000.
// SHOULD FAIL
proof fn test_mutation_page_index2ptr_wrong() {
    assert(spec_page_index2page_ptr(2usize) == 0x1000usize);
}

// Test 6: usize2page_entry_perm with bit 1 set should have write == true.
// Mutate: assert write == false.
// SHOULD FAIL
proof fn test_mutation_write_bit_false() {
    let v: usize = 0x2; // bit 1 set = write
    let perm = spec_usize2page_entry_perm(v);
    assert(perm.write == false);
}

// Test 7: usize2page_entry_perm with bit 63 set should have execute_disable == true.
// Mutate: assert execute_disable == false.
// SHOULD FAIL
proof fn test_mutation_execute_disable_false() {
    let v: usize = (1usize << 63);
    let perm = spec_usize2page_entry_perm(v);
    assert(perm.execute_disable == false);
}

// Test 8: create_entry_and_alloc_and_map_io ensures ret.0 <= 4.
// Mutate: assert ret.0 == 5 (impossible output).
// This models the postcondition directly; asserting 5 is allowed should fail.
// SHOULD FAIL
proof fn test_mutation_create_entry_returns_more_than_4() {
    let ret_pages: usize = 5;
    assert(ret_pages <= 4);
}

// Test 9: spec_subtract_mem_4k should preserve mem_2m.
// Mutate: assert mem_2m changes.
// SHOULD FAIL
proof fn test_mutation_quota_subtract_changes_mem_2m() {
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 96, mem_2m: 49, mem_1g: 10, pcid: 5, ioid: 3 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 4));
}

// Test 10: create_entry_and_alloc_and_map_io ensures io_space domain grows
// by exactly target_va. Mutate: assert domain grows by a different VA.
// SHOULD FAIL
proof fn test_mutation_io_space_grows_wrong_va(
    io_old: Set<VAddr>,
    target_va: VAddr,
    other_va: VAddr,
)
    requires
        target_va != other_va,
        !io_old.contains(target_va),
        !io_old.contains(other_va),
{
    let io_new = io_old.insert(target_va);
    // Mutated: assert it also contains other_va
    assert(io_new.contains(other_va));
}

// Test 11: usize2page_entry_perm with present bit set (v=1) should have present == true.
// Mutate: assert ps is also true (it shouldn't be; ps bit is bit 7).
// SHOULD FAIL
proof fn test_mutation_present_implies_ps() {
    let perm = spec_usize2page_entry_perm(1usize);
    assert(perm.ps == true);
}

// Test 12: page_ptr2page_index and page_index2page_ptr should round-trip.
// Mutate: assert round-trip gives wrong result for aligned ptr.
// SHOULD FAIL
proof fn test_mutation_roundtrip_off_by_one() {
    let ptr: usize = 0x3000;
    assert(spec_page_index2page_ptr(spec_page_ptr2page_index(ptr)) == ptr + 0x1000);
}

}
