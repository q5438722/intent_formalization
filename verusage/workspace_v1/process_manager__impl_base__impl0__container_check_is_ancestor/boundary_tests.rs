use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type VAddr = usize;
pub type CpuId = usize;
pub type IOid = usize;
pub const PROC_CHILD_LIST_LEN: usize = 10;
pub const MAX_NUM_THREADS_PER_PROC: usize = 128;
pub const MAX_CONTAINER_SCHEDULER_LEN: usize = 10;
pub const CONTAINER_PROC_LIST_LEN: usize = 10;
pub const CONTAINER_CHILD_LIST_LEN: usize = 10;
pub type Pcid = usize;

pub type ThreadPtr = usize;
pub type EndpointPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type EndpointIdx = usize;
pub type PagePerm4k = PointsTo<[u8; PAGE_SZ_4k]>;
pub const PAGE_SZ_4k: usize = 1usize << 12;
pub type SLLIndex = i32;
pub const MAX_NUM_THREADS_PER_ENDPOINT: usize = 128;


#[derive(Debug)]
pub struct Node<T> {
    pub value: Option<T>,
    pub next: SLLIndex,
    pub prev: SLLIndex,
}

#[verifier::reject_recursive_types(T)]
pub struct StaticLinkedList<T, const N: usize> {
    pub ar: [Node<T>; N],
    pub spec_seq: Ghost<Seq<T>>,
    pub value_list: Ghost<Seq<SLLIndex>>,
    pub value_list_head: SLLIndex,
    pub value_list_tail: SLLIndex,
    pub value_list_len: usize,
    pub free_list: Ghost<Seq<SLLIndex>>,
    pub free_list_head: SLLIndex,
    pub free_list_tail: SLLIndex,
    pub free_list_len: usize,
    pub size: usize,
    pub arr_seq: Ghost<Seq<Node<T>>>,
}

impl<T, const N: usize> StaticLinkedList<T, N> {
    pub open spec fn spec_len(&self) -> usize {
        self@.len() as usize
    }

    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_len))]
    pub fn len(&self) -> (l: usize)
        ensures
            l == self.value_list_len,
            self.wf() ==> l == self.len(),
            self.wf() ==> l == self@.len(),
    {
        unimplemented!()
    }

    pub open spec fn unique(&self) -> bool {
        forall|i: int, j: int|
            #![trigger self.spec_seq@[i], self.spec_seq@[j]]
            0 <= i < self.len() && 0 <= j < self.len() && i != j ==> self.spec_seq@[i]
                != self.spec_seq@[j]
    }

    pub open spec fn view(&self) -> Seq<T> {
        self.spec_seq@
    }

    #[verifier::external_body]
    pub closed spec fn get_node_ref(&self, v: T) -> SLLIndex
        recommends
            self.wf(),
            self@.contains(v),
    {
        unimplemented!()
    }

    #[verifier::external_body]
    pub closed spec fn wf(&self) -> bool {
        unimplemented!()
    }
}


pub struct Container {
    pub parent: Option<ContainerPtr>,
    pub parent_rev_ptr: Option<SLLIndex>,
    pub children: StaticLinkedList<ContainerPtr, CONTAINER_CHILD_LIST_LEN>,
    pub depth: usize,
    pub uppertree_seq: Ghost<Seq<ContainerPtr>>,
    pub subtree_set: Ghost<Set<ContainerPtr>>,
    pub root_process: Option<ProcPtr>,
    pub owned_procs: StaticLinkedList<ProcPtr, CONTAINER_PROC_LIST_LEN>,
    pub owned_endpoints: Ghost<Set<EndpointPtr>>,
    pub owned_threads: Ghost<Set<ThreadPtr>>,
    pub quota: Quota,
    pub owned_cpus: ArraySet<NUM_CPUS>,
    pub scheduler: StaticLinkedList<ThreadPtr, MAX_CONTAINER_SCHEDULER_LEN>,
    pub can_have_children: bool,
}


