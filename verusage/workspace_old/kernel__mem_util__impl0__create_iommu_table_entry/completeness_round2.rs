use vstd::prelude::*;

fn main() {}

verus! {

// === Constants ===
pub const KERNEL_MEM_END_L4INDEX: usize = 1;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_2m_MASK: u64 = 0x0000_ffff_ffe0_0000;
pub const MEM_1g_MASK: u64 = 0x0000_fffc_0000_0000;
pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;
pub const PAGE_ENTRY_USER_SHIFT: u64 = 2;
pub const PAGE_ENTRY_PS_SHIFT: u64 = 7;
pub const PAGE_ENTRY_EXECUTE_SHIFT: u64 = 63;
pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;
pub const PAGE_ENTRY_USER_MASK: u64 = 0x1u64 << PAGE_ENTRY_USER_SHIFT;
pub const PAGE_ENTRY_PS_MASK: u64 = 0x1u64 << PAGE_ENTRY_PS_SHIFT;
pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x1u64 << PAGE_ENTRY_EXECUTE_SHIFT;

pub type PAddr = usize;
pub type VAddr = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;

#[derive(Clone, Debug)]
pub struct PageEntryPerm {
    pub present: bool,
    pub ps: bool,
    pub write: bool,
    pub execute_disable: bool,
    pub user: bool,
}

#[derive(Clone, Debug)]
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

pub struct Quota {
    pub mem_4k: usize,
    pub mem_2m: usize,
    pub mem_1g: usize,
    pub pcid: usize,
    pub ioid: usize,
}

impl Quota {
    pub open spec fn spec_set_mem_4k(&self, v: usize) -> Self {
        Self { mem_4k: v, mem_2m: self.mem_2m, mem_1g: self.mem_1g, pcid: self.pcid, ioid: self.ioid }
    }
    pub open spec fn spec_subtract_mem_4k(&self, new: Self, k: usize) -> bool {
        &&& self.mem_4k - k == new.mem_4k
        &&& self.mem_2m == new.mem_2m
        &&& self.mem_1g == new.mem_1g
        &&& self.pcid == new.pcid
        &&& self.ioid == new.ioid
    }
}

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
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

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_index_valid(index: usize) -> bool {
    0 <= index < NUM_PAGES
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize
    recommends page_ptr_valid(ptr),
{ (ptr / 4096usize) as usize }

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize
    recommends page_index_valid(i),
{ (i * 4096) as usize }

pub open spec fn page_index_2m_valid(i: usize) -> bool {
    &&& i % 512 == 0
    &&& 0 <= i < NUM_PAGES
}

pub open spec fn page_index_1g_valid(i: usize) -> bool {
    &&& i % (512 * 512) as usize == 0
    &&& 0 <= i < NUM_PAGES
}

pub open spec fn page_ptr_2m_valid(ptr: usize) -> bool {
    ((ptr % 0x200000) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn page_ptr_1g_valid(ptr: usize) -> bool {
    ((ptr % 0x40000000) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64) >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn spec_va_2m_valid(va: usize) -> bool {
    (va & (!MEM_2m_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64) >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn spec_va_1g_valid(va: usize) -> bool {
    (va & (!MEM_1g_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64) >= KERNEL_MEM_END_L4INDEX as u64
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_va_4k_valid))]
pub fn va_4k_valid(va: usize) -> (ret: bool)
    ensures ret == spec_va_4k_valid(va),
{ unimplemented!() }

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_va_2m_valid))]
pub fn va_2m_valid(va: usize) -> (ret: bool)
    ensures ret == spec_va_2m_valid(va),
{ unimplemented!() }

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_va_1g_valid))]
pub fn va_1g_valid(va: usize) -> (ret: bool)
    ensures ret == spec_va_1g_valid(va),
{ unimplemented!() }

pub open spec fn spec_v2l1index(va: usize) -> L1Index {
    (va >> 12 & 0x1ff) as usize
}

pub open spec fn spec_v2l2index(va: usize) -> L2Index {
    (va >> 21 & 0x1ff) as usize
}

