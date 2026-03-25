use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type PageMapPtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type ThreadPtr = usize;
pub type EndpointPtr = usize;
pub type Pcid = usize;

pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

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
// Each test starts from valid inputs, then asserts a WRONG output or
// relation (mutated from the correct one). All tests SHOULD FAIL.

// Test 1: create_entry ensures ret.0 <= 3.
// Mutate: claim ret.0 > 3.
// SHOULD FAIL
proof fn test_mutation_create_entry_uses_more_than_3_pages(ret0: usize)
    requires
        ret0 <= 3,
{
    assert(ret0 > 3);
}

// Test 2: create_entry ensures free_pages decreases by exactly ret.0.
// Mutate: claim free_pages decreases by ret.0 + 1 (off by one).
// SHOULD FAIL
proof fn test_mutation_free_pages_off_by_one(
    old_free: usize,
    new_free: usize,
    ret0: usize,
)
    requires
        new_free == old_free - ret0,
        ret0 <= 3,
        old_free >= 3,
{
    assert(new_free == old_free - ret0 - 1);
}

// Test 3: create_entry preserves proc_dom (unchanged).
// Mutate: claim proc_dom changes.
// SHOULD FAIL
proof fn test_mutation_proc_dom_changes(
    old_dom: Set<ProcPtr>,
    new_dom: Set<ProcPtr>,
)
    requires
        old_dom =~= new_dom,
{
    assert(old_dom !== new_dom);
}

// Test 4: create_entry preserves thread_dom (unchanged).
// Mutate: claim thread_dom changes.
// SHOULD FAIL
proof fn test_mutation_thread_dom_changes(
    old_dom: Set<ThreadPtr>,
    new_dom: Set<ThreadPtr>,
)
    requires
        old_dom =~= new_dom,
{
    assert(old_dom !== new_dom);
}

// Test 5: create_entry preserves container_dom (unchanged).
// Mutate: claim container_dom changes.
// SHOULD FAIL
proof fn test_mutation_container_dom_changes(
    old_dom: Set<ContainerPtr>,
    new_dom: Set<ContainerPtr>,
)
    requires
        old_dom =~= new_dom,
{
    assert(old_dom !== new_dom);
}

// Test 6: Quota::spec_subtract_mem_4k subtracts from mem_4k, leaves others unchanged.
// Mutate: claim mem_2m also decreases.
// SHOULD FAIL
proof fn test_mutation_quota_subtract_changes_mem_2m() {
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 97, mem_2m: 49, mem_1g: 10, pcid: 5, ioid: 3 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 3));
}

// Test 7: create_entry preserves address space for other processes.
// Mutate: claim other process's address space differs.
// SHOULD FAIL
proof fn test_mutation_other_proc_addr_space_changes(
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

// Test 8: create_entry preserves page_mapping.
// Mutate: claim page_mapping changes.
// SHOULD FAIL
proof fn test_mutation_page_mapping_changes(
    old_mapping: Map<PagePtr, Set<(ProcPtr, VAddr)>>,
    new_mapping: Map<PagePtr, Set<(ProcPtr, VAddr)>>,
)
    requires
        old_mapping =~= new_mapping,
{
    assert(old_mapping !== new_mapping);
}

// Test 9: usize2page_entry(0) should produce an empty entry (present == false).
// Mutate: claim present == true for zero entry.
// SHOULD FAIL
proof fn test_mutation_zero_entry_present() {
    let entry = spec_usize2page_entry(0usize);
    assert(entry.perm.present == true);
}

// Test 10: page_ptr2page_index and page_index2page_ptr are inverses
// for valid indices. Mutate: claim they don't compose to identity.
// SHOULD FAIL
proof fn test_mutation_ptr_index_roundtrip_wrong() {
    let i: usize = 100;
    assert(spec_page_ptr2page_index(spec_page_index2page_ptr(i)) != i);
}

// Test 11: create_entry preserves endpoint_dom.
// Mutate: claim endpoint_dom changes.
// SHOULD FAIL
proof fn test_mutation_endpoint_dom_changes(
    old_dom: Set<EndpointPtr>,
    new_dom: Set<EndpointPtr>,
)
    requires
        old_dom =~= new_dom,
{
    assert(old_dom !== new_dom);
}

// Test 12: create_entry preserves physical page reference counters
// for mapped pages. Mutate: claim a mapped page's counter changes.
// SHOULD FAIL
proof fn test_mutation_ref_counter_changes(
    old_counter: nat,
    new_counter: nat,
)
    requires
        old_counter == new_counter,
{
    assert(old_counter != new_counter);
}

}