pub struct Process {
    pub owning_container: ContainerPtr,
    pub rev_ptr: SLLIndex,
    pub pcid: Pcid,
    pub ioid: Option<IOid>,
    pub owned_threads: StaticLinkedList<ThreadPtr, MAX_NUM_THREADS_PER_PROC>,
    pub parent: Option<ProcPtr>,
    pub parent_rev_ptr: Option<SLLIndex>,
    pub children: StaticLinkedList<ProcPtr, PROC_CHILD_LIST_LEN>,
    pub uppertree_seq: Ghost<Seq<ProcPtr>>,
    pub subtree_set: Ghost<Set<ProcPtr>>,
    pub depth: usize,
    pub dmd_paging_mode: DemandPagingMode,
}


#[derive(Clone, Copy, Debug)]
pub struct Cpu {
    pub owning_container: ContainerPtr,
    pub active: bool,
    pub current_thread: Option<ThreadPtr>,
}


pub struct Endpoint {
    pub queue: StaticLinkedList<ThreadPtr, MAX_NUM_THREADS_PER_ENDPOINT>,
    pub queue_state: EndpointState,
    pub rf_counter: usize,
    pub owning_threads: Ghost<Set<(ThreadPtr, EndpointIdx)>>,
    pub owning_container: ContainerPtr,
}


pub struct Thread {
    pub owning_container: ContainerPtr,
    pub owning_proc: ProcPtr,
    pub state: ThreadState,
    pub proc_rev_ptr: SLLIndex,
    pub scheduler_rev_ptr: Option<SLLIndex>,
    pub blocking_endpoint_ptr: Option<EndpointPtr>,
    pub blocking_endpoint_index: Option<EndpointIdx>,
    pub endpoint_rev_ptr: Option<SLLIndex>,
    pub running_cpu: Option<CpuId>,
    pub endpoint_descriptors: Array<Option<EndpointPtr>, MAX_NUM_ENDPOINT_DESCRIPTORS>,
    pub ipc_payload: IPCPayLoad,
    pub error_code: Option<RetValueType>,
    pub trap_frame: TrapFrameOption,
}


#[allow(inconsistent_fields)]
#[derive(Clone, Copy)]
pub enum IPCPayLoad {
    Message { va: VAddr, len: usize },
    Pages { va_range: VaRange4K },
    Endpoint { endpoint_index: EndpointIdx },
    Pci { bus: u8, dev: u8, fun: u8 },
    PageFault { vaddr: VAddr },
    Empty,
}

impl IPCPayLoad {
    pub open spec fn spec_get_payload_as_va_range(&self) -> Option<VaRange4K> {
        match self {
            IPCPayLoad::Pages { va_range: va_range } => Some(*va_range),
            _ => None,
        }
    }

    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_get_payload_as_va_range))]
    pub fn get_payload_as_va_range(&self) -> (ret: Option<VaRange4K>)
        ensures
            ret == self.spec_get_payload_as_va_range(),
    {
        unimplemented!()
    }
}


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
    SuccessSeqUsize { value: Ghost<Seq<usize>> },
    SuccessPairUsize { value1: usize, value2: usize },
    SuccessThreeUsize { value1: usize, value2: usize, value3: usize },
    ErrorNoQuota,
    ErrorVaInUse,
    CpuIdle,
    Error,
    Else,
    NoQuota,
    VaInUse,
}

pub const MAX_NUM_ENDPOINT_DESCRIPTORS: usize = 128;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const NUM_CPUS: usize = 32;

#[derive(Clone, Copy, Debug)]
pub enum DemandPagingMode {
    NoDMDPG,
    DirectParentPrc,
    AllParentProc,
    AllParentContainer,
}


pub struct TrapFrameOption {
    pub reg: Registers,
    pub exists: bool,
}

#[derive(Clone, Copy, Debug)]
#[repr(C, align(8))]
pub struct Registers {
    pub r15: u64, pub r14: u64, pub r13: u64, pub r12: u64,
    pub rbp: u64, pub rbx: u64, pub r11: u64, pub r10: u64,
    pub r9: u64, pub r8: u64, pub rcx: u64, pub rdx: u64,
    pub rsi: u64, pub rdi: u64, pub rax: u64,
    pub error_code: u64, pub rip: u64, pub cs: u64,
    pub flags: u64, pub rsp: u64, pub ss: u64,
}


pub struct Array<A, const N: usize>{
    pub seq: Ghost<Seq<A>>,
    pub ar: [A;N]
}

