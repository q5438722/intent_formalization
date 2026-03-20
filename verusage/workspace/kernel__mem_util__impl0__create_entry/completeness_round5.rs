use vstd::prelude::*;

fn main() {}

verus! {

pub type PAddr = usize;
pub type VAddr = usize;
pub type L4Index = usize;
pub type L3Index = usize;
pub type L2Index = usize;
pub type L1Index = usize;

pub const KERNEL_MEM_END_L4INDEX: usize = 1;
pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_2m_MASK: u64 = 0x0000_ffff_ffe0_0000;
pub const PAGE_ENTRY_PRESENT_MASK: u64 = 0x1;
pub const PAGE_ENTRY_WRITE_SHIFT: u64 = 1;
pub const PAGE_ENTRY_WRITE_MASK: u64 = 0x1u64 << PAGE_ENTRY_WRITE_SHIFT;

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
}

pub open spec fn usize2present(v: usize) -> bool {
    (v & PAGE_ENTRY_PRESENT_MASK as usize) != 0
}

pub open spec fn usize2write(v: usize) -> bool {
    (v & PAGE_ENTRY_WRITE_MASK as usize) != 0
}

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

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn spec_va_2m_valid(va: usize) -> bool {
    (va & (!MEM_2m_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
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

pub open spec fn spec_page_index_merge_2m_vaild(i: usize, j: usize) -> bool
    recommends page_index_2m_valid(i),
{
    i < j < i + 0x200
}

pub open spec fn spec_page_index_truncate_2m(index: usize) -> usize {
    (index / 512usize * 512usize) as usize
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
{
}


// =========================================================================
// COMPLETENESS ROUND 5: Cross-function Misuse & Edge Cases
// All tests should FAIL (verification errors expected)
// =========================================================================

// Test 1: Assert all VAs are 4k-valid (false: needs alignment and l4 >= 1)
proof fn test_fail_all_va_valid(va: usize) {
    assert(spec_va_4k_valid(va));
}

// Test 2: Assert present implies write (independent bits)
proof fn test_fail_present_implies_write(v: usize)
    requires usize2present(v)
{
    assert(usize2write(v));
}

// Test 3: Assert l1 index is injective (different VAs can have same l1 index)
proof fn test_fail_l1index_injective(va1: usize, va2: usize)
    requires spec_v2l1index(va1) == spec_v2l1index(va2)
{
    assert(va1 == va2);
}

// Test 4: Assert page_ptr_valid implies page_index_valid of raw ptr
proof fn test_fail_ptr_valid_implies_index_valid(ptr: usize)
    requires page_ptr_valid(ptr)
{
    assert(page_index_valid(ptr));
}

// Test 5: Assert 2m-valid VA implies l2 index is 0 (va_lemma says l1==0 only)
proof fn test_fail_2m_valid_implies_l2_zero(va: VAddr)
    requires spec_va_2m_valid(va)
{
    va_lemma();
    assert(0 == spec_v2l2index(va));
}

// Test 6: Assert set_mem_4k changes mem_2m (it doesn't)
proof fn test_fail_set_mem_4k_changes_mem_2m() {
    let q = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let q2 = q.spec_set_mem_4k(42);
    assert(q2.mem_2m == 42);
}

// Test 7: merge_2m at exact boundary (j == i + 0x200 is NOT strictly less)
proof fn test_fail_merge_2m_at_boundary() {
    assert(spec_page_index_merge_2m_vaild(0usize, 0x200usize));
}

// Test 8: truncate_2m preserves non-aligned value (it doesn't)
proof fn test_fail_truncate_preserves_non_aligned() {
    assert(spec_page_index_truncate_2m(511usize) == 511usize);
}


} // verus!
