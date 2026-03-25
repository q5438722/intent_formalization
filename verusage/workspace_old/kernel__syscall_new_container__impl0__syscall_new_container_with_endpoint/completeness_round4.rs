use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus!{

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
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;
pub type SLLIndex = i32;
pub type PagePerm4k = PointsTo<[u8; PAGE_SZ_4k]>;
pub type PagePerm2m = PointsTo<[u8; PAGE_SZ_2m]>;
pub type PagePerm1g = PointsTo<[u8; PAGE_SZ_1g]>;

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
pub const PAGE_SZ_2m: usize = 1usize << 21;
pub const PAGE_SZ_1g: usize = 1usize << 30;

#[repr(align(4096))]
pub struct DeviceTable {
    ar: [usize; 512],
}

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
}

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
    { unimplemented!() }

    pub open spec fn unique(&self) -> bool {
        forall|i: int, j: int|
            #![trigger self.spec_seq@[i], self.spec_seq@[j]]
            0 <= i < self.len() && 0 <= j < self.len() && i != j ==> self.spec_seq@[i] != self.spec_seq@[j]
    }

    pub open spec fn view(&self) -> Seq<T> {
        self.spec_seq@
    }

    #[verifier::external_body]
    pub closed spec fn get_node_ref(&self, v: T) -> SLLIndex { unimplemented!() }

    #[verifier::external_body]
    pub closed spec fn wf(&self) -> bool { unimplemented!() }
}

pub struct PageMap {
    pub ar: Array<usize, 512>,
    pub spec_seq: Ghost<Seq<PageEntry>>,
}

impl PageMap {
    pub open spec fn spec_index(&self, index: usize) -> PageEntry
        recommends 0 <= index < 512,
    { self.spec_seq@[index as int] }
}

#[derive(Clone,Debug)]
pub struct PageEntryPerm {
    pub present: bool,
    pub ps: bool,
    pub write: bool,
    pub execute_disable: bool,
    pub user: bool,
}

#[derive(Clone,Debug)]
pub struct PageEntry {
    pub addr: PAddr,
    pub perm: PageEntryPerm,
}

impl PageEntry {
    pub open spec fn is_empty(&self) -> bool {
        &&& self.addr == 0
        &&& self.perm.present == false
        &&& self.perm.ps == false
        &&& self.perm.write == false
        &&& self.perm.execute_disable == false
        &&& self.perm.user == false
    }
}

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

pub open spec fn usize2present(v: usize) -> bool { (v & PAGE_ENTRY_PRESENT_MASK as usize) != 0 }
pub open spec fn usize2ps(v: usize) -> bool { (v & PAGE_ENTRY_PS_MASK as usize) != 0 }
pub open spec fn usize2write(v: usize) -> bool { (v & PAGE_ENTRY_WRITE_MASK as usize) != 0 }
pub open spec fn usize2execute_disable(v: usize) -> bool { (v & PAGE_ENTRY_EXECUTE_MASK as usize) != 0 }
pub open spec fn usize2user(v: usize) -> bool { (v & PAGE_ENTRY_USER_MASK as usize) != 0 }

pub open spec fn spec_usize2page_entry_perm(v: usize) -> PageEntryPerm {
    PageEntryPerm { present: usize2present(v), ps: usize2ps(v), write: usize2write(v), execute_disable: usize2execute_disable(v), user: usize2user(v) }
}
pub open spec fn spec_usize2page_entry(v: usize) -> PageEntry {
    PageEntry { addr: usize2pa(v), perm: usize2page_entry_perm(v) }
}
pub open spec fn spec_usize2pa(v: usize) -> PAddr { v & MEM_MASK as usize }

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_usize2page_entry_perm))]
pub fn usize2page_entry_perm(v: usize) -> (ret: PageEntryPerm)
    ensures ret =~= spec_usize2page_entry_perm(v),
{ unimplemented!() }

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_usize2page_entry))]
pub fn usize2page_entry(v: usize) -> (ret: PageEntry)
    ensures ret =~= spec_usize2page_entry(v),
{ unimplemented!() }

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_usize2pa))]
pub fn usize2pa(v: usize) -> (ret: PAddr)
    ensures ret =~= spec_usize2pa(v), MEM_valid(ret),
{ unimplemented!() }

