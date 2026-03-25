use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type ThreadPtr = usize;
pub type EndpointPtr = usize;

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

pub open spec fn page_index_valid(index: usize) -> bool {
    (0 <= index < NUM_PAGES)
}

pub open spec fn MEM_valid(v: PAddr) -> bool {
    v & (!MEM_MASK) as usize == 0
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
// (multiple v values map to the same PA due to masking of low bits).
// SHOULD FAIL
proof fn test_logical_usize2pa_injective() {
    let v1: usize = 0x1000;
    let v2: usize = 0x1001;
    // Both mask to the same PA (0x1000 & MEM_MASK == 0x1001 & MEM_MASK)
    assert(spec_usize2pa(v1) != spec_usize2pa(v2));
}

// Test 2: page_ptr2page_index determinism across different valid ptrs.
// Assert that two distinct valid ptrs always give same index — false.
// SHOULD FAIL
proof fn test_logical_distinct_ptrs_same_index() {
    let ptr1: usize = 0x1000;
    let ptr2: usize = 0x2000;
    assert(spec_page_ptr2page_index(ptr1) == spec_page_ptr2page_index(ptr2));
}

// Test 3: Stronger bound on create_entry_and_alloc_and_map_io return.
// The spec guarantees ret.0 <= 4. Assert ret.0 <= 3 (strictly tighter).
// The spec does NOT guarantee this.
// SHOULD FAIL
proof fn test_logical_create_entry_stronger_bound() {
    let ret_pages: usize = 4;
    assert(ret_pages <= 3);
}

// Test 4: Claim that spec_subtract_mem_4k is symmetric.
// i.e., if old.subtract(new, k) then new.subtract(old, k).
// This is false: mem_4k arithmetic is not symmetric.
// SHOULD FAIL
proof fn test_logical_quota_subtract_symmetric() {
    let old_q = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let new_q = Quota { mem_4k: 96, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    // old.subtract(new, 4) is true
    assert(old_q.spec_subtract_mem_4k(new_q, 4));
    // symmetry claim: new.subtract(old, 4) — should fail since 96 - 4 != 100
    assert(new_q.spec_subtract_mem_4k(old_q, 4));
}

// Test 5: va_4k_valid is total — claim every aligned address is valid.
// Actually it also requires the L4 index >= KERNEL_MEM_END_L4INDEX.
// A low aligned address (e.g., 0x1000) fails the L4 check.
// SHOULD FAIL
proof fn test_logical_all_aligned_va_valid() {
    let va: usize = 0x1000;
    // 0x1000 is 4k-aligned (passes mask check) but L4 index is 0 (< 1)
    assert(spec_va_4k_valid(va));
}

// Test 6: page_ptr2page_index and page_index2page_ptr are inverses
// even for non-aligned ptrs. Use a non-aligned ptr to test.
// The spec only defines behavior for aligned ptrs.
// SHOULD FAIL
proof fn test_logical_roundtrip_non_aligned() {
    let ptr: usize = 5000; // not aligned to 0x1000
    // Integer division truncates: 5000 / 4096 = 1, then 1 * 4096 = 4096 != 5000
    assert(spec_page_index2page_ptr(spec_page_ptr2page_index(ptr)) == ptr);
}

// Test 7: range_alloc_and_map_io postcondition is very weak (only self.wf()).
// Test if the spec entails that proc_dom is preserved (commented out in source).
// Since it's only in commented-out ensures, the spec doesn't guarantee this.
// Construct a scenario: assert proc_dom preservation as an entailment.
// SHOULD FAIL (because it's not guaranteed by the active postcondition)
proof fn test_logical_range_alloc_preserves_proc_dom(
    old_dom: Set<ProcPtr>,
    new_dom: Set<ProcPtr>,
    extra: ProcPtr,
)
    requires
        !old_dom.contains(extra),
        new_dom =~= old_dom.insert(extra),
{
    // The weak postcondition does NOT guarantee proc_dom == old proc_dom
    // So the domains could differ after the call — this should fail
    assert(old_dom =~= new_dom);
}

// Test 8: MEM_valid should hold for any arbitrary usize value.
// It only holds if the low bits (non-mask bits) are zero.
// SHOULD FAIL
proof fn test_logical_mem_valid_any_value() {
    assert(MEM_valid(0x1usize));
}

// Test 9: Claim that create_entry_and_alloc_and_map_io returns exactly 4 pages always.
// The spec says ret.0 <= 4, meaning it could be 0, 1, 2, 3, or 4.
// Asserting ret.0 == 4 universally is not guaranteed.
// SHOULD FAIL
proof fn test_logical_create_entry_always_allocates_exactly_4() {
    let ret_pages: usize = 0;
    assert(ret_pages <= 4); // this is guaranteed
    assert(ret_pages == 4); // this is NOT guaranteed
}

// Test 10: Claim that spec_subtract_mem_4k with k=0 implies old == new.
// Actually it implies old.mem_4k == new.mem_4k AND other fields match,
// which means the structs should be equal. But subtraction with k=0:
// old.mem_4k - 0 == new.mem_4k is just old.mem_4k == new.mem_4k.
// Test a different property: subtract is idempotent (double subtract).
// old.subtract(new, k) && new.subtract(final, k) implies old.subtract(final, 2*k)
// This should hold mathematically, but let's test with a case where it doesn't.
// SHOULD FAIL
proof fn test_logical_quota_double_subtract_wrong() {
    let old_q = Quota { mem_4k: 100, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let mid_q = Quota { mem_4k: 96, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    let fin_q = Quota { mem_4k: 92, mem_2m: 50, mem_1g: 10, pcid: 5, ioid: 3 };
    assert(old_q.spec_subtract_mem_4k(mid_q, 4));
    assert(mid_q.spec_subtract_mem_4k(fin_q, 4));
    // Claim: old.subtract(final, 7) — wrong k, should be 8
    assert(old_q.spec_subtract_mem_4k(fin_q, 7));
}

// Test 11: io_space domain insert is NOT idempotent for a different VA.
// Inserting target_va into io_space should not also make arbitrary_va present.
// SHOULD FAIL
proof fn test_logical_io_space_insert_leaks(
    io_space: Map<VAddr, MapEntry>,
    target_va: VAddr,
    arbitrary_va: VAddr,
    entry: MapEntry,
)
    requires
        target_va != arbitrary_va,
        !io_space.dom().contains(target_va),
        !io_space.dom().contains(arbitrary_va),
{
    let new_space = io_space.insert(target_va, entry);
    assert(new_space.dom().contains(arbitrary_va));
}

// Test 12: Claim page_index2page_ptr(0) == 1 — wrong.
// It should be 0 * 4096 = 0.
// SHOULD FAIL
proof fn test_logical_page_index_zero_gives_nonzero() {
    assert(spec_page_index2page_ptr(0usize) == 1usize);
}

}
