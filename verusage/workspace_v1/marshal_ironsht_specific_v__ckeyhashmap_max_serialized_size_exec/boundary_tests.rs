extern crate verus_builtin_macros as builtin_macros;
use vstd::prelude::*;
use vstd::*;

fn main() {}

verus! {
    #[verifier::opaque]
    pub open spec fn ckeyhashmap_max_serialized_size() -> usize {
        0x100000
    }

    // Test 1: Assert result equals 0 (zero boundary)
    // SHOULD FAIL
    proof fn test_result_is_zero() {
        reveal(ckeyhashmap_max_serialized_size);
        assert(ckeyhashmap_max_serialized_size() == 0usize);
    }

    // Test 2: Assert result equals usize::MAX (max boundary)
    // SHOULD FAIL
    proof fn test_result_is_max() {
        reveal(ckeyhashmap_max_serialized_size);
        assert(ckeyhashmap_max_serialized_size() == usize::MAX);
    }

    // Test 3: Off-by-one below the correct value
    // SHOULD FAIL
    proof fn test_off_by_one_below() {
        reveal(ckeyhashmap_max_serialized_size);
        assert(ckeyhashmap_max_serialized_size() == (0x100000usize - 1));
    }

    // Test 4: Off-by-one above the correct value
    // SHOULD FAIL
    proof fn test_off_by_one_above() {
        reveal(ckeyhashmap_max_serialized_size);
        assert(ckeyhashmap_max_serialized_size() == (0x100000usize + 1));
    }
}
