use vstd::prelude::*;
use vstd::simple_pptr::*;

fn main() {}

verus! {

// ============================================================
// Minimal definitions from the target file needed for testing
// ============================================================

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

// Quota definition
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

// Page utility spec functions
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

// Helper proof functions from the target
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
// Correctness Tests — these should all PASS
// ============================================================

// --- Tests for set_lemma ---

// Test: set union with insert is associative
proof fn test_set_lemma_union_insert_assoc()
{
    set_lemma::<int>();
    let s1 = Set::<int>::empty().insert(1).insert(2);
    let s2 = Set::<int>::empty().insert(3).insert(4);
    let e: int = 5;
    assert((s1 + s2).insert(e) == s1 + (s2.insert(e)));
}

// Test: set union with insert commutative
proof fn test_set_lemma_union_insert_commut()
{
    set_lemma::<int>();
    let s1 = Set::<int>::empty().insert(10);
    let s2 = Set::<int>::empty().insert(20);
    let e: int = 30;
    assert(s1 + (s2.insert(e)) == s2 + (s1.insert(e)));
}

// Test: non-containment distributes over union
proof fn test_set_lemma_non_containment()
{
    set_lemma::<int>();
    let s1 = Set::<int>::empty().insert(1);
    let s2 = Set::<int>::empty().insert(2);
    let e: int = 3;
    assert(!(s1 + s2).contains(e) <==> !s1.contains(e) && !s2.contains(e));
}

// Test: set_lemma with empty sets
proof fn test_set_lemma_empty_sets()
{
    set_lemma::<usize>();
    let s1 = Set::<usize>::empty();
    let s2 = Set::<usize>::empty();
    let e: usize = 42;
    assert((s1 + s2).insert(e) == s1 + (s2.insert(e)));
    assert(!(s1 + s2).contains(e) <==> !s1.contains(e) && !s2.contains(e));
}

// Test: set_lemma with PagePtr type (as used in target)
proof fn test_set_lemma_page_ptr_type()
{
    set_lemma::<PagePtr>();
    let s1 = Set::<PagePtr>::empty().insert(0x1000usize);
    let s2 = Set::<PagePtr>::empty().insert(0x2000usize);
    let e: PagePtr = 0x3000usize;
    assert((s1 + s2).insert(e) == s1 + (s2.insert(e)));
}

// --- Tests for seq_push_lemma ---

// Test: push preserves existing containment
proof fn test_seq_push_preserves_containment()
{
    seq_push_lemma::<int>();
    let s = seq![1int, 2, 3];
    let v: int = 4;
    let x: int = 2;
    assert(s.contains(x));
    assert(s.push(v).contains(v));
    assert(s.push(v).contains(x));
}

// Test: push result contains pushed element
proof fn test_seq_push_contains_new()
{
    seq_push_lemma::<int>();
    let s = seq![10int, 20];
    let v: int = 30;
    assert(s.push(v).contains(v));
}

// Test: push of non-member with different value stays non-member
proof fn test_seq_push_non_member()
{
    seq_push_lemma::<int>();
    let s = seq![1int, 2, 3];
    let v: int = 4;
    let x: int = 5;
    assert(!s.contains(x));
    assert(v != x);
    assert(!s.push(v).contains(x));
}

// Test: push on empty seq
proof fn test_seq_push_empty()
{
    seq_push_lemma::<usize>();
    let s = Seq::<usize>::empty();
    let v: usize = 100;
    assert(s.push(v).contains(v));
}

// Test: seq_push with PagePtr type
proof fn test_seq_push_page_ptr()
{
    seq_push_lemma::<PagePtr>();
    let s = seq![0x1000usize, 0x2000usize];
    let v: PagePtr = 0x3000usize;
    assert(s.push(v).contains(v));
    assert(s.push(v).contains(0x1000usize));
}

// --- Tests for map_insert_lemma ---

// Test: insert doesn't affect other keys
proof fn test_map_insert_other_keys()
{
    map_insert_lemma::<int, int>();
    let m = Map::<int, int>::empty().insert(1, 100).insert(2, 200);
    let x: int = 3;
    let y: int = 2;
    let v: int = 300;
    assert(x != y);
    assert(m.insert(x, v)[y] == m[y]);
}

// Test: map insert with different types
proof fn test_map_insert_usize_types()
{
    map_insert_lemma::<usize, usize>();
    let m = Map::<usize, usize>::empty().insert(10usize, 1000usize);
    let x: usize = 20;
    let y: usize = 10;
    let v: usize = 2000;
    assert(x != y);
    assert(m.insert(x, v)[y] == m[y]);
}

// --- Tests for Quota::spec_subtract_mem_4k ---

// Test: spec_subtract_mem_4k with basic values
proof fn test_quota_subtract_basic()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 96, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 4));
}

// Test: spec_subtract_mem_4k with k=0 (identity)
proof fn test_quota_subtract_zero()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 0));
}

// Test: spec_subtract_mem_4k preserves other fields
proof fn test_quota_subtract_preserves_fields()
{
    let old_quota = Quota { mem_4k: 200, mem_2m: 77, mem_1g: 33, pcid: 11, ioid: 7 };
    let new_quota = Quota { mem_4k: 192, mem_2m: 77, mem_1g: 33, pcid: 11, ioid: 7 };
    assert(old_quota.spec_subtract_mem_4k(new_quota, 8));
    assert(old_quota.mem_2m == new_quota.mem_2m);
    assert(old_quota.mem_1g == new_quota.mem_1g);
    assert(old_quota.pcid == new_quota.pcid);
    assert(old_quota.ioid == new_quota.ioid);
}

