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
// Shared definitions
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
//            BOUNDARY TESTS (10 tests)
// ============================================================

// BOUNDARY TEST 1: thread_ptr NOT in thread_dom
// SHOULD FAIL: thread_ptr not contained in thread_dom
proof fn test_boundary_thread_ptr_not_in_dom()
{
    let thread_dom: Set<ThreadPtr> = Set::empty().insert(1).insert(2).insert(3);
    let thread_ptr: ThreadPtr = 999;
    assert(thread_dom.contains(thread_ptr)); // SHOULD FAIL
}

// BOUNDARY TEST 2: endpoint_index out of range
// SHOULD FAIL: endpoint_index at upper bound is out of range
proof fn test_boundary_endpoint_index_out_of_range()
{
    let endpoint_index: EndpointIdx = MAX_NUM_ENDPOINT_DESCRIPTORS;
    assert(endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS); // SHOULD FAIL
}

// BOUNDARY TEST 3: quota.mem_4k insufficient
// SHOULD FAIL: quota too low for the requirement
proof fn test_boundary_quota_insufficient()
{
    let mem_4k: usize = 1;
    let va_range_len: usize = 0;
    assert(mem_4k >= (va_range_len * 3 + 2) as usize); // SHOULD FAIL
}

// BOUNDARY TEST 4: process children list at capacity
// SHOULD FAIL: children list is full
proof fn test_boundary_proc_children_full()
{
    let children_count: usize = PROC_CHILD_LIST_LEN;
    assert(children_count < PROC_CHILD_LIST_LEN); // SHOULD FAIL
}

// BOUNDARY TEST 5: scheduler at capacity
// SHOULD FAIL: scheduler is full
proof fn test_boundary_scheduler_full()
{
    let scheduler_len: usize = MAX_CONTAINER_SCHEDULER_LEN;
    assert(scheduler_len < MAX_CONTAINER_SCHEDULER_LEN); // SHOULD FAIL
}

// BOUNDARY TEST 6: container proc list at capacity
// SHOULD FAIL: container proc list is full
proof fn test_boundary_container_proc_list_full()
{
    let proc_list_len: usize = CONTAINER_PROC_LIST_LEN;
    assert(proc_list_len < CONTAINER_PROC_LIST_LEN); // SHOULD FAIL
}

// BOUNDARY TEST 7: depth overflow (depth == usize::MAX)
// SHOULD FAIL: depth is at maximum
proof fn test_boundary_depth_overflow()
{
    let depth: usize = usize::MAX;
    assert(depth < usize::MAX); // SHOULD FAIL
}

// BOUNDARY TEST 8: page_ptr_1 == page_ptr_2
// SHOULD FAIL: page pointers must be distinct
proof fn test_boundary_page_ptrs_equal()
{
    let page_ptr_1: PagePtr = 42;
    let page_ptr_2: PagePtr = 42;
    assert(page_ptr_1 != page_ptr_2); // SHOULD FAIL
}

// BOUNDARY TEST 9: pcid not unique
// SHOULD FAIL: new_pcid collides with existing pcid
proof fn test_boundary_pcid_collision()
{
    let existing_pcid: Pcid = 7;
    let new_pcid: Pcid = 7;
    assert(existing_pcid != new_pcid); // SHOULD FAIL
}

// BOUNDARY TEST 10: page_ptr already in page_closure
// SHOULD FAIL: page_ptr already in closure
proof fn test_boundary_page_ptr_in_closure()
{
    let page_closure: Set<PagePtr> = Set::empty().insert(100).insert(200);
    let page_ptr_1: PagePtr = 100;
    assert(page_closure.contains(page_ptr_1) == false); // SHOULD FAIL
}

// ============================================================
//       BEHAVIORAL MUTATION TESTS (12 tests)
// ============================================================

// BEHAVIORAL MUTATION TEST 1: Quota subtraction wrong amount
// SHOULD FAIL: wrong subtraction amount (3 instead of 2)
proof fn test_mutation_quota_subtract_wrong_amount()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 97, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 2)); // SHOULD FAIL
}

// BEHAVIORAL MUTATION TEST 2: Quota mem_2m mutated
// SHOULD FAIL: mem_2m was changed
proof fn test_mutation_quota_mem_2m_changed()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 98, mem_2m: 49, mem_1g: 10, pcid: 5, ioid: 3 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 2)); // SHOULD FAIL
}

// BEHAVIORAL MUTATION TEST 3: Quota ioid mutated
// SHOULD FAIL: ioid was changed
proof fn test_mutation_quota_ioid_changed()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 98, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 2 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 2)); // SHOULD FAIL
}