pub open spec fn spec_v2l3index(va: usize) -> L3Index {
    (va >> 30 & 0x1ff) as usize
}

pub open spec fn spec_v2l4index(va: usize) -> L4Index {
    (va >> 39 & 0x1ff) as usize
}

pub open spec fn spec_va2index(va: usize) -> (L4Index, L3Index, L2Index, L1Index) {
    (spec_v2l4index(va), spec_v2l3index(va), spec_v2l2index(va), spec_v2l1index(va))
}

pub open spec fn spec_index2va(i: (L4Index, L3Index, L2Index, L1Index)) -> usize
    recommends i.0 <= 0x1ff, i.1 <= 0x1ff, i.2 <= 0x1ff, i.3 <= 0x1ff,
{
    (i.0 as usize) << 39 & (i.1 as usize) << 30 & (i.2 as usize) << 21 & (i.3 as usize) << 12
}

#[verifier::external_body]
#[verifier(external_body)]
pub proof fn va_lemma()
    ensures
        forall|va: VAddr|
            #![trigger spec_va_4k_valid(va), spec_v2l4index(va)]
            #![trigger spec_va_4k_valid(va), spec_v2l3index(va)]
            #![trigger spec_va_4k_valid(va), spec_v2l2index(va)]
            #![trigger spec_va_4k_valid(va), spec_v2l1index(va)]
            spec_va_4k_valid(va) ==> 0 <= spec_v2l4index(va) < 512 && 0 <= spec_v2l3index(va) < 512
                && 0 <= spec_v2l2index(va) < 512 && 0 <= spec_v2l1index(va) < 512,
        forall|va: VAddr|
            #![trigger spec_va_2m_valid(va), spec_v2l4index(va)]
            #![trigger spec_va_2m_valid(va), spec_v2l3index(va)]
            #![trigger spec_va_2m_valid(va), spec_v2l2index(va)]
            #![trigger spec_va_2m_valid(va), spec_v2l1index(va)]
            spec_va_2m_valid(va) ==> 0 <= spec_v2l4index(va) < 512 && 0 <= spec_v2l3index(va) < 512
                && 0 <= spec_v2l2index(va) < 512 && 0 == spec_v2l1index(va),
        forall|va: VAddr|
            #![trigger spec_va_1g_valid(va), spec_v2l4index(va)]
            #![trigger spec_va_1g_valid(va), spec_v2l3index(va)]
            #![trigger spec_va_1g_valid(va), spec_v2l2index(va)]
            #![trigger spec_va_1g_valid(va), spec_v2l1index(va)]
            spec_va_1g_valid(va) ==> 0 <= spec_v2l4index(va) < 512 && 0 <= spec_v2l3index(va) < 512
                && 0 == spec_v2l2index(va) && 0 == spec_v2l1index(va),
        forall|
            l4i: L4Index, l3i: L3Index, l2i: L2Index, l1i: L1Index,
            l4j: L4Index, l3j: L3Index, l2j: L2Index, l1j: L1Index,
        |
            #![trigger spec_index2va((l4i,l3i,l2i,l1i)), spec_index2va((l4j,l3j,l2j,l1j))]
            (l4i, l3i, l2i, l1i) =~= (l4j, l3j, l2j, l1j) && 0 <= l4i < 512 && 0 <= l3i < 512 && 0
                <= l2i < 512 && 0 <= l1i < 512 && 0 <= l4j < 512 && 0 <= l3j < 512 && 0 <= l2j < 512
                && 0 <= l1j < 512 <==> spec_index2va((l4i, l3i, l2i, l1i)) == spec_index2va(
                (l4j, l3j, l2j, l1j)),
        forall|
            l4i: L4Index, l3i: L3Index, l2i: L2Index, l1i: L1Index,
            l4j: L4Index, l3j: L3Index, l2j: L2Index, l1j: L1Index,
        |
            #![trigger spec_index2va((l4i,l3i,l2i,l1i)), spec_index2va((l4j,l3j,l2j,l1j))]
            (l4i, l3i, l2i, l1i) =~= (l4j, l3j, l2j, l1j) == false && 0 <= l4i < 512 && 0 <= l3i
                < 512 && 0 <= l2i < 512 && 0 <= l1i < 512 && 0 <= l4j < 512 && 0 <= l3j < 512 && 0
                <= l2j < 512 && 0 <= l1j < 512 <==> spec_index2va((l4i, l3i, l2i, l1i))
                != spec_index2va((l4j, l3j, l2j, l1j)),
        forall|l4i: L4Index, l3i: L3Index, l2i: L2Index, l1i: L1Index|
            #![trigger va_4k_valid(spec_index2va((l4i,l3i,l2i,l1i)))]
            0 <= l4i < 512 && 0 <= l3i < 512 && 0 <= l2i < 512 && 0 <= l1i < 512 ==> va_4k_valid(
                spec_index2va((l4i, l3i, l2i, l1i))),
        forall|va: VAddr, l4i: L4Index, l3i: L3Index, l2i: L2Index, l1i: L1Index|
            #![trigger spec_index2va((l4i,l3i,l2i,l1i)), spec_va2index(va)]
            va_4k_valid(va) && spec_va2index(va) == (l4i, l3i, l2i, l1i) <==> KERNEL_MEM_END_L4INDEX
                <= l4i < 512 && 0 <= l3i < 512 && 0 <= l2i < 512 && 0 <= l1i < 512 && spec_index2va(
                (l4i, l3i, l2i, l1i)) == va,
        forall|l4i: L4Index, l3i: L3Index, l2i: L2Index|
            #![trigger va_2m_valid(spec_index2va((l4i,l3i,l2i,0)))]
            0 <= l4i < 512 && 0 <= l3i < 512 && 0 <= l2i < 512 ==> va_2m_valid(
                spec_index2va((l4i, l3i, l2i, 0))),
{ unimplemented!() }