pub struct PageTable {
    pub cr3: PageMapPtr,
    pub pcid: Option<Pcid>,
    pub ioid: Option<IOid>,
    pub kernel_l4_end: usize,
    pub l4_table: Tracked<Map<PageMapPtr, PointsTo<PageMap>>>,
    pub l3_rev_map: Ghost<Map<PageMapPtr, (L4Index)>>,
    pub l3_tables: Tracked<Map<PageMapPtr, PointsTo<PageMap>>>,
    pub l2_rev_map: Ghost<Map<PageMapPtr, (L4Index, L3Index)>>,
    pub l2_tables: Tracked<Map<PageMapPtr, PointsTo<PageMap>>>,
    pub l1_rev_map: Ghost<Map<PageMapPtr, (L4Index, L3Index, L2Index)>>,
    pub l1_tables: Tracked<Map<PageMapPtr, PointsTo<PageMap>>>,
    pub mapping_4k: Ghost<Map<VAddr, MapEntry>>,
    pub mapping_2m: Ghost<Map<VAddr, MapEntry>>,
    pub mapping_1g: Ghost<Map<VAddr, MapEntry>>,
    pub kernel_entries: Ghost<Seq<PageEntry>>,
    pub tlb_mapping_4k: Ghost<Seq<Map<VAddr, MapEntry>>>,
    pub tlb_mapping_2m: Ghost<Seq<Map<VAddr, MapEntry>>>,
    pub tlb_mapping_1g: Ghost<Seq<Map<VAddr, MapEntry>>>,
}

impl PageTable {
    pub open spec fn is_empty(&self) -> bool {
        &&& forall|i: L4Index|
            #![trigger self.l4_table@[self.cr3].value()[i].perm.present]
            self.kernel_l4_end <= i < 512 ==> self.l4_table@[self.cr3].value()[i].is_empty()
        &&& self.l3_tables@.dom() == Set::<PageMapPtr>::empty()
        &&& self.l2_tables@.dom() == Set::<PageMapPtr>::empty()
        &&& self.l1_tables@.dom() == Set::<PageMapPtr>::empty()
        &&& self.mapping_4k() == Map::<VAddr, MapEntry>::empty()
        &&& self.mapping_2m() == Map::<VAddr, MapEntry>::empty()
        &&& self.mapping_1g() == Map::<VAddr, MapEntry>::empty()
    }
    pub open spec fn page_closure(&self) -> Set<PagePtr> {
        self.l3_tables@.dom() + self.l2_tables@.dom() + self.l1_tables@.dom() + self.l4_table@.dom()
    }
    pub open spec fn mapping_4k(&self) -> Map<VAddr, MapEntry> { self.mapping_4k@ }
    pub open spec fn mapping_2m(&self) -> Map<VAddr, MapEntry> { self.mapping_2m@ }
    pub open spec fn mapping_1g(&self) -> Map<VAddr, MapEntry> { self.mapping_1g@ }
    #[verifier::external_body]
    pub closed spec fn wf(&self) -> bool { unimplemented!() }
}

#[derive(Clone, Copy)]
pub struct Page {
    pub addr: PagePtr,
    pub state: PageState,
    pub is_io_page: bool,
    pub rev_pointer: SLLIndex,
    pub ref_count: usize,
    pub owning_container: Option<ContainerPtr>,
    pub mappings: Ghost<Set<(Pcid, VAddr)>>,
    pub io_mappings: Ghost<Set<(IOid, VAddr)>>,
}

pub struct PageAllocator {
    pub page_array: Array<Page, NUM_PAGES>,
    pub free_pages_4k: StaticLinkedList<PagePtr, NUM_PAGES>,
    pub free_pages_2m: StaticLinkedList<PagePtr, NUM_PAGES>,
    pub free_pages_1g: StaticLinkedList<PagePtr, NUM_PAGES>,
    pub allocated_pages_4k: Ghost<Set<PagePtr>>,
    pub allocated_pages_2m: Ghost<Set<PagePtr>>,
    pub allocated_pages_1g: Ghost<Set<PagePtr>>,
    pub mapped_pages_4k: Ghost<Set<PagePtr>>,
    pub mapped_pages_2m: Ghost<Set<PagePtr>>,
    pub mapped_pages_1g: Ghost<Set<PagePtr>>,
    pub page_perms_4k: Tracked<Map<PagePtr, PagePerm4k>>,
    pub page_perms_2m: Tracked<Map<PagePtr, PagePerm2m>>,
    pub page_perms_1g: Tracked<Map<PagePtr, PagePerm1g>>,
    pub container_map_4k: Ghost<Map<ContainerPtr, Set<PagePtr>>>,
    pub container_map_2m: Ghost<Map<ContainerPtr, Set<PagePtr>>>,
    pub container_map_1g: Ghost<Map<ContainerPtr, Set<PagePtr>>>,
}

