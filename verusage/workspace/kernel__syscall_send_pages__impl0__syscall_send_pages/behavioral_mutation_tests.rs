use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ThreadPtr = usize;
pub type ContainerPtr = usize;
pub type EndpointPtr = usize;
pub type EndpointIdx = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

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

#[allow(inconsistent_fields)]
#[derive(Clone, Copy)]
pub enum RetValueType {
    SuccessUsize { value: usize },
    Error,
    Else,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwitchDecision {
    NoSwitch,
    Switch,
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
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs/postconditions of syscall_send_pages,
// then asserts a WRONG output or relation.
// All tests SHOULD FAIL verification.

// Test 1: In "no receiver" (SEND queue) case, spec ensures sender is pushed to queue.
// The endpoint queue becomes old_queue.push(sender_thread_ptr).
// Mutate: claim queue is unchanged after blocking.
// SHOULD FAIL
proof fn test_mutation_queue_unchanged_after_block(
    old_queue: Seq<ThreadPtr>,
    new_queue: Seq<ThreadPtr>,
    sender_thread_ptr: ThreadPtr,
)
    requires
        new_queue =~= old_queue.push(sender_thread_ptr),
        old_queue.len() < MAX_NUM_THREADS_PER_ENDPOINT,
{
    assert(new_queue =~= old_queue);
}

// Test 2: In success case, spec ensures receiver endpoint queue advances by skip(1).
// Mutate: claim queue is unchanged.
// SHOULD FAIL
proof fn test_mutation_endpoint_queue_not_advanced(
    old_queue: Seq<ThreadPtr>,
    new_queue: Seq<ThreadPtr>,
)
    requires
        new_queue =~= old_queue.skip(1),
        old_queue.len() > 0,
{
    assert(new_queue =~= old_queue);
}

// Test 3: In "sender queue full" case, spec ensures old =~= new (no state change).
// Mutate: claim the thread domains differ.
// SHOULD FAIL
proof fn test_mutation_state_changes_when_queue_full(
    old_thread_dom: Set<ThreadPtr>,
    new_thread_dom: Set<ThreadPtr>,
)
    requires
        old_thread_dom =~= new_thread_dom,
{
    // Pick an arbitrary thread ptr and claim it's only in old
    assert(exists|t: ThreadPtr| old_thread_dom.contains(t) && !new_thread_dom.contains(t));
}

// Test 4: In blocking case, spec ensures thread state becomes BLOCKED.
// Mutate: claim thread state becomes SCHEDULED instead.
// SHOULD FAIL
proof fn test_mutation_blocked_becomes_scheduled() {
    let new_state = ThreadState::BLOCKED;
    assert(new_state == ThreadState::SCHEDULED);
}

// Test 5: In blocking case, spec ensures thread's ipc_payload va_range == sender_va_range.
// Mutate: claim the ipc_payload va_range is different (wrong start address).
// SHOULD FAIL
proof fn test_mutation_ipc_payload_va_start_wrong(
    sender_va_start: VAddr,
    payload_va_start: VAddr,
)
    requires
        payload_va_start == sender_va_start,
        sender_va_start != 0,
{
    assert(payload_va_start != sender_va_start);
}

// Test 6: In success case, spec ensures new address space for receiver contains
// the shared pages: new_space[receiver_va[i]] == old_space_sender[sender_va[i]].
// Mutate: claim receiver VA is NOT in receiver's new space.
// SHOULD FAIL
proof fn test_mutation_receiver_va_not_mapped(
    new_receiver_space: Map<VAddr, MapEntry>,
    receiver_va: VAddr,
    sender_entry: MapEntry,
)
    requires
        new_receiver_space.dom().contains(receiver_va),
        new_receiver_space[receiver_va].addr == sender_entry.addr,
{
    assert(!new_receiver_space.dom().contains(receiver_va));
}

// Test 7: In "receiver queue empty" case, spec ensures endpoint queue_state changes to SEND.
// Mutate: claim queue_state stays RECEIVE.
// SHOULD FAIL
proof fn test_mutation_queue_state_stays_receive() {
    let new_state = EndpointState::SEND;
    assert(new_state == EndpointState::RECEIVE);
}

// Test 8: NoSwitchNew ensures ret.error_code == error_code.
// With Error input, mutate: claim ret has Else error code.
// SHOULD FAIL
proof fn test_mutation_return_wrong_error_code() {
    let input_code = RetValueType::Error;
    let ret_code = input_code;
    // The spec says ret.error_code == error_code, so ret_code == Error
    // Mutate: claim it's Else
    assert(ret_code == RetValueType::Else);
}

// Test 9: In success case, spec ensures proc domain is unchanged.
// Mutate: claim proc domain grows.
// SHOULD FAIL
proof fn test_mutation_proc_domain_grows(
    old_proc_dom: Set<ProcPtr>,
    new_proc_dom: Set<ProcPtr>,
    extra_proc: ProcPtr,
)
    requires
        old_proc_dom =~= new_proc_dom,
        !old_proc_dom.contains(extra_proc),
{
    assert(new_proc_dom.contains(extra_proc));
}

// Test 10: In success case, spec ensures sender's endpoint_descriptors are preserved.
// Mutate: claim they differ.
// SHOULD FAIL
proof fn test_mutation_sender_descriptors_change(
    old_descriptors: Seq<Option<EndpointPtr>>,
    new_descriptors: Seq<Option<EndpointPtr>>,
)
    requires
        new_descriptors =~= old_descriptors,
        old_descriptors.len() > 0,
{
    assert(new_descriptors.len() != old_descriptors.len());
}

// Test 11: Quota subtraction: spec says old.mem_4k - ret == new.mem_4k.
// Mutate: claim quota unchanged when ret > 0.
// SHOULD FAIL
proof fn test_mutation_quota_unchanged_with_nonzero_ret() {
    let old_quota = Quota { mem_4k: 10, mem_2m: 5, mem_1g: 2, pcid: 3, ioid: 1 };
    let new_quota = Quota { mem_4k: 10, mem_2m: 5, mem_1g: 2, pcid: 3, ioid: 1 };
    let ret: usize = 3;
    assert(old_quota.spec_subtract_mem_4k(new_quota, ret));
}

// Test 12: In success case, spec says non-receiver procs' address spaces are preserved.
// Mutate: claim receiver's old mappings are lost.
// SHOULD FAIL
proof fn test_mutation_old_receiver_mapping_lost(
    old_space: Map<VAddr, MapEntry>,
    new_space: Map<VAddr, MapEntry>,
    va: VAddr,
)
    requires
        old_space.dom().contains(va),
        new_space.dom().contains(va),
        new_space[va].addr == old_space[va].addr,
{
    assert(new_space[va].addr != old_space[va].addr);
}

}