impl<A, const N: usize> Array<A, N> {
    #[verifier(inline)]
    pub open spec fn spec_index(self, i: int) -> A
        recommends self.seq@.len() == N, 0 <= i < N,
    { self.seq@[i] }

    #[verifier(inline)]
    pub open spec fn view(&self) -> Seq<A>{ self.seq@ }

    pub open spec fn wf(&self) -> bool{ self.seq@.len() == N }
}


pub struct ArraySet<const N: usize> {
    pub data: Array<bool, N>,
    pub len: usize,
    pub set: Ghost<Set<usize>>,
}

impl <const N: usize> ArraySet<N> {
    #[verifier::external_body]
    pub closed spec fn view(&self) -> Set<usize>{ unimplemented!() }
    #[verifier::external_body]
    pub closed spec fn wf(&self) -> bool{ unimplemented!() }
}


#[derive(Clone, Copy)]
pub struct VaRange4K {
    pub start: VAddr,
    pub len: usize,
    pub view: Ghost<Seq<VAddr>>,
}

impl VaRange4K {
    #[verifier::external_body]
    pub closed spec fn view(&self) -> Seq<VAddr> { unimplemented!() }
    pub open spec fn wf(&self) -> bool {
        &&& self.start + self.len * 4096 < usize::MAX
        &&& spec_va_4k_valid(self.start)
        &&& self@.len() == self.len
        &&& self@.no_duplicates()
        &&& forall|i: int| #![trigger self@[i]] 0 <= i < self.len ==> spec_va_4k_valid(self@[i])
        &&& self.view_match_spec()
    }
    #[verifier::external_body]
    pub closed spec fn view_match_spec(&self) -> bool { unimplemented!() }
}


#[derive(Clone, Copy, Debug)]
pub struct Quota{
    pub mem_4k:usize, pub mem_2m:usize, pub mem_1g:usize,
    pub pcid:usize, pub ioid:usize,
}


pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}


// ---- Container tree specs ----

pub open spec fn container_perms_wf(
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool {
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].is_init()
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].addr() == c_ptr
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().children.wf()
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().children.unique()
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr)
            ==> container_perms[c_ptr].value().uppertree_seq@.no_duplicates()
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr) ==> container_perms[c_ptr].value().children@.contains(c_ptr) == false
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr)
            ==> container_perms[c_ptr].value().subtree_set@.finite()
    &&& forall|c_ptr: ContainerPtr|
        #![trigger container_perms.dom().contains(c_ptr)]
        container_perms.dom().contains(c_ptr)
            ==> container_perms[c_ptr].value().uppertree_seq@.len()
            == container_perms[c_ptr].value().depth
}


#[verifier::external_body]
pub closed spec fn container_root_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool { unimplemented!() }

#[verifier::external_body]
pub closed spec fn container_childern_parent_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool { unimplemented!() }

#[verifier::external_body]
pub closed spec fn containers_linkedlist_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool { unimplemented!() }

#[verifier::external_body]
pub closed spec fn container_childern_depth_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool { unimplemented!() }

#[verifier::external_body]
pub closed spec fn container_subtree_set_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool { unimplemented!() }

#[verifier::external_body]
pub closed spec fn container_uppertree_seq_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool { unimplemented!() }

#[verifier::external_body]
pub closed spec fn container_subtree_set_exclusive(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool { unimplemented!() }

pub open spec fn container_tree_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
) -> bool {
    &&& container_root_wf(root_container, container_perms)
    &&& container_childern_parent_wf(root_container, container_perms)
    &&& containers_linkedlist_wf(root_container, container_perms)
    &&& container_childern_depth_wf(root_container, container_perms)
    &&& container_subtree_set_wf(root_container, container_perms)
    &&& container_uppertree_seq_wf(root_container, container_perms)
    &&& container_subtree_set_exclusive(root_container, container_perms)
}


// ---- Target function spec (container_tree_check_is_ancestor) ----

