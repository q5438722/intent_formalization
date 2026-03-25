use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type ThreadPtr = usize;
pub type EndpointPtr = usize;
pub type EndpointIdx = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;
pub type CpuId = usize;

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const CONTAINER_CHILD_LIST_LEN: usize = 10;
pub const CONTAINER_PROC_LIST_LEN: usize = 10;

#[derive(Clone, Copy, Debug)]
pub struct Quota {
    pub mem_4k: usize,
    pub mem_2m: usize,
    pub mem_1g: usize,
    pub pcid: usize,
    pub ioid: usize,
}

impl Quota {
    pub open spec fn spec_greater(&self, new: &Quota) -> bool {
        &&& self.mem_4k >= new.mem_4k
        &&& self.mem_2m >= new.mem_2m
        &&& self.mem_1g >= new.mem_1g
        &&& self.pcid >= new.pcid
        &&& self.ioid >= new.ioid
    }
}

// ===================== LOGICAL TESTS =====================
// Each test probes a property NOT explicitly guaranteed by the spec,
// testing determinism, stronger inequalities, structural assumptions,
// and cross-function misuse. All tests SHOULD FAIL verification.

// Test 1: Determinism — the spec does not guarantee that the new container
// pointer (page_ptr_1) is always the same given the same old state.
// Claim: two calls would produce the same new_container_ptr.
// SHOULD FAIL
proof fn test_logical_determinism_container_ptr(
    page_ptr_1a: PagePtr,
    page_ptr_1b: PagePtr,
)
    requires
        page_ptr_1a != 0,
        page_ptr_1b != 0,
{
    // Spec does not guarantee deterministic allocation
    assert(page_ptr_1a == page_ptr_1b);
}

// Test 2: The spec says new container gets exactly the init_quota.
// Claim a stronger property: that the new container's mem_4k is strictly greater
// than the init_quota (it should be exactly equal, not greater).
// SHOULD FAIL
proof fn test_logical_stronger_quota_inequality(
    init_quota_mem_4k: usize,
    new_container_mem_4k: usize,
)
    requires
        new_container_mem_4k == init_quota_mem_4k,
        init_quota_mem_4k < usize::MAX,
{
    // Stronger than guaranteed: strictly greater
    assert(new_container_mem_4k > init_quota_mem_4k);
}

// Test 3: The spec preserves old processes. Claim the spec also preserves
// the newly allocated process's pcid == old process's pcid (not guaranteed).
// SHOULD FAIL
proof fn test_logical_new_proc_same_pcid_as_caller(
    old_pcid: Pcid,
    new_pcid: Pcid,
)
    requires
        old_pcid != new_pcid,
{
    // Not guaranteed: new process has different pcid
    assert(old_pcid == new_pcid);
}

// Test 4: The spec does not guarantee that new_container_ptr, new_proc_ptr,
// and new_thread_ptr are in any particular order.
// Claim: new_container_ptr < new_proc_ptr < new_thread_ptr.
// SHOULD FAIL
proof fn test_logical_ptr_ordering(
    page_ptr_1: PagePtr,
    page_ptr_2: PagePtr,
    page_ptr_3: PagePtr,
)
    requires
        page_ptr_1 != page_ptr_2,
        page_ptr_1 != page_ptr_3,
        page_ptr_2 != page_ptr_3,
{
    // Not guaranteed: no ordering among allocated page pointers
    assert(page_ptr_1 < page_ptr_2 && page_ptr_2 < page_ptr_3);
}

// Test 5: The spec does not guarantee the new container has no owned_endpoints
// implies its owned_threads count is bounded by a specific number other than 1.
// Claim: the new container has at least 2 owned threads.
// SHOULD FAIL
proof fn test_logical_new_container_multiple_threads() {
    let owned_threads: Set<ThreadPtr> = Set::empty().insert(42);
    // The spec says exactly one thread is owned
    assert(owned_threads.len() >= 2);
}

// Test 6: The spec preserves old containers (other than the parent).
// Claim a stronger property: even the parent container is fully preserved.
// (In reality, the parent's children list is modified.)
// SHOULD FAIL
proof fn test_logical_parent_container_fully_preserved(
    old_children: Seq<ContainerPtr>,
    new_children: Seq<ContainerPtr>,
    new_child: ContainerPtr,
)
    requires
        new_children =~= old_children.push(new_child),
        old_children.len() > 0,
{
    // Incorrect: parent container IS modified (children list grows)
    assert(old_children =~= new_children);
}

// Test 7: The spec says the endpoint's owning_threads set gains (new_thread_ptr, 0).
// Claim: the endpoint's owning_threads set size stays the same (no growth).
// SHOULD FAIL
proof fn test_logical_endpoint_owning_threads_unchanged(
    old_owning_threads: Set<(ThreadPtr, EndpointIdx)>,
    new_entry: (ThreadPtr, EndpointIdx),
    new_owning_threads: Set<(ThreadPtr, EndpointIdx)>,
)
    requires
        !old_owning_threads.contains(new_entry),
        old_owning_threads.finite(),
        new_owning_threads =~= old_owning_threads.insert(new_entry),
{
    // Claim size did not change -- should fail because we inserted a new element
    assert(new_owning_threads.len() == old_owning_threads.len());
}

// Test 8: The spec does not guarantee that page_ptr_1 is page-aligned.
// Claim: page_ptr_1 % 0x1000 == 0. (The allocation may or may not guarantee this.)
// SHOULD FAIL
proof fn test_logical_page_ptr_alignment(
    page_ptr_1: PagePtr,
)
    requires
        page_ptr_1 > 0,
{
    // Not guaranteed by the spec of syscall_new_container_with_endpoint
    assert(page_ptr_1 % 0x1000 == 0);
}

// Test 9: The spec says the new container's owned_cpus is empty.
// Claim the stronger property: no container in the system has any owned CPUs.
// SHOULD FAIL
proof fn test_logical_all_containers_no_cpus(
    parent_owned_cpus: Set<CpuId>,
    new_owned_cpus: Set<CpuId>,
)
    requires
        new_owned_cpus =~= Set::<CpuId>::empty(),
{
    // Not guaranteed: parent container may have CPUs
    assert(parent_owned_cpus =~= Set::<CpuId>::empty());
}

// Test 10: Cross-function misuse — the requirement function returns false
// when mem_4k < 3 + init_quota.mem_4k. Claim that the syscall still succeeds
// (returns non-error) even when the requirement is not met.
// SHOULD FAIL
proof fn test_logical_requirement_false_but_success(
    requirement_met: bool,
    is_error: bool,
)
    requires
        requirement_met == false,
        requirement_met == false <==> is_error,
{
    // Claim: even though requirement is false, result is not error
    assert(!is_error);
}

}