// ============================================================================
// COMPLETENESS ROUND 2: Overly Strong Postconditions (all should FAIL)
// ============================================================================

// Test 1: l1 index range too tight (< 256 instead of < 512)
proof fn test_fail_l1_index_too_tight() {
    assert(forall|va: usize| #[trigger] spec_v2l1index(va) < 256) by (bit_vector);
}

// Test 2: l2 index range too tight
proof fn test_fail_l2_index_too_tight() {
    assert(forall|va: usize| #[trigger] spec_v2l2index(va) < 256) by (bit_vector);
}

// Test 3: l3 index range too tight
proof fn test_fail_l3_index_too_tight() {
    assert(forall|va: usize| #[trigger] spec_v2l3index(va) < 256) by (bit_vector);
}

// Test 4: l4 index range too tight
proof fn test_fail_l4_index_too_tight() {
    assert(forall|va: usize| #[trigger] spec_v2l4index(va) < 256) by (bit_vector);
}

// Test 5: spec_usize2pa too strong - claiming it equals v (no masking)
proof fn test_fail_usize2pa_equals_input() {
    assert(forall|v: usize| spec_usize2pa(v) == v) by (bit_vector);
}

// Test 6: create_iommu ret.0 bounded by 2 (too tight, spec says <= 3)
proof fn test_fail_ret_too_tight() {
    let ret_0: usize;
    assume(ret_0 <= 3);
    assert(ret_0 <= 2);
}

// Test 7: Quota subtract is identity (overly strong - no subtraction)
proof fn test_fail_quota_subtract_is_identity() {
    let q1 = Quota { mem_4k: 10, mem_2m: 5, mem_1g: 2, pcid: 1, ioid: 1 };
    let q2 = Quota { mem_4k: 10, mem_2m: 5, mem_1g: 2, pcid: 1, ioid: 1 };
    // Claiming subtract with k=3 gives same mem_4k (wrong!)
    assert(q1.spec_subtract_mem_4k(q2, 3usize));
}

// Test 8: MEM_valid holds for all values (too strong)
proof fn test_fail_mem_valid_universal() {
    assert(forall|v: usize| MEM_valid(v)) by (bit_vector);
}

} // verus!
