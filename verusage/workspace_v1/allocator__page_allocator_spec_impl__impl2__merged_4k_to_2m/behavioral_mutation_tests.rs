use vstd::prelude::*;

fn main() {}

verus!{

pub type PagePtr = usize;

pub const NUM_PAGES: usize = 2 * 1024 * 1024;

pub open spec fn page_ptr_valid(ptr: usize) -> bool {
    &&& ptr % 0x1000 == 0
    &&& ptr / 0x1000 < NUM_PAGES
}

pub open spec fn page_ptr_2m_valid(ptr: usize) -> bool {
    ((ptr % (0x200000)) == 0) && ((ptr / 4096) < NUM_PAGES)
}


// ===================== BEHAVIORAL MUTATION TESTS =====================
// Each test models the postconditions of merged_4k_to_2m and mutates
// the expected output/relation, checking if incorrect behaviors are rejected.
// All tests SHOULD FAIL verification.

// Test 1: free_pages_4k count should decrease by exactly 512, not 511.
// Postcondition: self.free_pages_4k().len() == old.free_pages_4k().len() - 512.
// Mutated: assert decrease is only 511.
// SHOULD FAIL
proof fn test_mutation_free4k_decrease_by_511(old_len: int, new_len: int)
    requires
        old_len >= 512,
        new_len == old_len - 512,
{
    assert(new_len == old_len - 511);
}

// Test 2: free_pages_2m count should increase by exactly 1, not 2.
// Postcondition: self.free_pages_2m().len() == old.free_pages_2m().len() + 1.
// Mutated: assert increase is 2.
// SHOULD FAIL
proof fn test_mutation_free2m_increase_by_2(old_len: int, new_len: int)
    requires
        old_len < NUM_PAGES,
        new_len == old_len + 1,
{
    assert(new_len == old_len + 2);
}

// Test 3: free_pages_1g count should remain unchanged after merge.
// Postcondition: self.free_pages_1g().len() == old.free_pages_1g().len().
// Mutated: assert count increased by 1.
// SHOULD FAIL
proof fn test_mutation_free1g_changed(old_len: int, new_len: int)
    requires
        new_len == old_len,
{
    assert(new_len == old_len + 1);
}

// Test 4: allocated_pages_4k should be preserved after merge.
// Postcondition: self.allocated_pages_4k() =~= old.allocated_pages_4k().
// Mutated: assert a new page appeared in allocated_pages_4k.
// SHOULD FAIL
proof fn test_mutation_allocated4k_gained_page(
    old_alloc: Set<PagePtr>,
    new_alloc: Set<PagePtr>,
    p: PagePtr
)
    requires
        new_alloc =~= old_alloc,
        !old_alloc.contains(p),
{
    assert(new_alloc.contains(p));
}

// Test 5: allocated_pages_2m should be preserved after merge.
// Postcondition: self.allocated_pages_2m() =~= old.allocated_pages_2m().
// Mutated: assert a page was removed from allocated_pages_2m.
// SHOULD FAIL
proof fn test_mutation_allocated2m_lost_page(
    old_alloc: Set<PagePtr>,
    new_alloc: Set<PagePtr>,
    p: PagePtr
)
    requires
        new_alloc =~= old_alloc,
        old_alloc.contains(p),
{
    assert(!new_alloc.contains(p));
}

}
