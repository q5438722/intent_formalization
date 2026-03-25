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
pub type Pcid = usize;
pub type PAddr = usize;
pub type VAddr = usize;
pub type SLLIndex = i32;

pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;

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
// BEHAVIORAL MUTATION TEST 1: Quota subtraction wrong amount
// The spec subtracts exactly 1 from mem_4k. Mutating to subtract 2
// should not satisfy the spec.
// SHOULD FAIL: Claiming quota subtracted 2 instead of 1
// ============================================================
proof fn test_mutation_quota_subtract_wrong_amount()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 98, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    // Spec says subtract 1, but we check subtract 1 with wrong result
    assert(old_quota.spec_subtract_mem_4k(new_quota, 1)); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 2: Quota subtract changes mem_2m
// The spec requires mem_2m to be unchanged. Mutating mem_2m should fail.
// SHOULD FAIL: mem_2m was mutated
// ============================================================
proof fn test_mutation_quota_mem_2m_changed()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 99, mem_2m: 49, mem_1g: 10, pcid: 5, ioid: 3 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 1)); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 3: Quota subtract changes pcid
// The spec requires pcid to be unchanged.
// SHOULD FAIL: pcid was mutated
// ============================================================
proof fn test_mutation_quota_pcid_changed()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 99, mem_2m: 50, mem_1g: 10, pcid: 4, ioid: 3 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 1)); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 4: Thread domain unchanged on failure
// When requirement fails, new =~= old. The thread domain should be unchanged.
// If we claim thread_dom changed on failure, it should fail.
// SHOULD FAIL: Asserting thread domain grew despite requirement failure
// ============================================================
proof fn test_mutation_thread_dom_changes_on_failure()
{
    let old_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2);
    let new_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2).insert(3);
    // On requirement failure, new =~= old, so domains must match
    assert(old_dom =~= new_dom); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 5: Proc domain should NOT change on success
// The spec says old.proc_dom() =~= new.proc_dom() on success.
// Mutating proc domain should be rejected.
// SHOULD FAIL: Proc domain was mutated
// ============================================================
proof fn test_mutation_proc_dom_changes_on_success()
{
    let old_dom: Set<ProcPtr> = Set::empty().insert(10).insert(20);
    let new_dom: Set<ProcPtr> = Set::empty().insert(10).insert(20).insert(30);
    assert(old_dom =~= new_dom); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 6: Container domain should NOT change
// The spec says old.container_dom() =~= new.container_dom().
// SHOULD FAIL: Container domain was mutated
// ============================================================
proof fn test_mutation_container_dom_changes()
{
    let old_dom: Set<ContainerPtr> = Set::empty().insert(100);
    let new_dom: Set<ContainerPtr> = Set::empty().insert(100).insert(200);
    assert(old_dom =~= new_dom); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 7: Endpoint domain should NOT change
// The spec says old.endpoint_dom() =~= new.endpoint_dom().
// SHOULD FAIL: Endpoint domain was mutated
// ============================================================
proof fn test_mutation_endpoint_dom_changes()
{
    let old_dom: Set<EndpointPtr> = Set::empty().insert(50);
    let new_dom: Set<EndpointPtr> = Set::empty().insert(50).insert(60);
    assert(old_dom =~= new_dom); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 8: owned_threads push should add exactly 1
// The spec says new.get_proc(target).owned_threads@ =~=
//   old.get_proc(target).owned_threads@.push(new_thread_ptr).
// If we push a different thread ptr, it should not match.
// SHOULD FAIL: Pushed wrong thread ptr
// ============================================================
proof fn test_mutation_owned_threads_wrong_push()
{
    let old_threads: Seq<ThreadPtr> = seq![1usize, 2usize, 3usize];
    let expected = old_threads.push(4usize); // correct new_thread_ptr = 4
    let mutated = old_threads.push(5usize);  // wrong new_thread_ptr = 5
    assert(expected =~= mutated); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 9: Endpoint owning_threads wrong insert
// The spec inserts (new_thread_ptr, 0) into endpoint owning_threads.
// Inserting with wrong endpoint_idx descriptor (1 instead of 0) should fail.
// SHOULD FAIL: Wrong descriptor index in owning_threads insert
// ============================================================
proof fn test_mutation_endpoint_owning_threads_wrong_idx()
{
    let old_set: Set<(ThreadPtr, EndpointIdx)> = Set::empty().insert((10usize, 0usize));
    let new_thread_ptr: ThreadPtr = 20;
    let correct = old_set.insert((new_thread_ptr, 0usize));
    let mutated = old_set.insert((new_thread_ptr, 1usize)); // wrong: idx 1 instead of 0
    assert(correct =~= mutated); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 10: New thread endpoint_descriptors wrong
// The spec says endpoint_descriptors =~= Seq::new(128, |i| None).update(0, Some(endpoint_ptr)).
// Mutating to have the endpoint at index 1 instead of 0 should fail.
// SHOULD FAIL: Endpoint descriptor at wrong index
// ============================================================
proof fn test_mutation_endpoint_descriptors_wrong_index()
{
    let ep_ptr: EndpointPtr = 42;
    let correct: Seq<Option<EndpointPtr>> = Seq::new(
        MAX_NUM_ENDPOINT_DESCRIPTORS as nat,
        |i: int| { None },
    ).update(0, Some(ep_ptr));
    let mutated: Seq<Option<EndpointPtr>> = Seq::new(
        MAX_NUM_ENDPOINT_DESCRIPTORS as nat,
        |i: int| { None },
    ).update(1, Some(ep_ptr));
    assert(correct =~= mutated); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 11: Physical page mapping should NOT change
// The spec says new.get_physical_page_mapping() =~= old.get_physical_page_mapping().
// SHOULD FAIL: Claiming page mapping changed
// ============================================================
proof fn test_mutation_page_mapping_changes()
{
    let old_map: Map<PagePtr, Set<(ProcPtr, VAddr)>> = Map::empty();
    let new_map: Map<PagePtr, Set<(ProcPtr, VAddr)>> = Map::empty().insert(
        1usize, Set::empty().insert((2usize, 3usize))
    );
    assert(old_map =~= new_map); // SHOULD FAIL
}

} // verus!
