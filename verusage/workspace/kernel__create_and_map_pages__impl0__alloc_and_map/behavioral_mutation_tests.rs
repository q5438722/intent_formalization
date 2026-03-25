use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;

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
// Each test starts from valid inputs, then asserts a WRONG output
// or relation (mutated from the correct postcondition).
// All tests SHOULD FAIL verification.

// Test 1: alloc_and_map ensures get_num_of_free_pages decreases by 1.
// Mutate: claim free pages stays the same.
// SHOULD FAIL
proof fn test_mutation_free_pages_unchanged(old_free: usize, new_free: usize)
    requires
        new_free == old_free - 1,
        old_free >= 1,
{
    assert(new_free == old_free);
}

// Test 2: alloc_and_map ensures the new VA is inserted into the
// target proc's address space. Mutate: claim VA is NOT in space.
// SHOULD FAIL
proof fn test_mutation_va_not_in_new_address_space(
    old_space: Map<VAddr, MapEntry>,
    new_space: Map<VAddr, MapEntry>,
    target_va: VAddr,
    ret: MapEntry,
)
    requires
        new_space =~= old_space.insert(target_va, ret),
{
    assert(!new_space.dom().contains(target_va));
}

// Test 3: alloc_and_map ensures quota is subtracted by 1.
// Mutate: claim quota.mem_4k is unchanged.
// SHOULD FAIL
proof fn test_mutation_quota_unchanged() {
    let old_quota = Quota { mem_4k: 10, mem_2m: 5, mem_1g: 2, pcid: 3, ioid: 1 };
    let new_quota = Quota { mem_4k: 10, mem_2m: 5, mem_1g: 2, pcid: 3, ioid: 1 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 1));
}

// Test 4: alloc_and_map ensures non-target proc address spaces
// are preserved. Mutate: claim they can differ.
// SHOULD FAIL
proof fn test_mutation_other_proc_space_changes(
    old_space: Map<VAddr, MapEntry>,
    new_space: Map<VAddr, MapEntry>,
    va: VAddr,
    entry: MapEntry,
)
    requires
        old_space =~= new_space,
        old_space.dom().contains(va),
{
    assert(old_space[va].addr != new_space[va].addr);
}

// Test 5: alloc_and_map_4k ensures free_pages_2m is preserved.
// Mutate: claim free_pages_2m changes.
// SHOULD FAIL
proof fn test_mutation_free_2m_changes(
    old_free_2m: Set<PagePtr>,
    new_free_2m: Set<PagePtr>,
)
    requires
        new_free_2m =~= old_free_2m,
{
    assert(new_free_2m !== old_free_2m);
}

// Test 6: alloc_and_map_4k ensures mapped_pages_4k inserts ret.
// Mutate: claim mapped_pages_4k stays the same.
// SHOULD FAIL
proof fn test_mutation_mapped_pages_unchanged(
    old_mapped: Set<PagePtr>,
    new_mapped: Set<PagePtr>,
    ret: PagePtr,
)
    requires
        new_mapped =~= old_mapped.insert(ret),
        !old_mapped.contains(ret),
{
    assert(new_mapped =~= old_mapped);
}

// Test 7: alloc_and_map_4k ensures page_mappings(ret) contains (pcid, va).
// Mutate: claim page_mappings(ret) is empty.
// SHOULD FAIL
proof fn test_mutation_page_mappings_empty(
    mappings: Set<(Pcid, VAddr)>,
    pcid: Pcid,
    va: VAddr,
)
    requires
        mappings =~= Set::<(Pcid, VAddr)>::empty().insert((pcid, va)),
{
    assert(mappings =~= Set::<(Pcid, VAddr)>::empty());
}

// Test 8: usize2page_entry(0) should produce an empty entry.
// Mutate: claim present == true when it should be false.
// SHOULD FAIL
proof fn test_mutation_zero_entry_present() {
    let entry = spec_usize2page_entry(0usize);
    assert(entry.perm.present == true);
}

// Test 9: spec_usize2pa(0) should return 0.
// Mutate: claim result != 0.
// SHOULD FAIL
proof fn test_mutation_usize2pa_zero_nonzero() {
    let result = spec_usize2pa(0usize);
    assert(result != 0);
}

// Test 10: spec_subtract_mem_4k should preserve mem_2m.
// Mutate: claim mem_2m also changes.
// SHOULD FAIL
proof fn test_mutation_quota_subtract_changes_mem_2m() {
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 99, mem_2m: 49, mem_1g: 10, pcid: 5, ioid: 3 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 1));
}

// Test 11: alloc_and_map_4k ensures ret was NOT previously mapped.
// Mutate: claim it was already mapped.
// SHOULD FAIL
proof fn test_mutation_ret_already_mapped(was_mapped: bool)
    requires
        was_mapped == false,
{
    assert(was_mapped == true);
}

// Test 12: alloc_and_map ensures page_mapping domain gains ret.addr.
// Mutate: claim domain is unchanged.
// SHOULD FAIL
proof fn test_mutation_page_mapping_domain_unchanged(
    old_dom: Set<PagePtr>,
    new_dom: Set<PagePtr>,
    ret_addr: PagePtr,
)
    requires
        new_dom == old_dom.insert(ret_addr),
        !old_dom.contains(ret_addr),
{
    assert(new_dom =~= old_dom);
}

}
