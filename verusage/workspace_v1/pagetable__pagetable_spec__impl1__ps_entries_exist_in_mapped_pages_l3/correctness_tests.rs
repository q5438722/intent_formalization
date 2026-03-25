use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus!{

pub type CpuId = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type PageMapPtr = usize;
pub type Pcid = usize;
pub type IOid = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;

pub const MEM_1g_MASK: u64 = 0x0000_fffc_0000_0000;

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
            0 <= i < 512 ==> (usize2page_entry(self.ar@[i]) =~= self.spec_seq@[i])
    }

    pub open spec fn spec_index(&self, index: usize) -> PageEntry
        recommends 0 <= index < 512,
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
            && ret.perm.write == false && ret.perm.execute_disable == false && ret.perm.user == false,
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
    pub open spec fn mapping_4k(&self) -> Map<VAddr, MapEntry> { self.mapping_4k@ }
    pub open spec fn mapping_2m(&self) -> Map<VAddr, MapEntry> { self.mapping_2m@ }
    pub open spec fn mapping_1g(&self) -> Map<VAddr, MapEntry> { self.mapping_1g@ }

    pub open spec fn page_not_mapped(&self, pa: PAddr) -> bool {
        &&& forall|va: VAddr|
            #![trigger self.mapping_4k().dom().contains(va), self.mapping_4k()[va].addr]
                self.mapping_4k().dom().contains(va) ==> self.mapping_4k()[va].addr != pa
        &&& forall|va: VAddr|
            #![trigger self.mapping_2m().dom().contains(va), self.mapping_2m()[va].addr]
                self.mapping_2m().dom().contains(va) ==> self.mapping_2m()[va].addr != pa
        &&& forall|va: VAddr|
            #![trigger self.mapping_1g().dom().contains(va), self.mapping_1g()[va].addr]
                self.mapping_1g().dom().contains(va) ==> self.mapping_1g()[va].addr != pa
    }

    pub open spec fn wf_l4(&self) -> bool {
        &&& self.l4_table@.dom() =~= Set::<PageMapPtr>::empty().insert(self.cr3)
        &&& self.cr3 == self.l4_table@[self.cr3].addr()
        &&& self.l4_table@[self.cr3].is_init()
        &&& self.l4_table@[self.cr3].value().wf()
        &&& forall|i: L4Index|
            #![trigger self.l2_tables@.dom().contains(self.l4_table@[self.cr3].value()[i].addr)]
            #![trigger self.l1_tables@.dom().contains(self.l4_table@[self.cr3].value()[i].addr)]
            self.kernel_l4_end <= i < 512 && self.l4_table@[self.cr3].value()[i].perm.present
                ==> self.l2_tables@.dom().contains(self.l4_table@[self.cr3].value()[i].addr) == false
                && self.l1_tables@.dom().contains(self.l4_table@[self.cr3].value()[i].addr) == false
                && self.cr3 != self.l4_table@[self.cr3].value()[i].addr
        &&& forall|i: L4Index|
            #![trigger self.l4_table@[self.cr3].value()[i].addr]
            self.kernel_l4_end <= i < 512 && self.l4_table@[self.cr3].value()[i].perm.present
                ==> self.cr3 != self.l4_table@[self.cr3].value()[i].addr
        &&& forall|i: L4Index|
            #![trigger self.l3_tables@.dom().contains(self.l4_table@[self.cr3].value()[i].addr)]
            self.kernel_l4_end <= i < 512 && self.l4_table@[self.cr3].value()[i].perm.present
                && !self.l4_table@[self.cr3].value()[i].perm.ps
                ==> self.l3_tables@.dom().contains(self.l4_table@[self.cr3].value()[i].addr)
        &&& forall|i: L4Index|
            #![trigger self.l4_table@[self.cr3].value()[i].perm.ps]
            self.kernel_l4_end <= i < 512 && self.l4_table@[self.cr3].value()[i].perm.present
                ==> !self.l4_table@[self.cr3].value()[i].perm.ps
    }

    pub open spec fn wf_l3(&self) -> bool {
        &&& forall|p: PageMapPtr|
            #![trigger self.l3_tables@[p].addr()]
            self.l3_tables@.dom().contains(p) ==> self.l3_tables@[p].addr() == p
        &&& forall|p: PageMapPtr|
            #![trigger self.l3_tables@[p].is_init()]
            self.l3_tables@.dom().contains(p) ==> self.l3_tables@[p].is_init()
        &&& forall|p: PageMapPtr|
            #![trigger self.l3_tables@[p].value().wf()]
            self.l3_tables@.dom().contains(p) ==> self.l3_tables@[p].value().wf()
        &&& forall|p: PageMapPtr|
            #![trigger self.l3_rev_map@.dom().contains(p)]
            #![trigger self.l3_rev_map@[p]]
            self.l3_tables@.dom().contains(p) ==> self.kernel_l4_end <= self.l3_rev_map@[p] < 512
                && self.l3_rev_map@.dom().contains(p)
                && self.spec_resolve_mapping_l4(self.l3_rev_map@[p]).is_Some()
                && self.spec_resolve_mapping_l4(self.l3_rev_map@[p]).get_Some_0().addr == p
        &&& forall|p: PageMapPtr, i: L3Index|
            #![trigger self.l3_tables@.dom().contains(p), self.l3_tables@[p].value()[i].perm.present, self.l3_tables@.dom().contains(self.l3_tables@[p].value()[i].addr)]
            #![trigger self.l3_tables@.dom().contains(p), self.l3_tables@[p].value()[i].perm.present, self.l1_tables@.dom().contains(self.l3_tables@[p].value()[i].addr)]
            #![trigger self.l3_tables@.dom().contains(p), self.l3_tables@[p].value()[i].perm.present, self.l3_tables@[p].value()[i].addr]
            self.l3_tables@.dom().contains(p) && 0 <= i < 512
                && self.l3_tables@[p].value()[i].perm.present
                ==> self.l3_tables@.dom().contains(self.l3_tables@[p].value()[i].addr) == false
                && self.l1_tables@.dom().contains(self.l3_tables@[p].value()[i].addr) == false
                && self.cr3 != self.l3_tables@[p].value()[i].addr
        &&& forall|p: PageMapPtr, i: L3Index|
            #![trigger self.l3_tables@[p].value()[i].perm.present, self.l3_tables@[p].value()[i].perm.ps, self.l2_tables@.dom().contains(self.l3_tables@[p].value()[i].addr)]
            self.l3_tables@.dom().contains(p) && 0 <= i < 512
                && self.l3_tables@[p].value()[i].perm.present
                && !self.l3_tables@[p].value()[i].perm.ps
                ==> self.l2_tables@.dom().contains(self.l3_tables@[p].value()[i].addr)
    }

    pub open spec fn spec_resolve_mapping_l4(&self, l4i: L4Index) -> Option<PageEntry>
        recommends self.kernel_l4_end <= l4i < 512,
    {
        if self.l4_table@[self.cr3].value()[l4i].perm.present || l4i < self.kernel_l4_end {
            Some(self.l4_table@[self.cr3].value()[l4i])
        } else {
            None
        }
    }

    pub open spec fn spec_resolve_mapping_1g_l3(&self, l4i: L4Index, l3i: L3Index) -> Option<PageEntry>
        recommends
            self.kernel_l4_end <= l4i < 512,
            0 <= l3i < 512,
    {
        if self.spec_resolve_mapping_l4(l4i).is_None() {
            None
        } else if !self.l3_tables@[self.spec_resolve_mapping_l4(l4i).get_Some_0().addr].value()[l3i].perm.present
            || !self.l3_tables@[self.spec_resolve_mapping_l4(l4i).get_Some_0().addr].value()[l3i].perm.ps {
            None
        } else {
            Some(self.l3_tables@[self.spec_resolve_mapping_l4(l4i).get_Some_0().addr].value()[l3i])
        }
    }

    pub open spec fn wf_mapping_1g(&self) -> bool {
        &&& forall|va: VAddr|
            #![trigger va_1g_valid(va), self.mapping_1g@.dom().contains(va)]
            self.mapping_1g@.dom().contains(va) ==> va_1g_valid(va)
        &&& forall|l4i: L4Index, l3i: L3Index|
            #![trigger self.mapping_1g@[spec_index2va((l4i,l3i,0,0))]]
            #![trigger self.spec_resolve_mapping_1g_l3(l4i,l3i)]
            self.kernel_l4_end <= l4i < 512 && 0 <= l3i < 512
                ==> self.mapping_1g@.dom().contains(spec_index2va((l4i, l3i, 0, 0)))
                == self.spec_resolve_mapping_1g_l3(l4i, l3i).is_Some()
        &&& forall|l4i: L4Index, l3i: L3Index|
            #![trigger self.mapping_1g@[spec_index2va((l4i,l3i,0,0))]]
            #![trigger self.spec_resolve_mapping_1g_l3(l4i,l3i)]
            self.kernel_l4_end <= l4i < 512 && 0 <= l3i < 512
                && self.spec_resolve_mapping_1g_l3(l4i, l3i).is_Some()
                ==> self.mapping_1g@[spec_index2va((l4i, l3i, 0, 0))].addr
                    == self.spec_resolve_mapping_1g_l3(l4i, l3i).get_Some_0().addr
                && self.mapping_1g@[spec_index2va((l4i, l3i, 0, 0))].write
                    == self.spec_resolve_mapping_1g_l3(l4i, l3i).get_Some_0().perm.write
                && self.mapping_1g@[spec_index2va((l4i, l3i, 0, 0))].execute_disable
                    == self.spec_resolve_mapping_1g_l3(l4i, l3i).get_Some_0().perm.execute_disable
        &&& forall|va: VAddr|
            #![trigger self.mapping_1g@.dom().contains(va), page_ptr_1g_valid(self.mapping_1g@[va].addr)]
            self.mapping_1g@.dom().contains(va) ==> page_ptr_1g_valid(self.mapping_1g@[va].addr)
    }

    pub proof fn ps_entries_exist_in_mapped_pages_l3(&self)
        requires
            self.wf_l4(),
            self.wf_l3(),
            self.wf_mapping_1g(),
        ensures
            forall|p: PageMapPtr, i: L3Index|
                #![trigger self.l3_tables@[p].value()[i].addr]
                self.l3_tables@.dom().contains(p) && 0 <= i < 512
                    && self.l3_tables@[p].value()[i].perm.present
                    && self.l3_tables@[p].value()[i].perm.ps
                    ==> self.page_not_mapped(self.l3_tables@[p].value()[i].addr) == false,
            forall|p: PageMapPtr, i: L3Index|
                #![trigger self.l3_tables@[p].value()[i].addr]
                self.l3_tables@.dom().contains(p) && 0 <= i < 512
                    && self.l3_tables@[p].value()[i].perm.present
                    && !self.l3_tables@[p].value()[i].perm.ps
                    ==> self.l2_tables@.dom().contains(self.l3_tables@[p].value()[i].addr),
    {
        assert(forall|p: PageMapPtr, i: L3Index|
                #![trigger self.l3_tables@[p].value()[i].addr]
                self.l3_tables@.dom().contains(p) && 0 <= i < 512
                    && self.l3_tables@[p].value()[i].perm.present
                    && self.l3_tables@[p].value()[i].perm.ps ==>
                    self.spec_resolve_mapping_1g_l3(self.l3_rev_map@[p], i).is_Some()
                    &&
                    self.spec_resolve_mapping_1g_l3(self.l3_rev_map@[p], i).unwrap().addr == self.l3_tables@[p].value()[i].addr
                    &&
                    self.mapping_1g@.dom().contains(spec_index2va((self.l3_rev_map@[p], i, 0, 0)))
                    &&
                    self.mapping_1g()[spec_index2va((self.l3_rev_map@[p], i, 0, 0))].addr == self.l3_tables@[p].value()[i].addr
        );
    }
}

