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
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
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
// Each test starts from valid inputs/postconditions, then asserts
// a WRONG output or relation (mutated from the correct postcondition).
// All tests SHOULD FAIL verification.

// Test 1: create_entry_and_share ensures free_pages decreases by ret.
// Mutate: claim free pages stays the same.
// SHOULD FAIL
proof fn test_mutation_free_pages_unchanged(old_free: usize, new_free: usize, ret: usize)
    requires
        new_free == old_free - ret,
        old_free >= 3,
        ret <= 3,
        ret >= 1,
{
    assert(new_free == old_free);
}

// Test 2: create_entry_and_share ensures target_va is inserted into
// target's address space. Mutate: claim target_va is NOT in space.
// SHOULD FAIL
proof fn test_mutation_target_va_not_in_new_space(
    old_space: Map<VAddr, MapEntry>,
    new_space: Map<VAddr, MapEntry>,
    target_va: VAddr,
    src_entry: MapEntry,
)
    requires
        new_space =~= old_space.insert(target_va, src_entry),
{
    assert(!new_space.dom().contains(target_va));
}

// Test 3: create_entry_and_share ensures the entry at target_va equals
// the source entry. Mutate: claim the addr differs.
// SHOULD FAIL
proof fn test_mutation_shared_entry_addr_differs(
    new_space: Map<VAddr, MapEntry>,
    target_va: VAddr,
    src_entry: MapEntry,
)
    requires
        new_space.dom().contains(target_va),
        new_space[target_va].addr == src_entry.addr,
        src_entry.addr != 0,
{
    assert(new_space[target_va].addr != src_entry.addr);
}

// Test 4: create_entry_and_share ensures quota is subtracted by ret.
// Mutate: claim quota.mem_4k is unchanged.
// SHOULD FAIL
proof fn test_mutation_quota_unchanged_after_share() {
    let old_quota = Quota { mem_4k: 10, mem_2m: 5, mem_1g: 2, pcid: 3, ioid: 1 };
    let new_quota = Quota { mem_4k: 10, mem_2m: 5, mem_1g: 2, pcid: 3, ioid: 1 };
    // create_entry_and_share subtracts ret (1..=3) from mem_4k
    assert(old_quota.spec_subtract_mem_4k(new_quota, 2));
}

// Test 5: create_entry_and_share ensures other procs' address spaces
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

// Test 6: create_entry_and_share ensures ref counter for src page
// increases by exactly 1. Mutate: claim it increases by 2.
// SHOULD FAIL
proof fn test_mutation_ref_counter_increases_by_two(
    old_ref: nat,
    new_ref: nat,
)
    requires
        new_ref == old_ref + 1,
{
    assert(new_ref == old_ref + 2);
}

// Test 7: create_entry_and_share ensures ref counters for OTHER
// mapped pages are unchanged. Mutate: claim they change.
// SHOULD FAIL
proof fn test_mutation_other_page_ref_counter_changes(
    old_ref: nat,
    new_ref: nat,
)
    requires
        old_ref == new_ref,
        old_ref > 0,
{
    assert(new_ref != old_ref);
}

// Test 8: create_entry_and_share ensures page_mapping for src page
// gains (target_proc_ptr, target_va). Mutate: claim it's unchanged.
// SHOULD FAIL
proof fn test_mutation_page_mapping_unchanged(
    old_mapping: Set<(ProcPtr, VAddr)>,
    new_mapping: Set<(ProcPtr, VAddr)>,
    target_proc_ptr: ProcPtr,
    target_va: VAddr,
)
    requires
        new_mapping == old_mapping.insert((target_proc_ptr, target_va)),
        !old_mapping.contains((target_proc_ptr, target_va)),
{
    assert(new_mapping =~= old_mapping);
}

// Test 9: create_entry_and_share ensures page_mapping domain
// is unchanged. Mutate: claim domain gains a new page.
// SHOULD FAIL
proof fn test_mutation_page_mapping_domain_grows(
    old_dom: Set<PagePtr>,
    new_dom: Set<PagePtr>,
    extra_page: PagePtr,
)
    requires
        old_dom == new_dom,
        !old_dom.contains(extra_page),
{
    assert(new_dom.contains(extra_page));
}

// Test 10: create_entry_and_share ensures proc_dom is unchanged.
// Mutate: claim a new proc appears.
// SHOULD FAIL
proof fn test_mutation_proc_dom_grows(
    old_dom: Set<ProcPtr>,
    new_dom: Set<ProcPtr>,
    new_proc: ProcPtr,
)
    requires
        old_dom == new_dom,
        !old_dom.contains(new_proc),
{
    assert(new_dom.contains(new_proc));
}

// Test 11: create_entry_and_share ensures page_is_mapped status
// is unchanged for all pages. Mutate: claim some page's
// mapped status flips.
// SHOULD FAIL
proof fn test_mutation_page_mapped_status_flips(
    old_is_mapped: bool,
    new_is_mapped: bool,
)
    requires
        old_is_mapped == new_is_mapped,
        old_is_mapped == true,
{
    assert(new_is_mapped == false);
}

// Test 12: create_entry_and_share ensures container_owned_pages
// are preserved for all containers. Mutate: claim they differ.
// SHOULD FAIL
proof fn test_mutation_container_owned_pages_change(
    old_owned: Set<PagePtr>,
    new_owned: Set<PagePtr>,
    extra: PagePtr,
)
    requires
        old_owned =~= new_owned,
{
    assert(new_owned.contains(extra) && !old_owned.contains(extra));
}

}
