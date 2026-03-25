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

// SyscallReturnStruct and RetValueType for behavioral tests
#[derive(Clone, Copy, Debug)]
pub enum SwitchDecision {
    NoSwitch,
    NoThread,
    Switch,
}

#[allow(inconsistent_fields)]
#[derive(Clone, Copy)]
pub enum RetValueType {
    SuccessPairUsize { value1: usize, value2: usize },
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

    pub open spec fn get_return_vaule_pair_usize(&self) -> Option<(usize, usize)> {
        match self.error_code {
            RetValueType::SuccessPairUsize { value1, value2 } => Some((value1, value2)),
            _ => None,
        }
    }
}

// ============================================================
// BEHAVIORAL MUTATION TEST 1: Quota subtraction wrong amount
// The spec subtracts exactly 2 from mem_4k on success.
// Claiming subtraction of 3 should fail.
// SHOULD FAIL: wrong subtraction amount (3 instead of 2)
// ============================================================
proof fn test_mutation_quota_subtract_wrong_amount()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 97, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    // 100 - 2 = 98, not 97
    assert(old_quota.spec_subtract_mem_4k(new_quota, 2)); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 2: Quota mem_2m mutated
// The spec requires mem_2m unchanged after subtraction.
// Mutating mem_2m should fail.
// SHOULD FAIL: mem_2m was changed
// ============================================================
proof fn test_mutation_quota_mem_2m_changed()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 98, mem_2m: 49, mem_1g: 10, pcid: 5, ioid: 3 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 2)); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 3: Quota ioid mutated
// The spec requires ioid unchanged after subtraction.
// SHOULD FAIL: ioid was changed
// ============================================================
proof fn test_mutation_quota_ioid_changed()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 98, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 2 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 2)); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 4: proc_dom grows by wrong element
// The spec ensures proc_dom inserts page_ptr_1 (the new proc pointer).
// Inserting a different pointer should yield a different set.
// SHOULD FAIL: wrong pointer inserted into proc_dom
// ============================================================
proof fn test_mutation_proc_dom_wrong_insertion()
{
    let old_proc_dom: Set<ProcPtr> = Set::empty().insert(1).insert(2);
    let page_ptr_1: ProcPtr = 100;
    let wrong_ptr: ProcPtr = 200;
    let correct_dom = old_proc_dom.insert(page_ptr_1);
    let wrong_dom = old_proc_dom.insert(wrong_ptr);
    assert(correct_dom =~= wrong_dom); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 5: thread_dom grows by wrong element
// The spec ensures thread_dom inserts page_ptr_2 (the new thread pointer).
// Inserting a different pointer should yield a different set.
// SHOULD FAIL: wrong pointer inserted into thread_dom
// ============================================================
proof fn test_mutation_thread_dom_wrong_insertion()
{
    let old_thread_dom: Set<ThreadPtr> = Set::empty().insert(10).insert(20);
    let page_ptr_2: ThreadPtr = 300;
    let wrong_ptr: ThreadPtr = 400;
    let correct_dom = old_thread_dom.insert(page_ptr_2);
    let wrong_dom = old_thread_dom.insert(wrong_ptr);
    assert(correct_dom =~= wrong_dom); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 6: owned_procs uses insert instead of push
// The spec ensures owned_procs@ =~= old.owned_procs@.push(new_proc_ptr).
// Using a sequence with a different ordering should fail.
// SHOULD FAIL: push is ordered, insert at wrong position differs
// ============================================================
proof fn test_mutation_owned_procs_wrong_order()
{
    let old_seq: Seq<ProcPtr> = seq![1, 2, 3];
    let new_proc_ptr: ProcPtr = 100;
    let correct: Seq<ProcPtr> = old_seq.push(new_proc_ptr); // [1, 2, 3, 100]
    let wrong: Seq<ProcPtr> = seq![100, 1, 2, 3]; // prepend instead of append
    assert(correct =~= wrong); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 7: new proc has wrong pcid
// The spec ensures self.get_proc(page_ptr_1).pcid =~= new_pcid.
// Claiming a different pcid should fail.
// SHOULD FAIL: pcid mismatch
// ============================================================
proof fn test_mutation_new_proc_wrong_pcid()
{
    let new_pcid: Pcid = 42;
    let actual_pcid: Pcid = 43;
    assert(actual_pcid == new_pcid); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 8: new proc ioid is Some instead of None
// The spec ensures self.get_proc(page_ptr_1).ioid.is_None().
// Claiming it is Some should fail.
// SHOULD FAIL: ioid should be None
// ============================================================
proof fn test_mutation_new_proc_ioid_not_none()
{
    let ioid: Option<IOid> = Some(5usize);
    assert(ioid.is_None()); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 9: endpoint_dom changes (should stay the same)
// The spec ensures endpoint_dom() == old.endpoint_dom().
// Adding an endpoint to the domain should fail.
// SHOULD FAIL: endpoint_dom must not change
// ============================================================
proof fn test_mutation_endpoint_dom_changes()
{
    let old_endpoint_dom: Set<EndpointPtr> = Set::empty().insert(10).insert(20);
    let new_endpoint_dom: Set<EndpointPtr> = old_endpoint_dom.insert(30);
    assert(new_endpoint_dom =~= old_endpoint_dom); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 10: container_dom changes (should stay the same)
// The spec ensures container_dom() == old.container_dom().
// Adding a container to the domain should fail.
// SHOULD FAIL: container_dom must not change
// ============================================================
proof fn test_mutation_container_dom_changes()
{
    let old_container_dom: Set<ContainerPtr> = Set::empty().insert(1).insert(2);
    let new_container_dom: Set<ContainerPtr> = old_container_dom.insert(3);
    assert(new_container_dom =~= old_container_dom); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 11: new thread endpoint_descriptors wrong slot
// The spec ensures new_thread.endpoint_descriptors@[0] == Some(endpoint_ptr)
// and all other slots are None. Setting slot 1 instead of 0 should fail.
// SHOULD FAIL: endpoint at wrong index
// ============================================================
proof fn test_mutation_endpoint_descriptor_wrong_slot()
{
    let endpoint_ptr: EndpointPtr = 50;
    let correct: Seq<Option<EndpointPtr>> = Seq::new(
        MAX_NUM_ENDPOINT_DESCRIPTORS as nat,
        |i: int| { None },
    ).update(0, Some(endpoint_ptr));
    let wrong: Seq<Option<EndpointPtr>> = Seq::new(
        MAX_NUM_ENDPOINT_DESCRIPTORS as nat,
        |i: int| { None },
    ).update(1, Some(endpoint_ptr));
    assert(correct =~= wrong); // SHOULD FAIL
}

// ============================================================
// BEHAVIORAL MUTATION TEST 12: owned_threads of new proc is wrong
// The spec ensures new_proc.owned_threads@ ==
//   Seq::<ThreadPtr>::empty().push(page_ptr_2).
// Using a different thread pointer should fail.
// SHOULD FAIL: wrong thread in owned_threads
// ============================================================
proof fn test_mutation_owned_threads_wrong_thread()
{
    let page_ptr_2: ThreadPtr = 300;
    let wrong_ptr: ThreadPtr = 301;
    let correct: Seq<ThreadPtr> = Seq::<ThreadPtr>::empty().push(page_ptr_2);
    let wrong: Seq<ThreadPtr> = Seq::<ThreadPtr>::empty().push(wrong_ptr);
    assert(correct =~= wrong); // SHOULD FAIL
}

} // verus!
