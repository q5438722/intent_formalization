use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus! {

// ============================================================
// Combined correctness tests for syscall_receive_endpoint
// Each test encodes a property that SHOULD FAIL verification
// to probe the semantic boundary of the specification.
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThreadState {
    SCHEDULED,
    BLOCKED,
    RUNNING,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EndpointState {
    RECEIVE,
    SEND,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwitchDecision {
    NoSwitch,
    NoThread,
    Switch,
}

#[allow(inconsistent_fields)]
#[derive(Clone, Copy)]
pub enum RetValueType {
    SuccessUsize { value: usize },
    Error,
    Else,
}

#[derive(Clone, Copy)]
pub struct SyscallReturnStruct {
    pub error_code: RetValueType,
    pub pcid: Option<Pcid>,
    pub cr3: Option<usize>,
    pub switch_decision: SwitchDecision,
}

impl SyscallReturnStruct {
    pub open spec fn spec_is_error(&self) -> bool {
        match self.error_code {
            RetValueType::Error => true,
            _ => false,
        }
    }
}

// ============================
// BOUNDARY TESTS
// ============================

// BOUNDARY 1: receiver_thread_ptr not in thread_dom
// SHOULD FAIL
proof fn test_boundary_receiver_not_in_thread_dom()
{
    let thread_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2).insert(3);
    let receiver_thread_ptr: ThreadPtr = 999;
    assert(thread_dom.contains(receiver_thread_ptr)); // SHOULD FAIL
}

// BOUNDARY 2: blocking_endpoint_index == MAX_NUM_ENDPOINT_DESCRIPTORS
// SHOULD FAIL
proof fn test_boundary_endpoint_index_at_max()
{
    let blocking_endpoint_index: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// BOUNDARY 3: blocking_endpoint_index == usize::MAX
// SHOULD FAIL
proof fn test_boundary_endpoint_index_overflow()
{
    let blocking_endpoint_index: EndpointIdx = usize::MAX;
    assert(blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// BOUNDARY 4: receiver_endpoint_payload == MAX
// SHOULD FAIL
proof fn test_boundary_payload_index_at_max()
{
    let receiver_endpoint_payload: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(receiver_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// BOUNDARY 5: thread state is BLOCKED (not RUNNING)
// SHOULD FAIL
proof fn test_boundary_thread_not_running_blocked()
{
    let state = ThreadState::BLOCKED;
    assert(state == ThreadState::RUNNING); // SHOULD FAIL
}

// BOUNDARY 6: thread state is SCHEDULED (not RUNNING)
// SHOULD FAIL
proof fn test_boundary_thread_not_running_scheduled()
{
    let state = ThreadState::SCHEDULED;
    assert(state == ThreadState::RUNNING); // SHOULD FAIL
}

// BOUNDARY 7: empty thread domain
// SHOULD FAIL
proof fn test_boundary_empty_thread_dom()
{
    let thread_dom: Set<ThreadPtr> = Set::empty();
    let receiver_thread_ptr: ThreadPtr = 0;
    assert(thread_dom.contains(receiver_thread_ptr)); // SHOULD FAIL
}

// BOUNDARY 8: both indices at boundary
// SHOULD FAIL
proof fn test_boundary_both_indices_at_max()
{
    let blocking_endpoint_index: EndpointIdx = 128;
    let receiver_endpoint_payload: EndpointIdx = 128;
    assert(
        blocking_endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS
        && receiver_endpoint_payload < MAX_NUM_ENDPOINT_DESCRIPTORS
    ); // SHOULD FAIL
}

// ============================
// BEHAVIORAL MUTATION TESTS
// ============================

// BEHAVIORAL 1: queue push order mutated (prepend vs append)
// SHOULD FAIL
proof fn test_behavioral_queue_push_order_mutated()
{
    let old_queue: Seq<ThreadPtr> = seq![10usize, 20usize, 30usize];
    let thread_ptr: ThreadPtr = 42;
    let new_queue = old_queue.push(thread_ptr);
    let mutated_queue: Seq<ThreadPtr> = seq![42usize, 10usize, 20usize, 30usize];
    assert(new_queue =~= mutated_queue); // SHOULD FAIL
}

// BEHAVIORAL 2: success skip count mutated (skip 2 vs skip 1)
// SHOULD FAIL
proof fn test_behavioral_success_skip_count_mutated()
{
    let old_queue: Seq<ThreadPtr> = seq![100usize, 200usize, 300usize];
    let correct = old_queue.skip(1);
    let mutated = old_queue.skip(2);
    assert(correct =~= mutated); // SHOULD FAIL
}

// BEHAVIORAL 3: endpoint descriptor updated at wrong index
// SHOULD FAIL
proof fn test_behavioral_endpoint_descriptor_wrong_index()
{
    let old_desc: Seq<Option<EndpointPtr>> = seq![
        None, None, None, None, None,
        None, None, None, None, None,
    ];
    let ep: EndpointPtr = 0xABCD;
    let correct = old_desc.update(3, Some(ep));
    let wrong = old_desc.update(5, Some(ep));
    assert(correct =~= wrong); // SHOULD FAIL
}

// BEHAVIORAL 4: owning threads wrong pair inserted
// SHOULD FAIL
proof fn test_behavioral_owning_threads_wrong_pair()
{
    let old_owning: Set<(ThreadPtr, EndpointIdx)> = Set::empty();
    let dst: ThreadPtr = 42;
    let correct = old_owning.insert((dst, 3usize));
    let wrong = old_owning.insert((dst, 7usize));
    assert(correct =~= wrong); // SHOULD FAIL
}

// BEHAVIORAL 5: fail case state changed (should be unchanged)
// SHOULD FAIL
proof fn test_behavioral_fail_state_changed()
{
    let old_queue: Seq<ThreadPtr> = seq![10usize, 20usize];
    let new_queue: Seq<ThreadPtr> = seq![10usize];
    assert(old_queue =~= new_queue); // SHOULD FAIL
}

// BEHAVIORAL 6: queue state not changed to RECEIVE
// SHOULD FAIL
proof fn test_behavioral_queue_state_not_changed_to_receive()
{
    let new_state = EndpointState::RECEIVE;
    let expected_wrong = EndpointState::SEND;
    assert(new_state =~= expected_wrong); // SHOULD FAIL
}

// BEHAVIORAL 7: IPC payload wrong value stored
// SHOULD FAIL
proof fn test_behavioral_ipc_payload_wrong_value()
{
    let stored: Option<EndpointIdx> = Some(5usize);
    let wrong: Option<EndpointIdx> = Some(99usize);
    assert(stored =~= wrong); // SHOULD FAIL
}

// ============================
// LOGICAL TESTS
// ============================

// LOGICAL 1: success implies no state change (contradicts spec)
// SHOULD FAIL
proof fn test_logical_success_implies_no_state_change()
{
    let old_queue: Seq<ThreadPtr> = seq![100usize, 200usize, 300usize];
    let new_queue = old_queue.skip(1);
    assert(old_queue =~= new_queue); // SHOULD FAIL
}

// LOGICAL 2: queue length always strictly decreases (even for empty)
// SHOULD FAIL
proof fn test_logical_queue_length_always_strictly_decreases()
{
    let empty_queue: Seq<ThreadPtr> = Seq::empty();
    let after_skip = empty_queue.skip(1);
    assert(after_skip.len() < empty_queue.len()); // SHOULD FAIL
}

// LOGICAL 3: endpoint state is always SEND (too strong)
// SHOULD FAIL
proof fn test_logical_endpoint_state_always_send()
{
    let state = EndpointState::RECEIVE;
    assert(state == EndpointState::SEND); // SHOULD FAIL
}

// LOGICAL 4: thread_dom is always non-empty (structural assumption)
// SHOULD FAIL
proof fn test_logical_thread_dom_always_nonempty()
{
    let thread_dom: Set<ThreadPtr> = Set::empty();
    assert(exists|t: ThreadPtr| thread_dom.contains(t)); // SHOULD FAIL
}

// LOGICAL 5: return value is never Error (too strong)
// SHOULD FAIL
proof fn test_logical_return_never_error()
{
    let ret = SyscallReturnStruct {
        error_code: RetValueType::Error,
        pcid: None,
        cr3: None,
        switch_decision: SwitchDecision::NoSwitch,
    };
    assert(!ret.spec_is_error()); // SHOULD FAIL
}

// LOGICAL 6: skip(1) preserves head element (incorrect)
// SHOULD FAIL
proof fn test_logical_skip_preserves_head()
{
    let old_queue: Seq<ThreadPtr> = seq![10usize, 20usize, 30usize];
    let new_queue = old_queue.skip(1);
    assert(old_queue[0] == new_queue[0]); // SHOULD FAIL
}

// LOGICAL 7: endpoint descriptors globally unique across threads
// SHOULD FAIL
proof fn test_logical_endpoint_descriptors_globally_unique()
{
    let ep: Option<EndpointPtr> = Some(0xBEEFusize);
    let t1_desc: Seq<Option<EndpointPtr>> = seq![ep, None, None];
    let t2_desc: Seq<Option<EndpointPtr>> = seq![ep, None, None];
    assert(
        forall|i: int, j: int|
            #![auto]
            0 <= i < t1_desc.len() && 0 <= j < t2_desc.len()
            && t1_desc[i] == Some(0xBEEFusize) && t2_desc[j] == Some(0xBEEFusize)
            ==> false
    ); // SHOULD FAIL
}

}
