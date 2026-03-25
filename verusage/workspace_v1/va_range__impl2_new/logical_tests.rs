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

#[verifier(when_used_as_spec(spec_va_4k_valid))]
pub fn va_4k_valid(va: usize) -> (ret: bool)
    ensures
        ret == spec_va_4k_valid(va),
{
    (va & (!MEM_4k_MASK) as usize == 0) && (va as u64 >> 39u64 & 0x1ffu64)
        >= KERNEL_MEM_END_L4INDEX as u64
}

#[verifier::external_body]
#[verifier(external_body)]
pub proof fn va_range_lemma()
    ensures
        forall|va: VAddr, len: usize, i: usize, j: usize|
            #![trigger spec_va_4k_range_valid(va,len), spec_va_add_range(va, i), spec_va_add_range(va, j)]
            va_4k_valid(va) && spec_va_4k_range_valid(va, len) && 0 <= i < len && 0 <= i < len ==> (
            (i == j) == (spec_va_add_range(va, i) == spec_va_add_range(va, j))),
{
    unimplemented!()
}

// === Logical Tests ===

// Test 1: NOT all addresses are valid 4K addresses
// SHOULD FAIL
proof fn test_logical_universal_validity() {
    assert(forall|va: usize| spec_va_4k_valid(va));
}

// Test 2: Range validity for len does NOT imply validity for len+1
// SHOULD FAIL
proof fn test_logical_range_extension() {
    let va: usize = 0x80_0000_0000usize;
    assume(spec_va_4k_valid(va));
    assume(spec_va_4k_range_valid(va, 5usize));
    assert(spec_va_4k_range_valid(va, 6usize));
}

// Test 3: Monotonicity of va_add_range does NOT hold universally (overflow)
// SHOULD FAIL
proof fn test_logical_universal_monotonicity() {
    assert(forall|va: usize, i: usize, j: usize|
        i < j ==> spec_va_add_range(va, i) < spec_va_add_range(va, j));
}

// Test 4: The external_body lemma's bug (j unconstrained) enables
// unsound reasoning when spec_va_add_range wraps around via overflow.
// j=2^52 causes (va + j*4096) to wrap modulo 2^64 back to va.
// SHOULD FAIL (but may PASS — indicating spec weakness from unsound lemma)
proof fn test_logical_lemma_unsoundness() {
    va_range_lemma();
    let va: usize = 0x80_0000_0000usize;
    let len: usize = 2usize;
    let i: usize = 0usize;
    let j: usize = 4503599627370496usize; // 2^52: (va + 2^52 * 4096) wraps mod 2^64

    assume(spec_va_4k_valid(va));
    assume(spec_va_4k_range_valid(va, len));
    // Overflow wrap: spec_va_add_range(va, 2^52) == va
    assume(spec_va_add_range(va, j) == va);

    // Trigger terms for the lemma's forall quantifier
    assert(spec_va_add_range(va, i) == va);

    // Lemma instantiates with i=0, j=2^52:
    //   (0 == 2^52) == (spec_va_add_range(va,0) == spec_va_add_range(va,2^52))
    //   = false == (va == va)
    //   = false == true
    //   = false
    // From false, anything follows.
    assert(false);
}

}
