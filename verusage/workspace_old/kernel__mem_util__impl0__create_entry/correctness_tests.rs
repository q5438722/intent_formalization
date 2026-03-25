use vstd::prelude::*;

fn main() {}

verus! {

// ===================== Type aliases =====================
pub type PAddr = usize;
pub type VAddr = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;

// ===================== Constants =====================
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

// ===================== Structs =====================
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

#[derive(Clone, Copy, Debug)]
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

// ===================== Spec functions: page entry parsing =====================
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

pub open spec fn spec_usize2page_entry(v: usize) -> PageEntry {
    PageEntry { addr: spec_usize2pa(v), perm: spec_usize2page_entry_perm(v) }
}

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

// ===================== Spec functions: VA conversion =====================
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

// ===================== Spec functions: VA validity =====================
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

// ===================== Spec functions: page ptr/index =====================
pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

pub open spec fn page_index_2m_valid(i: usize) -> bool {
    &&& i % 512 == 0
    &&& 0 <= i < NUM_PAGES
}

pub open spec fn page_index_1g_valid(i: usize) -> bool {
    &&& i % (512 * 512) as usize == 0
    &&& 0 <= i < NUM_PAGES
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize
    recommends
        page_ptr_valid(ptr),
{
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize
    recommends
        page_index_valid(i),
{
    (i * 4096) as usize
}

pub open spec fn spec_page_index_truncate_2m(index: usize) -> usize {
    (index / 512usize * 512usize) as usize
}

pub open spec fn spec_page_index_truncate_1g(index: usize) -> usize {
    (index / 512usize / 512usize * 512usize * 512usize) as usize
}

pub open spec fn page_ptr_2m_valid(ptr: usize) -> bool {
    ((ptr % (0x200000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn page_ptr_1g_valid(ptr: usize) -> bool {
    ((ptr % (0x40000000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn spec_page_index_merge_2m_vaild(i: usize, j: usize) -> bool
    recommends
        page_index_2m_valid(i),
{
    i < j < i + 0x200
}

pub open spec fn spec_page_index_merge_1g_vaild(i: usize, j: usize) -> bool
    recommends
        page_index_1g_valid(i),
{
    i < j < i + 0x40000
}

// va_lemma (external_body, trusted axiom)
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
{
}


// =========================================================================
// CORRECTNESS TESTS — all should PASS (verify successfully)
// =========================================================================

// --- Quota tests ---

proof fn test_quota_set_mem_4k_preserves_other_fields() {
    let q = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let q2 = q.spec_set_mem_4k(42);
    assert(q2.mem_4k == 42);
    assert(q2.mem_2m == 50);
    assert(q2.mem_1g == 10);
    assert(q2.pcid == 5);
    assert(q2.ioid == 3);
}

proof fn test_quota_subtract_mem_4k_correct() {
    let old_q = Quota { mem_4k: 10, mem_2m: 20, mem_1g: 5, pcid: 2, ioid: 1 };
    let new_q = Quota { mem_4k: 7, mem_2m: 20, mem_1g: 5, pcid: 2, ioid: 1 };
    assert(old_q.spec_subtract_mem_4k(new_q, 3));
}

proof fn test_quota_subtract_zero() {
    let q = Quota { mem_4k: 10, mem_2m: 20, mem_1g: 5, pcid: 2, ioid: 1 };
    assert(q.spec_subtract_mem_4k(q, 0));
}

proof fn test_quota_set_then_subtract() {
    let old_q = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_q = old_q.spec_set_mem_4k(97);
    assert(old_q.spec_subtract_mem_4k(new_q, 3));
}

// --- Page entry bit parsing tests (concrete) ---

proof fn test_usize2present_zero() {
    assert(!usize2present(0usize)) by (bit_vector);
}

proof fn test_usize2present_one() {
    assert(usize2present(1usize)) by (bit_vector);
}

proof fn test_usize2present_two() {
    assert(!usize2present(2usize)) by (bit_vector);
}

proof fn test_usize2write_zero() {
    assert(!usize2write(0usize)) by (bit_vector);
}

proof fn test_usize2write_two() {
    assert(usize2write(2usize)) by (bit_vector);
}

proof fn test_usize2write_one() {
    assert(!usize2write(1usize)) by (bit_vector);
}

proof fn test_usize2user_zero() {
    assert(!usize2user(0usize)) by (bit_vector);
}

proof fn test_usize2user_four() {
    assert(usize2user(4usize)) by (bit_vector);
}

proof fn test_usize2ps_zero() {
    assert(!usize2ps(0usize)) by (bit_vector);
}

proof fn test_usize2ps_0x80() {
    assert(usize2ps(0x80usize)) by (bit_vector);
}

proof fn test_usize2execute_disable_zero() {
    assert(!usize2execute_disable(0usize)) by (bit_vector);
}

proof fn test_usize2present_all_low_bits() {
    assert(usize2present(0xFFusize)) by (bit_vector);
}

proof fn test_usize2write_0xff() {
    assert(usize2write(0xFFusize)) by (bit_vector);
}

proof fn test_usize2user_0xff() {
    assert(usize2user(0xFFusize)) by (bit_vector);
}

proof fn test_usize2ps_0xff() {
    assert(usize2ps(0xFFusize)) by (bit_vector);
}

// --- spec_usize2pa tests ---

proof fn test_spec_usize2pa_zero() {
    assert(spec_usize2pa(0usize) == 0usize) by (bit_vector);
}

proof fn test_spec_usize2pa_page_aligned() {
    assert(spec_usize2pa(0x1000usize) == 0x1000usize) by (bit_vector);
}

proof fn test_spec_usize2pa_masks_low_bits() {
    assert(spec_usize2pa(0x1001usize) == 0x1000usize) by (bit_vector);
}

proof fn test_spec_usize2pa_preserves_mem_valid(v: usize) {
    assert(MEM_valid(spec_usize2pa(v))) by (bit_vector);
}

// --- MEM_valid tests ---

proof fn test_mem_valid_zero() {
    assert(MEM_valid(0usize)) by (bit_vector);
}

proof fn test_mem_valid_page_aligned() {
    assert(MEM_valid(0x1000usize)) by (bit_vector);
}

proof fn test_mem_valid_invalid_low_bit() {
    assert(!MEM_valid(1usize)) by (bit_vector);
}

proof fn test_mem_valid_invalid_low_bits() {
    assert(!MEM_valid(0xFFFusize)) by (bit_vector);
}

// --- PageEntry::is_empty tests ---

proof fn test_page_entry_is_empty_for_zero() {
    let entry = spec_usize2page_entry(0usize);
    assert(spec_usize2pa(0usize) == 0usize) by (bit_vector);
    assert(!usize2present(0usize)) by (bit_vector);
    assert(!usize2ps(0usize)) by (bit_vector);
    assert(!usize2write(0usize)) by (bit_vector);
    assert(!usize2execute_disable(0usize)) by (bit_vector);
    assert(!usize2user(0usize)) by (bit_vector);
    assert(entry.is_empty());
}

// --- VA index extraction (concrete values) ---

proof fn test_v2l1index_zero() {
    assert(spec_v2l1index(0usize) == 0usize) by (bit_vector);
}

proof fn test_v2l1index_0x1000() {
    assert(spec_v2l1index(0x1000usize) == 1usize) by (bit_vector);
}

proof fn test_v2l1index_max_l1() {
    assert(spec_v2l1index(0x1FF000usize) == 511usize) by (bit_vector);
}

proof fn test_v2l2index_zero() {
    assert(spec_v2l2index(0usize) == 0usize) by (bit_vector);
}

proof fn test_v2l2index_0x200000() {
    assert(spec_v2l2index(0x200000usize) == 1usize) by (bit_vector);
}

proof fn test_v2l3index_zero() {
    assert(spec_v2l3index(0usize) == 0usize) by (bit_vector);
}

proof fn test_v2l3index_0x40000000() {
    assert(spec_v2l3index(0x40000000usize) == 1usize) by (bit_vector);
}

proof fn test_v2l4index_zero() {
    assert(spec_v2l4index(0usize) == 0usize) by (bit_vector);
}

// --- Parameterized VA index bounds ---

proof fn test_v2l1index_bound(va: usize) {
    assert(spec_v2l1index(va) < 512usize) by (bit_vector);
}

proof fn test_v2l2index_bound(va: usize) {
    assert(spec_v2l2index(va) < 512usize) by (bit_vector);
}

proof fn test_v2l3index_bound(va: usize) {
    assert(spec_v2l3index(va) < 512usize) by (bit_vector);
}

proof fn test_v2l4index_bound(va: usize) {
    assert(spec_v2l4index(va) < 512usize) by (bit_vector);
}

// --- spec_va2index tests ---

proof fn test_va2index_zero() {
    let idx = spec_va2index(0usize);
    assert(spec_v2l4index(0usize) == 0usize) by (bit_vector);
    assert(spec_v2l3index(0usize) == 0usize) by (bit_vector);
    assert(spec_v2l2index(0usize) == 0usize) by (bit_vector);
    assert(spec_v2l1index(0usize) == 0usize) by (bit_vector);
    assert(idx.0 == 0 && idx.1 == 0 && idx.2 == 0 && idx.3 == 0);
}

// --- VA validity parameterized tests ---

proof fn test_va_4k_valid_implies_l4_ge_1(va: VAddr)
    requires spec_va_4k_valid(va)
{
    assert((va as u64 >> 39u64 & 0x1ffu64) >= KERNEL_MEM_END_L4INDEX as u64);
}

proof fn test_va_2m_valid_implies_4k_valid(va: VAddr)
    requires spec_va_2m_valid(va)
{
    // 2m-valid means aligned to 2m boundary, which is stricter than 4k alignment
    // MEM_2m_MASK bits are a subset of MEM_4k_MASK bits
    assert(va & (!MEM_2m_MASK) as usize == 0 ==> va & (!MEM_4k_MASK) as usize == 0)
        by (bit_vector);
    assert(spec_va_4k_valid(va));
}

// --- Page ptr/index conversion tests ---

proof fn test_page_ptr2index_zero() {
    assert(spec_page_ptr2page_index(0usize) == 0usize);
}

proof fn test_page_ptr2index_4096() {
    assert(spec_page_ptr2page_index(4096usize) == 1usize);
}

proof fn test_page_ptr2index_8192() {
    assert(spec_page_ptr2page_index(8192usize) == 2usize);
}

proof fn test_page_index2ptr_zero() {
    assert(spec_page_index2page_ptr(0usize) == 0usize);
}

proof fn test_page_index2ptr_one() {
    assert(spec_page_index2page_ptr(1usize) == 4096usize);
}

proof fn test_page_index2ptr_two() {
    assert(spec_page_index2page_ptr(2usize) == 8192usize);
}

// --- Page validity predicates ---

proof fn test_page_ptr_valid_zero() {
    assert(page_ptr_valid(0usize));
}

proof fn test_page_ptr_valid_4096() {
    assert(page_ptr_valid(4096usize));
}

proof fn test_page_ptr_valid_invalid_alignment() {
    assert(!page_ptr_valid(1usize));
}

proof fn test_page_index_valid_zero() {
    assert(page_index_valid(0usize));
}

proof fn test_page_index_valid_max_minus_1() {
    assert(page_index_valid((NUM_PAGES - 1) as usize));
}

proof fn test_page_index_invalid_at_max() {
    assert(!page_index_valid(NUM_PAGES as usize));
}

// --- Page index truncation tests ---

proof fn test_truncate_2m_aligned() {
    assert(spec_page_index_truncate_2m(512usize) == 512usize);
}

proof fn test_truncate_2m_unaligned() {
    assert(spec_page_index_truncate_2m(513usize) == 512usize);
}

proof fn test_truncate_2m_zero() {
    assert(spec_page_index_truncate_2m(0usize) == 0usize);
}

proof fn test_truncate_2m_below_boundary() {
    assert(spec_page_index_truncate_2m(511usize) == 0usize);
}

proof fn test_truncate_1g_aligned() {
    assert(spec_page_index_truncate_1g(262144usize) == 262144usize);
}

proof fn test_truncate_1g_unaligned() {
    assert(spec_page_index_truncate_1g(262145usize) == 262144usize);
}

proof fn test_truncate_1g_zero() {
    assert(spec_page_index_truncate_1g(0usize) == 0usize);
}

// --- page_index_2m_valid / page_index_1g_valid ---

proof fn test_page_index_2m_valid_zero() {
    assert(page_index_2m_valid(0usize));
}

proof fn test_page_index_2m_valid_512() {
    assert(page_index_2m_valid(512usize));
}

proof fn test_page_index_2m_valid_not_aligned() {
    assert(!page_index_2m_valid(1usize));
}

proof fn test_page_index_1g_valid_zero() {
    assert(page_index_1g_valid(0usize));
}

proof fn test_page_index_1g_valid_zero_is_valid() {
    // 0 % anything == 0, and 0 < NUM_PAGES
    assert(page_index_1g_valid(0usize));
}

// --- page_ptr_2m_valid / page_ptr_1g_valid ---

proof fn test_page_ptr_2m_valid_zero() {
    assert(page_ptr_2m_valid(0usize));
}

proof fn test_page_ptr_2m_valid_2m() {
    assert(page_ptr_2m_valid(0x200000usize));
}

// --- va_lemma usage tests ---

proof fn test_va_lemma_4k_index_bounds(va: VAddr)
    requires spec_va_4k_valid(va)
{
    va_lemma();
    assert(0 <= spec_v2l4index(va) < 512);
    assert(0 <= spec_v2l3index(va) < 512);
    assert(0 <= spec_v2l2index(va) < 512);
    assert(0 <= spec_v2l1index(va) < 512);
}

proof fn test_va_lemma_2m_l1_is_zero(va: VAddr)
    requires spec_va_2m_valid(va)
{
    va_lemma();
    assert(0 <= spec_v2l4index(va) < 512);
    assert(0 <= spec_v2l3index(va) < 512);
    assert(0 <= spec_v2l2index(va) < 512);
    assert(0 == spec_v2l1index(va));
}

proof fn test_va_lemma_1g_l1_l2_are_zero(va: VAddr)
    requires spec_va_1g_valid(va)
{
    va_lemma();
    assert(0 <= spec_v2l4index(va) < 512);
    assert(0 <= spec_v2l3index(va) < 512);
    assert(0 == spec_v2l2index(va));
    assert(0 == spec_v2l1index(va));
}

// --- Parameterized tests for bit parsing ---

proof fn test_param_present_implies_nonzero(v: usize)
    requires usize2present(v)
{
    assert(v != 0usize) by (bit_vector)
        requires (v & PAGE_ENTRY_PRESENT_MASK as usize) != 0;
}

proof fn test_param_all_false_means_zero_low_bits(v: usize)
    requires
        !usize2present(v),
        !usize2write(v),
        !usize2user(v),
{
    assert(v & 7usize == 0usize) by (bit_vector)
        requires
            (v & PAGE_ENTRY_PRESENT_MASK as usize) == 0,
            (v & PAGE_ENTRY_WRITE_MASK as usize) == 0,
            (v & PAGE_ENTRY_USER_MASK as usize) == 0;
}

// --- Roundtrip page ptr/index ---

proof fn test_roundtrip_index_to_ptr(i: usize)
    requires
        page_index_valid(i),
{
    let ptr = spec_page_index2page_ptr(i);
    assert(ptr == (i * 4096) as usize);
}

// --- spec_page_index_merge tests ---

proof fn test_merge_2m_valid_in_range() {
    assert(spec_page_index_merge_2m_vaild(0usize, 1usize));
}

proof fn test_merge_2m_valid_upper_bound() {
    assert(spec_page_index_merge_2m_vaild(0usize, 0x1FFusize));
}

proof fn test_merge_1g_valid_in_range() {
    assert(spec_page_index_merge_1g_vaild(0usize, 1usize));
}

proof fn test_merge_1g_valid_upper_bound() {
    assert(spec_page_index_merge_1g_vaild(0usize, 0x3FFFFusize));
}

// --- Combined page entry parsing test ---

proof fn test_page_entry_perm_present_write() {
    let perm = spec_usize2page_entry_perm(3usize);
    assert(usize2present(3usize)) by (bit_vector);
    assert(usize2write(3usize)) by (bit_vector);
    assert(!usize2user(3usize)) by (bit_vector);
    assert(!usize2ps(3usize)) by (bit_vector);
    assert(!usize2execute_disable(3usize)) by (bit_vector);
    assert(perm.present == true);
    assert(perm.write == true);
    assert(perm.user == false);
    assert(perm.ps == false);
    assert(perm.execute_disable == false);
}

proof fn test_page_entry_perm_present_write_user_ps() {
    // v = 0x87 = bits 0,1,2,7 set → present, write, user, ps (no execute_disable since bit 63 is architecture-dependent)
    let v: usize = 0x87usize;
    assert(usize2present(0x87usize)) by (bit_vector);
    assert(usize2write(0x87usize)) by (bit_vector);
    assert(usize2user(0x87usize)) by (bit_vector);
    assert(usize2ps(0x87usize)) by (bit_vector);
    assert(!usize2execute_disable(0x87usize)) by (bit_vector);
    let perm = spec_usize2page_entry_perm(v);
    assert(perm.present);
    assert(perm.write);
    assert(perm.user);
    assert(perm.ps);
    assert(!perm.execute_disable);
}

// --- Quota::spec_subtract_mem_4k consistency with create_entry's postcondition ---
// create_entry guarantees: old_quota.spec_subtract_mem_4k(new_quota, ret.0)
// where ret.0 <= 3. Test that this relationship is coherent.

proof fn test_quota_subtract_bounds(k: usize)
    requires k <= 3
{
    let old_q = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_q = old_q.spec_set_mem_4k((100 - k) as usize);
    assert(old_q.spec_subtract_mem_4k(new_q, k));
    assert(new_q.mem_4k >= 97);
}

// --- Additional edge-case tests ---

proof fn test_page_entry_not_empty_with_addr() {
    // A page entry with a non-zero address (but all perms false) is not empty
    let entry = PageEntry {
        addr: 0x1000,
        perm: PageEntryPerm {
            present: false,
            ps: false,
            write: false,
            execute_disable: false,
            user: false,
        },
    };
    assert(!entry.is_empty());
}

proof fn test_page_entry_not_empty_with_present() {
    let entry = PageEntry {
        addr: 0,
        perm: PageEntryPerm {
            present: true,
            ps: false,
            write: false,
            execute_disable: false,
            user: false,
        },
    };
    assert(!entry.is_empty());
}


} // verus!
