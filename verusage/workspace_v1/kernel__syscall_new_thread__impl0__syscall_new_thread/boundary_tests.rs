use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus! {

// ============================================================
// Minimal type/const definitions from target file
// ============================================================

pub type IOid = usize;
pub type CpuId = usize;
pub type ThreadPtr = usize;
pub type ProcPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type PageMapPtr = usize;
pub type Pcid = usize;
pub type PAddr = usize;
pub type VAddr = usize;
pub type SLLIndex = i32;
pub type PagePerm4k = PointsTo<[u8; PAGE_SZ_4k]>;

pub const NUM_CPUS: usize = 32;
pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const CONTAINER_PROC_LIST_LEN: usize = 10;
pub const CONTAINER_CHILD_LIST_LEN: usize = 10;
pub const PROC_CHILD_LIST_LEN: usize = 10;
pub const CONTAINER_ENDPOINT_LIST_LEN: usize = 10;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const PAGE_SZ_4k: usize = 1usize << 12;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const PCID_MAX: usize = 4096;
pub const IOID_MAX: usize = 4096;

// ============================================================
// Quota definition
// ============================================================
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

// ============================================================
// BOUNDARY TEST 1: thread_ptr NOT in thread_dom
// The precondition requires old(self).thread_dom().contains(thread_ptr).
// Using a thread_ptr not in the domain violates the precondition.
// SHOULD FAIL: thread_ptr not contained in thread_dom
// ============================================================
proof fn test_boundary_thread_ptr_not_in_dom()
{
    let thread_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2).insert(3);
    let thread_ptr: ThreadPtr = 999; // not in domain
    assert(thread_dom.contains(thread_ptr)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 2: thread_dom is empty — no valid thread_ptr exists
// If the thread domain is empty, no thread_ptr can satisfy the
// precondition thread_dom.contains(thread_ptr).
// SHOULD FAIL: empty domain cannot contain any thread
// ============================================================
proof fn test_boundary_empty_thread_dom()
{
    let thread_dom: Set<ThreadPtr> = Set::empty();
    let thread_ptr: ThreadPtr = 0;
    assert(thread_dom.contains(thread_ptr)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 3: MAX_NUM_THREADS_PER_PROC boundary
// When owned_threads.len() == MAX_NUM_THREADS_PER_PROC (128),
// the requirement returns false (thread list is full).
// Asserting the thread list is NOT full should fail.
// SHOULD FAIL: thread list at capacity is full
// ============================================================
proof fn test_boundary_thread_list_at_max()
{
    let thread_count: usize = MAX_NUM_THREADS_PER_PROC; // 128
    // requirement checks: owned_threads.len() >= MAX_NUM_THREADS_PER_PROC => false
    assert(!(thread_count >= MAX_NUM_THREADS_PER_PROC)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 4: quota mem_4k == 0 means requirement fails
// The requirement checks: quota.mem_4k < 1 => false.
// With mem_4k == 0, the requirement should be false.
// Asserting that 0 >= 1 should fail.
// SHOULD FAIL: zero quota does not satisfy mem_4k >= 1
// ============================================================
proof fn test_boundary_zero_quota()
{
    let mem_4k: usize = 0;
    assert(mem_4k >= 1); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 5: scheduler at MAX_CONTAINER_SCHEDULER_LEN
// When scheduler.len() == MAX_CONTAINER_SCHEDULER_LEN (10),
// the requirement returns false (scheduler is full).
// SHOULD FAIL: scheduler at capacity is full
// ============================================================
proof fn test_boundary_scheduler_at_max()
{
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN; // 10
    assert(!(scheduler_len >= MAX_CONTAINER_SCHEDULER_LEN)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 6: free_pages_4k count == 0
// The requirement checks: free_pages_4k.len() <= 0 => false.
// With 0 free pages, the requirement should be false.
// SHOULD FAIL: zero free pages does not satisfy > 0
// ============================================================
proof fn test_boundary_zero_free_pages()
{
    let free_pages: usize = 0;
    assert(free_pages > 0); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 7: quota subtract with mem_4k == 0 and k == 1
// Subtracting 1 from quota with mem_4k == 0 should be impossible
// in the valid range. In Verus spec-level math, 0 - 1 is negative
// and cannot equal any usize. Claiming it produces usize::MAX
// (wrapping) should be rejected by spec-level int arithmetic.
// SHOULD FAIL: 0 - 1 cannot produce a valid usize quota
// ============================================================
proof fn test_boundary_quota_subtract_underflow()
{
    let old_quota = Quota { mem_4k: 0, mem_2m: 10, mem_1g: 5, pcid: 2, ioid: 1 };
    let new_quota = Quota { mem_4k: usize::MAX, mem_2m: 10, mem_1g: 5, pcid: 2, ioid: 1 };
    // In spec-level integer arithmetic: 0 - 1 = -1, and -1 != usize::MAX
    assert(old_quota.spec_subtract_mem_4k(new_quota, 1)); // SHOULD FAIL
}

} // verus!
