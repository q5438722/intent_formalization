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

// ---- Logical Tests ----
// These test properties NOT explicitly guaranteed by the spec.
// If the spec is correct, all should FAIL verification.

// SHOULD FAIL: Valid VAs are not unique — many different addresses can be valid
proof fn test_logical_all_valid_vas_equal()
{
    assert(forall|va1: usize, va2: usize|
        spec_va_4k_valid(va1) && spec_va_4k_valid(va2) ==> va1 == va2
    );
}

// SHOULD FAIL: va_add_range does NOT always preserve 4K validity (can cross boundaries)
proof fn test_logical_add_preserves_validity()
{
    assert(forall|va: usize, i: usize|
        spec_va_4k_valid(va) ==> spec_va_4k_valid(spec_va_add_range(va, i))
    );
}

// SHOULD FAIL: Range validity for n does NOT imply validity for n+1
proof fn test_logical_range_extensible()
{
    assert(forall|va: usize, len: usize|
        #![trigger spec_va_4k_range_valid(va, len)]
        spec_va_4k_range_valid(va, len) ==> spec_va_4k_range_valid(va, (len + 1) as usize)
    );
}

// SHOULD FAIL: va_add_range is NOT globally injective (overflow causes collisions)
proof fn test_logical_global_injectivity()
{
    assert(forall|va1: usize, i1: usize, va2: usize, i2: usize|
        (va1 != va2 || i1 != i2) ==>
            spec_va_add_range(va1, i1) != spec_va_add_range(va2, i2)
    );
}

// SHOULD FAIL: 4K validity does NOT imply 2MB alignment (stronger than required)
proof fn test_logical_stronger_alignment()
{
    assert(forall|va: usize|
        spec_va_4k_valid(va) ==> va % 0x200000 == 0
    );
}

}
