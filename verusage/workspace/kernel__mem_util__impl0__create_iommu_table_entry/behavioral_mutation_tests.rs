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
pub type IOid = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;

pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

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

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid postcondition values and mutates them.
// All tests SHOULD FAIL verification.

// Test 1: ret.0 is guaranteed <= 3.
// Mutate: claim ret.0 == 4.
// SHOULD FAIL
proof fn test_behavioral_ret_exceeds_upper_bound(ret0: usize)
    requires
        ret0 <= 3,
{
    assert(ret0 == 4);
}

// Test 2: free pages decrease by exactly ret.0.
// Mutate: free pages decrease by ret.0 + 1.
// SHOULD FAIL
proof fn test_behavioral_wrong_free_page_decrease(
    old_free: usize,
    new_free: usize,
    ret0: usize,
)
    requires
        ret0 <= 3,
        old_free >= 3,
        new_free == old_free - ret0,
{
    assert(new_free == old_free - ret0 - 1);
}

// Test 3: proc_dom must be preserved (unchanged).
// Mutate: claim a new proc was added.
// SHOULD FAIL
proof fn test_behavioral_proc_dom_changed(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
    extra_proc: ProcPtr,
)
    requires
        new_proc_dom =~= old_proc_dom,
        !old_proc_dom.contains(extra_proc),
{
    assert(new_proc_dom.contains(extra_proc));
}

// Test 4: thread_dom must be preserved.
// Mutate: claim a thread was removed.
// SHOULD FAIL
proof fn test_behavioral_thread_dom_changed(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
    t_ptr: ThreadPtr,
)
    requires
        new_thread_dom =~= old_thread_dom,
        old_thread_dom.contains(t_ptr),
{
    assert(!new_thread_dom.contains(t_ptr));
}

// Test 5: container_dom must be preserved.
// Mutate: claim a new container was added.
// SHOULD FAIL
proof fn test_behavioral_container_dom_changed(
    old_container_dom: Set<ContainerPtr>,
    new_container_dom: Set<ContainerPtr>,
    extra_c: ContainerPtr,
)
    requires
        new_container_dom =~= old_container_dom,
        !old_container_dom.contains(extra_c),
{
    assert(new_container_dom.contains(extra_c));
}

// Test 6: endpoint_dom must be preserved.
// Mutate: claim an endpoint was removed.
// SHOULD FAIL
proof fn test_behavioral_endpoint_dom_changed(
    old_ep_dom: Set<EndpointPtr>,
    new_ep_dom: Set<EndpointPtr>,
    e_ptr: EndpointPtr,
)
    requires
        new_ep_dom =~= old_ep_dom,
        old_ep_dom.contains(e_ptr),
{
    assert(!new_ep_dom.contains(e_ptr));
}

// Test 7: quota subtraction must match spec_subtract_mem_4k.
// Mutate: subtract one extra from mem_4k.
// SHOULD FAIL
proof fn test_behavioral_quota_wrong_subtraction(
    old_quota: Quota,
    new_quota: Quota,
    ret0: usize,
)
    requires
        ret0 <= 3,
        old_quota.mem_4k >= 3,
        old_quota.spec_subtract_mem_4k(new_quota, ret0),
{
    assert(new_quota.mem_4k == old_quota.mem_4k - ret0 - 1);
}

// Test 8: page_mapping must be unchanged.
// Mutate: claim page_mapping changed (new entry added).
// SHOULD FAIL
proof fn test_behavioral_page_mapping_changed(
    old_mapping: Map<PagePtr, Set<(ProcPtr, VAddr)>>,
    new_mapping: Map<PagePtr, Set<(ProcPtr, VAddr)>>,
    page: PagePtr,
)
    requires
        new_mapping =~= old_mapping,
        !old_mapping.dom().contains(page),
{
    assert(new_mapping.dom().contains(page));
}

// Test 9: IO space for all procs with IOMMU tables is preserved.
// Mutate: claim IO space changed for some proc.
// SHOULD FAIL
proof fn test_behavioral_io_space_changed(
    old_io_space: Map<VAddr, MapEntry>,
    new_io_space: Map<VAddr, MapEntry>,
    va: VAddr,
)
    requires
        new_io_space =~= old_io_space,
        !old_io_space.dom().contains(va),
{
    assert(new_io_space.dom().contains(va));
}

// Test 10: other containers' owned pages must be preserved.
// Mutate: claim owned pages changed for an unrelated container.
// SHOULD FAIL
proof fn test_behavioral_container_owned_pages_changed(
    old_pages: Set<PagePtr>,
    new_pages: Set<PagePtr>,
    p: PagePtr,
)
    requires
        new_pages =~= old_pages,
        !old_pages.contains(p),
{
    assert(new_pages.contains(p));
}

}
