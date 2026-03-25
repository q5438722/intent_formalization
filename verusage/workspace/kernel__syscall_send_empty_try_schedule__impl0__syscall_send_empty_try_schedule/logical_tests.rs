use vstd::prelude::*;

fn main() {}

verus!{

pub type CpuId = usize;
pub type ThreadPtr = usize;
pub type EndpointIdx = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type ProcPtr = usize;
pub type PagePtr = usize;
pub type Pcid = usize;

pub const NUM_CPUS: usize = 32;
pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;

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

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize {
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize {
    (i * 4096) as usize
}

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// Tests check for unintended entailment: determinism, stronger bounds,
// structural assumptions, and cross-function reasoning.
// All tests SHOULD FAIL verification.

// Test 1: syscall_send_empty_try_schedule only ensures self.wf().
// It does NOT guarantee any specific return value. Claim it always
// returns Error.
// SHOULD FAIL
proof fn test_logical_always_returns_error(
    ret_error_code: RetValueType,
    kernel_wf: bool,
)
    requires
        kernel_wf == true,  // only postcondition
{
    assert(ret_error_code == RetValueType::Error);
}

// Test 2: syscall_send_empty_try_schedule only ensures self.wf().
// It does NOT guarantee the switch_decision. Claim it is always NoSwitch.
// SHOULD FAIL
proof fn test_logical_always_noswitch(
    switch: SwitchDecision,
    kernel_wf: bool,
)
    requires
        kernel_wf == true,
{
    assert(switch == SwitchDecision::NoSwitch);
}

// Test 3: The postcondition does NOT guarantee the kernel state is
// unchanged. Claim self == old(self) (kernel unchanged).
// SHOULD FAIL
proof fn test_logical_kernel_unchanged(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
    kernel_wf: bool,
)
    requires
        kernel_wf == true,
{
    assert(old_thread_dom =~= new_thread_dom);
}

// Test 4: The postcondition does NOT guarantee the sender thread
// is still RUNNING. Claim it remains RUNNING.
// SHOULD FAIL
proof fn test_logical_sender_still_running(
    post_state: ThreadState,
    kernel_wf: bool,
)
    requires
        kernel_wf == true,
{
    assert(post_state == ThreadState::RUNNING);
}

// Test 5: The postcondition does NOT guarantee any specific CPU state.
// Claim the CPU still has a current thread after the syscall.
// SHOULD FAIL
proof fn test_logical_cpu_still_has_thread(
    current_thread: Option<ThreadPtr>,
    kernel_wf: bool,
)
    requires
        kernel_wf == true,
{
    assert(current_thread.is_Some());
}

// Test 6: Determinism — the spec does NOT guarantee that the same
// inputs always produce the same return. Claim determinism.
// SHOULD FAIL
proof fn test_logical_determinism(
    ret1: SyscallReturnStruct,
    ret2: SyscallReturnStruct,
    kernel_wf1: bool,
    kernel_wf2: bool,
)
    requires
        kernel_wf1 == true,
        kernel_wf2 == true,
{
    assert(ret1.switch_decision == ret2.switch_decision);
}

// Test 7: The postcondition does NOT guarantee that endpoint queues
// are preserved. Claim they are unchanged.
// SHOULD FAIL
proof fn test_logical_endpoint_queue_preserved(
    old_queue: Seq<ThreadPtr>,
    new_queue: Seq<ThreadPtr>,
    kernel_wf: bool,
)
    requires
        kernel_wf == true,
{
    assert(old_queue =~= new_queue);
}

// Test 8: The postcondition does NOT guarantee scheduler length bounds.
// Claim the scheduler has at most MAX_CONTAINER_SCHEDULER_LEN entries.
// (The spec actually allows exactly MAX_CONTAINER_SCHEDULER_LEN after
// adding a thread.)
// SHOULD FAIL
proof fn test_logical_scheduler_strict_bound(
    scheduler_len: usize,
    kernel_wf: bool,
)
    requires
        kernel_wf == true,
{
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN);
}

// Test 9: is_send and is_receive are NOT guaranteed to be mutually
// exclusive for an arbitrary bool pair. The spec constrains them
// individually but claim their xor holds without spec context.
// SHOULD FAIL
proof fn test_logical_send_receive_exclusive(
    is_s: bool,
    is_r: bool,
)
{
    // Without constraining is_s and is_r to come from the same EndpointState,
    // we cannot prove XOR.
    assert(is_s != is_r);
}

// Test 10: The spec does NOT guarantee that the return pcid (when Some)
// is different from the sender's pcid. Claim they always differ.
// SHOULD FAIL
proof fn test_logical_pcid_always_different(
    ret_pcid: Option<Pcid>,
    sender_pcid: Pcid,
    kernel_wf: bool,
)
    requires
        kernel_wf == true,
        ret_pcid.is_Some(),
{
    assert(ret_pcid.unwrap() != sender_pcid);
}

// Test 11: page_ptr_valid and page_index_valid are different domains.
// The spec does NOT imply one from the other for arbitrary values.
// Claim: page_ptr_valid(x) implies x < NUM_PAGES.
// (It implies x/0x1000 < NUM_PAGES, not x < NUM_PAGES.)
// SHOULD FAIL
proof fn test_logical_page_ptr_implies_small_value(ptr: usize)
    requires
        ptr % 0x1000 == 0,
        ptr / 0x1000 < NUM_PAGES,
{
    assert(ptr < NUM_PAGES);
}

// Test 12: The postcondition of syscall_send_empty_try_schedule does NOT
// guarantee that page_closure is preserved. Claim it is.
// SHOULD FAIL
proof fn test_logical_page_closure_preserved(
    old_closure: Set<PagePtr>,
    new_closure: Set<PagePtr>,
    kernel_wf: bool,
)
    requires
        kernel_wf == true,
{
    assert(old_closure =~= new_closure);
}

// Test 13: The postcondition does NOT guarantee that the return cr3
// is always None. Claim it is.
// SHOULD FAIL
proof fn test_logical_cr3_always_none(
    cr3: Option<usize>,
    kernel_wf: bool,
)
    requires
        kernel_wf == true,
{
    assert(cr3.is_None());
}

// Test 14: Cross-function: schedule_running_thread changes the
// scheduler for the sender's container, but the spec does NOT guarantee
// that other containers' schedulers are bounded by a specific length.
// Claim all schedulers have length < 5.
// SHOULD FAIL
proof fn test_logical_all_schedulers_small(
    scheduler_len: usize,
    kernel_wf: bool,
)
    requires
        kernel_wf == true,
{
    assert(scheduler_len < 5);
}

// Test 15: The spec does NOT guarantee the return value's error_code
// field is always Else when switch_decision is Switch. These are
// unrelated in the general postcondition. Claim they're linked.
// SHOULD FAIL
proof fn test_logical_switch_implies_else(
    switch: SwitchDecision,
    error_code: RetValueType,
    kernel_wf: bool,
)
    requires
        kernel_wf == true,
{
    assert(switch == SwitchDecision::Switch ==> error_code == RetValueType::Else);
}

}
