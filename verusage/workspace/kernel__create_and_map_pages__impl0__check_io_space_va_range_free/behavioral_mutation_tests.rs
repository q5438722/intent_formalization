use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type IOid = usize;
pub type Pcid = usize;
pub type PagePtr = usize;

pub const MEM_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs or postcondition relationships,
// then asserts a WRONG output or mutated relation.
// All tests SHOULD FAIL verification.

// Test 1: check_io_space_va_range_free ensures ret == io_space_range_free(...).
// If io_space_range_free is true, ret must be true.
// Mutate: claim ret is false when range is free.
// SHOULD FAIL
proof fn test_mutation_range_free_returns_false(
    io_space_dom: Set<VAddr>,
    va_seq: Seq<VAddr>,
    ret: bool,
)
    requires
        va_seq.len() == 2,
        !io_space_dom.contains(va_seq[0]),
        !io_space_dom.contains(va_seq[1]),
        // Simulating: io_space_range_free == true
        ret == true,
{
    assert(ret == false);
}

// Test 2: If any VA in the range IS in the IO space, io_space_range_free is false.
// Mutate: claim ret is true when a VA is present.
// SHOULD FAIL
proof fn test_mutation_range_not_free_returns_true(
    io_space_dom: Set<VAddr>,
    va_seq: Seq<VAddr>,
)
    requires
        va_seq.len() >= 1,
        io_space_dom.contains(va_seq[0]),
{
    // io_space_range_free should be false, but we claim it's true
    let range_free: bool = forall|j: int|
        #![auto]
        0 <= j < va_seq.len() ==> !io_space_dom.contains(va_seq[j]);
    assert(range_free == true);
}

// Test 3: resolve_iommu_table_mapping ensures that if mapping contains va,
// ret is Some. Mutate: claim ret is None when mapping has the VA.
// SHOULD FAIL
proof fn test_mutation_resolve_some_returns_none(
    mapping_dom: Set<VAddr>,
    va: VAddr,
    ret_is_some: bool,
)
    requires
        mapping_dom.contains(va),
        // The ensures says: mapping.dom().contains(va) == ret.is_Some()
        ret_is_some == mapping_dom.contains(va),
{
    assert(ret_is_some == false);
}

// Test 4: resolve_iommu_table_mapping ensures that if mapping does NOT
// contain va, ret is None. Mutate: claim ret is Some.
// SHOULD FAIL
proof fn test_mutation_resolve_none_returns_some(
    mapping_dom: Set<VAddr>,
    va: VAddr,
    ret_is_some: bool,
)
    requires
        !mapping_dom.contains(va),
        ret_is_some == mapping_dom.contains(va),
{
    assert(ret_is_some == true);
}

// Test 5: check_io_space_va_range_free iterates over all indices.
// If the first VA is free but the second is not, ret should be false.
// Mutate: claim ret is true.
// SHOULD FAIL
proof fn test_mutation_partial_range_free_returns_true(
    io_space_dom: Set<VAddr>,
    va0: VAddr,
    va1: VAddr,
)
    requires
        !io_space_dom.contains(va0),
        io_space_dom.contains(va1),
{
    // Only index 0 is free, index 1 is occupied => range_free should be false
    let range_free: bool =
        !io_space_dom.contains(va0) && !io_space_dom.contains(va1);
    assert(range_free == true);
}

// Test 6: The postcondition ties ret to io_space_range_free exactly.
// Mutate: claim ret is the NEGATION of io_space_range_free.
// SHOULD FAIL
proof fn test_mutation_ret_negated(
    range_free: bool,
    ret: bool,
)
    requires
        ret == range_free,
{
    assert(ret == !range_free);
}

// Test 7: io_space_range_free checks all j in [0, len).
// If we have 3 VAs all free, range_free is true.
// Mutate: claim it's false.
// SHOULD FAIL
proof fn test_mutation_all_free_but_claim_false(
    io_space_dom: Set<VAddr>,
    va0: VAddr,
    va1: VAddr,
    va2: VAddr,
)
    requires
        !io_space_dom.contains(va0),
        !io_space_dom.contains(va1),
        !io_space_dom.contains(va2),
        va0 != va1 && va1 != va2 && va0 != va2,
{
    let all_free: bool =
        !io_space_dom.contains(va0) &&
        !io_space_dom.contains(va1) &&
        !io_space_dom.contains(va2);
    assert(all_free == false);
}

// Test 8: The function preserves self (it takes &self, not &mut self).
// After calling, io_space domain should be unchanged.
// Mutate: claim domain changes.
// SHOULD FAIL
proof fn test_mutation_io_space_domain_changes(
    old_dom: Set<VAddr>,
    new_dom: Set<VAddr>,
)
    requires
        new_dom =~= old_dom,
{
    assert(new_dom !== old_dom);
}

// Test 9: io_space_range_free for a single-element range checks only va_seq[0].
// If va_seq[0] is not in io_space, range_free == true.
// Mutate: claim range_free == false.
// SHOULD FAIL
proof fn test_mutation_single_element_free_returns_false(
    io_space_dom: Set<VAddr>,
    va: VAddr,
)
    requires
        !io_space_dom.contains(va),
{
    // For len=1, range_free = !io_space_dom.contains(va) = true
    let range_free: bool = !io_space_dom.contains(va);
    assert(range_free == false);
}

// Test 10: check_io_space_va_range_free returns bool.
// When io_space is completely empty, any valid range should be free.
// Mutate: claim some range is NOT free even with empty io_space.
// SHOULD FAIL
proof fn test_mutation_empty_io_space_range_not_free(
    io_space_dom: Set<VAddr>,
    va: VAddr,
)
    requires
        io_space_dom =~= Set::<VAddr>::empty(),
{
    assert(io_space_dom.contains(va));
}

// Test 11: The loop invariant maintains that all checked indices are free.
// If the loop completes (returns true), ALL indices are free.
// Mutate: claim some index was skipped (exists j free is false).
// SHOULD FAIL
proof fn test_mutation_loop_skips_index(
    io_space_dom: Set<VAddr>,
    va_seq: Seq<VAddr>,
    len: int,
)
    requires
        len == 3,
        forall|j: int| #![auto] 0 <= j < len ==> !io_space_dom.contains(va_seq[j]),
{
    // All indices are free, claim index 1 is NOT free
    assert(io_space_dom.contains(va_seq[1]));
}

// Test 12: The function uses resolve_iommu_table_mapping to check each VA.
// If resolve returns None, the VA is not mapped.
// Mutate: claim that None means the VA IS mapped.
// SHOULD FAIL
proof fn test_mutation_none_means_mapped(
    mapping_dom: Set<VAddr>,
    va: VAddr,
    resolve_is_some: bool,
)
    requires
        resolve_is_some == mapping_dom.contains(va),
        !mapping_dom.contains(va),
{
    // resolve_is_some should be false, mapping_dom doesn't contain va
    assert(mapping_dom.contains(va));
}

}
