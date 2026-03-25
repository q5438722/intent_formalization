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

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize
    recommends
        ptr % 0x1000 == 0,
        ptr / 0x1000 < NUM_PAGES,
{
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize
    recommends
        0 <= i < NUM_PAGES,
{
    (i * 4096) as usize
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// These probe unintended reasoning: determinism, stronger inequalities,
// structural/global assumptions, cross-function misuse.
// All tests SHOULD FAIL verification.

// Test 1: The spec does NOT guarantee determinism — two invocations
// with the same preconditions could produce different return values
// (Error vs Else depending on internal state).
// Claim: the return type is always Error.
// SHOULD FAIL
proof fn test_logical_return_always_error(
    endpoint_exists: bool,
    queue_state_is_send: bool,
    queue_len: usize,
)
    requires
        endpoint_exists,
        !queue_state_is_send,
        queue_len > 0,
{
    // The spec says if receiver exists, many more checks follow.
    // There's no guarantee the result is always Error.
    let ret = RetValueType::Else;
    assert(ret == RetValueType::Error);
}

// Test 2: The spec for success case does NOT guarantee that the sender's
// thread state is changed. The sender is not blocked in the success path.
// Claim: sender state must become BLOCKED in success case.
// SHOULD FAIL
proof fn test_logical_sender_always_blocked(
    sender_state: ThreadState,
)
    requires
        sender_state == ThreadState::RUNNING,
{
    // In the success path, the spec preserves threads t_ptr != receiver_thread_ptr
    // The sender is not the receiver, so sender's state should stay RUNNING.
    assert(sender_state == ThreadState::BLOCKED);
}

// Test 3: range_create_and_share_mapping ensures free_pages decreases by ret.
// The spec does NOT guarantee ret == 3 * va_range.len.
// Claim: ret is always exactly 3 * len.
// SHOULD FAIL
proof fn test_logical_ret_always_3x_len(
    ret: usize,
    len: usize,
)
    requires
        ret <= 3 * len,
        len > 0,
{
    assert(ret == 3 * len);
}

// Test 4: The spec says in error cases old =~= new. But this doesn't mean
// that all error cases produce the same return value.
// Claim: all error cases return RetValueType::Error.
// SHOULD FAIL (because the "no_receiver" blocking case also returns Error
// but modifies state, and different paths may exist)
proof fn test_logical_error_return_identical(
    ret1: RetValueType,
    ret2: RetValueType,
)
    requires
        ret1 == RetValueType::Error,
{
    assert(ret1 == ret2);
}

// Test 5: The spec does NOT assert that page_ptr2page_index and
// page_index2page_ptr are strict inverses for ALL usize values.
// The roundtrip only works for valid page pointers.
// Claim: roundtrip holds for arbitrary usize.
// SHOULD FAIL
proof fn test_logical_roundtrip_arbitrary() {
    let ptr: usize = 5000; // Not page-aligned
    assert(spec_page_index2page_ptr(spec_page_ptr2page_index(ptr)) == ptr);
}

// Test 6: The spec says endpoint queue_state is preserved in the success case
// for the blocking endpoint. Claim: queue_state is always RECEIVE after success.
// This is not guaranteed — if it was RECEIVE before, it stays RECEIVE,
// but the spec just says new == old for queue_state.
// SHOULD FAIL (can't prove it's RECEIVE without knowing old state)
proof fn test_logical_queue_state_always_receive(
    old_state: EndpointState,
    new_state: EndpointState,
)
    requires
        new_state == old_state,
{
    assert(new_state == EndpointState::RECEIVE);
}

// Test 7: The spec for success case says new address space entries at receiver_va[i]
// equal old sender entries at sender_va[i]. But spec does NOT guarantee the
// physical address is non-zero.
// Claim: shared page address is always non-zero.
// SHOULD FAIL
proof fn test_logical_shared_page_addr_nonzero(
    shared_addr: PAddr,
)
    requires
        shared_addr == shared_addr, // tautology — no constraint on value
{
    assert(shared_addr != 0);
}

// Test 8: The spec does NOT assert that va_range.len > 0 in the preconditions
// (va_range.wf() doesn't explicitly require len > 0).
// Claim: len is always > 0.
// SHOULD FAIL
proof fn test_logical_va_range_len_always_positive(
    len: usize,
)
    requires
        len * 4096 < usize::MAX, // part of wf()
{
    assert(len > 0);
}

// Test 9: The spec says containers for non-receiver containers are preserved.
// This does NOT imply the receiver's container quota is preserved.
// Claim: all container quotas are preserved after success.
// SHOULD FAIL
proof fn test_logical_all_container_quota_preserved(
    old_quota: Quota,
    new_quota: Quota,
)
    requires
        old_quota.mem_4k >= 3,
{
    assert(old_quota.mem_4k == new_quota.mem_4k);
}

// Test 10: The spec says physical page mapping domain is preserved (dom is same).
// This does NOT mean the mapping values are identical for pages involved in sharing.
// Claim: all page mappings are identical.
// SHOULD FAIL
proof fn test_logical_all_page_mappings_identical(
    old_mapping: Map<PagePtr, Set<(ProcPtr, VAddr)>>,
    new_mapping: Map<PagePtr, Set<(ProcPtr, VAddr)>>,
    page_ptr: PagePtr,
)
    requires
        old_mapping.dom() =~= new_mapping.dom(),
        old_mapping.dom().contains(page_ptr),
{
    assert(old_mapping[page_ptr] =~= new_mapping[page_ptr]);
}

// Test 11: The spec does NOT guarantee that the receiver's thread state
// changes after success (schedule_blocked_thread doesn't specify the
// new thread state in its postcondition for the head thread).
// Claim: receiver thread is RUNNING after success.
// SHOULD FAIL
proof fn test_logical_receiver_becomes_running(
    receiver_state: ThreadState,
)
    requires
        receiver_state == ThreadState::BLOCKED,
{
    assert(receiver_state == ThreadState::RUNNING);
}

// Test 12: NoSwitchNew ensures switch_decision == NoSwitch.
// Claim: there exists a code path where switch_decision is Switch.
// SHOULD FAIL (but tests if spec allows Switch)
proof fn test_logical_switch_decision_never_switch() {
    let decision = SwitchDecision::NoSwitch;
    assert(decision == SwitchDecision::Switch);
}

}
