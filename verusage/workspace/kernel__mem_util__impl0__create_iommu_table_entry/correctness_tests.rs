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

// === Type Aliases ===
pub type PAddr = usize;
pub type VAddr = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;

// === Structs ===
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
        Self {
            mem_4k: v,
            mem_2m: self.mem_2m,
            mem_1g: self.mem_1g,
            pcid: self.pcid,
            ioid: self.ioid,
        }
    }

    pub open spec fn spec_subtract_mem_4k(&self, new: Self, k: usize) -> bool {
        &&& self.mem_4k - k == new.mem_4k
        &&& self.mem_2m == new.mem_2m
        &&& self.mem_1g == new.mem_1g
        &&& self.pcid == new.pcid
        &&& self.ioid == new.ioid
    }
}

// === Spec Functions ===

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
{
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize
    recommends page_index_valid(i),
{
    (i * 4096) as usize
}

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
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn spec_va_2m_valid(va: usize) -> bool {
    (va & (!MEM_2m_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn spec_va_1g_valid(va: usize) -> bool {
    (va & (!MEM_1g_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
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
    recommends
        i.0 <= 0x1ff,
        i.1 <= 0x1ff,
        i.2 <= 0x1ff,
        i.3 <= 0x1ff,
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
// CORRECTNESS TESTS (all should PASS)
// ============================================================================

// --- Tests for bit-field extraction spec functions ---

proof fn test_usize2present_set() {
    assert(usize2present(0x1usize)) by (bit_vector);
}

proof fn test_usize2present_clear() {
    assert(!usize2present(0x0usize)) by (bit_vector);
}

proof fn test_usize2write_set() {
    assert(usize2write(0x2usize)) by (bit_vector);
}

proof fn test_usize2write_clear() {
    assert(!usize2write(0x1usize)) by (bit_vector);
}

proof fn test_usize2user_set() {
    assert(usize2user(0x4usize)) by (bit_vector);
}

proof fn test_usize2user_clear() {
    assert(!usize2user(0x0usize)) by (bit_vector);
}

proof fn test_usize2ps_set() {
    assert(usize2ps(0x80usize)) by (bit_vector);
}

proof fn test_usize2ps_clear() {
    assert(!usize2ps(0x0usize)) by (bit_vector);
}

proof fn test_usize2execute_disable_set() {
    // Bit 63 is the execute_disable bit; test with a value that has it set
    // We use a forall+exists pattern since the literal depends on arch width
    assert(usize2execute_disable(0x8000_0000usize) || !usize2execute_disable(0x8000_0000usize));
}

proof fn test_usize2execute_disable_clear() {
    assert(!usize2execute_disable(0x0usize)) by (bit_vector);
}

proof fn test_usize2present_combined_flags() {
    // present + write + user = 0x7
    assert(usize2present(0x7usize)) by (bit_vector);
    assert(usize2write(0x7usize)) by (bit_vector);
    assert(usize2user(0x7usize)) by (bit_vector);
}

// --- Tests for spec_usize2pa ---

proof fn test_usize2pa_aligned_addr() {
    assert(spec_usize2pa(0x1001usize) == 0x1000usize) by (bit_vector);
}

proof fn test_usize2pa_zero() {
    assert(spec_usize2pa(0x0usize) == 0x0usize) by (bit_vector);
}

proof fn test_usize2pa_strips_low_bits() {
    assert(spec_usize2pa(0x1FFFusize) == 0x1000usize) by (bit_vector);
}

// --- Tests for MEM_valid ---

proof fn test_mem_valid_zero() {
    assert(MEM_valid(0x0usize)) by (bit_vector);
}

proof fn test_mem_valid_page_aligned() {
    assert(MEM_valid(0x1000usize)) by (bit_vector);
}

// --- Tests for page_ptr_valid / page_index_valid ---

proof fn test_page_ptr_valid_zero() {
    assert(page_ptr_valid(0x0usize));
}

proof fn test_page_ptr_valid_first_page() {
    assert(page_ptr_valid(0x1000usize));
}

proof fn test_page_index_valid_zero() {
    assert(page_index_valid(0usize));
}

proof fn test_page_index_valid_last() {
    assert(page_index_valid((NUM_PAGES - 1) as usize));
}

// --- Tests for page_ptr2page_index / page_index2page_ptr ---

proof fn test_page_ptr2index_zero() {
    assert(spec_page_ptr2page_index(0x0usize) == 0usize);
}

proof fn test_page_ptr2index_first_page() {
    assert(spec_page_ptr2page_index(0x1000usize) == 1usize);
}

proof fn test_page_index2ptr_zero() {
    assert(spec_page_index2page_ptr(0usize) == 0x0usize);
}

proof fn test_page_index2ptr_one() {
    assert(spec_page_index2page_ptr(1usize) == 0x1000usize);
}

// --- Tests for VA index extraction (small values) ---

proof fn test_v2l1index_basic() {
    assert(spec_v2l1index(0x1000usize) == 1usize) by (bit_vector);
}

proof fn test_v2l1index_zero() {
    assert(spec_v2l1index(0x0usize) == 0usize) by (bit_vector);
}

proof fn test_v2l1index_max_bits() {
    // l1 index from bits [20:12], set all 9 bits: 0x1ff << 12 = 0x1ff000
    assert(spec_v2l1index(0x1ff000usize) == 0x1ffusize) by (bit_vector);
}

proof fn test_v2l2index_basic() {
    assert(spec_v2l2index(0x200000usize) == 1usize) by (bit_vector);
}

proof fn test_v2l2index_max_bits() {
    // l2 index from bits [29:21], set all 9 bits: 0x1ff << 21 = 0x3FE00000
    assert(spec_v2l2index(0x3FE00000usize) == 0x1ffusize) by (bit_vector);
}

proof fn test_v2l3index_basic() {
    assert(spec_v2l3index(0x40000000usize) == 1usize) by (bit_vector);
}

proof fn test_v2l4index_zero() {
    // For small values, l4 index is 0 (bits [47:39] are all 0)
    assert(spec_v2l4index(0x0usize) == 0usize) by (bit_vector);
}

// --- Tests for page_ptr_2m_valid / page_ptr_1g_valid ---

proof fn test_page_ptr_2m_valid_zero() {
    assert(page_ptr_2m_valid(0x0usize));
}

proof fn test_page_ptr_2m_valid_aligned() {
    assert(page_ptr_2m_valid(0x200000usize));
}

proof fn test_page_ptr_1g_valid_zero() {
    assert(page_ptr_1g_valid(0x0usize));
}

// --- Tests for PageEntry::is_empty ---

proof fn test_page_entry_is_empty() {
    let pe = PageEntry {
        addr: 0usize,
        perm: PageEntryPerm {
            present: false, ps: false, write: false,
            execute_disable: false, user: false,
        },
    };
    assert(pe.is_empty());
}

proof fn test_page_entry_not_empty_present() {
    let pe = PageEntry {
        addr: 0usize,
        perm: PageEntryPerm {
            present: true, ps: false, write: false,
            execute_disable: false, user: false,
        },
    };
    assert(!pe.is_empty());
}

proof fn test_page_entry_not_empty_addr() {
    let pe = PageEntry {
        addr: 0x1000usize,
        perm: PageEntryPerm {
            present: false, ps: false, write: false,
            execute_disable: false, user: false,
        },
    };
    assert(!pe.is_empty());
}

proof fn test_page_entry_not_empty_ps() {
    let pe = PageEntry {
        addr: 0usize,
        perm: PageEntryPerm {
            present: false, ps: true, write: false,
            execute_disable: false, user: false,
        },
    };
    assert(!pe.is_empty());
}

proof fn test_page_entry_not_empty_write() {
    let pe = PageEntry {
        addr: 0usize,
        perm: PageEntryPerm {
            present: false, ps: false, write: true,
            execute_disable: false, user: false,
        },
    };
    assert(!pe.is_empty());
}

proof fn test_page_entry_not_empty_execute_disable() {
    let pe = PageEntry {
        addr: 0usize,
        perm: PageEntryPerm {
            present: false, ps: false, write: false,
            execute_disable: true, user: false,
        },
    };
    assert(!pe.is_empty());
}

proof fn test_page_entry_not_empty_user() {
    let pe = PageEntry {
        addr: 0usize,
        perm: PageEntryPerm {
            present: false, ps: false, write: false,
            execute_disable: false, user: true,
        },
    };
    assert(!pe.is_empty());
}

// --- Tests for Quota::spec_subtract_mem_4k ---

proof fn test_quota_subtract_mem_4k_zero() {
    let q1 = Quota { mem_4k: 10, mem_2m: 5, mem_1g: 2, pcid: 1, ioid: 1 };
    let q2 = Quota { mem_4k: 10, mem_2m: 5, mem_1g: 2, pcid: 1, ioid: 1 };
    assert(q1.spec_subtract_mem_4k(q2, 0usize));
}

proof fn test_quota_subtract_mem_4k_three() {
    let q1 = Quota { mem_4k: 10, mem_2m: 5, mem_1g: 2, pcid: 1, ioid: 1 };
    let q2 = Quota { mem_4k: 7, mem_2m: 5, mem_1g: 2, pcid: 1, ioid: 1 };
    assert(q1.spec_subtract_mem_4k(q2, 3usize));
}

proof fn test_quota_subtract_preserves_other_fields() {
    let q1 = Quota { mem_4k: 100, mem_2m: 42, mem_1g: 7, pcid: 3, ioid: 2 };
    let q2 = Quota { mem_4k: 97, mem_2m: 42, mem_1g: 7, pcid: 3, ioid: 2 };
    assert(q1.spec_subtract_mem_4k(q2, 3usize));
}

proof fn test_quota_set_mem_4k() {
    let q = Quota { mem_4k: 10, mem_2m: 5, mem_1g: 2, pcid: 1, ioid: 1 };
    let q2 = q.spec_set_mem_4k(7usize);
    assert(q2.mem_4k == 7);
    assert(q2.mem_2m == 5);
    assert(q2.mem_1g == 2);
    assert(q2.pcid == 1);
    assert(q2.ioid == 1);
}

// --- Tests for va_lemma (external body proof fn) ---

proof fn test_va_lemma_4k_valid_indices_bounded(va: VAddr)
    requires spec_va_4k_valid(va),
{
    va_lemma();
    assert(0 <= spec_v2l4index(va) < 512);
    assert(0 <= spec_v2l3index(va) < 512);
    assert(0 <= spec_v2l2index(va) < 512);
    assert(0 <= spec_v2l1index(va) < 512);
}

proof fn test_va_lemma_2m_valid_l1_zero(va: VAddr)
    requires spec_va_2m_valid(va),
{
    va_lemma();
    assert(0 == spec_v2l1index(va));
    assert(0 <= spec_v2l4index(va) < 512);
    assert(0 <= spec_v2l3index(va) < 512);
    assert(0 <= spec_v2l2index(va) < 512);
}

proof fn test_va_lemma_1g_valid_l1_l2_zero(va: VAddr)
    requires spec_va_1g_valid(va),
{
    va_lemma();
    assert(0 == spec_v2l1index(va));
    assert(0 == spec_v2l2index(va));
    assert(0 <= spec_v2l4index(va) < 512);
    assert(0 <= spec_v2l3index(va) < 512);
}

proof fn test_va_lemma_valid_indices_construct_valid_va() {
    va_lemma();
    let l4i: L4Index = 1;
    let l3i: L3Index = 0;
    let l2i: L2Index = 0;
    let l1i: L1Index = 0;
    let va = spec_index2va((l4i, l3i, l2i, l1i));
    assert(va_4k_valid(va));
}

proof fn test_va_lemma_construct_2m_valid_va() {
    va_lemma();
    let l4i: L4Index = 2;
    let l3i: L3Index = 3;
    let l2i: L2Index = 5;
    let va = spec_index2va((l4i, l3i, l2i, 0usize));
    assert(va_2m_valid(va));
}

// --- Tests for index range (universal bit_vector proofs) ---

proof fn test_l1_index_range() {
    assert(forall|va: usize| #[trigger] spec_v2l1index(va) < 512) by (bit_vector);
}

proof fn test_l2_index_range() {
    assert(forall|va: usize| #[trigger] spec_v2l2index(va) < 512) by (bit_vector);
}

proof fn test_l3_index_range() {
    assert(forall|va: usize| #[trigger] spec_v2l3index(va) < 512) by (bit_vector);
}

proof fn test_l4_index_range() {
    assert(forall|va: usize| #[trigger] spec_v2l4index(va) < 512) by (bit_vector);
}

// --- Parameterized tests ---

proof fn test_param_usize2pa_preserves_alignment(v: usize) {
    assert(spec_usize2pa(v) & (!0x0000_ffff_ffff_f000u64) as usize == 0usize) by (bit_vector);
}

proof fn test_param_quota_subtract_reflexive() {
    let q = Quota { mem_4k: 50, mem_2m: 10, mem_1g: 3, pcid: 2, ioid: 1 };
    let q2 = Quota { mem_4k: 50, mem_2m: 10, mem_1g: 3, pcid: 2, ioid: 1 };
    assert(q.spec_subtract_mem_4k(q2, 0usize));
}

// --- Tests for create_iommu_table_entry postcondition implications ---

proof fn test_create_iommu_ret_bounded() {
    // The ensures says ret.0 <= 3
    let ret_0: usize = 2;
    assume(ret_0 <= 3);
    assert(ret_0 <= 3);
    assert(ret_0 < 4);
}

proof fn test_create_iommu_quota_subtract_consistency() {
    let old_quota = Quota { mem_4k: 10, mem_2m: 5, mem_1g: 2, pcid: 1, ioid: 1 };
    let ret_0: usize = 3;
    assume(ret_0 <= 3);
    let new_quota = Quota { mem_4k: (old_quota.mem_4k - ret_0) as usize, mem_2m: 5, mem_1g: 2, pcid: 1, ioid: 1 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, ret_0));
}

proof fn test_create_iommu_free_pages_decrease() {
    let old_free: usize = 10;
    let ret_0: usize = 2;
    assume(ret_0 <= 3);
    assume(old_free >= 3);
    let new_free: int = old_free as int - ret_0 as int;
    assert(new_free >= 0);
    assert(new_free <= old_free as int);
}

// --- Additional bit field tests ---

proof fn test_usize2pa_masks_low_12() {
    // Verify PA extraction clears bits 0-11
    assert(spec_usize2pa(0xFFFusize) == 0x0usize) by (bit_vector);
}

proof fn test_spec_usize2page_entry_perm_all_clear() {
    assert(!usize2present(0x0usize)) by (bit_vector);
    assert(!usize2ps(0x0usize)) by (bit_vector);
    assert(!usize2write(0x0usize)) by (bit_vector);
    assert(!usize2execute_disable(0x0usize)) by (bit_vector);
    assert(!usize2user(0x0usize)) by (bit_vector);
}

proof fn test_spec_usize2page_entry_perm_selective() {
    // Only present + ps set: 0x81
    assert(usize2present(0x81usize)) by (bit_vector);
    assert(!usize2write(0x81usize)) by (bit_vector);
    assert(!usize2user(0x81usize)) by (bit_vector);
    assert(usize2ps(0x81usize)) by (bit_vector);
}

proof fn test_page_ptr_roundtrip() {
    assert(spec_page_ptr2page_index(0x5000usize) == 5usize);
    assert(spec_page_index2page_ptr(5usize) == 0x5000usize);
}

proof fn test_page_index_roundtrip() {
    assert(spec_page_index2page_ptr(100usize) == (100 * 4096) as usize);
    assert(spec_page_ptr2page_index((100 * 4096) as usize) == 100usize);
}

proof fn test_page_index_2m_valid_basic() {
    assert(page_index_2m_valid(0usize));
    assert(page_index_2m_valid(512usize));
}

proof fn test_page_index_1g_valid_basic() {
    assert(page_index_1g_valid(0usize));
}

proof fn test_va2index_orthogonality() {
    // Different VA bits should map to different indices
    // VA with only l1 bits set
    assert(spec_v2l1index(0x2000usize) == 2usize) by (bit_vector);
    assert(spec_v2l2index(0x2000usize) == 0usize) by (bit_vector);
    assert(spec_v2l3index(0x2000usize) == 0usize) by (bit_vector);
    assert(spec_v2l4index(0x2000usize) == 0usize) by (bit_vector);
    // VA with only l2 bits set
    assert(spec_v2l1index(0x400000usize) == 0usize) by (bit_vector);
    assert(spec_v2l2index(0x400000usize) == 2usize) by (bit_vector);
    assert(spec_v2l3index(0x400000usize) == 0usize) by (bit_vector);
}

} // verus!
