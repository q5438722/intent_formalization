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
// Round 3: Negated/Contradicted Postconditions — all should FAIL
// ============================================================

// Test: negate set_lemma's union-insert associativity
proof fn test_set_lemma_negate_assoc()
{
    set_lemma::<int>();
    let s1 = Set::<int>::empty().insert(1);
    let s2 = Set::<int>::empty().insert(2);
    let e: int = 3;
    assert((s1 + s2).insert(e) != s1 + (s2.insert(e))); // negation of a true property
}

// Test: negate set_lemma's commutativity
proof fn test_set_lemma_negate_commut()
{
    set_lemma::<int>();
    let s1 = Set::<int>::empty().insert(10);
    let s2 = Set::<int>::empty().insert(20);
    let e: int = 30;
    assert(s1 + (s2.insert(e)) != s2 + (s1.insert(e))); // negation
}

// Test: negate seq_push contains pushed element
proof fn test_seq_push_negate_contains()
{
    seq_push_lemma::<int>();
    let s = seq![1int, 2];
    let v: int = 3;
    assert(!s.push(v).contains(v)); // negation: push(v) always contains v
}

// Test: negate seq_push preserves old elements
proof fn test_seq_push_negate_preserves()
{
    seq_push_lemma::<int>();
    let s = seq![1int, 2, 3];
    let v: int = 4;
    let x: int = 2;
    assert(s.contains(x));
    assert(!s.push(v).contains(x)); // negation: push preserves containment
}

// Test: negate map_insert_lemma (claim insert DOES affect other keys)
proof fn test_map_insert_negate()
{
    map_insert_lemma::<int, int>();
    let m = Map::<int, int>::empty().insert(1, 100).insert(2, 200);
    let x: int = 3;
    let y: int = 1;
    let v: int = 300;
    assert(m.insert(x, v)[y] != m[y]); // negation: insert at 3 shouldn't affect key 1
}

// Test: negate Quota subtract identity
proof fn test_quota_subtract_negate()
{
    let old_quota = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_quota = Quota { mem_4k: 96, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    // This IS a valid subtraction, so negating should fail
    assert(!old_quota.spec_subtract_mem_4k(new_quota, 4));
}

// Test: negate PageEntry::is_empty for an actually empty entry
proof fn test_page_entry_negate_is_empty()
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
    assert(!entry.is_empty()); // negation: this entry IS empty
}

} // verus!
