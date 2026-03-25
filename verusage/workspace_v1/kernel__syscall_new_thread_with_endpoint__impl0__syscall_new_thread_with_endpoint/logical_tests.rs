use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus! {

// ============================================================
// Minimal type/const definitions from target file
// ============================================================

pub type IOid = usize;
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
// LOGICAL TEST 1: Determinism - two thread_doms with same insert yield same result
// The spec does NOT guarantee that new_thread_ptr is deterministic.
// We test that two different new_thread_ptrs produce the same thread_dom,
// which should be false.
// SHOULD FAIL: New thread ptr is not deterministic
// ============================================================
proof fn test_logical_determinism_thread_ptr()
{
    let old_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2);
    let new_dom_a: Set<ThreadPtr> = old_dom.insert(3);
    let new_dom_b: Set<ThreadPtr> = old_dom.insert(4);
    // Two different new_thread_ptrs produce different domains
    assert(new_dom_a =~= new_dom_b); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 2: Stronger inequality - quota subtract preserves non-negative
// The spec does NOT explicitly require that mem_4k >= k before subtraction.
// We test whether the spec entails mem_4k > 0 after subtraction.
// Actually: if old.mem_4k == 1 and k == 1, new.mem_4k == 0. Asserting > 0 fails.
// SHOULD FAIL: Claiming quota stays positive after exact drain
// ============================================================
proof fn test_logical_stronger_quota_positive()
{
    let old_quota = Quota { mem_4k: 1, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 0, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    assume(old_quota.spec_subtract_mem_4k(new_quota, 1));
    assert(new_quota.mem_4k > 0); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 3: Thread domain growth is exactly 1
// The spec says old.thread_dom().insert(new_thread_ptr) =~= new.thread_dom().
// But it does NOT guarantee that new_thread_ptr was not already in old.thread_dom().
// However, if it WAS already present, the set wouldn't grow.
// We test the stronger claim: |new_dom| == |old_dom| + 2, which is false.
// SHOULD FAIL: Thread domain grew by more than 1
// ============================================================
proof fn test_logical_thread_dom_grows_by_two()
{
    let old_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2);
    let new_thread_1: ThreadPtr = 3;
    let new_thread_2: ThreadPtr = 4;
    let new_dom: Set<ThreadPtr> = old_dom.insert(new_thread_1).insert(new_thread_2);
    // The spec only allows inserting ONE new thread
    // Claiming two inserts produces same result as one should fail
    let spec_dom: Set<ThreadPtr> = old_dom.insert(new_thread_1);
    assert(new_dom =~= spec_dom); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 4: Endpoint queue_state is NOT allowed to change
// The spec says: new.get_endpoint(ep).queue_state =~= old.get_endpoint(ep).queue_state
// We test that the spec does NOT allow queue_state to change by
// asserting two different states are equal.
// SHOULD FAIL: Endpoint queue states differ
// ============================================================

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EndpointState {
    RECEIVE,
    SEND,
}

proof fn test_logical_endpoint_queue_state_changes()
{
    let old_state = EndpointState::RECEIVE;
    let new_state = EndpointState::SEND;
    // Spec guarantees queue_state is preserved
    assert(old_state =~= new_state); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 5: New thread belongs to a DIFFERENT container
// The spec says: new.get_thread(new_thread_ptr).owning_container == target_container_ptr
// where target_container_ptr = old.get_thread(thread_id).owning_container.
// We test the false claim that the new thread could be in a different container.
// SHOULD FAIL: New thread assigned to wrong container
// ============================================================
proof fn test_logical_new_thread_wrong_container()
{
    let target_container: ContainerPtr = 100;
    let other_container: ContainerPtr = 200;
    // The spec ties the new thread to target_container.
    // Claiming it's in other_container should fail.
    assert(target_container == other_container); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 6: Cross-function misuse - using endpoint_index 0 maps to index 1
// The spec places the endpoint at descriptor index 0 for the new thread:
//   endpoint_descriptors =~= Seq::new(128, |i| None).update(0, Some(target_endpoint_ptr))
// We test the false claim that querying index 1 yields the endpoint.
// SHOULD FAIL: Endpoint not at index 1
// ============================================================
proof fn test_logical_endpoint_at_wrong_descriptor_index()
{
    let ep_ptr: EndpointPtr = 42;
    let descriptors: Seq<Option<EndpointPtr>> = Seq::new(
        MAX_NUM_ENDPOINT_DESCRIPTORS as nat,
        |i: int| { None },
    ).update(0, Some(ep_ptr));
    // The endpoint is at index 0, not index 1
    assert(descriptors[1] == Some(ep_ptr)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 7: Structural assumption - owned_threads push implies containment of old elements
// The spec says new.owned_threads@ = old.owned_threads@.push(new_thread_ptr).
// We test the false claim that pushing removes an old element.
// SHOULD FAIL: Push does not remove existing elements
// ============================================================
proof fn test_logical_push_removes_old()
{
    let old_threads: Seq<ThreadPtr> = seq![10usize, 20usize, 30usize];
    let new_threads = old_threads.push(40usize);
    // 10 should still be in the new sequence
    // Asserting it's not should fail
    assert(!new_threads.contains(10usize)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 8: Global assumption - set insert idempotence misuse
// If new_thread_ptr is already in old_dom, insert is idempotent.
// We test the false claim that inserting an existing element grows the set.
// SHOULD FAIL: Inserting existing element doesn't grow the set
// ============================================================
proof fn test_logical_set_insert_idempotent_grows()
{
    let s: Set<ThreadPtr> = Set::empty().insert(1).insert(2).insert(3);
    let s2 = s.insert(2); // 2 already present
    // s2 should equal s since 2 was already in the set
    // Asserting they differ should fail
    assert(!(s =~= s2)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 9: Stronger property - new thread must have empty endpoint descriptors except index 0
// The spec says descriptors = Seq::new(128, |i| None).update(0, Some(ep_ptr))
// We test the false claim that index 127 is Some.
// SHOULD FAIL: Last descriptor is None, not Some
// ============================================================
proof fn test_logical_last_descriptor_not_none()
{
    let ep_ptr: EndpointPtr = 42;
    let descriptors: Seq<Option<EndpointPtr>> = Seq::new(
        MAX_NUM_ENDPOINT_DESCRIPTORS as nat,
        |i: int| { None },
    ).update(0, Some(ep_ptr));
    assert(descriptors[127] != None::<EndpointPtr>); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 10: Stronger invariant - quota subtract with k=0 implies mem_4k grows
// The spec only says self.mem_4k - k == new.mem_4k.
// When k == 0, new.mem_4k == old.mem_4k. It does NOT grow.
// We test the false claim that the quota grew.
// SHOULD FAIL: Quota does not increase on zero-subtract
// ============================================================
proof fn test_logical_quota_zero_subtract_grows()
{
    let old_quota = Quota { mem_4k: 50, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 50, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    assume(old_quota.spec_subtract_mem_4k(new_quota, 0));
    // False claim: quota increased
    assert(new_quota.mem_4k > old_quota.mem_4k); // SHOULD FAIL
}

} // verus!
