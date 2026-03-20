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
// COMPLETENESS ROUND 5: Cross-function Misuse & Edge Cases — should all FAIL
// ========================================================================

// page_ptr_valid does NOT imply page_ptr_2m_valid (4096 is ptr_valid but not 2m)
proof fn test_fail_ptr_valid_implies_2m_valid(ptr: usize)
    requires page_ptr_valid(ptr),
{
    assert(page_ptr_2m_valid(ptr));
}

// page_ptr_valid does NOT imply page_ptr_1g_valid
proof fn test_fail_ptr_valid_implies_1g_valid(ptr: usize)
    requires page_ptr_valid(ptr),
{
    assert(page_ptr_1g_valid(ptr));
}

// page_index_valid does NOT imply page_index_2m_valid
proof fn test_fail_index_valid_implies_2m(i: usize)
    requires page_index_valid(i),
{
    assert(page_index_2m_valid(i));
}

// spec_page_index_truncate_2m does NOT equal identity for non-2m indices
proof fn test_fail_truncate_2m_non_identity() {
    assert(spec_page_index_truncate_2m(1usize) == 1usize); // should be 0
}

// Merge range: j==i is NOT in range (strict inequality)
proof fn test_fail_merge_2m_j_equals_i() {
    assert(spec_page_index_merge_2m_vaild(0usize, 0usize)); // requires i < j
}

// Merge range: j >= i + 0x200 is NOT in range
proof fn test_fail_merge_2m_j_out_of_range() {
    assert(spec_page_index_merge_2m_vaild(0usize, 0x200usize)); // requires j < i + 0x200
}

// All flags false does NOT mean v==0 (only low/specific bits matter)
proof fn test_fail_no_flags_means_zero(v: usize)
    requires
        !usize2present(v),
        !usize2write(v),
        !usize2user(v),
        !usize2ps(v),
{
    assert(v == 0usize); // Wrong: v could be e.g. 0x10 (bit 4 set)
}

// spec_usize2pa is NOT injective (low 12 bits stripped)
proof fn test_fail_usize2pa_injective() {
    assert(spec_usize2pa(0x1000usize) != spec_usize2pa(0x1001usize)) by (bit_vector);
}

} // end verus!
