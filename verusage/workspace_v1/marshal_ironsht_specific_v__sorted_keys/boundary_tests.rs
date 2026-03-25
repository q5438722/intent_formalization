use vstd::prelude::*;

fn main() {}

verus! {

#[derive(Eq,PartialEq,Hash)]
pub struct SHTKey {
    pub ukey: u64,
}

pub type AbstractKey = SHTKey;
pub type CKey = SHTKey;

pub struct CKeyKV {
    pub k: CKey,
    pub v: Vec<u8>,
}

pub open spec fn ckeykvlt(a: CKeyKV, b: CKeyKV) -> bool {
    a.k.ukey < b.k.ukey
}

pub open spec fn spec_sorted_keys(v: Vec<CKeyKV>) -> bool {
    forall |i: int, j: int| 0 <= i && i + 1 < v.len() && j == i+1 ==> #[trigger] ckeykvlt(v@[i], v@[j])
}

// --- Boundary Test 1: Equal keys should NOT be considered sorted ---
// ckeykvlt uses strict <, so equal keys should yield false
// SHOULD FAIL
proof fn test_boundary_equal_keys_sorted() {
    assert(forall |kv: CKeyKV| ckeykvlt(kv, kv));
}

// --- Boundary Test 2: Descending pair should NOT be sorted ---
// If a.ukey > b.ukey, then ckeykvlt(a, b) should be false
// SHOULD FAIL
proof fn test_boundary_descending_pair() {
    assert(forall |a: CKeyKV, b: CKeyKV| a.k.ukey > b.k.ukey ==> ckeykvlt(a, b));
}

// --- Boundary Test 3: Maximum u64 value edge case ---
// ckeykvlt(a, b) should not hold when a.ukey == u64::MAX
// because no u64 value can be greater than u64::MAX
// SHOULD FAIL
proof fn test_boundary_max_u64_sorted() {
    assert(forall |a: CKeyKV| a.k.ukey == u64::MAX ==>
        (exists |b: CKeyKV| ckeykvlt(a, b)));
}

// --- Boundary Test 4: Two elements with equal keys are NOT sorted ---
// A two-element vector with equal keys violates strict ordering
// SHOULD FAIL
proof fn test_boundary_two_equal_elements_sorted() {
    assert(forall |v: Vec<CKeyKV>|
        v.len() == 2 && v@[0].k.ukey == v@[1].k.ukey ==> spec_sorted_keys(v));
}

}
