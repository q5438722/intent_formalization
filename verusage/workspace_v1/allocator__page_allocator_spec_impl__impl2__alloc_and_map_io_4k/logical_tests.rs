use vstd::prelude::*;

fn main() {}

verus!{

pub type Pcid = usize;
pub type VAddr = usize;
pub type IOid = usize;
pub type PagePtr = usize;
pub type ContainerPtr = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_ptr_2m_valid(ptr: usize) -> bool {
    ((ptr % (0x200000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}

pub open spec fn page_ptr_1g_valid(ptr: usize) -> bool {
    ((ptr % (0x40000000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}


// ===================== LOGICAL TESTS =====================
// Each test generates a property NOT explicitly guaranteed by the spec.
// These probe for unintended reasoning the spec might allow.
// All tests SHOULD FAIL verification.

// Test 1: Determinism — two calls with same inputs produce same ret.
// The spec does not guarantee determinism; the allocated page may differ.
// SHOULD FAIL
proof fn test_logical_determinism(
    old_free1: Set<PagePtr>,
    old_free2: Set<PagePtr>,
    new_free1: Set<PagePtr>,
    new_free2: Set<PagePtr>,
    ret1: PagePtr,
    ret2: PagePtr,
)
    requires
        old_free1 =~= old_free2,
        old_free1.contains(ret1),
        old_free2.contains(ret2),
        new_free1 =~= old_free1.remove(ret1),
        new_free2 =~= old_free2.remove(ret2),
{
    // Spec does not guarantee that the same free set yields the same page
    assert(ret1 == ret2);
}

// Test 2: Stronger inequality — ret must be the minimum valid pointer.
// The spec only says ret comes from the free set; it does not say which one.
// SHOULD FAIL
proof fn test_logical_ret_is_minimum(
    free_set: Set<PagePtr>,
    ret: PagePtr,
    other: PagePtr,
)
    requires
        free_set.contains(ret),
        free_set.contains(other),
        page_ptr_valid(ret),
        page_ptr_valid(other),
        ret != other,
{
    // Spec does not guarantee ret is the smallest page ptr
    assert(ret < other);
}

// Test 3: Cross-function misuse — treating a 4k-allocated page as 2m-valid.
// alloc_and_map_io_4k ensures page_ptr_valid(ret) but NOT page_ptr_2m_valid(ret).
// SHOULD FAIL
proof fn test_logical_4k_implies_2m_valid(
    ret: PagePtr,
)
    requires
        page_ptr_valid(ret),
{
    // A 4k-valid page is not necessarily 2m-aligned
    assert(page_ptr_2m_valid(ret));
}

// Test 4: Structural assumption — free set removal is idempotent.
// If we remove ret from the free set and then remove it again, the result
// should be the same. But the spec says nothing about double-removal semantics
// in the allocator context. Here we test that removing from the already-removed
// set doesn't equal the original set.
// SHOULD FAIL
proof fn test_logical_double_remove_restores(
    old_free: Set<PagePtr>,
    ret: PagePtr,
)
    requires
        old_free.contains(ret),
        old_free.len() > 1,
{
    let after_remove = old_free.remove(ret);
    // Removing ret again from after_remove should still not equal original
    assert(after_remove.remove(ret) =~= old_free);
}

// Test 5: Global uniqueness — io_mapping (ioid, va) is globally unique.
// The spec only sets io_mappings for the returned page. It does NOT guarantee
// that no other page already has the same (ioid, va) io_mapping.
// SHOULD FAIL
proof fn test_logical_io_mapping_globally_unique(
    ret_io: Set<(IOid, VAddr)>,
    other_io: Set<(IOid, VAddr)>,
    ioid: IOid,
    va: VAddr,
)
    requires
        ret_io =~= Set::<(IOid, VAddr)>::empty().insert((ioid, va)),
        other_io.contains((ioid, va)),
{
    // Spec does not prevent another page from having the same io_mapping
    assert(ret_io.disjoint(other_io));
}

// Test 6: Monotonicity — the mapped set strictly grows by exactly one element.
// The spec says mapped_pages_4k() =~= old.mapped_pages_4k().insert(ret).
// This implies |new| = |old| + 1 only if ret was not already mapped.
// We test the INCORRECT claim that the mapped set can grow by more than 1.
// SHOULD FAIL
proof fn test_logical_mapped_grows_by_two(
    old_mapped: Set<PagePtr>,
    new_mapped: Set<PagePtr>,
    ret: PagePtr,
    extra: PagePtr,
)
    requires
        !old_mapped.contains(ret),
        !old_mapped.contains(extra),
        ret != extra,
        new_mapped =~= old_mapped.insert(ret),
{
    // Spec says only ret is added; extra should not be in new_mapped
    assert(new_mapped.contains(extra));
}

// Test 7: Implication inversion — if page_is_mapped(ret) after, then it was mapped before.
// The spec says !old.page_is_mapped(ret) and self.page_is_mapped(ret).
// So the implication goes: was NOT mapped => IS mapped. Inverting this should fail.
// SHOULD FAIL
proof fn test_logical_mapped_implies_was_mapped(
    was_mapped: bool,
    is_mapped: bool,
)
    requires
        was_mapped == false,
        is_mapped == true,
{
    // Wrong direction: being mapped now does NOT imply was mapped before
    assert(was_mapped == true);
}

}