// BEHAVIORAL MUTATION TEST 4: proc_dom grows by wrong element
// SHOULD FAIL: wrong pointer inserted into proc_dom
proof fn test_mutation_proc_dom_wrong_insertion()
{
    let old_proc_dom: Set<ProcPtr> = Set::empty().insert(1).insert(2);
    let page_ptr_1: ProcPtr = 100;
    let wrong_ptr: ProcPtr = 200;
    let correct_dom = old_proc_dom.insert(page_ptr_1);
    let wrong_dom = old_proc_dom.insert(wrong_ptr);
    assert(correct_dom =~= wrong_dom); // SHOULD FAIL
}

// BEHAVIORAL MUTATION TEST 5: thread_dom grows by wrong element
// SHOULD FAIL: wrong pointer inserted into thread_dom
proof fn test_mutation_thread_dom_wrong_insertion()
{
    let old_thread_dom: Set<ThreadPtr> = Set::empty().insert(10).insert(20);
    let page_ptr_2: ThreadPtr = 300;
    let wrong_ptr: ThreadPtr = 400;
    let correct_dom = old_thread_dom.insert(page_ptr_2);
    let wrong_dom = old_thread_dom.insert(wrong_ptr);
    assert(correct_dom =~= wrong_dom); // SHOULD FAIL
}

// BEHAVIORAL MUTATION TEST 6: owned_procs uses wrong order
// SHOULD FAIL: push is ordered, prepend differs from append
proof fn test_mutation_owned_procs_wrong_order()
{
    let old_seq: Seq<ProcPtr> = seq![1, 2, 3];
    let new_proc_ptr: ProcPtr = 100;
    let correct: Seq<ProcPtr> = old_seq.push(new_proc_ptr);
    let wrong: Seq<ProcPtr> = seq![100, 1, 2, 3];
    assert(correct =~= wrong); // SHOULD FAIL
}

// BEHAVIORAL MUTATION TEST 7: new proc has wrong pcid
// SHOULD FAIL: pcid mismatch
proof fn test_mutation_new_proc_wrong_pcid()
{
    let new_pcid: Pcid = 42;
    let actual_pcid: Pcid = 43;
    assert(actual_pcid == new_pcid); // SHOULD FAIL
}

// BEHAVIORAL MUTATION TEST 8: new proc ioid is Some instead of None
// SHOULD FAIL: ioid should be None
proof fn test_mutation_new_proc_ioid_not_none()
{
    let ioid: Option<IOid> = Some(5usize);
    assert(ioid is None); // SHOULD FAIL
}

// BEHAVIORAL MUTATION TEST 9: endpoint_dom changes
// SHOULD FAIL: endpoint_dom must not change
proof fn test_mutation_endpoint_dom_changes()
{
    let old_endpoint_dom: Set<EndpointPtr> = Set::empty().insert(10).insert(20);
    let new_endpoint_dom: Set<EndpointPtr> = old_endpoint_dom.insert(30);
    assert(new_endpoint_dom =~= old_endpoint_dom); // SHOULD FAIL
}

// BEHAVIORAL MUTATION TEST 10: container_dom changes
// SHOULD FAIL: container_dom must not change
proof fn test_mutation_container_dom_changes()
{
    let old_container_dom: Set<ContainerPtr> = Set::empty().insert(1).insert(2);
    let new_container_dom: Set<ContainerPtr> = old_container_dom.insert(3);
    assert(new_container_dom =~= old_container_dom); // SHOULD FAIL
}

// BEHAVIORAL MUTATION TEST 11: new thread endpoint_descriptors wrong slot
// SHOULD FAIL: endpoint at wrong index
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

// BEHAVIORAL MUTATION TEST 12: owned_threads of new proc is wrong
// SHOULD FAIL: wrong thread in owned_threads
proof fn test_mutation_owned_threads_wrong_thread()
{
    let page_ptr_2: ThreadPtr = 300;
    let wrong_ptr: ThreadPtr = 301;
    let correct: Seq<ThreadPtr> = Seq::<ThreadPtr>::empty().push(page_ptr_2);
    let wrong: Seq<ThreadPtr> = Seq::<ThreadPtr>::empty().push(wrong_ptr);
    assert(correct =~= wrong); // SHOULD FAIL
}

// ============================================================
//           LOGICAL TESTS (10 tests)
// ============================================================

// LOGICAL TEST 1: Determinism of new_proc_ptr
// SHOULD FAIL: new_proc_ptr is not deterministic
proof fn test_logical_determinism_new_proc_ptr()
{
    let old_dom: Set<ProcPtr> = Set::empty().insert(1).insert(2);
    let new_ptr_a: ProcPtr = 3;
    let new_ptr_b: ProcPtr = 4;
    let dom_a: Set<ProcPtr> = old_dom.insert(new_ptr_a);
    let dom_b: Set<ProcPtr> = old_dom.insert(new_ptr_b);
    assert(dom_a =~= dom_b); // SHOULD FAIL
}

