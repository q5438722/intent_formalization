use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus!{

pub type IOid = usize;
pub type Pcid = usize;
pub type ProcPtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type PageMapPtr = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;

type PagePtr = usize;

#[repr(align(4096))]
pub struct DeviceTable {
    ar: [usize; 512],
}


pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
}

pub struct PageMap {
    pub ar: Array<usize, 512>,
    pub spec_seq: Ghost<Seq<PageEntry>>,
}

impl PageMap {

    pub open spec fn wf(&self) -> bool {
        &&& self.ar.wf()
        &&& self.spec_seq@.len() == 512
        &&& forall|i: int|
            #![trigger usize2page_entry(self.ar@[i])]
            0 <= i < 512 ==> (usize2page_entry(self.ar@[i])
                =~= self.spec_seq@[i])
    }

    pub open spec fn spec_index(&self, index: usize) -> PageEntry
        recommends
            0 <= index < 512,
    {
        self.spec_seq@[index as int]
    }

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

pub open spec fn usize2present(v: usize) -> bool {
    (v & PAGE_ENTRY_PRESENT_MASK as usize) != 0
}

pub open spec fn usize2ps(v: usize) -> bool {
    (v & PAGE_ENTRY_PS_MASK as usize) != 0
}

pub open spec fn usize2write(v: usize) -> bool {
    (v & PAGE_ENTRY_WRITE_MASK as usize) != 0
}

pub open spec fn usize2execute_disable(v: usize) -> bool {
    (v & PAGE_ENTRY_EXECUTE_MASK as usize) != 0
}

pub open spec fn usize2user(v: usize) -> bool {
    (v & PAGE_ENTRY_USER_MASK as usize) != 0
}

pub open spec fn spec_usize2page_entry_perm(v: usize) -> PageEntryPerm {
    PageEntryPerm {
        present: usize2present(v),
        ps: usize2ps(v),
        write: usize2write(v),
        execute_disable: usize2execute_disable(v),
        user: usize2user(v),
    }
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_usize2page_entry_perm))]
pub fn usize2page_entry_perm(v: usize) -> (ret: PageEntryPerm)
    ensures
        ret =~= spec_usize2page_entry_perm(v),
        v == 0 ==> ret.present == false && ret.ps == false && ret.write == false
            && ret.execute_disable == false && ret.user == false,
{
    unimplemented!()
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_usize2page_entry))]
pub fn usize2page_entry(v: usize) -> (ret: PageEntry)
    ensures
        ret =~= spec_usize2page_entry(v),
        v == 0 ==> ret.addr == 0 && ret.perm.present == false && ret.perm.ps == false
            && ret.perm.write == false && ret.perm.execute_disable == false && ret.perm.user
            == false,
{
    unimplemented!()
}


pub open spec fn spec_usize2page_entry(v: usize) -> PageEntry {
    PageEntry { addr: usize2pa(v), perm: usize2page_entry_perm(v) }
}

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_usize2pa))]
pub fn usize2pa(v: usize) -> (ret: PAddr)
    ensures
        ret =~= spec_usize2pa(v),
        MEM_valid(ret),
{
    unimplemented!()
}


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

    pub open   spec fn page_closure(&self) -> Set<PagePtr> {
        self.l3_tables@.dom() + self.l2_tables@.dom() + self.l1_tables@.dom() + self.l4_table@.dom()
    }

    pub open   spec fn mapping_4k(&self) -> Map<VAddr, MapEntry> {
        self.mapping_4k@
    }

    pub open   spec fn mapping_2m(&self) -> Map<VAddr, MapEntry> {
        self.mapping_2m@
    }

    pub open   spec fn mapping_1g(&self) -> Map<VAddr, MapEntry> {
        self.mapping_1g@
    }

    pub open   spec fn page_not_mapped(&self, pa: PAddr) -> bool {
        &&& forall 
            |va: VAddr|
            #![trigger self.mapping_4k().dom().contains(va), self.mapping_4k()[va].addr]
                self.mapping_4k().dom().contains(va) ==> self.mapping_4k()[va].addr != pa
        &&& forall 
            |va: VAddr|
            #![trigger self.mapping_2m().dom().contains(va), self.mapping_2m()[va].addr]
                self.mapping_2m().dom().contains(va) ==> self.mapping_2m()[va].addr != pa
        &&& forall 
            |va: VAddr|
            #![trigger self.mapping_1g().dom().contains(va), self.mapping_1g()[va].addr]
                self.mapping_1g().dom().contains(va) ==> self.mapping_1g()[va].addr != pa
    }

    pub open   spec fn spec_resolve_mapping_l4(&self, l4i: L4Index) -> Option<PageEntry>
        recommends
            self.kernel_l4_end <= l4i < 512,
    {
        if self.l4_table@[self.cr3].value()[l4i].perm.present || l4i < self.kernel_l4_end {
            Some(self.l4_table@[self.cr3].value()[l4i])
        } else {
            None
        }
    }

	#[verifier::external_body]
    pub closed   spec fn wf(&self) -> bool {
		unimplemented!()
	}

}


