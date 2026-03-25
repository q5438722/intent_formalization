use vstd::prelude::*;

fn main() {}

verus! {

// ===== Type aliases =====
pub type PAddr = usize;
pub type Pcid = usize;

// ===== Constants =====
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;
pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;
pub const PAGE_ENTRY_USER_SHIFT: u64 = 2;
pub const PAGE_ENTRY_PS_SHIFT: u64 = 7;
pub const PAGE_ENTRY_EXECUTE_SHIFT: u64 = 63;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;
pub const PAGE_ENTRY_USER_MASK: u64 = 0x1u64 << PAGE_ENTRY_USER_SHIFT;
pub const PAGE_ENTRY_PS_MASK: u64 = 0x1u64 << PAGE_ENTRY_PS_SHIFT;
pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x1u64 << PAGE_ENTRY_EXECUTE_SHIFT;

// ===== Open spec functions =====

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
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
    ((ptr % (0x200000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn page_ptr_1g_valid(ptr: usize) -> bool {
    ((ptr % (0x40000000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn spec_page_index_truncate_2m(index: usize) -> usize {
    (index / 512usize * 512usize) as usize
}

pub open spec fn spec_page_index_truncate_1g(index: usize) -> usize {
    (index / 512usize / 512usize * 512usize * 512usize) as usize
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

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

// ===== Structs =====

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
    PageEntry { addr: usize2pa(v), perm: usize2page_entry_perm(v) }
}

// ===== External body functions =====

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

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_page_ptr2page_index))]
pub fn page_ptr2page_index(ptr: usize) -> (ret: usize)
    requires
        ptr % 0x1000 == 0,
    ensures
        ret == spec_page_ptr2page_index(ptr),
{
    unimplemented!()
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_page_index2page_ptr))]
pub fn page_index2page_ptr(i: usize) -> (ret: usize)
    requires
        0 <= i < NUM_PAGES,
    ensures
        ret == spec_page_index2page_ptr(i),
{
    unimplemented!()
}

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_va_4k_valid))]
pub fn va_4k_valid(va: usize) -> (ret: bool)
    ensures
        ret == spec_va_4k_valid(va),
{
    unimplemented!()
}

// SyscallReturnStruct and related types

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwitchDecision {
    NoSwitch,
    NoThread,
    Switch,
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

#[derive(Clone, Copy)]
pub struct SyscallReturnStruct {
    pub error_code: RetValueType,
    pub pcid: Option<Pcid>,
    pub cr3: Option<usize>,
    pub switch_decision: SwitchDecision,
}

impl SyscallReturnStruct {
    #[verifier::external_body]
    pub fn NoSwitchNew(error_code: RetValueType) -> (ret: Self)
        ensures
            ret.error_code == error_code,
            ret.pcid.is_None(),
            ret.cr3.is_None(),
            ret.switch_decision == SwitchDecision::NoSwitch,
    {
        unimplemented!()
    }
}


// =========================================================
// CORRECTNESS TESTS - All should PASS (verify successfully)
// =========================================================

// --- Page validity with concrete values ---

proof fn test_page_ptr_valid_zero() {
    assert(page_ptr_valid(0usize));
}

proof fn test_page_ptr_valid_4096() {
    assert(page_ptr_valid(4096usize));
}

proof fn test_page_ptr_valid_8192() {
    assert(page_ptr_valid(8192usize));
}

proof fn test_page_ptr_invalid_unaligned() {
    assert(!page_ptr_valid(4095usize));
}

proof fn test_page_ptr_invalid_1() {
    assert(!page_ptr_valid(1usize));
}

proof fn test_page_index_valid_zero() {
    assert(page_index_valid(0usize));
}

proof fn test_page_index_valid_one() {
    assert(page_index_valid(1usize));
}

proof fn test_page_index_valid_max_minus_1() {
    assert(page_index_valid((NUM_PAGES - 1) as usize));
}

proof fn test_page_index_invalid_num_pages() {
    assert(!page_index_valid(NUM_PAGES));
}

// --- Page ptr/index conversion with concrete values ---

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

// --- 2M/1G page validity ---

proof fn test_page_index_2m_valid_zero() {
    assert(page_index_2m_valid(0usize));
}

proof fn test_page_index_2m_valid_512() {
    assert(page_index_2m_valid(512usize));
}

proof fn test_page_index_2m_valid_1024() {
    assert(page_index_2m_valid(1024usize));
}

proof fn test_page_index_2m_invalid_1() {
    assert(!page_index_2m_valid(1usize));
}

proof fn test_page_index_2m_invalid_511() {
    assert(!page_index_2m_valid(511usize));
}

proof fn test_page_index_1g_valid_zero() {
    assert(page_index_1g_valid(0usize));
}

proof fn test_page_index_1g_invalid_num_pages() {
    // NUM_PAGES is not 1g-valid because NUM_PAGES is out of range
    assert(!page_index_1g_valid(NUM_PAGES));
}

// --- 2M/1G ptr validity ---

proof fn test_page_ptr_2m_valid_zero() {
    assert(page_ptr_2m_valid(0usize));
}

proof fn test_page_ptr_2m_valid_2m() {
    assert(page_ptr_2m_valid(0x200000usize));
}

proof fn test_page_ptr_2m_invalid_4096() {
    assert(!page_ptr_2m_valid(4096usize));
}

proof fn test_page_ptr_1g_valid_zero() {
    assert(page_ptr_1g_valid(0usize));
}

// --- Truncation ---

proof fn test_truncate_2m_zero() {
    assert(spec_page_index_truncate_2m(0usize) == 0usize);
}

proof fn test_truncate_2m_512() {
    assert(spec_page_index_truncate_2m(512usize) == 512usize);
}

proof fn test_truncate_2m_513() {
    assert(spec_page_index_truncate_2m(513usize) == 512usize);
}

proof fn test_truncate_2m_1023() {
    assert(spec_page_index_truncate_2m(1023usize) == 512usize);
}

proof fn test_truncate_1g_zero() {
    assert(spec_page_index_truncate_1g(0usize) == 0usize);
}

// --- PageEntry is_empty ---

proof fn test_page_entry_is_empty() {
    let entry = PageEntry {
        addr: 0,
        perm: PageEntryPerm {
            present: false,
            ps: false,
            write: false,
            execute_disable: false,
            user: false,
        },
    };
    assert(entry.is_empty());
}

proof fn test_page_entry_not_empty_present() {
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

proof fn test_page_entry_not_empty_addr() {
    let entry = PageEntry {
        addr: 4096,
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

proof fn test_page_entry_not_empty_write() {
    let entry = PageEntry {
        addr: 0,
        perm: PageEntryPerm {
            present: false,
            ps: false,
            write: true,
            execute_disable: false,
            user: false,
        },
    };
    assert(!entry.is_empty());
}

// --- Parameterized tests ---

proof fn test_param_page_index_valid(i: usize)
    requires
        0 <= i < NUM_PAGES,
{
    assert(page_index_valid(i));
}

proof fn test_param_page_ptr_valid(ptr: usize)
    requires
        ptr % 0x1000 == 0,
        ptr / 0x1000 < NUM_PAGES,
{
    assert(page_ptr_valid(ptr));
}

proof fn test_param_page_index_2m_implies_valid(i: usize)
    requires
        page_index_2m_valid(i),
{
    assert(page_index_valid(i));
}

// --- NoSwitchNew exec tests ---

fn test_no_switch_new_error() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::Error);
    assert(ret.error_code == RetValueType::Error);
    assert(ret.pcid.is_None());
    assert(ret.cr3.is_None());
    assert(ret.switch_decision == SwitchDecision::NoSwitch);
}

fn test_no_switch_new_no_quota() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::ErrorNoQuota);
    assert(ret.error_code == RetValueType::ErrorNoQuota);
    assert(ret.pcid.is_None());
    assert(ret.cr3.is_None());
    assert(ret.switch_decision == SwitchDecision::NoSwitch);
}