pub struct Array<A, const N: usize>{
    pub seq: Ghost<Seq<A>>,
    pub ar: [A;N]
}

impl<A, const N: usize> Array<A, N> {
    #[verifier(inline)]
    pub open spec fn view(&self) -> Seq<A>{ self.seq@ }
    pub open spec fn wf(&self) -> bool{ self.seq@.len() == N }
}

pub open spec fn page_ptr_1g_valid(ptr: usize) -> bool {
    ((ptr % (0x40000000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn spec_va_2m_valid(va: usize) -> bool {
    (va & (!MEM_2m_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64) >= KERNEL_MEM_END_L4INDEX as u64
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_va_2m_valid))]
pub fn va_2m_valid(va: usize) -> (ret: bool)
    ensures ret == spec_va_2m_valid(va),
{ unimplemented!() }

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_va_2m_valid))]
pub fn va_1g_valid(va: usize) -> (ret: bool)
    ensures ret == spec_va_1g_valid(va),
{ unimplemented!() }

pub open spec fn spec_va_1g_valid(va: usize) -> bool {
    (va & (!MEM_1g_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64) >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn spec_index2va(i: (L4Index, L3Index, L2Index, L1Index)) -> usize
    recommends
        i.0 <= 0x1ff, i.1 <= 0x1ff, i.2 <= 0x1ff, i.3 <= 0x1ff,
{
    (i.0 as usize) << 39 & (i.1 as usize) << 30 & (i.2 as usize) << 21 & (i.3 as usize) << 12
}

pub const KERNEL_MEM_END_L4INDEX: usize = 1;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_2m_MASK: u64 = 0x0000_ffff_ffe0_0000;
pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;
pub const PAGE_ENTRY_USER_SHIFT: u64 = 2;
pub const PAGE_ENTRY_PS_SHIFT: u64 = 7;
pub const PAGE_ENTRY_EXECUTE_SHIFT: u64 = 63;
pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;
pub const PAGE_ENTRY_USER_MASK: u64 = 0x1u64 << PAGE_ENTRY_USER_SHIFT;
pub const PAGE_ENTRY_PS_MASK: u64 = 0x1u64 << PAGE_ENTRY_PS_SHIFT;
pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x1u64 << PAGE_ENTRY_EXECUTE_SHIFT;

// =========================================================================
// COMBINED CORRECTNESS TESTS — All tests should FAIL verification
// =========================================================================

impl PageTable {

    // =====================================================
    // BOUNDARY TESTS: Violate preconditions / edge cases
    // =====================================================

    // SHOULD FAIL: missing wf_l4 precondition
    pub proof fn boundary_test_missing_wf_l4(&self)
        requires
            self.wf_l3(),
            self.wf_mapping_1g(),
        ensures
            forall|p: PageMapPtr, i: L3Index|
                #![trigger self.l3_tables@[p].value()[i].addr]
                self.l3_tables@.dom().contains(p) && 0 <= i < 512
                    && self.l3_tables@[p].value()[i].perm.present
                    && self.l3_tables@[p].value()[i].perm.ps
                    ==> self.page_not_mapped(self.l3_tables@[p].value()[i].addr) == false,
    {
        self.ps_entries_exist_in_mapped_pages_l3();
    }

    // SHOULD FAIL: missing wf_l3 precondition
    pub proof fn boundary_test_missing_wf_l3(&self)
        requires
            self.wf_l4(),
            self.wf_mapping_1g(),
        ensures
            forall|p: PageMapPtr, i: L3Index|
                #![trigger self.l3_tables@[p].value()[i].addr]
                self.l3_tables@.dom().contains(p) && 0 <= i < 512
                    && self.l3_tables@[p].value()[i].perm.present
                    && self.l3_tables@[p].value()[i].perm.ps
                    ==> self.page_not_mapped(self.l3_tables@[p].value()[i].addr) == false,
    {
        self.ps_entries_exist_in_mapped_pages_l3();
    }

    // SHOULD FAIL: missing wf_mapping_1g precondition
    pub proof fn boundary_test_missing_wf_mapping_1g(&self)
        requires
            self.wf_l4(),
            self.wf_l3(),
        ensures
            forall|p: PageMapPtr, i: L3Index|
                #![trigger self.l3_tables@[p].value()[i].addr]
                self.l3_tables@.dom().contains(p) && 0 <= i < 512
                    && self.l3_tables@[p].value()[i].perm.present
                    && self.l3_tables@[p].value()[i].perm.ps
                    ==> self.page_not_mapped(self.l3_tables@[p].value()[i].addr) == false,
    {
        self.ps_entries_exist_in_mapped_pages_l3();
    }

    // SHOULD FAIL: missing ALL preconditions
    pub proof fn boundary_test_missing_all_preconditions(&self)
        ensures
            forall|p: PageMapPtr, i: L3Index|
                #![trigger self.l3_tables@[p].value()[i].addr]
                self.l3_tables@.dom().contains(p) && 0 <= i < 512
                    && self.l3_tables@[p].value()[i].perm.present
                    && self.l3_tables@[p].value()[i].perm.ps
                    ==> self.page_not_mapped(self.l3_tables@[p].value()[i].addr) == false,
    {
        self.ps_entries_exist_in_mapped_pages_l3();
    }

    // SHOULD FAIL: l2 ensures without wf_l3
    pub proof fn boundary_test_l2_ensures_without_wf_l3(&self)
        requires
            self.wf_l4(),
            self.wf_mapping_1g(),
        ensures
            forall|p: PageMapPtr, i: L3Index|
                #![trigger self.l3_tables@[p].value()[i].addr]
                self.l3_tables@.dom().contains(p) && 0 <= i < 512
                    && self.l3_tables@[p].value()[i].perm.present
                    && !self.l3_tables@[p].value()[i].perm.ps
                    ==> self.l2_tables@.dom().contains(self.l3_tables@[p].value()[i].addr),
    {
        self.ps_entries_exist_in_mapped_pages_l3();
    }

    // ===========================================================
    // BEHAVIORAL MUTATION TESTS: Mutate expected outputs/relations
    // ===========================================================

    // SHOULD FAIL: negate first postcondition — assert PS entries ARE unmapped
    pub proof fn mutation_test_ps_entry_is_unmapped(&self, p: PageMapPtr, i: L3Index)
        requires
            self.wf_l4(),
            self.wf_l3(),
            self.wf_mapping_1g(),
            self.l3_tables@.dom().contains(p),
            0 <= i < 512,
            self.l3_tables@[p].value()[i].perm.present,
            self.l3_tables@[p].value()[i].perm.ps,
    {
        self.ps_entries_exist_in_mapped_pages_l3();
        assert(self.page_not_mapped(self.l3_tables@[p].value()[i].addr));
    }

    // SHOULD FAIL: negate second postcondition — non-PS NOT in l2
    pub proof fn mutation_test_non_ps_entry_not_in_l2(&self, p: PageMapPtr, i: L3Index)
        requires
            self.wf_l4(),
            self.wf_l3(),
            self.wf_mapping_1g(),
            self.l3_tables@.dom().contains(p),
            0 <= i < 512,
            self.l3_tables@[p].value()[i].perm.present,
            !self.l3_tables@[p].value()[i].perm.ps,
    {
        self.ps_entries_exist_in_mapped_pages_l3();
        assert(!self.l2_tables@.dom().contains(self.l3_tables@[p].value()[i].addr));
    }

    // SHOULD FAIL: mutate mapping address
    pub proof fn mutation_test_mapped_addr_differs(&self, p: PageMapPtr, i: L3Index)
        requires
            self.wf_l4(),
            self.wf_l3(),
            self.wf_mapping_1g(),
            self.l3_tables@.dom().contains(p),
            0 <= i < 512,
            self.l3_tables@[p].value()[i].perm.present,
            self.l3_tables@[p].value()[i].perm.ps,
    {
        self.ps_entries_exist_in_mapped_pages_l3();
        let l4i = self.l3_rev_map@[p];
        let va = spec_index2va((l4i, i, 0, 0));
        assert(self.mapping_1g@[va].addr != self.l3_tables@[p].value()[i].addr);
    }

    // SHOULD FAIL: PS entry address is zero
    pub proof fn mutation_test_ps_entry_addr_is_zero(&self, p: PageMapPtr, i: L3Index)
        requires
            self.wf_l4(),
            self.wf_l3(),
            self.wf_mapping_1g(),
            self.l3_tables@.dom().contains(p),
            0 <= i < 512,
            self.l3_tables@[p].value()[i].perm.present,
            self.l3_tables@[p].value()[i].perm.ps,
    {
        self.ps_entries_exist_in_mapped_pages_l3();
        assert(self.l3_tables@[p].value()[i].addr == 0);
    }

    // SHOULD FAIL: PS entries in l2_tables (swapped postcondition)
    pub proof fn mutation_test_ps_entry_in_l2_tables(&self, p: PageMapPtr, i: L3Index)
        requires
            self.wf_l4(),
            self.wf_l3(),
            self.wf_mapping_1g(),
            self.l3_tables@.dom().contains(p),
            0 <= i < 512,
            self.l3_tables@[p].value()[i].perm.present,
            self.l3_tables@[p].value()[i].perm.ps,
    {
        self.ps_entries_exist_in_mapped_pages_l3();
        assert(self.l2_tables@.dom().contains(self.l3_tables@[p].value()[i].addr));
    }

    // ================================================================
    // LOGICAL TESTS: Properties NOT explicitly guaranteed by the spec
    // ================================================================

    // SHOULD FAIL: PS entry also in mapping_4k
    pub proof fn logical_test_ps_entry_also_in_mapping_4k(&self, p: PageMapPtr, i: L3Index)
        requires
            self.wf_l4(),
            self.wf_l3(),
            self.wf_mapping_1g(),
            self.l3_tables@.dom().contains(p),
            0 <= i < 512,
            self.l3_tables@[p].value()[i].perm.present,
            self.l3_tables@[p].value()[i].perm.ps,
    {
        self.ps_entries_exist_in_mapped_pages_l3();
        let addr = self.l3_tables@[p].value()[i].addr;
        assert(exists|va: VAddr|
            self.mapping_4k@.dom().contains(va) && self.mapping_4k@[va].addr == addr
        );
    }

    // SHOULD FAIL: all L3 entries must be present
    pub proof fn logical_test_all_l3_entries_present(&self, p: PageMapPtr)
        requires
            self.wf_l4(),
            self.wf_l3(),
            self.wf_mapping_1g(),
            self.l3_tables@.dom().contains(p),
    {
        assert(forall|i: L3Index|
            #![trigger self.l3_tables@[p].value()[i].perm.present]
            0 <= i < 512 ==> self.l3_tables@[p].value()[i].perm.present
        );
    }

    // SHOULD FAIL: PS entries must be writable
    pub proof fn logical_test_ps_entry_must_be_writable(&self, p: PageMapPtr, i: L3Index)
        requires
            self.wf_l4(),
            self.wf_l3(),
            self.wf_mapping_1g(),
            self.l3_tables@.dom().contains(p),
            0 <= i < 512,
            self.l3_tables@[p].value()[i].perm.present,
            self.l3_tables@[p].value()[i].perm.ps,
    {
        self.ps_entries_exist_in_mapped_pages_l3();
        assert(self.l3_tables@[p].value()[i].perm.write);
    }

    // SHOULD FAIL: two PS entries must have different physical addresses
    pub proof fn logical_test_ps_entries_unique_addrs(&self, p: PageMapPtr, i1: L3Index, i2: L3Index)
        requires
            self.wf_l4(),
            self.wf_l3(),
            self.wf_mapping_1g(),
            self.l3_tables@.dom().contains(p),
            0 <= i1 < 512,
            0 <= i2 < 512,
            i1 != i2,
            self.l3_tables@[p].value()[i1].perm.present,
            self.l3_tables@[p].value()[i1].perm.ps,
            self.l3_tables@[p].value()[i2].perm.present,
            self.l3_tables@[p].value()[i2].perm.ps,
    {
        self.ps_entries_exist_in_mapped_pages_l3();
        assert(self.l3_tables@[p].value()[i1].addr != self.l3_tables@[p].value()[i2].addr);
    }

    // SHOULD FAIL: PS entries must not have execute_disable
    pub proof fn logical_test_ps_entry_executable(&self, p: PageMapPtr, i: L3Index)
        requires
            self.wf_l4(),
            self.wf_l3(),
            self.wf_mapping_1g(),
            self.l3_tables@.dom().contains(p),
            0 <= i < 512,
            self.l3_tables@[p].value()[i].perm.present,
            self.l3_tables@[p].value()[i].perm.ps,
    {
        self.ps_entries_exist_in_mapped_pages_l3();
        assert(!self.l3_tables@[p].value()[i].perm.execute_disable);
    }
}

}
