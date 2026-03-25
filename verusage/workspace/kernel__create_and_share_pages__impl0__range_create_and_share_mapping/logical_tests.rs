use vstd::prelude::*;

fn main() {}

verus!{

pub type ProcPtr = usize;
pub type ContainerPtr = usize;
pub type PagePtr = usize;
pub type VAddr = usize;
pub type PAddr = usize;
pub type Pcid = usize;
pub type IOid = usize;
pub type ThreadPtr = usize;
pub type EndpointPtr = usize;

pub struct MapEntry {
    pub addr: PAddr,
    pub write: bool,
    pub execute_disable: bool,
}

pub const NUM_PAGES: usize = 2 * 1024 * 1024;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;
pub const KERNEL_MEM_END_L4INDEX: usize = 1;

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

// ===================== LOGICAL TESTS =====================
// Test properties NOT explicitly guaranteed by the specification.
// These probe whether the spec allows unintended logical inferences.
// All tests SHOULD FAIL verification.

// Test 1: Determinism — the spec does NOT guarantee that ret is deterministic.
// create_entry_and_share only ensures ret <= 3. Two calls with same args
// could return different values.
// SHOULD FAIL
proof fn test_logical_ret_is_deterministic(
    ret1: usize,
    ret2: usize,
)
    requires
        ret1 <= 3,
        ret2 <= 3,
{
    assert(ret1 == ret2);
}

// Test 2: Stronger inequality — spec says ret <= 3,
// but does NOT guarantee ret > 0 (ret could be 0).
// SHOULD FAIL
proof fn test_logical_ret_always_positive(
    ret: usize,
)
    requires
        ret <= 3,
{
    assert(ret > 0);
}

// Test 3: The spec does NOT guarantee ret == 3 exactly.
// It only guarantees ret <= 3.
// SHOULD FAIL
proof fn test_logical_ret_equals_max(
    ret: usize,
)
    requires
        ret <= 3,
{
    assert(ret == 3);
}

// Test 4: The spec does NOT guarantee that free pages decrease
// by exactly 3 per range entry. ret is bounded by 3 but not fixed.
// SHOULD FAIL
proof fn test_logical_free_pages_decrease_exact(
    old_free: usize,
    new_free: usize,
    ret: usize,
    range_len: usize,
)
    requires
        new_free == old_free - ret,
        ret <= 3 * range_len,
        range_len == 1,
        old_free >= 3,
{
    assert(new_free == old_free - 3);
}

// Test 5: Cross-function misuse — range_create_and_share_mapping does NOT
// guarantee that calling it with range_len == 0 is a no-op that leaves
// free pages strictly unchanged AND ret == 0.
// The spec doesn't explicitly constrain the empty-range case this way.
// SHOULD FAIL
proof fn test_logical_zero_length_forces_ret_zero(
    ret: usize,
)
    requires
        ret <= 3 * 0usize,
{
    // Even though 3 * 0 = 0 implies ret <= 0 implies ret == 0,
    // let's test the stronger claim that ret is nonzero — should fail
    assert(ret > 0);
}

// Test 6: The spec preserves page_is_mapped status for all pages.
// It does NOT guarantee that some specific page becomes mapped.
// SHOULD FAIL
proof fn test_logical_new_page_becomes_mapped(
    old_mapped: bool,
    new_mapped: bool,
    page: PagePtr,
)
    requires
        new_mapped == old_mapped,
        old_mapped == false,
{
    assert(new_mapped == true);
}

// Test 7: The spec preserves page_mapping for pages unrelated to src.
// But does NOT guarantee that the src page's mapping set is unchanged.
// SHOULD FAIL
proof fn test_logical_src_page_mapping_unchanged(
    old_mapping: Set<(ProcPtr, VAddr)>,
    new_mapping: Set<(ProcPtr, VAddr)>,
    target_proc_ptr: ProcPtr,
    target_va: VAddr,
)
    requires
        new_mapping == old_mapping.insert((target_proc_ptr, target_va)),
        !old_mapping.contains((target_proc_ptr, target_va)),
{
    assert(new_mapping == old_mapping);
}

// Test 8: The spec does NOT guarantee that the address space of
// the target proc has exactly range_len new entries. It could have
// had entries before that are preserved.
// SHOULD FAIL
proof fn test_logical_target_space_exact_size(
    old_size: nat,
    new_size: nat,
    range_len: usize,
)
    requires
        old_size == 5,
        range_len == 3,
        // The spec ensures entries are added, not that size == range_len
        new_size >= old_size,
{
    assert(new_size == range_len as nat);
}

// Test 9: Structural assumption — the spec does NOT guarantee that
// containers not owning the target process are completely unmodified
// in ALL fields. It only specifies certain fields are preserved.
// Try to assert a fictional field is unchanged — unrelated prop.
// SHOULD FAIL
proof fn test_logical_unrelated_container_quota_unchanged(
    old_quota: usize,
    new_quota: usize,
    ret: usize,
)
    requires
        old_quota - ret == new_quota,
        ret > 0,
        old_quota >= ret,
{
    assert(old_quota == new_quota);
}

// Test 10: The spec does NOT guarantee any ordering on which VAs
// in the target range are mapped. Assert index 0 mapped implies
// the last index must also be mapped — not entailed.
// SHOULD FAIL
proof fn test_logical_partial_mapping_implies_full(
    mapped_indices: Set<int>,
    range_len: int,
)
    requires
        range_len == 10,
        mapped_indices.contains(0),
        !mapped_indices.contains(range_len - 1),
{
    assert(mapped_indices.contains(range_len - 1));
}

// Test 11: The spec preserves VAs outside target_va_range in the target space.
// But does NOT guarantee that ALL pages mapped to the src proc are
// shared to the target proc — only those in the range.
// SHOULD FAIL
proof fn test_logical_all_src_pages_shared(
    src_space: Map<VAddr, MapEntry>,
    target_space: Map<VAddr, MapEntry>,
    extra_va: VAddr,
)
    requires
        src_space.dom().contains(extra_va),
        !target_space.dom().contains(extra_va),
{
    assert(target_space.dom().contains(extra_va));
}

// Test 12: The spec ensures page_mapping domain is unchanged.
// This does NOT mean that mapping *values* are all unchanged —
// src page's mapping set grows. Assert all values unchanged — should fail.
// SHOULD FAIL
proof fn test_logical_all_page_mapping_values_unchanged(
    old_val: Set<(ProcPtr, VAddr)>,
    new_val: Set<(ProcPtr, VAddr)>,
    added: (ProcPtr, VAddr),
)
    requires
        new_val == old_val.insert(added),
        !old_val.contains(added),
{
    assert(new_val =~= old_val);
}

}
