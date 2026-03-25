use vstd::prelude::*;

fn main() {}

verus! {

global size_of usize == 8;

// ---- Copied spec definitions from source ----

pub const KERNEL_MEM_END_L4INDEX: usize = 1;
pub const MEM_4k_MASK: u64 = 0x0000_ffff_ffff_f000;

pub open spec fn spec_va_4k_valid(va: usize) -> bool {
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

pub open spec fn spec_va_add_range(va: usize, i: usize) -> usize {
    (va + (i * 4096)) as usize
}

pub open spec fn spec_va_4k_range_valid(va: usize, len: usize) -> bool {
    forall|i: usize|
        #![trigger spec_va_add_range(va, i)]
        0 <= i < len ==> spec_va_4k_valid(spec_va_add_range(va, i))
}

// ---- Behavioral Mutation Tests ----
// These tests mutate expected outputs or relations.
// If the spec is correct, all should FAIL verification.

// SHOULD FAIL: Empty range (len=0) is vacuously valid; asserting it's invalid is wrong
proof fn test_mutation_empty_range_is_invalid()
{
    let va: usize = 0x80_0000_0000usize;
    assert(!spec_va_4k_range_valid(va, 0));
}

// SHOULD FAIL: Adding 0 pages should NOT change the address
proof fn test_mutation_add_zero_changes_address()
{
    let va: usize = 0x80_0000_0000usize;
    assert(spec_va_add_range(va, 0) != va);
}

// SHOULD FAIL: Wrong step size — va_add_range uses 4096, not 1
proof fn test_mutation_wrong_step_size()
{
    let va: usize = 0x80_0000_0000usize;
    assert(spec_va_add_range(va, 1) == (va + 1) as usize);
}

// SHOULD FAIL: Adding 1 page should NOT equal the base address
proof fn test_mutation_add_one_equals_base()
{
    let va: usize = 0x80_0000_0000usize;
    assert(spec_va_add_range(va, 1) == va);
}

// SHOULD FAIL: Adding 2 pages should NOT equal adding 1 page
proof fn test_mutation_two_pages_equals_one()
{
    let va: usize = 0x80_0000_0000usize;
    assert(spec_va_add_range(va, 2) == spec_va_add_range(va, 1));
}

}