// --- Tests for page utility spec functions ---

// Test: page_ptr_valid for aligned page pointers
proof fn test_page_ptr_valid_basic()
{
    assert(page_ptr_valid(0x1000usize));
    assert(page_ptr_valid(0x2000usize));
    assert(page_ptr_valid(0usize));
}

// Test: page_index_valid for valid indices
proof fn test_page_index_valid_basic()
{
    assert(page_index_valid(0usize));
    assert(page_index_valid(1usize));
    assert(page_index_valid((NUM_PAGES - 1) as usize));
}

// Test: page_ptr2page_index and page_index2page_ptr roundtrip
proof fn test_page_index_roundtrip()
{
    assert(spec_page_ptr2page_index(0x1000usize) == 1usize);
    assert(spec_page_ptr2page_index(0x2000usize) == 2usize);
    assert(spec_page_index2page_ptr(1usize) == 0x1000usize);
    assert(spec_page_index2page_ptr(2usize) == 0x2000usize);
}

// Test: page_index_2m_valid
proof fn test_page_index_2m_valid()
{
    assert(page_index_2m_valid(0usize));
    assert(page_index_2m_valid(512usize));
    assert(page_index_2m_valid(1024usize));
}

// Test: page_index_1g_valid
proof fn test_page_index_1g_valid()
{
    assert(page_index_1g_valid(0usize));
    assert((512usize * 512usize) as usize == 262144usize);
    assert(page_index_1g_valid(262144usize));
}

// Test: page_ptr_2m_valid
proof fn test_page_ptr_2m_valid()
{
    assert(page_ptr_2m_valid(0usize));
    assert(page_ptr_2m_valid(0x200000usize));
}

// Test: PageEntry::is_empty
proof fn test_page_entry_is_empty()
{
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

// Test: PageEntry not empty when addr != 0
proof fn test_page_entry_not_empty_addr()
{
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

// Test: PageEntry not empty when present is true
proof fn test_page_entry_not_empty_present()
{
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

// Test: spec_page_index_truncate_2m
proof fn test_page_index_truncate_2m()
{
    assert(spec_page_index_truncate_2m(0usize) == 0usize);
    assert(spec_page_index_truncate_2m(511usize) == 0usize);
    assert(spec_page_index_truncate_2m(512usize) == 512usize);
    assert(spec_page_index_truncate_2m(513usize) == 512usize);
}

// Test: spec_page_index_merge_2m_vaild
proof fn test_page_index_merge_2m_valid()
{
    assert(spec_page_index_merge_2m_vaild(0usize, 1usize));
    assert(spec_page_index_merge_2m_vaild(0usize, 0x1FFusize));
}

// --- Parameterized tests ---

// Test: set_lemma holds for arbitrary sets and element
proof fn test_set_lemma_param(s1: Set<int>, s2: Set<int>, e: int)
{
    set_lemma::<int>();
    assert((s1 + s2).insert(e) == s1 + (s2.insert(e)));
    assert(s1 + (s2.insert(e)) == s2 + (s1.insert(e)));
    assert(!(s1 + s2).contains(e) <==> !s1.contains(e) && !s2.contains(e));
}

// Test: seq_push_lemma holds for arbitrary seq and values
proof fn test_seq_push_param(s: Seq<int>, v: int)
{
    seq_push_lemma::<int>();
    assert(s.push(v).contains(v));
}

// Test: map_insert_lemma holds for arbitrary map and keys
proof fn test_map_insert_param(m: Map<int, int>, x: int, y: int, v: int)
    requires x != y
{
    map_insert_lemma::<int, int>();
    assert(m.insert(x, v)[y] == m[y]);
}

// Test: spec_subtract_mem_4k is correct for arbitrary valid quota and k
proof fn test_quota_subtract_param(q: Quota, k: usize)
    requires
        q.mem_4k >= k,
{
    let new_q = Quota {
        mem_4k: (q.mem_4k - k) as usize,
        mem_2m: q.mem_2m,
        mem_1g: q.mem_1g,
        pcid: q.pcid,
        ioid: q.ioid,
    };
    assert(q.spec_subtract_mem_4k(new_q, k));
}

// Test: page_ptr_valid for any valid page pointer
proof fn test_page_ptr_valid_param(ptr: usize)
    requires
        ptr % 0x1000 == 0,
        ptr / 0x1000 < NUM_PAGES,
{
    assert(page_ptr_valid(ptr));
}

// Test: spec_usize2pa for 0 is 0
proof fn test_spec_usize2pa_zero()
{
    assert(spec_usize2pa(0usize) == 0usize) by (bit_vector);
}

// Test: spec_usize2present for 0 is false
proof fn test_spec_usize2present_zero()
{
    assert(!usize2present(0usize)) by (bit_vector);
}

// Test: spec_usize2present for 1 is true
proof fn test_spec_usize2present_one()
{
    assert(usize2present(1usize)) by (bit_vector);
}

// Test: spec_usize2write for 2 is true (bit 1 set)
proof fn test_spec_usize2write_bit1()
{
    assert(usize2write(2usize)) by (bit_vector);
}

// Test: spec_usize2user for 4 is true (bit 2 set)
proof fn test_spec_usize2user_bit2()
{
    assert(usize2user(4usize)) by (bit_vector);
}

} // verus!
