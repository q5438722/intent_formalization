use vstd::prelude::*;

fn main() {}

verus!{

pub type PagePtr = usize;
pub type Pcid = usize;
pub type VAddr = usize;
pub type IOid = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}


// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test starts from valid inputs and mutates the expected output/relation.
// These test whether the spec rejects incorrect behaviors.
// Models the postconditions of free_page_4k:
//   free_pages_4k() =~= old.free_pages_4k().insert(target_ptr)
//   allocated_pages_4k() =~= old.allocated_pages_4k().remove(target_ptr)
//   free_pages_2m/1g, allocated_pages_2m/1g, mapped_pages_* all unchanged
// All tests SHOULD FAIL verification.

// Test 1: After freeing, the target_ptr MUST be in the free set.
// Mutated assertion: target_ptr is NOT in the free set after insert.
// Models: free_page_4k postcondition free_pages_4k() =~= old.insert(target_ptr).
// SHOULD FAIL
proof fn test_mutation_freed_not_in_free_set(
    old_free: Set<PagePtr>,
    new_free: Set<PagePtr>,
    target_ptr: PagePtr,
)
    requires
        new_free =~= old_free.insert(target_ptr),
{
    assert(!new_free.contains(target_ptr));
}

// Test 2: After freeing, target_ptr MUST be removed from allocated set.
// Mutated assertion: target_ptr is still in allocated set after remove.
// Models: free_page_4k postcondition allocated_pages_4k() =~= old.remove(target_ptr).
// SHOULD FAIL
proof fn test_mutation_freed_still_allocated(
    old_allocated: Set<PagePtr>,
    new_allocated: Set<PagePtr>,
    target_ptr: PagePtr,
)
    requires
        old_allocated.contains(target_ptr),
        new_allocated =~= old_allocated.remove(target_ptr),
{
    assert(new_allocated.contains(target_ptr));
}

// Test 3: free_pages_2m is unchanged after free_page_4k.
// Mutated assertion: a new page appeared in free_pages_2m.
// Models: free_page_4k postcondition free_pages_2m() =~= old.free_pages_2m().
// SHOULD FAIL
proof fn test_mutation_2m_free_gained_page(
    old_free_2m: Set<PagePtr>,
    new_free_2m: Set<PagePtr>,
    p: PagePtr,
)
    requires
        new_free_2m =~= old_free_2m,
        !old_free_2m.contains(p),
{
    assert(new_free_2m.contains(p));
}

// Test 4: mapped_pages_4k is unchanged after free_page_4k.
// Mutated assertion: a page was removed from mapped_pages_4k.
// Models: free_page_4k postcondition mapped_pages_4k() =~= old.mapped_pages_4k().
// SHOULD FAIL
proof fn test_mutation_mapped_pages_lost_page(
    old_mapped: Set<PagePtr>,
    new_mapped: Set<PagePtr>,
    p: PagePtr,
)
    requires
        new_mapped =~= old_mapped,
        old_mapped.contains(p),
{
    assert(!new_mapped.contains(p));
}

// Test 5: Removing target_ptr should not affect other allocated pages.
// Mutated assertion: another page (other_ptr != target_ptr) was also removed.
// Models: free_page_4k only removes target_ptr from allocated_pages_4k.
// SHOULD FAIL
proof fn test_mutation_other_page_also_removed(
    old_allocated: Set<PagePtr>,
    new_allocated: Set<PagePtr>,
    target_ptr: PagePtr,
    other_ptr: PagePtr,
)
    requires
        old_allocated.contains(target_ptr),
        old_allocated.contains(other_ptr),
        target_ptr != other_ptr,
        new_allocated =~= old_allocated.remove(target_ptr),
{
    assert(!new_allocated.contains(other_ptr));
}

}