#[verifier::external_body]
#[verifier::spinoff_prover]
pub fn container_tree_check_is_ancestor(
    root_container: ContainerPtr,
    container_perms: &Tracked<Map<ContainerPtr, PointsTo<Container>>>,
    a_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
) -> (ret: bool)
    requires
        container_perms_wf(container_perms@),
        container_tree_wf(root_container, container_perms@),
        container_perms@.dom().contains(a_ptr),
        container_perms@.dom().contains(child_ptr),
        container_perms@[a_ptr].value().depth < container_perms@[child_ptr].value().depth,
    ensures
        ret == container_perms@[child_ptr].value().uppertree_seq@.contains(a_ptr),
        ret == container_perms@[a_ptr].value().subtree_set@.contains(child_ptr),
{
    unimplemented!()
}


// ============================================================
// BOUNDARY TESTS: Violate preconditions
// Each test SHOULD FAIL verification.
// ============================================================

// SHOULD FAIL
// Test B1: ancestor_ptr not in domain — cannot conclude subtree membership
proof fn test_boundary_ancestor_not_in_domain(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    a_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        !container_perms.dom().contains(a_ptr),  // VIOLATES: a_ptr must be in domain
        container_perms.dom().contains(child_ptr),
    ensures
        container_perms[a_ptr].value().subtree_set@.contains(child_ptr) == false,
{
    // Cannot access container_perms[a_ptr] when a_ptr is not in domain
}

// SHOULD FAIL
// Test B2: child_ptr not in domain — cannot conclude uppertree membership
proof fn test_boundary_child_not_in_domain(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    a_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(a_ptr),
        !container_perms.dom().contains(child_ptr),  // VIOLATES: child_ptr must be in domain
    ensures
        container_perms[child_ptr].value().uppertree_seq@.contains(a_ptr) == false,
{
    // Cannot access container_perms[child_ptr] when child_ptr is not in domain
}

// SHOULD FAIL
// Test B3: depth equal (not strictly less) — try to assert the ensures of container_tree_check_is_ancestor
proof fn test_boundary_equal_depth_check_ancestor(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    a_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(a_ptr),
        container_perms.dom().contains(child_ptr),
        container_perms[a_ptr].value().depth == container_perms[child_ptr].value().depth,  // VIOLATES: must be strictly less
    ensures
        // Try to derive the postcondition consistency even with equal depth
        container_perms[child_ptr].value().uppertree_seq@.contains(a_ptr)
            == container_perms[a_ptr].value().subtree_set@.contains(child_ptr),
{
    // The postcondition of container_tree_check_is_ancestor only holds when depth < child's depth
}

// SHOULD FAIL
// Test B4: ancestor deeper than child — cannot use the function's ensures
proof fn test_boundary_ancestor_deeper_than_child(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    a_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(a_ptr),
        container_perms.dom().contains(child_ptr),
        container_perms[a_ptr].value().depth > container_perms[child_ptr].value().depth,  // VIOLATES: reversed
    ensures
        container_perms[a_ptr].value().subtree_set@.contains(child_ptr),
{
    // Deeper ancestor cannot be an ancestor of a shallower node
}

// SHOULD FAIL
// Test B5: no container_tree_wf — cannot derive ensures consistency
proof fn test_boundary_no_tree_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    a_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
)
    requires
        container_perms_wf(container_perms),
        // MISSING: container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(a_ptr),
        container_perms.dom().contains(child_ptr),
        container_perms[a_ptr].value().depth < container_perms[child_ptr].value().depth,
    ensures
        container_perms[child_ptr].value().uppertree_seq@.contains(a_ptr)
            == container_perms[a_ptr].value().subtree_set@.contains(child_ptr),
{
    // Without tree well-formedness, uppertree_seq and subtree_set may disagree
}

// SHOULD FAIL
// Test B6: no container_perms_wf — depth/uppertree_seq relation breaks
proof fn test_boundary_no_perms_wf(
    root_container: ContainerPtr,
    container_perms: Map<ContainerPtr, PointsTo<Container>>,
    a_ptr: ContainerPtr,
    child_ptr: ContainerPtr,
)
    requires
        // MISSING: container_perms_wf(container_perms),
        container_tree_wf(root_container, container_perms),
        container_perms.dom().contains(a_ptr),
        container_perms.dom().contains(child_ptr),
        container_perms[a_ptr].value().depth < container_perms[child_ptr].value().depth,
    ensures
        container_perms[a_ptr].value().uppertree_seq@.len() == container_perms[a_ptr].value().depth,
{
    // Without container_perms_wf, depth == uppertree_seq.len() is not guaranteed
}


}