impl PageAllocator {
    pub open spec fn page_is_mapped(&self, p: PagePtr) -> bool {
        ||| self.mapped_pages_4k().contains(p)
        ||| self.mapped_pages_2m().contains(p)
        ||| self.mapped_pages_1g().contains(p)
    }
    #[verifier::external_body]
    pub closed spec fn free_pages_4k(&self) -> Set<PagePtr> { unimplemented!() }
    #[verifier::external_body]
    pub closed spec fn free_pages_2m(&self) -> Set<PagePtr> { unimplemented!() }
    #[verifier::external_body]
    pub closed spec fn free_pages_1g(&self) -> Set<PagePtr> { unimplemented!() }
    #[verifier::external_body]
    pub closed spec fn allocated_pages_4k(&self) -> Set<PagePtr> { unimplemented!() }
    #[verifier::external_body]
    pub closed spec fn allocated_pages_2m(&self) -> Set<PagePtr> { unimplemented!() }
    #[verifier::external_body]
    pub closed spec fn allocated_pages_1g(&self) -> Set<PagePtr> { unimplemented!() }
    #[verifier::external_body]
    pub closed spec fn mapped_pages_4k(&self) -> Set<PagePtr> { unimplemented!() }
    #[verifier::external_body]
    pub closed spec fn mapped_pages_2m(&self) -> Set<PagePtr> { unimplemented!() }
    #[verifier::external_body]
    pub closed spec fn mapped_pages_1g(&self) -> Set<PagePtr> { unimplemented!() }
    #[verifier::external_body]
    pub closed spec fn page_mappings(&self, p: PagePtr) -> Set<(Pcid, VAddr)> { unimplemented!() }
    #[verifier::external_body]
    pub closed spec fn page_io_mappings(&self, p: PagePtr) -> Set<(Pcid, VAddr)> { unimplemented!() }
    #[verifier::external_body]
    pub closed spec fn get_container_owned_pages(&self, c_ptr: ContainerPtr) -> Set<PagePtr> { unimplemented!() }
    pub open spec fn wf(&self) -> bool { true }
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

impl Endpoint {
    pub open spec fn rf_counter_is_full(&self) -> bool {
        self.rf_counter == usize::MAX
    }
}

pub struct ProcessManager {
    pub root_container: ContainerPtr,
    pub container_perms: Tracked<Map<ContainerPtr, PointsTo<Container>>>,
    pub process_perms: Tracked<Map<ProcPtr, PointsTo<Process>>>,
    pub thread_perms: Tracked<Map<ThreadPtr, PointsTo<Thread>>>,
    pub endpoint_perms: Tracked<Map<EndpointPtr, PointsTo<Endpoint>>>,
    pub cpu_list: Array<Cpu, NUM_CPUS>,
}

impl ProcessManager {
    #[verifier(inline)]
    pub open spec fn container_dom(&self) -> Set<ContainerPtr> { self.container_perms@.dom() }
    #[verifier(inline)]
    pub open spec fn proc_dom(&self) -> Set<ProcPtr> { self.process_perms@.dom() }
    #[verifier(inline)]
    pub open spec fn thread_dom(&self) -> Set<ThreadPtr> { self.thread_perms@.dom() }
    #[verifier(inline)]
    pub open spec fn endpoint_dom(&self) -> Set<EndpointPtr> { self.endpoint_perms@.dom() }

    #[verifier(inline)]
    pub open spec fn spec_get_container(&self, c_ptr: ContainerPtr) -> &Container {
        &self.container_perms@[c_ptr].value()
    }
    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_get_container))]
    pub fn get_container(&self, container_ptr: ContainerPtr) -> (ret: &Container)
        requires self.container_dom().contains(container_ptr),
        ensures self.get_container(container_ptr) == ret,
    { unimplemented!() }

    #[verifier(inline)]
    pub open spec fn spec_get_proc(&self, proc_ptr: ProcPtr) -> &Process {
        &self.process_perms@[proc_ptr].value()
    }
    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_get_proc))]
    pub fn get_proc(&self, proc_ptr: ProcPtr) -> (ret: &Process)
        requires self.proc_dom().contains(proc_ptr),
        ensures ret =~= self.get_proc(proc_ptr),
    { unimplemented!() }

    #[verifier(inline)]
    pub open spec fn spec_get_thread(&self, thread_ptr: ThreadPtr) -> &Thread {
        &self.thread_perms@[thread_ptr].value()
    }
    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_get_thread))]
    pub fn get_thread(&self, thread_ptr: ThreadPtr) -> (ret: &Thread)
        requires self.thread_dom().contains(thread_ptr),
        ensures ret == self.get_thread(thread_ptr),
    { unimplemented!() }

    #[verifier(inline)]
    pub open spec fn spec_get_endpoint(&self, endpoint_ptr: EndpointPtr) -> &Endpoint {
        &self.endpoint_perms@[endpoint_ptr].value()
    }
    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_get_endpoint))]
    pub fn get_endpoint(&self, endpoint_ptr: EndpointPtr) -> (ret: &Endpoint)
        requires self.endpoint_dom().contains(endpoint_ptr),
        ensures ret == self.get_endpoint(endpoint_ptr),
    { unimplemented!() }

    pub open spec fn spec_get_endpoint_ptr_by_endpoint_idx(
        &self, thread_ptr: ThreadPtr, endpoint_index: EndpointIdx,
    ) -> Option<EndpointPtr> {
        self.get_thread(thread_ptr).endpoint_descriptors@[endpoint_index as int]
    }
    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_get_endpoint_ptr_by_endpoint_idx))]
    pub fn get_endpoint_ptr_by_endpoint_idx(
        &self, thread_ptr: ThreadPtr, endpoint_index: EndpointIdx,
    ) -> (ret: Option<EndpointPtr>)
        requires self.thread_dom().contains(thread_ptr),
                 0 <= endpoint_index < MAX_NUM_ENDPOINT_DESCRIPTORS,
        ensures ret == self.get_endpoint_ptr_by_endpoint_idx(thread_ptr, endpoint_index),
                ret.is_Some() ==> self.endpoint_dom().contains(ret.unwrap()),
    { unimplemented!() }

    pub open spec fn page_closure(&self) -> Set<PagePtr> {
        self.container_perms@.dom() + self.process_perms@.dom() + self.thread_perms@.dom()
            + self.endpoint_perms@.dom()
    }
    #[verifier::external_body]
    pub closed spec fn wf(&self) -> bool { unimplemented!() }
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
            IPCPayLoad::Pages { va_range } => Some(*va_range),
            _ => None,
        }
    }
    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_get_payload_as_va_range))]
    pub fn get_payload_as_va_range(&self) -> (ret: Option<VaRange4K>)
        ensures ret == self.spec_get_payload_as_va_range(),
    { unimplemented!() }
}