pub struct Array<A, const N: usize>{
    pub seq: Ghost<Seq<A>>,
    pub ar: [A;N]
}

impl<A, const N: usize> Array<A, N> {

    #[verifier(inline)]
    pub open spec fn spec_index(self, i: int) -> A
        recommends self.seq@.len() == N,
                   0 <= i < N,
    {
        self.seq@[i]
    }

    #[verifier(inline)]
    pub open spec fn view(&self) -> Seq<A>{
        self.seq@
    }

    pub open spec fn wf(&self) -> bool{
        self.seq@.len() == N
    }

}


pub struct ArrayVec<T, const N: usize> {
    pub data: Array<T, N>,
    pub len: usize,
}

impl<T: Copy, const N: usize> ArrayVec<T, N> {

    pub open spec fn spec_len(&self) -> usize {
        self.len
    }

    pub open spec fn spec_capacity(&self) -> usize {
        N
    }

    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_len))]
    pub fn len(&self) -> (ret: usize)
        requires
            self.wf(),
        ensures
            ret == self.spec_len(),
    {
        unimplemented!()
    }

    #[verifier::external_body]
    #[verifier(when_used_as_spec(spec_capacity))]
    pub const fn capacity(&self) -> (ret: usize)
        ensures
            ret == self.spec_capacity(),
    {
        unimplemented!()
    }

    pub open spec fn view(&self) -> Seq<T>
        recommends self.wf(),
    {
        self.view_until(self.len() as nat)
    }

    pub open spec fn view_until(&self, len: nat) -> Seq<T>
        recommends
            0 <= len <= self.len() as nat,
    {
        self.data@.subrange(0,len as int)
    }

    pub open spec fn wf(&self) -> bool {
        &&& 0 <= N <= usize::MAX
        &&& self.len() <= self.capacity()
        &&& self.data.wf()
    }

}


pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}


pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub const PCID_MAX: usize = 4096;

pub const IOID_MAX: usize = 4096;

pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;

pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;

pub const PAGE_ENTRY_USER_SHIFT: u64 = 2;

pub const PAGE_ENTRY_PS_SHIFT: u64 = 7;

pub const PAGE_ENTRY_EXECUTE_SHIFT: u64 = 63;

pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;

pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;

pub const PAGE_ENTRY_USER_MASK: u64 = 0x1u64 << PAGE_ENTRY_USER_SHIFT;

pub const PAGE_ENTRY_PS_MASK: u64 = 0x1u64 << PAGE_ENTRY_PS_SHIFT;

pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x1u64 << PAGE_ENTRY_EXECUTE_SHIFT;


// =============================================================================
// LOGICAL TESTS
// =============================================================================

// Test 1: page_ptr_valid does NOT uniquely identify a single pointer.
// Many valid pointers exist. Asserting two valid pointers must be equal should fail.
// SHOULD FAIL
proof fn test_logical_valid_ptrs_not_unique(p1: usize, p2: usize)
    requires
        page_ptr_valid(p1),
        page_ptr_valid(p2),
{
    assert(p1 == p2);
}

// Test 2: MEM_valid does NOT imply the value is zero.
// Many non-zero values satisfy MEM_valid.
// SHOULD FAIL
proof fn test_logical_mem_valid_not_zero(v: PAddr)
    requires
        MEM_valid(v),
{
    assert(v == 0usize);
}

// Test 3: spec_resolve_mapping_l4 for two different valid indices are NOT forced to agree.
// Different L4 entries are independent.
// SHOULD FAIL
proof fn test_logical_different_l4_indices_independent(pt: PageTable, l4a: L4Index, l4b: L4Index)
    requires
        pt.kernel_l4_end == KERNEL_MEM_END_L4INDEX,
        KERNEL_MEM_END_L4INDEX <= l4a < 512,
        KERNEL_MEM_END_L4INDEX <= l4b < 512,
        l4a != l4b,
        pt.spec_resolve_mapping_l4(l4a).is_Some(),
{
    assert(pt.spec_resolve_mapping_l4(l4b).is_Some());
}

// Test 4: page_closure includes l4_table domain. But page_ptr_valid alone does NOT
// imply membership in any page_closure. This tests cross-function misuse.
// SHOULD FAIL
proof fn test_logical_valid_ptr_not_in_closure(pt: PageTable, ptr: usize)
    requires
        page_ptr_valid(ptr),
{
    assert(pt.page_closure().contains(ptr));
}

// Test 5: An empty PageTable has empty mappings, but is_empty does NOT imply
// a stronger property like cr3 == 0. The spec says nothing about cr3 value.
// SHOULD FAIL
proof fn test_logical_empty_does_not_constrain_cr3(pt: PageTable)
    requires
        pt.is_empty(),
{
    assert(pt.cr3 == 0usize);
}

}