fn test_no_switch_new_va_in_use() {
    let ret = SyscallReturnStruct::NoSwitchNew(RetValueType::ErrorVaInUse);
    assert(ret.error_code == RetValueType::ErrorVaInUse);
    assert(ret.pcid.is_None());
}

// --- External body function ensures tests ---

fn test_usize2page_entry_perm_zero() {
    let perm = usize2page_entry_perm(0usize);
    assert(perm.present == false);
    assert(perm.ps == false);
    assert(perm.write == false);
    assert(perm.execute_disable == false);
    assert(perm.user == false);
}

fn test_usize2page_entry_zero() {
    let entry = usize2page_entry(0usize);
    assert(entry.addr == 0);
    assert(entry.perm.present == false);
    assert(entry.perm.ps == false);
    assert(entry.perm.write == false);
    assert(entry.perm.execute_disable == false);
    assert(entry.perm.user == false);
}

fn test_usize2pa_mem_valid() {
    let pa = usize2pa(0usize);
    assert(MEM_valid(pa));
}

fn test_usize2pa_nonzero_mem_valid() {
    let pa = usize2pa(0x1000usize);
    assert(MEM_valid(pa));
}

fn test_usize2pa_large_mem_valid() {
    let pa = usize2pa(0xFFFFusize);
    assert(MEM_valid(pa));
}

fn test_exec_page_ptr2index_zero() {
    let idx = page_ptr2page_index(0usize);
    assert(idx == spec_page_ptr2page_index(0usize));
}

fn test_exec_page_ptr2index_4096() {
    let idx = page_ptr2page_index(4096usize);
    assert(idx == spec_page_ptr2page_index(4096usize));
}

fn test_exec_page_index2ptr_zero() {
    let ptr = page_index2page_ptr(0usize);
    assert(ptr == spec_page_index2page_ptr(0usize));
}

fn test_exec_page_index2ptr_one() {
    let ptr = page_index2page_ptr(1usize);
    assert(ptr == spec_page_index2page_ptr(1usize));
}

fn test_exec_page_index2ptr_100() {
    let ptr = page_index2page_ptr(100usize);
    assert(ptr == spec_page_index2page_ptr(100usize));
}

} // verus!