// LOGICAL TEST 2: Stronger inequality — quota stays positive
// SHOULD FAIL: quota can reach zero
proof fn test_logical_stronger_quota_stays_positive()
{
    let old_quota = Quota { mem_4k: 2, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 0, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    assume(old_quota.spec_subtract_mem_4k(new_quota, 2));
    assert(new_quota.mem_4k > 0); // SHOULD FAIL
}

// LOGICAL TEST 3: proc_dom and thread_dom grow by same element
// SHOULD FAIL: page_ptr_1 and page_ptr_2 are distinct allocations
proof fn test_logical_proc_thread_same_ptr()
{
    let page_ptr_1: usize = 100;
    let page_ptr_2: usize = 200;
    assert(page_ptr_1 == page_ptr_2); // SHOULD FAIL
}

// LOGICAL TEST 4: page_closure grows by exactly 2, not 3
// SHOULD FAIL: only two pages are added
proof fn test_logical_page_closure_grows_by_three()
{
    let old_closure: Set<PagePtr> = Set::empty().insert(1).insert(2);
    let p1: PagePtr = 100;
    let p2: PagePtr = 200;
    let p3: PagePtr = 300;
    let correct = old_closure.insert(p1).insert(p2);
    let wrong = old_closure.insert(p1).insert(p2).insert(p3);
    assert(correct =~= wrong); // SHOULD FAIL
}

// LOGICAL TEST 5: new_proc.owned_threads has length > 1
// SHOULD FAIL: owned_threads has exactly one element
proof fn test_logical_owned_threads_length_two()
{
    let page_ptr_2: ThreadPtr = 300;
    let owned_threads: Seq<ThreadPtr> = Seq::<ThreadPtr>::empty().push(page_ptr_2);
    assert(owned_threads.len() >= 2); // SHOULD FAIL
}

// LOGICAL TEST 6: old thread removed from thread_dom
// SHOULD FAIL: old threads must still be in new thread_dom
proof fn test_logical_old_thread_removed()
{
    let old_thread: ThreadPtr = 10;
    let old_dom: Set<ThreadPtr> = Set::empty().insert(old_thread).insert(20);
    let page_ptr_2: ThreadPtr = 300;
    let new_dom = old_dom.insert(page_ptr_2);
    assert(new_dom.contains(old_thread) == false); // SHOULD FAIL
}

// LOGICAL TEST 7: owned_endpoints changed
// SHOULD FAIL: owned_endpoints must not change
proof fn test_logical_owned_endpoints_differ()
{
    let old_endpoints: Set<EndpointPtr> = Set::empty().insert(5).insert(6);
    let new_endpoints: Set<EndpointPtr> = Set::empty().insert(5).insert(7);
    assert(old_endpoints =~= new_endpoints); // SHOULD FAIL
}

// LOGICAL TEST 8: Subtree set unchanged after syscall
// SHOULD FAIL: subtree must grow by new_proc_ptr
proof fn test_logical_subtree_unchanged()
{
    let old_subtree: Set<ProcPtr> = Set::empty().insert(1).insert(2);
    let page_ptr_1: ProcPtr = 100;
    let new_subtree = old_subtree.insert(page_ptr_1);
    assert(old_subtree =~= new_subtree); // SHOULD FAIL
}

// LOGICAL TEST 9: endpoint_dom grows (cross-function misuse)
// SHOULD FAIL: no new endpoint is created
proof fn test_logical_cross_fn_endpoint_dom_grows()
{
    let old_endpoint_dom: Set<EndpointPtr> = Set::empty().insert(10).insert(20);
    let new_endpoint: EndpointPtr = 99;
    let new_endpoint_dom = old_endpoint_dom.insert(new_endpoint);
    assert(new_endpoint_dom =~= old_endpoint_dom); // SHOULD FAIL
}

// LOGICAL TEST 10: New thread endpoint_descriptors slot 0 is None
// SHOULD FAIL: slot 0 must be Some
proof fn test_logical_new_thread_slot_0_is_none()
{
    let endpoint_ptr: EndpointPtr = 50;
    let descriptors: Seq<Option<EndpointPtr>> = Seq::new(
        MAX_NUM_ENDPOINT_DESCRIPTORS as nat,
        |i: int| { None },
    ).update(0, Some(endpoint_ptr));
    assert(descriptors[0] is None); // SHOULD FAIL
}

} // verus!