pub struct PCIBitMap {
    pub bit_map: [[[u8; 32]; 256]; IOID_MAX],
    pub ghost_map: Ghost<Map<(IOid, u8, u8, u8), bool>>,
}
impl PCIBitMap {
    pub open spec fn wf(&self) -> bool {
        forall|ioid: IOid, bus: u8, dev: u8, fun: u8|
            #![auto]
            0 <= ioid < IOID_MAX && 0 <= bus < 256 && 0 <= dev < 32 && 0 <= fun < 8
                <==> self.ghost_map@.dom().contains((ioid, bus, dev, fun))
    }
}

#[repr(align(4096))]
pub struct RootTable {
    root: [usize; 512],
    seq_ar: Ghost<Seq<Seq<Seq<Option<(IOid, usize)>>>>>,
    deviecs: [DeviceTable; 256],
}
impl RootTable {
    #[verifier::external_body]
    pub closed spec fn wf(&self) -> bool { unimplemented!() }
    #[verifier::external_body]
    pub closed spec fn resolve(&self, bus: u8, dev: u8, fun: u8) -> Option<(IOid, usize)> { unimplemented!() }
}

pub struct MemoryManager {
    pub kernel_entries: Array<usize, KERNEL_MEM_END_L4INDEX>,
    pub kernel_entries_ghost: Ghost<Seq<PageEntry>>,
    pub free_pcids: ArrayVec<Pcid, PCID_MAX>,
    pub pcid_to_proc_ptr: Array<Option<ProcPtr>, PCID_MAX>,
    pub page_tables: Array<Option<PageTable>, PCID_MAX>,
    pub page_table_pages: Ghost<Map<PagePtr, Pcid>>,
    pub free_ioids: ArrayVec<IOid, IOID_MAX>,
    pub ioid_to_proc_ptr: Array<Option<ProcPtr>, IOID_MAX>,
    pub iommu_tables: Array<Option<PageTable>, IOID_MAX>,
    pub iommu_table_pages: Ghost<Map<PagePtr, IOid>>,
    pub root_table: RootTable,
    pub root_table_cache: Ghost<Seq<Seq<Seq<Option<(IOid, usize)>>>>>,
    pub pci_bitmap: PCIBitMap,
}

impl MemoryManager {
    pub open spec fn pcid_to_proc_ptr(&self, pcid: Pcid) -> ProcPtr {
        self.pcid_to_proc_ptr@[pcid as int].unwrap()
    }
    pub open spec fn pcid_active(&self, pcid: Pcid) -> bool {
        &&& 0 <= pcid < PCID_MAX
        &&& self.get_free_pcids_as_set().contains(pcid) == false
    }
    pub open spec fn get_free_pcids_as_set(&self) -> Set<IOid> { self.free_pcids@.to_set() }
    pub open spec fn get_free_ioids_as_set(&self) -> Set<IOid> { self.free_ioids@.to_set() }
    pub open spec fn get_pagetable_mapping_by_pcid(&self, pcid: Pcid) -> Map<VAddr, MapEntry> {
        self.page_tables@[pcid as int].unwrap().mapping_4k()
    }
    pub open spec fn page_closure(&self) -> Set<PagePtr> {
        self.iommu_table_pages@.dom() + self.page_table_pages@.dom()
    }
    #[verifier::external_body]
    pub closed spec fn wf(&self) -> bool { unimplemented!() }
}

pub struct Kernel {
    pub page_alloc: PageAllocator,
    pub mem_man: MemoryManager,
    pub proc_man: ProcessManager,
    pub page_mapping: Ghost<Map<PagePtr, Set<(ProcPtr, VAddr)>>>,
    pub page_io_mapping: Ghost<Map<PagePtr, Set<(ProcPtr, VAddr)>>>,
}

