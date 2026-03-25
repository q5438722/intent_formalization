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
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;
pub type PageMapPtr = usize;
pub type ThreadPtr = usize;
pub type EndpointPtr = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

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

    pub open spec fn spec_set_mem_4k(&self, v: usize) -> Self {
        Quota {
            mem_4k: v,
            mem_2m: self.mem_2m,
            mem_1g: self.mem_1g,
            pcid: self.pcid,
            ioid: self.ioid,
        }
    }
}

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs, then mutates expected outputs
// or relations. All tests SHOULD FAIL verification.

// Test 1: alloc_and_map_io ensures free_pages decreases by exactly 1.
// Mutate: claim free_pages stays the same.
// SHOULD FAIL
proof fn test_mutation_free_pages_unchanged(
    old_free: usize,
    new_free: usize,
)
    requires
        old_free > 0,
        new_free == old_free - 1,
{
    assert(new_free == old_free);
}

// Test 2: alloc_and_map_io ensures the returned page's IO mappings contain (ioid, va).
// Mutate: claim the IO mapping set is empty after the call.
// SHOULD FAIL
proof fn test_mutation_io_mapping_empty(
    ioid: IOid,
    va: VAddr,
)
{
    let io_mappings: Set<(IOid, VAddr)> = Set::empty().insert((ioid, va));
    assert(io_mappings =~= Set::<(IOid, VAddr)>::empty());
}

// Test 3: alloc_and_map_io ensures get_io_space is old IO space plus the new entry.
// Mutate: claim the IO space is unchanged.
// SHOULD FAIL
proof fn test_mutation_io_space_unchanged(
    old_io_space: Map<VAddr, MapEntry>,
    target_va: VAddr,
    ret: MapEntry,
)
    requires
        !old_io_space.dom().contains(target_va),
{
    let new_io_space = old_io_space.insert(target_va, ret);
    assert(new_io_space.dom() =~= old_io_space.dom());
}

// Test 4: alloc_and_map_io ensures proc_dom() is unchanged.
// Mutate: claim a new proc was added.
// SHOULD FAIL
proof fn test_mutation_proc_dom_grows(
    old_proc_dom: Set<ProcPtr>,
    new_proc_ptr: ProcPtr,
)
    requires
        !old_proc_dom.contains(new_proc_ptr),
        old_proc_dom =~= old_proc_dom,  // trivially true, just context
{
    let new_proc_dom = old_proc_dom;
    assert(new_proc_dom.contains(new_proc_ptr));
}

// Test 5: alloc_and_map_io ensures address_space is preserved for all procs.
// Mutate: claim address space changed for target proc.
// SHOULD FAIL
proof fn test_mutation_address_space_changed(
    old_addr_space: Map<VAddr, MapEntry>,
    new_addr_space: Map<VAddr, MapEntry>,
    fake_va: VAddr,
    fake_entry: MapEntry,
)
    requires
        old_addr_space =~= new_addr_space,
        !old_addr_space.dom().contains(fake_va),
{
    assert(new_addr_space.dom().contains(fake_va));
}

// Test 6: alloc_and_map_io ensures IO space of OTHER procs is unchanged.
// Mutate: claim another proc's IO space got an extra entry.
// SHOULD FAIL
proof fn test_mutation_other_proc_io_space_changed(
    other_io_space: Map<VAddr, MapEntry>,
    extra_va: VAddr,
    extra_entry: MapEntry,
)
    requires
        !other_io_space.dom().contains(extra_va),
{
    let modified = other_io_space.insert(extra_va, extra_entry);
    assert(modified =~= other_io_space);
}

// Test 7: alloc_and_map_io ensures container quota is decremented by 1.
// Mutate: claim quota is decremented by 2.
// SHOULD FAIL
proof fn test_mutation_quota_double_decrement(
    old_quota: Quota,
)
    requires
        old_quota.mem_4k >= 2,
{
    let new_quota = old_quota.spec_set_mem_4k((old_quota.mem_4k - 1) as usize);
    assert(old_quota.spec_subtract_mem_4k(new_quota, 2));
}

// Test 8: alloc_and_map_io ensures the returned MapEntry has write == true.
// Mutate: claim write is false.
// SHOULD FAIL
proof fn test_mutation_ret_write_false() {
    let ret = MapEntry { addr: 0x1000, write: true, execute_disable: false };
    assert(ret.write == false);
}

// Test 9: alloc_and_map_io ensures the returned MapEntry has execute_disable == false.
// Mutate: claim execute_disable is true.
// SHOULD FAIL
proof fn test_mutation_ret_execute_disable_true() {
    let ret = MapEntry { addr: 0x1000, write: true, execute_disable: false };
    assert(ret.execute_disable == true);
}

// Test 10: alloc_and_map_io ensures container_dom is unchanged.
// Mutate: claim a container was removed.
// SHOULD FAIL
proof fn test_mutation_container_dom_shrinks(
    old_container_dom: Set<ContainerPtr>,
    c_ptr: ContainerPtr,
)
    requires
        old_container_dom.contains(c_ptr),
{
    let new_container_dom = old_container_dom;
    assert(!new_container_dom.contains(c_ptr));
}

// Test 11: alloc_and_map_io ensures other containers' owned_pages are unchanged.
// Mutate: claim another container lost a page.
// SHOULD FAIL
proof fn test_mutation_other_container_pages_changed(
    old_pages: Set<PagePtr>,
    new_pages: Set<PagePtr>,
    page: PagePtr,
)
    requires
        old_pages.contains(page),
        old_pages =~= new_pages,
{
    assert(!new_pages.contains(page));
}

// Test 12: alloc_and_map_io ensures the target container's owned pages
// grow by exactly the new page. Mutate: claim pages grow by two pages.
// SHOULD FAIL
proof fn test_mutation_container_pages_grow_by_two(
    old_pages: Set<PagePtr>,
    ret: PagePtr,
    extra: PagePtr,
)
    requires
        !old_pages.contains(ret),
        !old_pages.contains(extra),
        ret != extra,
{
    let expected = old_pages.insert(ret);
    let mutated = old_pages.insert(ret).insert(extra);
    assert(mutated =~= expected);
}

}
