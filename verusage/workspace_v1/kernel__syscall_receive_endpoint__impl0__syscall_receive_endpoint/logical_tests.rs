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
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EndpointState {
    RECEIVE,
    SEND,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThreadState {
    SCHEDULED,
    BLOCKED,
    RUNNING,
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

// ============================================================
// LOGICAL TEST 1: Determinism of success/failure outcome
// The spec does NOT guarantee that the syscall is deterministic
// given the same inputs but different kernel states. Two valid
// kernel states with the same arguments could yield different
// error/success outcomes. Asserting mutual exclusion of all
// outcomes for all states is not entailed.
// SHOULD FAIL
// ============================================================
proof fn test_logical_success_implies_no_state_change_false()
{
    // The spec says: on success, state changes (is_pass_endpoint_completed).
    // Asserting success implies NO state change contradicts the spec.
    // This tests that success and "old =~= new" are incompatible,
    // but the spec doesn't give us this as a standalone property.
    let old_queue: Seq<ThreadPtr> = seq![100usize, 200usize, 300usize];
    let new_queue = old_queue.skip(1);

    // Claiming success means queues are unchanged (false)
    assert(old_queue =~= new_queue); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 2: Stronger inequality — queue length strictly decreases
// The spec says on success, queue becomes old.skip(1).
// But it does NOT entail that queue length is ALWAYS strictly less
// (e.g., if original queue had length 0, skip(1) on an empty 
// sequence is still empty). Assert len strictly decreases always.
// SHOULD FAIL
// ============================================================
proof fn test_logical_queue_length_always_strictly_decreases()
{
    let empty_queue: Seq<ThreadPtr> = Seq::empty();
    let after_skip = empty_queue.skip(1);
    // skip(1) on empty is still empty, so length doesn't decrease
    assert(after_skip.len() < empty_queue.len()); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 3: Cross-function misuse — endpoint state is always SEND
// The spec handles both RECEIVE and SEND endpoint states differently.
// Asserting that the endpoint state must always be SEND is not
// guaranteed — RECEIVE is equally valid.
// SHOULD FAIL
// ============================================================
proof fn test_logical_endpoint_state_always_send()
{
    let state = EndpointState::RECEIVE;
    assert(state == EndpointState::SEND); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 4: Structural assumption — thread_dom is always non-empty
// The spec requires thread_dom().contains(receiver_thread_ptr),
// which implies the domain is non-empty. But this is a precondition,
// not a general truth about all kernel states.
// An empty thread_dom is a valid kernel state structurally.
// SHOULD FAIL
// ============================================================
proof fn test_logical_thread_dom_always_nonempty()
{
    let thread_dom: Set<ThreadPtr> = Set::empty();
    // Assert that thread_dom always has at least one element
    assert(exists|t: ThreadPtr| thread_dom.contains(t)); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 5: Return value is never Error (too strong)
// The spec allows both success and error returns depending on
// conditions. Asserting the return is never Error is too strong.
// SHOULD FAIL
// ============================================================
proof fn test_logical_return_never_error()
{
    let ret = SyscallReturnStruct {
        error_code: RetValueType::Error,
        pcid: None,
        cr3: None,
        switch_decision: SwitchDecision::NoSwitch,
    };
    // Asserting the syscall never returns Error
    assert(!ret.spec_is_error()); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 6: Queue ordering — skip(1) preserves elements from index 1
// The spec says new_queue = old.skip(1), meaning old[1..] == new[0..].
// Asserting old[0] == new[0] (i.e., no skip happened) should fail.
// SHOULD FAIL
// ============================================================
proof fn test_logical_skip_preserves_head()
{
    let old_queue: Seq<ThreadPtr> = seq![10usize, 20usize, 30usize];
    let new_queue = old_queue.skip(1);
    // old[0] is 10, new[0] is 20 after skip(1)
    assert(old_queue[0] == new_queue[0]); // SHOULD FAIL
}

// ============================================================
// LOGICAL TEST 7: Global assumption — endpoint descriptors are unique across threads
// The spec does NOT guarantee that no two threads share the same
// endpoint_ptr in their descriptors. This is a structural assumption
// that is not entailed.
// SHOULD FAIL
// ============================================================
proof fn test_logical_endpoint_descriptors_globally_unique()
{
    // Two threads could both have the same endpoint_ptr at different indices
    let ep: Option<EndpointPtr> = Some(0xBEEF);
    let thread1_descriptors: Seq<Option<EndpointPtr>> = seq![ep, None, None];
    let thread2_descriptors: Seq<Option<EndpointPtr>> = seq![ep, None, None];

    // Assert they cannot share the same endpoint_ptr (not guaranteed by spec)
    assert(
        forall|i: int, j: int|
            #![auto]
            0 <= i < thread1_descriptors.len() && 0 <= j < thread2_descriptors.len()
            && thread1_descriptors[i].is_Some() && thread2_descriptors[j].is_Some()
            ==> thread1_descriptors[i] != thread2_descriptors[j]
    ); // SHOULD FAIL
}

}