impl Kernel {
    pub open spec fn wf(&self) -> bool {
        &&& self.mem_man.wf()
        &&& self.page_alloc.wf()
        &&& self.proc_man.wf()
    }
    pub open spec fn proc_dom(&self) -> Set<ProcPtr> { self.proc_man.proc_dom() }
    pub open spec fn thread_dom(&self) -> Set<ThreadPtr> { self.proc_man.thread_dom() }
    pub open spec fn container_dom(&self) -> Set<ContainerPtr> { self.proc_man.container_dom() }
    pub open spec fn endpoint_dom(&self) -> Set<EndpointPtr> { self.proc_man.endpoint_dom() }
    pub open spec fn get_proc(&self, p_ptr: ProcPtr) -> &Process { self.proc_man.get_proc(p_ptr) }
    pub open spec fn get_thread(&self, t_ptr: ThreadPtr) -> &Thread { self.proc_man.get_thread(t_ptr) }
    pub open spec fn get_container(&self, c_ptr: ContainerPtr) -> &Container { self.proc_man.get_container(c_ptr) }
    pub open spec fn get_container_quota(&self, c_ptr: ContainerPtr) -> Quota { self.proc_man.get_container(c_ptr).quota }
    pub open spec fn get_endpoint(&self, e_ptr: EndpointPtr) -> &Endpoint { self.proc_man.get_endpoint(e_ptr) }
    pub open spec fn get_address_space(&self, p_ptr: ProcPtr) -> Map<VAddr, MapEntry> {
        self.mem_man.get_pagetable_mapping_by_pcid(self.get_proc(p_ptr).pcid)
    }
    pub open spec fn get_container_owned_pages(&self, c_ptr: ContainerPtr) -> Set<PagePtr> {
        self.page_alloc.get_container_owned_pages(c_ptr)
    }
    pub open spec fn get_physical_page_mapping(&self) -> Map<PagePtr, Set<(ProcPtr, VAddr)>> {
        self.page_mapping@
    }
    pub open spec fn get_is_process_thread_list_full(&self, p_ptr: ProcPtr) -> bool {
        self.get_proc(p_ptr).owned_threads.len() >= MAX_NUM_THREADS_PER_PROC
    }
    pub open spec fn get_is_children_list_full(&self, c_ptr: ContainerPtr) -> bool {
        self.get_container(c_ptr).children.len() >= CONTAINER_CHILD_LIST_LEN
    }
    pub open spec fn get_num_of_free_pages(&self) -> usize {
        self.page_alloc.free_pages_4k.len()
    }
    pub open spec fn get_is_pcid_exhausted(&self) -> bool {
        self.mem_man.free_pcids.len() == 0
    }
    pub open spec fn get_endpoint_ptr_by_endpoint_idx(
        &self, t_ptr: ThreadPtr, endpoint_index: EndpointIdx,
    ) -> Option<EndpointPtr> {
        self.proc_man.get_thread(t_ptr).endpoint_descriptors@[endpoint_index as int]
    }
    pub open spec fn get_endpoint_shareable(
        &self, t_ptr: ThreadPtr, endpoint_index: EndpointIdx,
    ) -> bool {
        &&& self.get_endpoint_ptr_by_endpoint_idx(t_ptr, endpoint_index).is_Some()
        &&& self.get_endpoint(
            self.get_endpoint_ptr_by_endpoint_idx(t_ptr, endpoint_index).unwrap(),
        ).rf_counter != usize::MAX
    }
    pub open spec fn get_physical_page_reference_counter(&self, page_ptr: PagePtr) -> nat {
        self.page_alloc.page_mappings(page_ptr).len() + self.page_alloc.page_io_mappings(page_ptr).len()
    }
    pub open spec fn address_space_range_shareable(
        &self, target_proc_ptr: ProcPtr, va_range: &VaRange4K,
    ) -> bool {
        &&& forall|j: int| #![auto] 0 <= j < va_range.len ==> self.get_address_space(target_proc_ptr).dom().contains(va_range@[j])
        &&& forall|j: int| #![auto] 0 <= j < va_range.len ==> self.get_physical_page_reference_counter(
                self.get_address_space(target_proc_ptr)[va_range@[j]].addr,
            ) <= usize::MAX - va_range.len
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThreadState { SCHEDULED, BLOCKED, RUNNING }

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EndpointState { RECEIVE, SEND }

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(inconsistent_fields)]
pub enum PageState {
    Unavailable4k, Unavailable2m, Unavailable1g, Pagetable,
    Allocated4k, Allocated2m, Allocated1g,
    Free4k, Free2m, Free1g,
    Mapped4k, Mapped2m, Mapped1g,
    Merged2m, Merged1g, Io,
}

