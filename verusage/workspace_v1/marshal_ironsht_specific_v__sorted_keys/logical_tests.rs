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

// --- Logical Test 1: Sorted implies non-empty ---
// Empty vectors are vacuously sorted, so this should NOT hold
// SHOULD FAIL
proof fn test_logical_sorted_implies_nonempty() {
    assert(forall |v: Vec<CKeyKV>| spec_sorted_keys(v) ==> v.len() > 0);
}

// --- Logical Test 2: ckeykvlt is reflexive ---
// Strict < is NOT reflexive: x < x is false
// SHOULD FAIL
proof fn test_logical_reflexivity() {
    assert(forall |a: CKeyKV| ckeykvlt(a, a));
}

// --- Logical Test 3: Sorted keys must start at zero ---
// The spec imposes no constraint on the absolute value of keys
// SHOULD FAIL
proof fn test_logical_first_key_is_zero() {
    assert(forall |v: Vec<CKeyKV>|
        spec_sorted_keys(v) && v.len() > 0 ==> v@[0].k.ukey == 0);
}

// --- Logical Test 4: ckeykvlt is total (trichotomy claim) ---
// Claim: for any two CKeyKV, one must be less than the other
// This is false when they have equal keys
// SHOULD FAIL
proof fn test_logical_totality() {
    assert(forall |a: CKeyKV, b: CKeyKV| ckeykvlt(a, b) || ckeykvlt(b, a));
}

// --- Logical Test 5: Sorted implies unique values ---
// While strict ordering does imply unique keys, this test claims
// a stronger unintended structural property: all v values must be empty
// SHOULD FAIL
proof fn test_logical_sorted_implies_empty_values() {
    assert(forall |v: Vec<CKeyKV>|
        spec_sorted_keys(v) && v.len() > 0 ==> v@[0].v@.len() == 0);
}

}
