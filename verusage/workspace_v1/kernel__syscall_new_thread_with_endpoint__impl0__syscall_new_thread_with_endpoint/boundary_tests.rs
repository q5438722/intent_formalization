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
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;
pub type SLLIndex = i32;
pub type PagePerm4k = PointsTo<[u8; PAGE_SZ_4k]>;
pub type PagePerm2m = PointsTo<[u8; PAGE_SZ_2m]>;
pub type PagePerm1g = PointsTo<[u8; PAGE_SZ_1g]>;

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
pub const PAGE_SZ_2m: usize = 1usize << 21;
pub const PAGE_SZ_1g: usize = 1usize << 30;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const PCID_MAX: usize = 4096;
pub const IOID_MAX: usize = 4096;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

// ============================================================
// Quota definition (needed for spec_subtract_mem_4k)
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
// BOUNDARY TEST 1: endpoint_index at exact upper boundary
// The precondition requires 0 <= endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS (128).
// endpoint_index = 128 should violate the precondition.
// SHOULD FAIL: Trying to use endpoint_index == MAX_NUM_ENDPOINT_DESCRIPTORS
// ============================================================
proof fn test_boundary_endpoint_index_at_max()
{
    let endpoint_index: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS; // 128 == invalid
    // The precondition requires endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS
    // This assertion claims 128 < 128, which is false.
    assert(endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 2: endpoint_index exceeds upper boundary (large value)
// SHOULD FAIL: An extremely large endpoint_index should not satisfy the precondition.
// ============================================================
proof fn test_boundary_endpoint_index_overflow()
{
    let endpoint_index: EndpointIdx = usize::MAX;
    assert(0 <= endpoint_index && endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 3: quota mem_4k == 0 should fail the requirement
// The requirement checks: old.get_container_quota(c_id).mem_4k < 1 => false
// So mem_4k == 0 means the requirement is not satisfied.
// A correct spec should NOT allow a successful thread creation with zero quota.
// SHOULD FAIL: asserting that zero quota satisfies the subtract spec for k=1
// ============================================================
proof fn test_boundary_zero_quota_subtract()
{
    let old_quota = Quota { mem_4k: 0, mem_2m: 10, mem_1g: 5, pcid: 2, ioid: 1 };
    // Attempting to subtract 1 from mem_4k == 0 should underflow / be invalid
    // In Verus, 0usize - 1 is undefined / wraps. The spec says self.mem_4k - k == new.mem_4k.
    // If the spec allows this, it's a weakness.
    let new_quota = Quota { mem_4k: usize::MAX, mem_2m: 10, mem_1g: 5, pcid: 2, ioid: 1 };
    // This would be true under wrapping arithmetic: 0 - 1 == usize::MAX
    // The spec should reject this because quota can't go negative.
    // We assert this holds, expecting the verifier to reject it or accept it
    // (if it accepts, the spec is too weak).
    assert(old_quota.spec_subtract_mem_4k(new_quota, 1)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 4: quota subtract with k larger than mem_4k
// Subtracting more than available should not produce a valid new quota.
// SHOULD FAIL: spec_subtract_mem_4k with k > mem_4k claims valid result
// ============================================================
proof fn test_boundary_quota_subtract_exceeds()
{
    let old_quota = Quota { mem_4k: 5, mem_2m: 10, mem_1g: 5, pcid: 2, ioid: 1 };
    // Subtracting 10 from 5: in spec-level integer arithmetic, 5 - 10 = -5.
    // There is no valid usize that equals -5, so there should be no valid new_quota.
    // We try to claim there exists a valid new_quota. This should fail.
    let new_quota = Quota { mem_4k: 0, mem_2m: 10, mem_1g: 5, pcid: 2, ioid: 1 };
    // 5 - 10 != 0, so this should fail
    assert(old_quota.spec_subtract_mem_4k(new_quota, 10)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 5: endpoint_index exactly 0 boundary (valid edge)
// This should succeed if spec is correct - testing that 0 is accepted.
// We negate the condition to make the test SHOULD FAIL.
// SHOULD FAIL: Asserting 0 is NOT a valid endpoint_index
// ============================================================
proof fn test_boundary_endpoint_index_zero_invalid()
{
    let endpoint_index: EndpointIdx = 0;
    // 0 IS a valid index, so asserting it's invalid should fail
    assert(!(0 <= endpoint_index && endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 6: MAX_NUM_THREADS_PER_PROC boundary
// When thread count == MAX_NUM_THREADS_PER_PROC, the requirement should be false.
// We test that at exact max, adding is still considered valid (SHOULD FAIL).
// SHOULD FAIL: Asserting that a full thread list is not full
// ============================================================
proof fn test_boundary_thread_list_full_not_detected()
{
    let thread_count: usize = MAX_NUM_THREADS_PER_PROC; // 128
    // The requirement checks: owned_threads.len() >= MAX_NUM_THREADS_PER_PROC => false
    // So is_full = (thread_count >= 128) = true
    // Asserting is_full is false should fail
    assert(!(thread_count >= MAX_NUM_THREADS_PER_PROC)); // SHOULD FAIL
}

// ============================================================
// BOUNDARY TEST 7: MAX_CONTAINER_SCHEDULER_LEN boundary
// When scheduler.len() == MAX_CONTAINER_SCHEDULER_LEN, requirement fails.
// SHOULD FAIL: Claiming scheduler is not full when it's at max
// ============================================================
proof fn test_boundary_scheduler_full_not_detected()
{
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN; // 10
    assert(!(scheduler_len >= MAX_CONTAINER_SCHEDULER_LEN)); // SHOULD FAIL
}

} // verus!