#[allow(inconsistent_fields)]
#[derive(Clone, Copy)]
pub enum RetValueType {
    SuccessUsize { value: usize },
    SuccessSeqUsize { value: Ghost<Seq<usize>> },
    SuccessPairUsize { value1: usize, value2: usize },
    SuccessThreeUsize { value1: usize, value2: usize, value3: usize },
    ErrorNoQuota, ErrorVaInUse, CpuIdle, Error, Else, NoQuota, VaInUse,
}

pub const KERNEL_MEM_END_L4INDEX: usize = 1;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const PCID_MAX: usize = 4096;
pub const IOID_MAX: usize = 4096;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;
pub const PAGE_ENTRY_USER_SHIFT: u64 = 2;
pub const PAGE_ENTRY_PS_SHIFT: u64 = 7;
pub const PAGE_ENTRY_EXECUTE_SHIFT: u64 = 63;
pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;
pub const PAGE_ENTRY_USER_MASK: u64 = 0x1u64 << PAGE_ENTRY_USER_SHIFT;
pub const PAGE_ENTRY_PS_MASK: u64 = 0x1u64 << PAGE_ENTRY_PS_SHIFT;
pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x1u64 << PAGE_ENTRY_EXECUTE_SHIFT;

#[derive(Clone, Copy, Debug)]
pub enum DemandPagingMode { NoDMDPG, DirectParentPrc, AllParentProc, AllParentContainer }

#[derive(Clone, Copy, Debug)]
pub enum SwitchDecision { NoSwitch, NoThread, Switch }

#[derive(Clone, Copy)]
pub struct SyscallReturnStruct {
    pub error_code: RetValueType,
    pub pcid: Option<Pcid>,
    pub cr3: Option<usize>,
    pub switch_decision: SwitchDecision,
}

impl SyscallReturnStruct {
    pub open spec fn get_return_vaule_three_usize(&self) -> Option<(usize, usize, usize)> {
        match self.error_code {
            RetValueType::SuccessThreeUsize { value1, value2, value3 } => Some((value1, value2, value3)),
            _ => None,
        }
    }
    pub open spec fn spec_is_error(&self) -> bool {
        match self.error_code {
            RetValueType::Error => true,
            _ => false,
        }
    }
    #[verifier(when_used_as_spec(spec_is_error))]
    pub fn is_error(&self) -> (ret: bool)
        ensures ret == self.is_error(),
    {
        match self.error_code {
            RetValueType::Error => true,
            _ => false,
        }
    }
    #[verifier::external_body]
    pub fn NoSwitchNew(error_code: RetValueType) -> (ret: Self)
        ensures ret.error_code == error_code,
                ret.pcid.is_None(),
                ret.cr3.is_None(),
                ret.switch_decision == SwitchDecision::NoSwitch,
    { unimplemented!() }
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
    pub open spec fn spec_index(self, i: int) -> A { self.seq@[i] }
    #[verifier(inline)]
    pub open spec fn view(&self) -> Seq<A> { self.seq@ }
    pub open spec fn wf(&self) -> bool { self.seq@.len() == N }
}

pub struct ArraySet<const N: usize> {
    pub data: Array<bool, N>,
    pub len: usize,
    pub set: Ghost<Set<usize>>,
}
impl <const N: usize> ArraySet<N> {
    #[verifier::external_body]
    pub closed spec fn view(&self) -> Set<usize> { unimplemented!() }
    #[verifier::external_body]
    pub closed spec fn wf(&self) -> bool { unimplemented!() }
}

pub struct ArrayVec<T, const N: usize> {
    pub data: Array<T, N>,
    pub len: usize,
}
impl<T: Copy, const N: usize> ArrayVec<T, N> {
    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_len))]
    pub fn len(&self) -> (ret: usize)
        requires self.wf(),
        ensures ret == self.spec_len(),
    { unimplemented!() }
    pub open spec fn spec_len(&self) -> usize { self.len }
    pub open spec fn spec_capacity(&self) -> usize { N }
    pub open spec fn view(&self) -> Seq<T> { self.view_until(self.len() as nat) }
    pub open spec fn view_until(&self, len: nat) -> Seq<T> { self.data@.subrange(0,len as int) }
    pub open spec fn wf(&self) -> bool {
        &&& 0 <= N <= usize::MAX
        &&& self.len() <= N
        &&& self.data.wf()
    }
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
    pub open spec fn spec_greater(&self, new: &Quota) -> bool {
        &&& self.mem_4k >= new.mem_4k
        &&& self.mem_2m >= new.mem_2m
        &&& self.mem_1g >= new.mem_1g
        &&& self.pcid >= new.pcid
        &&& self.ioid >= new.ioid
    }
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64) >= KERNEL_MEM_END_L4INDEX as u64
}

// ===== Specs under test =====

