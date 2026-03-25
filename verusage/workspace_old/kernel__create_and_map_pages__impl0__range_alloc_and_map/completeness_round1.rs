use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus! {

pub type PAddr = usize;
pub type VAddr = usize;
pub type PagePtr = usize;
pub type ProcPtr = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;
pub const PAGE_ENTRY_USER_SHIFT: u64 = 2;
pub const PAGE_ENTRY_PS_SHIFT: u64 = 7;
pub const PAGE_ENTRY_EXECUTE_SHIFT: u64 = 63;
pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;
pub const PAGE_ENTRY_USER_MASK: u64 = 0x1u64 << PAGE_ENTRY_USER_SHIFT;
pub const PAGE_ENTRY_PS_MASK: u64 = 0x1u64 << PAGE_ENTRY_PS_SHIFT;
pub const PAGE_ENTRY_EXECUTE_MASK: u64 = 0x1u64 << PAGE_ENTRY_EXECUTE_SHIFT;

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
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

pub open spec fn spec_usize2page_entry(v: usize) -> PageEntry {
    PageEntry { addr: usize2pa(v), perm: usize2page_entry_perm(v) }
}

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
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

#[verifier::external_body]
#[verifier(when_used_as_spec(spec_usize2pa))]
pub fn usize2pa(v: usize) -> (ret: PAddr)
    ensures
        ret =~= spec_usize2pa(v),
        MEM_valid(ret),
{
    unimplemented!()
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

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
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

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
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

#[verifier(external_body)]
pub proof fn set_lemma<A>()
    ensures
        forall|s1: Set<A>, s2: Set<A>, e: A|
            (s1 + s2).insert(e) == s1 + (s2.insert(e)) && s1 + (s2.insert(e)) == s2 + (s1.insert(e))
                && (s1 + s2).insert(e) == s2 + (s1.insert(e)) && (!(s1 + s2).contains(e)
                <==> !s1.contains(e) && !s2.contains(e)),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn seq_push_lemma<A>()
    ensures
        forall|s: Seq<A>, v: A, x: A|
            s.contains(x) ==> s.push(v).contains(v) && s.push(v).contains(x),
        forall|s: Seq<A>, v: A| #![auto] s.push(v).contains(v),
        forall|s: Seq<A>, v: A, x: A| !s.contains(x) && v != x ==> !s.push(v).contains(x),
{
    unimplemented!()
}

#[verifier::external_body]
pub proof fn map_insert_lemma<A, B>()
    ensures
        forall|m: Map<A, B>, x: A, y: A, v: B| x != y ==> m.insert(x, v)[y] == m[y],
{
    unimplemented!()
}

// ============================================================
// Round 1: Precondition Violations — all should FAIL
// ============================================================

// Test: map_insert_lemma used with x == y (violates x != y precondition in ensures)
// The lemma doesn't require x != y as a precondition, but the ensures only
// guarantees the property when x != y. Asserting it when x == y should fail.
proof fn test_map_insert_same_key()
{
    map_insert_lemma::<int, int>();
    let m = Map::<int, int>::empty().insert(1, 100);
    let x: int = 2;
    let y: int = 2; // x == y
    let v: int = 200;
    // The ensures says: x != y ==> m.insert(x, v)[y] == m[y]
    // With x == y, the antecedent is false, so we can't conclude anything
    // Asserting m.insert(x, v)[y] == m[y] when x == y should fail (v != m[y])
    assert(m.insert(x, v)[y] == m[y]);
}

// Test: page_ptr_valid for unaligned pointer (violates alignment)
proof fn test_page_ptr_valid_unaligned()
{
    assert(page_ptr_valid(0x1001usize)); // not 4k-aligned, should fail
}

// Test: page_index_valid for out-of-range index
proof fn test_page_index_valid_out_of_range()
{
    assert(page_index_valid(NUM_PAGES)); // exactly NUM_PAGES, should fail
}

// Test: page_index_2m_valid for non-512-aligned index
proof fn test_page_index_2m_valid_unaligned()
{
    assert(page_index_2m_valid(1usize)); // 1 % 512 != 0, should fail
}

// Test: page_index_1g_valid for non-262144-aligned index
proof fn test_page_index_1g_valid_unaligned()
{
    assert(page_index_1g_valid(512usize)); // 512 % (512*512) != 0, should fail
}

// Test: page_ptr_2m_valid for non-2MB-aligned pointer
proof fn test_page_ptr_2m_valid_unaligned()
{
    assert(page_ptr_2m_valid(0x1000usize)); // 0x1000 % 0x200000 != 0, should fail
}

// Test: page_ptr_1g_valid for non-1GB-aligned pointer
proof fn test_page_ptr_1g_valid_unaligned()
{
    assert(page_ptr_1g_valid(0x200000usize)); // 0x200000 % 0x40000000 != 0, should fail
}

} // verus!
