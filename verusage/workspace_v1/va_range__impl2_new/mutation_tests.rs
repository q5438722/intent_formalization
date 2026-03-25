use vstd::prelude::*;

fn main() {}

pub type VAddr = usize;

verus! {

global size_of usize == 8;

// === Definitions from target ===

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

// === Behavioral Mutation Tests ===

// Test 1: Adding zero pages should NOT change address (assert it does)
// SHOULD FAIL
proof fn test_mutation_zero_offset_differs() {
    let va: usize = 0x80_0000_0000usize;
    assert(spec_va_add_range(va, 0usize) != va);
}

// Test 2: Adding one page should change address (assert it doesn't)
// SHOULD FAIL
proof fn test_mutation_one_page_equals_base() {
    let va: usize = 0x80_0000_0000usize;
    assert(spec_va_add_range(va, 1usize) == va);
}

// Test 3: Stride is 4096 per page, not 8192 (assert wrong stride)
// SHOULD FAIL
proof fn test_mutation_wrong_stride() {
    let va: usize = 0x80_0000_0000usize;
    assert(spec_va_add_range(va, 1usize) == (va + 8192) as usize);
}

// Test 4: Assert reversed ordering of offsets (2-page < 1-page)
// SHOULD FAIL
proof fn test_mutation_reversed_ordering() {
    let va: usize = 0x80_0000_0000usize;
    assert(spec_va_add_range(va, 2usize) < spec_va_add_range(va, 1usize));
}

}