pub open spec fn syscall_new_container_with_endpoint_requirement(
    old: Kernel,
    thread_ptr: ThreadPtr,
    endpoint_index: EndpointIdx,
    pt_regs: Registers,
    va_range: VaRange4K,
    init_quota: Quota,
) -> bool {
    let proc_ptr = old.get_thread(thread_ptr).owning_proc;
    let pcid = old.get_proc(proc_ptr).pcid;
    let container_ptr = old.get_thread(thread_ptr).owning_container;
    if old.get_is_process_thread_list_full(proc_ptr) {
        false
    } else if old.get_container_quota(container_ptr).mem_4k < 3 + init_quota.mem_4k {
        false
    } else if old.get_container_quota(container_ptr).mem_2m < init_quota.mem_2m {
        false
    } else if old.get_container_quota(container_ptr).mem_1g < init_quota.mem_1g {
        false
    } else if old.get_container_quota(container_ptr).pcid < 1 + init_quota.pcid {
        false
    } else if old.get_container_quota(container_ptr).ioid < init_quota.ioid {
        false
    } else if old.get_container(container_ptr).depth == usize::MAX {
        false
    } else if old.get_num_of_free_pages() < 3 + init_quota.mem_4k {
        false
    } else if old.get_is_pcid_exhausted() {
        false
    } else if old.get_endpoint_shareable(thread_ptr, endpoint_index) == false {
        false
    } else if old.address_space_range_shareable(proc_ptr, &va_range) == false {
        false
    } else if old.get_is_children_list_full(container_ptr) {
        false
    } else if init_quota.mem_4k < 3 * va_range.len {
        false
    } else {
        true
    }
}

pub open spec fn syscall_new_container_with_endpoint_spec(
    old: Kernel,
    new: Kernel,
    thread_ptr: ThreadPtr,
    endpoint_index: EndpointIdx,
    pt_regs: Registers,
    va_range: VaRange4K,
    init_quota: Quota,
    ret: SyscallReturnStruct,
) -> bool {
    let proc_ptr = old.get_thread(thread_ptr).owning_proc;
    let pcid = old.get_proc(proc_ptr).pcid;
    let container_ptr = old.get_thread(thread_ptr).owning_container;
    let endpoint_ptr = old.get_endpoint_ptr_by_endpoint_idx(thread_ptr, endpoint_index).unwrap();
    let (new_container_ptr, new_proc_ptr, new_thread_ptr) =
        ret.get_return_vaule_three_usize().unwrap();

    &&& syscall_new_container_with_endpoint_requirement(old, thread_ptr, endpoint_index, pt_regs, va_range, init_quota) == false ==> old == new
    &&& syscall_new_container_with_endpoint_requirement(old, thread_ptr, endpoint_index, pt_regs, va_range, init_quota) ==>
        old.endpoint_dom() =~= new.endpoint_dom()
        && forall|t_ptr: ThreadPtr|
            #![trigger new.get_thread(t_ptr)]
            old.thread_dom().contains(t_ptr) ==> new.get_thread(t_ptr) =~= old.get_thread(t_ptr)
        && forall|p_ptr: ProcPtr|
            #![trigger new.get_proc(p_ptr)]
            old.proc_dom().contains(p_ptr) ==> new.get_proc(p_ptr) =~= old.get_proc(p_ptr)
        && forall|e_ptr: EndpointPtr|
            #![trigger new.get_endpoint(e_ptr)]
            new.endpoint_dom().contains(e_ptr) && e_ptr != endpoint_ptr
            ==> old.get_endpoint(e_ptr) =~= new.get_endpoint(e_ptr)
        && forall|c: ContainerPtr|
            #![trigger new.get_container_owned_pages(c)]
            old.container_dom().contains(c) ==> old.get_container_owned_pages(c) =~= new.get_container_owned_pages(c)
        && forall|p_ptr: ProcPtr|
            #![trigger new.get_address_space(p_ptr)]
            old.proc_dom().contains(p_ptr) ==> new.get_address_space(p_ptr) =~= old.get_address_space(p_ptr)
        && new.get_container(container_ptr).owned_endpoints@ =~= old.get_container(container_ptr).owned_endpoints@
        && new.get_physical_page_mapping().dom() =~= old.get_physical_page_mapping().dom()
        && old.container_dom().insert(new_container_ptr) =~= new.container_dom()
        && old.proc_dom().insert(new_proc_ptr) =~= new.proc_dom()
        && old.thread_dom().insert(new_thread_ptr) =~= new.thread_dom()
        && new.get_proc(new_proc_ptr).owned_threads@ =~= Seq::<ThreadPtr>::empty().push(new_thread_ptr)
        && new.get_proc(new_proc_ptr).owning_container == new_container_ptr
        && new.get_container(container_ptr).owned_threads@ =~= old.get_container(container_ptr).owned_threads@
        && new.get_container(container_ptr).owned_procs@ =~= old.get_container(container_ptr).owned_procs@
        && new.get_container(container_ptr).children@ =~= old.get_container(container_ptr).children@.push(new_container_ptr)
        && new.get_container(new_container_ptr).owned_threads@ == Set::<ThreadPtr>::empty().insert(new_thread_ptr)
        && new.get_container(new_container_ptr).owned_procs@ == Seq::<ProcPtr>::empty().push(new_proc_ptr)
        && new.get_container(new_container_ptr).children@ == Seq::<ContainerPtr>::empty()
        && new.get_thread(new_thread_ptr).owning_container == new_container_ptr
        && new.get_thread(new_thread_ptr).endpoint_descriptors@ =~= Seq::new(
            MAX_NUM_ENDPOINT_DESCRIPTORS as nat, |i: int| { None },
        ).update(0, Some(endpoint_ptr))
        && new.get_endpoint(endpoint_ptr).owning_threads@ =~= old.get_endpoint(endpoint_ptr).owning_threads@.insert((new_thread_ptr, 0))
        && new.get_container_owned_pages(new_container_ptr) == Set::<PagePtr>::empty()
}

