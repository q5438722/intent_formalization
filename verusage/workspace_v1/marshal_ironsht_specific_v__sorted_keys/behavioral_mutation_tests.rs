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

// --- Behavioral Mutation Test 1: Reverse the comparison ---
// If a < b, then b < a should NOT hold (strict ordering is antisymmetric)
// SHOULD FAIL
proof fn test_mutation_reverse_ordering() {
    assert(forall |a: CKeyKV, b: CKeyKV| ckeykvlt(a, b) ==> ckeykvlt(b, a));
}

// --- Behavioral Mutation Test 2: Greater-or-equal implies ckeykvlt ---
// Mutate: claim ckeykvlt holds even when a.ukey >= b.ukey
// SHOULD FAIL
proof fn test_mutation_geq_implies_lt() {
    assert(forall |a: CKeyKV, b: CKeyKV| a.k.ukey >= b.k.ukey ==> ckeykvlt(a, b));
}

// --- Behavioral Mutation Test 3: Negate sorted result ---
// A vector that IS sorted (consecutive ascending) should NOT be reported as unsorted
// SHOULD FAIL
proof fn test_mutation_sorted_reported_unsorted() {
    assert(forall |v: Vec<CKeyKV>|
        (v.len() == 2 && v@[0].k.ukey < v@[1].k.ukey) ==> !spec_sorted_keys(v));
}

// --- Behavioral Mutation Test 4: Unsorted vector claimed as sorted ---
// A vector with a descending consecutive pair should NOT be sorted
// SHOULD FAIL
proof fn test_mutation_unsorted_claimed_sorted() {
    assert(forall |v: Vec<CKeyKV>|
        (v.len() == 2 && v@[0].k.ukey > v@[1].k.ukey) ==> spec_sorted_keys(v));
}

}
