use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn spec_usize2pa(v: usize) -> PAddr {
    v & MEM_MASK as usize
}

pub open spec fn spec_page_ptr2page_index(ptr: usize) -> usize {
    (ptr / 4096usize) as usize
}

pub open spec fn spec_page_index2page_ptr(i: usize) -> usize {
    (i * 4096) as usize
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
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

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// These test for unintended reasoning: determinism, injectivity,
// stronger bounds, and cross-function misuse.
// All tests SHOULD FAIL verification.

// Test 1: usize2pa injectivity — different inputs MUST give different outputs.
// The spec only says usize2pa(v) = v & MEM_MASK, which is NOT injective
// (multiple v values map to the same PA due to masking).
// SHOULD FAIL
proof fn test_logical_usize2pa_injective() {
    let v1: usize = 0x1000;
    let v2: usize = 0x1001;
    // Both should mask to the same PA (low bits differ)
    assert(spec_usize2pa(v1) != spec_usize2pa(v2));
}

// Test 2: create_entry_and_alloc_and_map ensures ret.0 <= 4.
// The spec does NOT guarantee ret.0 == 4 exactly.
// Claim: it always uses exactly 4 pages.
// SHOULD FAIL
proof fn test_logical_create_entry_always_uses_4(ret0: usize)
    requires
        ret0 <= 4,
{
    assert(ret0 == 4);
}

// Test 3: create_entry_and_alloc_and_map ensures ret.0 <= 4.
// The spec does NOT guarantee ret.0 >= 1.
// Claim: it always uses at least 1 page.
// SHOULD FAIL
proof fn test_logical_create_entry_uses_at_least_1(ret0: usize)
    requires
        ret0 <= 4,
{
    assert(ret0 >= 1);
}

// Test 4: page_ptr2page_index and page_index2page_ptr composition
// is NOT guaranteed to be the identity for arbitrary usize values
// (only for valid page_ptr). Claim it works for any value.
// SHOULD FAIL
proof fn test_logical_ptr_index_identity_arbitrary() {
    let ptr: usize = 0x1234; // not 4k-aligned
    assert(spec_page_index2page_ptr(spec_page_ptr2page_index(ptr)) == ptr);
}

// Test 5: Two calls to range_alloc_and_map might return different
// page sets. The spec doesn't guarantee determinism —
// same inputs don't imply same outputs.
// Claim: return pages are deterministic.
// SHOULD FAIL
proof fn test_logical_range_alloc_determinism(
    pages1: Seq<PagePtr>,
    pages2: Seq<PagePtr>,
    len: int,
)
    requires
        pages1.len() == len,
        pages2.len() == len,
        len > 0,
{
    assert(pages1 =~= pages2);
}

// Test 6: range_alloc_and_map spec does NOT guarantee that
// the total pages used (ret.0) == 4 * va_range.len exactly.
// It only says ret.0 is subtracted from free pages.
// Claim: ret.0 == 4 * va_range.len always.
// SHOULD FAIL
proof fn test_logical_range_alloc_exact_page_usage(
    ret0: usize,
    va_range_len: usize,
)
    requires
        va_range_len > 0,
{
    assert(ret0 == 4 * va_range_len);
}

// Test 7: The spec preserves container fields for the owning container,
// but does NOT guarantee the quota's mem_4k subtracts exactly
// 4 * va_range.len. Claim it does.
// SHOULD FAIL
proof fn test_logical_quota_exact_subtraction(
    old_quota: Quota,
    new_quota: Quota,
    va_range_len: usize,
)
    requires
        old_quota.spec_subtract_mem_4k(new_quota, va_range_len),
{
    // Stronger claim: the subtracted amount is always exactly 4 * va_range_len
    assert(old_quota.mem_4k - new_quota.mem_4k == 4 * va_range_len);
}

// Test 8: Allocated pages from range_alloc_and_map are not guaranteed
// to be contiguous in physical memory. Claim they are.
// SHOULD FAIL
proof fn test_logical_allocated_pages_contiguous(
    pages: Seq<PagePtr>,
)
    requires
        pages.len() >= 2,
{
    // Claim: consecutive pages differ by exactly 4096
    assert(pages[1] == pages[0] + 4096);
}

// Test 9: va_4k_valid does NOT imply the VA is page_ptr_valid.
// These are different validity domains (virtual vs physical).
// Claim: va_4k_valid implies page_ptr_valid.
// SHOULD FAIL
proof fn test_logical_va_valid_implies_page_ptr_valid(va: VAddr)
    requires
        spec_va_4k_valid(va),
{
    assert(page_ptr_valid(va));
}

// Test 10: The spec preserves page_mapping for old pages.
// But it does NOT guarantee that NEW page mappings are unique
// across different calls. Claim: all page addresses returned
// are globally unique.
// SHOULD FAIL
proof fn test_logical_global_page_uniqueness(
    pages1: Seq<PagePtr>,
    pages2: Seq<PagePtr>,
)
    requires
        pages1.len() > 0,
        pages2.len() > 0,
{
    // No spec property guarantees pages from two separate calls are disjoint
    assert(!pages1.contains(pages2[0]));
}

// Test 11: range_alloc_and_map does NOT guarantee returned pages
// are in increasing order. Claim: pages are sorted.
// SHOULD FAIL
proof fn test_logical_returned_pages_sorted(
    pages: Seq<PagePtr>,
)
    requires
        pages.len() >= 2,
{
    assert(pages[0] < pages[1]);
}

// Test 12: The spec says address space for non-target procs is preserved.
// But does NOT say the target proc's address space ONLY has new entries.
// Claim: the target proc's old entries are removed (wrong).
// SHOULD FAIL
proof fn test_logical_target_proc_old_entries_removed(
    old_space: Map<VAddr, MapEntry>,
    new_space: Map<VAddr, MapEntry>,
    old_va: VAddr,
)
    requires
        old_space.dom().contains(old_va),
        // The spec actually preserves old entries not in va_range
{
    // Wrong claim: old entries don't survive
    assert(!new_space.dom().contains(old_va));
}

}