// ===== COMPLETENESS ROUND 4: Wrong Specific Values (should all FAIL) =====

// Test 1: Assert new thread's owning_container is the OLD container (wrong — should be new)
proof fn test_wrong_new_thread_container(
    old_k: Kernel, new_k: Kernel, thread_ptr: ThreadPtr,
    endpoint_index: EndpointIdx, pt_regs: Registers,
    va_range: VaRange4K, init_quota: Quota, ret: SyscallReturnStruct,
)
    requires
        old_k.thread_dom().contains(thread_ptr),
        old_k.proc_dom().contains(old_k.get_thread(thread_ptr).owning_proc),
        old_k.container_dom().contains(old_k.get_thread(thread_ptr).owning_container),
        syscall_new_container_with_endpoint_requirement(old_k, thread_ptr, endpoint_index, pt_regs, va_range, init_quota),
        syscall_new_container_with_endpoint_spec(old_k, new_k, thread_ptr, endpoint_index, pt_regs, va_range, init_quota, ret),
        ret.get_return_vaule_three_usize().is_Some(),
{
    let container_ptr = old_k.get_thread(thread_ptr).owning_container;
    let (nc, _, nt) = ret.get_return_vaule_three_usize().unwrap();
    // Should fail: new thread belongs to new container, not old
    assert(new_k.get_thread(nt).owning_container == container_ptr);
}

// Test 2: Assert new proc's owning_container is the old container (wrong)
proof fn test_wrong_new_proc_container(
    old_k: Kernel, new_k: Kernel, thread_ptr: ThreadPtr,
    endpoint_index: EndpointIdx, pt_regs: Registers,
    va_range: VaRange4K, init_quota: Quota, ret: SyscallReturnStruct,
)
    requires
        old_k.thread_dom().contains(thread_ptr),
        old_k.proc_dom().contains(old_k.get_thread(thread_ptr).owning_proc),
        old_k.container_dom().contains(old_k.get_thread(thread_ptr).owning_container),
        syscall_new_container_with_endpoint_requirement(old_k, thread_ptr, endpoint_index, pt_regs, va_range, init_quota),
        syscall_new_container_with_endpoint_spec(old_k, new_k, thread_ptr, endpoint_index, pt_regs, va_range, init_quota, ret),
        ret.get_return_vaule_three_usize().is_Some(),
{
    let container_ptr = old_k.get_thread(thread_ptr).owning_container;
    let (nc, np, _) = ret.get_return_vaule_three_usize().unwrap();
    // Should fail: new proc belongs to new container, not old
    assert(new_k.get_proc(np).owning_container == container_ptr);
}

// Test 3: Assert new container has 2 procs (wrong — has exactly 1)
proof fn test_wrong_new_container_two_procs(
    old_k: Kernel, new_k: Kernel, thread_ptr: ThreadPtr,
    endpoint_index: EndpointIdx, pt_regs: Registers,
    va_range: VaRange4K, init_quota: Quota, ret: SyscallReturnStruct,
)
    requires
        old_k.thread_dom().contains(thread_ptr),
        old_k.proc_dom().contains(old_k.get_thread(thread_ptr).owning_proc),
        old_k.container_dom().contains(old_k.get_thread(thread_ptr).owning_container),
        syscall_new_container_with_endpoint_requirement(old_k, thread_ptr, endpoint_index, pt_regs, va_range, init_quota),
        syscall_new_container_with_endpoint_spec(old_k, new_k, thread_ptr, endpoint_index, pt_regs, va_range, init_quota, ret),
        ret.get_return_vaule_three_usize().is_Some(),
{
    let (nc, np, nt) = ret.get_return_vaule_three_usize().unwrap();
    // Should fail: owned_procs has exactly 1 proc, not 2
    assert(new_k.get_container(nc).owned_procs@.len() == 2);
}

// Test 4: Assert that Error is NOT an error (wrong)
proof fn test_wrong_error_not_error(ret: SyscallReturnStruct)
    requires ret.error_code == RetValueType::Error,
{
    // Should fail: Error IS an error
    assert(!ret.is_error());
}

} // end verus!
