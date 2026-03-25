use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus! {

// ========== Type aliases ==========
pub type PAddr = usize;
pub type VAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;

// ========== Constants ==========
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;
pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x2;
pub const PAGE_ENTRY_USER_MASK: u64 = 0x4;
pub const PAGE_ENTRY_PS_MASK: u64 = 0x80;
pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x8000_0000_0000_0000;

// ========== Structs ==========
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

// ========== Spec functions ==========
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

pub open spec fn spec_usize2page_entry(v: usize) -> PageEntry {
    PageEntry { addr: spec_usize2pa(v), perm: spec_usize2page_entry_perm(v) }
}

// ========== External body functions ==========
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

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_usize2pa))]
pub fn usize2pa(v: usize) -> (ret: PAddr)
    ensures
        ret =~= spec_usize2pa(v),
        MEM_valid(ret),
{
    unimplemented!()
}

// ========== Page utility spec functions ==========
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

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_page_ptr2page_index))]
pub fn page_ptr2page_index(ptr: usize) -> (ret: usize)
    requires ptr % 0x1000 == 0,
    ensures ret == spec_page_ptr2page_index(ptr),
{
    unimplemented!()
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_page_index2page_ptr))]
pub fn page_index2page_ptr(i: usize) -> (ret: usize)
    requires 0 <= i < NUM_PAGES,
    ensures ret == spec_page_index2page_ptr(i),
{
    unimplemented!()
}

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

