use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type IOid = usize;
pub type Pcid = usize;
pub type PagePtr = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;
pub const IOID_MAX: usize = 4096;

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

// ===================== LOGICAL TESTS =====================
// Each test encodes a property NOT explicitly guaranteed by the spec.
// These test for unintended reasoning: determinism, stronger bounds,
// cross-function misuse, structural/global assumptions.
// All tests SHOULD FAIL verification.

// Test 1: check_io_space_va_range_free does NOT guarantee that the IO space
// mapping is empty just because the queried range is free.
// Claim: if range is free, then io_space is completely empty.
// SHOULD FAIL
proof fn test_logical_range_free_implies_io_space_empty(
    io_space_dom: Set<VAddr>,
    va_seq: Seq<VAddr>,
    other_va: VAddr,
)
    requires
        va_seq.len() == 1,
        !io_space_dom.contains(va_seq[0]),
        other_va != va_seq[0],
{
    // Range is free for va_seq, but io_space may have other mappings
    assert(!io_space_dom.contains(other_va));
}

// Test 2: The spec does NOT guarantee that if an IO space has ANY free VA,
// then ALL VAs are free. Claim: if one VA is free, all are free.
// SHOULD FAIL
proof fn test_logical_one_free_implies_all_free(
    io_space_dom: Set<VAddr>,
    va1: VAddr,
    va2: VAddr,
)
    requires
        !io_space_dom.contains(va1),
        va1 != va2,
{
    // va1 is free but va2 may or may not be free
    assert(!io_space_dom.contains(va2));
}

// Test 3: check_io_space_va_range_free does NOT provide information about
// the PCID address space. Claim: range free in IO space implies
// range free in regular address space.
// SHOULD FAIL
proof fn test_logical_io_free_implies_pcid_free(
    io_space_dom: Set<VAddr>,
    pcid_space_dom: Set<VAddr>,
    va: VAddr,
)
    requires
        !io_space_dom.contains(va),
{
    // IO space being free for va does NOT imply PCID space is also free
    assert(!pcid_space_dom.contains(va));
}

// Test 4: The spec does NOT guarantee a stronger inequality on range size.
// Claim: if range is free, then len must be > 0.
// (Actually len can be 0 and range is vacuously free.)
// SHOULD FAIL
proof fn test_logical_free_range_implies_positive_len(
    len: usize,
    range_free: bool,
)
    requires
        len == 0,
        range_free == true, // vacuously true for len=0
{
    // range_free is true but len is 0
    assert(len > 0);
}

// Test 5: The spec does NOT relate different processes' IO spaces.
// Claim: if proc1's IO range is free, then proc2's IO range is also free.
// SHOULD FAIL
proof fn test_logical_cross_proc_io_space_free(
    io_space_1: Set<VAddr>,
    io_space_2: Set<VAddr>,
    va: VAddr,
)
    requires
        !io_space_1.contains(va),
{
    assert(!io_space_2.contains(va));
}

// Test 6: The function takes &self (immutable). The spec does NOT guarantee
// that calling check_io_space_va_range_free with a SUPERSET of the range
// also returns true. Claim: subset-free implies superset-free.
// SHOULD FAIL
proof fn test_logical_subset_free_implies_superset_free(
    io_space_dom: Set<VAddr>,
    va_small: Seq<VAddr>,
    va_extra: VAddr,
)
    requires
        va_small.len() == 1,
        !io_space_dom.contains(va_small[0]),
        va_extra != va_small[0],
{
    // Small range is free, but va_extra might be in io_space
    assert(!io_space_dom.contains(va_extra));
}

// Test 7: The spec does NOT guarantee that two different processes with
// the same ioid have the same IO space. Claim: same ioid implies same IO space.
// (In practice each proc has a unique ioid, but the spec doesn't
// make this obvious from check_io_space_va_range_free alone.)
// SHOULD FAIL
proof fn test_logical_same_ioid_same_io_space(
    io_space_1: Set<VAddr>,
    io_space_2: Set<VAddr>,
    ioid: IOid,
)
    requires
        ioid < IOID_MAX,
{
    assert(io_space_1 =~= io_space_2);
}

// Test 8: The spec does NOT guarantee that checking the range doesn't
// affect other kernel state. Claim: page_alloc state changes.
// (It's &self so nothing changes, but the spec doesn't link
// arbitrary other components explicitly.)
// SHOULD FAIL
proof fn test_logical_check_changes_page_alloc(
    old_mapped: Set<PagePtr>,
    new_mapped: Set<PagePtr>,
    page: PagePtr,
)
    requires
        old_mapped.contains(page),
{
    // Claim: after check_io_space_va_range_free, the page is no longer mapped
    assert(!new_mapped.contains(page));
}

// Test 9: The spec does NOT imply that va_4k_valid and being in IO space
// are mutually exclusive. Claim: a VA can't be both valid and in IO space.
// SHOULD FAIL
proof fn test_logical_va_valid_not_in_io_space(
    io_space_dom: Set<VAddr>,
    va: VAddr,
)
    requires
        spec_va_4k_valid(va),
{
    assert(!io_space_dom.contains(va));
}

// Test 10: The function checks whether each VA in va_range is NOT in IO space.
// The spec does NOT guarantee order-independence (although it is).
// Claim: checking va_range in reverse order gives a DIFFERENT result.
// SHOULD FAIL
proof fn test_logical_order_dependence(
    io_space_dom: Set<VAddr>,
    va0: VAddr,
    va1: VAddr,
    forward_result: bool,
    reverse_result: bool,
)
    requires
        forward_result == (!io_space_dom.contains(va0) && !io_space_dom.contains(va1)),
        reverse_result == (!io_space_dom.contains(va1) && !io_space_dom.contains(va0)),
{
    // Both should be equal due to commutativity of &&
    // Claim they differ:
    assert(forward_result != reverse_result);
}

// Test 11: The spec does NOT guarantee that if a range of length N is free,
// then extending it by one more VA at the end is also free.
// Claim: free for [0..N) implies free for [0..N+1).
// SHOULD FAIL
proof fn test_logical_free_range_extends(
    io_space_dom: Set<VAddr>,
    va_seq: Seq<VAddr>,
)
    requires
        va_seq.len() == 3,
        forall|j: int| #![auto] 0 <= j < 2 ==> !io_space_dom.contains(va_seq[j]),
{
    // First 2 are free, but the 3rd might not be
    assert(!io_space_dom.contains(va_seq[2]));
}

// Test 12: The spec says ret == io_space_range_free(target_proc_ptr, va_range).
// It does NOT guarantee that the result is independent of target_proc_ptr
// when two procs share the same ioid (which shouldn't happen normally).
// Claim: result is always true regardless of which proc we check.
// SHOULD FAIL
proof fn test_logical_result_independent_of_proc(
    io_space_1: Set<VAddr>,
    io_space_2: Set<VAddr>,
    va: VAddr,
)
    requires
        !io_space_1.contains(va),
        io_space_2.contains(va),
{
    // Different procs, different io_spaces => different results
    assert(!io_space_2.contains(va));
}

}
