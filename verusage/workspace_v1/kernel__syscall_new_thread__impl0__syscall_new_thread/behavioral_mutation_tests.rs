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

// SyscallReturnStruct and RetValueType for behavioral tests
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
}

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
// BEHAVIORAL MUTATION TEST 1: Quota subtraction wrong amount
// The spec subtracts exactly 1 from mem_4k on success.
// Claiming subtraction of 2 should fail.
// SHOULD FAIL: wrong subtraction amount (2 instead of 1)
// ============================================================
proof fn test_mutation_quota_subtract_wrong_amount()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 98, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    // 100 - 1 = 99, not 98
    assert(old_quota.spec_subtract_mem_4k(new_quota, 1)); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 2: Quota mem_2m mutated
// The spec requires mem_2m unchanged. Mutating it should fail.
// SHOULD FAIL: mem_2m was changed
// ============================================================
proof fn test_mutation_quota_mem_2m_changed()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 99, mem_2m: 49, mem_1g: 10, pcid: 5, ioid: 3 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 1)); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 3: Quota ioid mutated
// The spec requires ioid unchanged after subtraction.
// SHOULD FAIL: ioid was changed
// ============================================================
proof fn test_mutation_quota_ioid_changed()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 99, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 2 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 1)); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 4: Error/success biconditional inverted
// The ensures says: requirement == false <==> ret.is_error()
// If requirement is true (all conditions met), ret should NOT be error.
// Claiming success produces error should fail.
// SHOULD FAIL: requirement true but claiming error
// ============================================================
proof fn test_mutation_success_returns_error()
{
    // Simulate: all requirements met (not full, has quota, not full sched, has pages)
    let requirement_met: bool = true;
    let is_error: bool = true; // mutated: should be false
    // The spec says: requirement == false <==> is_error
    // So requirement == true ==> is_error == false
    assert(requirement_met == false <==> is_error); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 5: Error returns success
// When requirement is false, the return MUST be error.
// Claiming non-error on failure should fail.
// SHOULD FAIL: requirement false but claiming success
// ============================================================
proof fn test_mutation_failure_returns_success()
{
    let requirement_met: bool = false;
    let is_error: bool = false; // mutated: should be true
    assert(requirement_met == false <==> is_error); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 6: Thread domain unchanged on success
// The spec says thread_dom grows by insert(ret) on success.
// Claiming thread_dom is unchanged should fail.
// SHOULD FAIL: thread_dom must grow on success
// ============================================================
proof fn test_mutation_thread_dom_unchanged_on_success()
{
    let old_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2);
    let new_thread_ptr: ThreadPtr = 3;
    let new_dom: Set<ThreadPtr> = old_dom.insert(new_thread_ptr);
    // old_dom != new_dom since 3 was added
    assert(old_dom =~= new_dom); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 7: Page closure unchanged on success
// The spec says page_closure grows by insert(new_page_ptr).
// Claiming it stays the same should fail.
// SHOULD FAIL: page_closure must grow on success
// ============================================================
proof fn test_mutation_page_closure_unchanged()
{
    let old_pages: Set<PagePtr> = Set::empty().insert(100).insert(200);
    let new_page_ptr: PagePtr = 300;
    let new_pages: Set<PagePtr> = old_pages.insert(new_page_ptr);
    assert(old_pages =~= new_pages); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 8: proc_dom changes on success
// The spec ensures proc_dom is unchanged: old.proc_dom() =~= new.proc_dom().
// Claiming proc_dom grew should fail.
// SHOULD FAIL: proc_dom was mutated
// ============================================================
proof fn test_mutation_proc_dom_changes()
{
    let old_dom: Set<ProcPtr> = Set::empty().insert(10).insert(20);
    let new_dom: Set<ProcPtr> = old_dom.insert(30);
    assert(old_dom =~= new_dom); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 9: container_dom changes on success
// The spec ensures container_dom is unchanged.
// SHOULD FAIL: container_dom was mutated
// ============================================================
proof fn test_mutation_container_dom_changes()
{
    let old_dom: Set<ContainerPtr> = Set::empty().insert(100);
    let new_dom: Set<ContainerPtr> = old_dom.insert(200);
    assert(old_dom =~= new_dom); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 10: pcid changes for the calling proc
// The spec says old.get_proc(proc_ptr).pcid =~= new.get_proc(proc_ptr).pcid.
// Claiming pcid changed should fail.
// SHOULD FAIL: pcid was mutated for the owning process
// ============================================================
proof fn test_mutation_pcid_changed()
{
    let old_pcid: Pcid = 42;
    let new_pcid: Pcid = 43;
    assert(old_pcid =~= new_pcid); // SHOULD FAIL
}

} // verus!