pub open spec fn page_ptr_2m_valid(ptr: usize) -> bool {
    ((ptr % 0x200000) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn page_ptr_1g_valid(ptr: usize) -> bool {
    ((ptr % 0x40000000) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn spec_page_index_truncate_2m(index: usize) -> usize {
    (index / 512usize * 512usize) as usize
}

pub open spec fn spec_page_index_merge_2m_vaild(i: usize, j: usize) -> bool
    recommends page_index_2m_valid(i),
{
    i < j < i + 0x200
}

pub open spec fn spec_page_index_merge_1g_vaild(i: usize, j: usize) -> bool
    recommends page_index_1g_valid(i),
{
    i < j < i + 0x40000
}

// ========================================================================
// CORRECTNESS TESTS — These should all PASS
// ========================================================================

// ---- Bit flag extraction: zero yields false for all flags ----
proof fn test_usize2present_zero() {
    assert(!usize2present(0usize)) by (bit_vector);
}

proof fn test_usize2write_zero() {
    assert(!usize2write(0usize)) by (bit_vector);
}

proof fn test_usize2user_zero() {
    assert(!usize2user(0usize)) by (bit_vector);
}

proof fn test_usize2ps_zero() {
    assert(!usize2ps(0usize)) by (bit_vector);
}

proof fn test_usize2execute_disable_zero() {
    assert(!usize2execute_disable(0usize)) by (bit_vector);
}

// ---- Bit flag extraction: correct bit set yields true ----
proof fn test_usize2present_one() {
    assert(usize2present(1usize)) by (bit_vector);
}

proof fn test_usize2write_two() {
    assert(usize2write(2usize)) by (bit_vector);
}

proof fn test_usize2user_four() {
    assert(usize2user(4usize)) by (bit_vector);
}

proof fn test_usize2ps_128() {
    assert(usize2ps(128usize)) by (bit_vector);
}

// ---- Bit flag extraction: wrong bit set yields false ----
proof fn test_usize2present_not_two() {
    assert(!usize2present(2usize)) by (bit_vector);
}

proof fn test_usize2write_not_one() {
    assert(!usize2write(1usize)) by (bit_vector);
}

proof fn test_usize2user_not_one() {
    assert(!usize2user(1usize)) by (bit_vector);
}

proof fn test_usize2ps_not_one() {
    assert(!usize2ps(1usize)) by (bit_vector);
}

// ---- Multiple flags: v=3 has present+write ----
proof fn test_perm_v3_present_write() {
    assert(usize2present(3usize)) by (bit_vector);
    assert(usize2write(3usize)) by (bit_vector);
    assert(!usize2ps(3usize)) by (bit_vector);
    assert(!usize2user(3usize)) by (bit_vector);
}

// ---- Multiple flags: v=7 has present+write+user ----
proof fn test_perm_v7() {
    assert(usize2present(7usize)) by (bit_vector);
    assert(usize2write(7usize)) by (bit_vector);
    assert(usize2user(7usize)) by (bit_vector);
    assert(!usize2ps(7usize)) by (bit_vector);
}

// ---- spec_usize2pa tests ----
proof fn test_usize2pa_zero() {
    assert(spec_usize2pa(0usize) == 0usize) by (bit_vector);
}

proof fn test_usize2pa_aligned() {
    assert(spec_usize2pa(0x1000usize) == 0x1000usize) by (bit_vector);
}

proof fn test_usize2pa_strips_low_bits() {
    assert(spec_usize2pa(0x1FFFusize) == 0x1000usize) by (bit_vector);
}

proof fn test_usize2pa_preserves_middle() {
    assert(spec_usize2pa(0xABCD000usize) == 0xABCD000usize) by (bit_vector);
}

// ---- MEM_valid tests ----
proof fn test_mem_valid_zero() {
    assert(MEM_valid(0usize)) by (bit_vector);
}

proof fn test_mem_valid_4k() {
    assert(MEM_valid(0x1000usize)) by (bit_vector);
}

proof fn test_mem_valid_5k() {
    assert(MEM_valid(0x5000usize)) by (bit_vector);
}

// ---- spec_usize2page_entry: v=0 is empty ----
proof fn test_entry_v0_is_empty() {
    assert(!usize2present(0usize)) by (bit_vector);
    assert(!usize2write(0usize)) by (bit_vector);
    assert(!usize2user(0usize)) by (bit_vector);
    assert(!usize2ps(0usize)) by (bit_vector);
    assert(!usize2execute_disable(0usize)) by (bit_vector);
    assert(spec_usize2pa(0usize) == 0usize) by (bit_vector);

    let entry = spec_usize2page_entry(0usize);
    assert(entry.is_empty());
}

// ---- PageEntry::is_empty: constructed empty entry ----
proof fn test_page_entry_is_empty() {
    let entry = PageEntry {
        addr: 0,
        perm: PageEntryPerm { present: false, ps: false, write: false, execute_disable: false, user: false },
    };
    assert(entry.is_empty());
}

// ---- PageEntry::is_empty: non-empty cases ----
proof fn test_page_entry_not_empty_present() {
    let entry = PageEntry {
        addr: 0,
        perm: PageEntryPerm { present: true, ps: false, write: false, execute_disable: false, user: false },
    };
    assert(!entry.is_empty());
}

proof fn test_page_entry_not_empty_addr() {
    let entry = PageEntry {
        addr: 0x1000,
        perm: PageEntryPerm { present: false, ps: false, write: false, execute_disable: false, user: false },
    };
    assert(!entry.is_empty());
}

proof fn test_page_entry_not_empty_write() {
    let entry = PageEntry {
        addr: 0,
        perm: PageEntryPerm { present: false, ps: false, write: true, execute_disable: false, user: false },
    };
    assert(!entry.is_empty());
}

proof fn test_page_entry_not_empty_ps() {
    let entry = PageEntry {
        addr: 0,
        perm: PageEntryPerm { present: false, ps: true, write: false, execute_disable: false, user: false },
    };
    assert(!entry.is_empty());
}

proof fn test_page_entry_not_empty_user() {
    let entry = PageEntry {
        addr: 0,
        perm: PageEntryPerm { present: false, ps: false, write: false, execute_disable: false, user: true },
    };
    assert(!entry.is_empty());
}

proof fn test_page_entry_not_empty_exec_disable() {
    let entry = PageEntry {
        addr: 0,
        perm: PageEntryPerm { present: false, ps: false, write: false, execute_disable: true, user: false },
    };
    assert(!entry.is_empty());
}

// ---- page_ptr_valid tests ----
proof fn test_page_ptr_valid_zero() {
    assert(page_ptr_valid(0usize));
}

proof fn test_page_ptr_valid_4096() {
    assert(page_ptr_valid(4096usize));
}

proof fn test_page_ptr_valid_large() {
    assert(page_ptr_valid(0x64000usize));
}

// ---- page_index_valid tests ----
proof fn test_page_index_valid_zero() {
    assert(page_index_valid(0usize));
}

proof fn test_page_index_valid_one() {
    assert(page_index_valid(1usize));
}

proof fn test_page_index_valid_max() {
    assert(page_index_valid(0x1FFFFFusize));
}

// ---- page_index_2m_valid tests ----
proof fn test_page_index_2m_valid_zero() {
    assert(page_index_2m_valid(0usize));
}

proof fn test_page_index_2m_valid_512() {
    assert(page_index_2m_valid(512usize));
}

proof fn test_page_index_2m_valid_1024() {
    assert(page_index_2m_valid(1024usize));
}

// ---- page_index_1g_valid ----
proof fn test_page_index_1g_valid_zero() {
    assert(page_index_1g_valid(0usize));
}

// ---- page_ptr_2m_valid / page_ptr_1g_valid ----
proof fn test_page_ptr_2m_valid_zero() {
    assert(page_ptr_2m_valid(0usize));
}

proof fn test_page_ptr_1g_valid_zero() {
    assert(page_ptr_1g_valid(0usize));
}

// ---- spec_page_ptr2page_index concrete ----
proof fn test_ptr2index_zero() {
    assert(spec_page_ptr2page_index(0usize) == 0usize);
}

proof fn test_ptr2index_4096() {
    assert(spec_page_ptr2page_index(4096usize) == 1usize);
}

proof fn test_ptr2index_8192() {
    assert(spec_page_ptr2page_index(8192usize) == 2usize);
}

// ---- spec_page_index2page_ptr concrete ----
proof fn test_index2ptr_zero() {
    assert(spec_page_index2page_ptr(0usize) == 0usize);
}

proof fn test_index2ptr_one() {
    assert(spec_page_index2page_ptr(1usize) == 4096usize);
}

proof fn test_index2ptr_two() {
    assert(spec_page_index2page_ptr(2usize) == 8192usize);
}

// ---- Implication tests ----
proof fn test_ptr_valid_implies_aligned(ptr: usize)
    requires page_ptr_valid(ptr),
{
    assert(ptr % 0x1000 == 0);
}

proof fn test_2m_implies_valid(i: usize)
    requires page_index_2m_valid(i),
{
    assert(page_index_valid(i));
}

proof fn test_1g_implies_valid(i: usize)
    requires page_index_1g_valid(i),
{
    assert(page_index_valid(i));
}

proof fn test_valid_ptr_gives_valid_index(ptr: usize)
    requires page_ptr_valid(ptr),
{
    assert(page_index_valid(spec_page_ptr2page_index(ptr)));
}

// ---- Truncation ----
proof fn test_truncate_2m_identity(i: usize)
    requires page_index_2m_valid(i),
{
    assert(spec_page_index_truncate_2m(i) == i);
}

// ---- Merge range tests ----
proof fn test_merge_2m_valid_range() {
    assert(spec_page_index_merge_2m_vaild(0usize, 1usize));
    assert(spec_page_index_merge_2m_vaild(0usize, 0x1FFusize));
}

proof fn test_merge_1g_valid_range() {
    assert(spec_page_index_merge_1g_vaild(0usize, 1usize));
}

} // end verus!
