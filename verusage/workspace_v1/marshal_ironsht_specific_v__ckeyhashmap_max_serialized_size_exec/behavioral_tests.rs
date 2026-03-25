extern crate verus_builtin_macros as builtin_macros;
use vstd::prelude::*;
use vstd::*;

fn main() {}

verus! {
    #[verifier::opaque]
    pub open spec fn ckeyhashmap_max_serialized_size() -> usize {
        0x100000
    }

    // Test 1: Negate the correct postcondition — result is NOT 0x100000
    // SHOULD FAIL
    proof fn test_result_negated() {
        reveal(ckeyhashmap_max_serialized_size);
        assert(ckeyhashmap_max_serialized_size() != 0x100000usize);
    }

    // Test 2: Mutate to doubled value
    // SHOULD FAIL
    proof fn test_result_doubled() {
        reveal(ckeyhashmap_max_serialized_size);
        assert(ckeyhashmap_max_serialized_size() == 0x200000usize);
    }

    // Test 3: Mutate to halved value
    // SHOULD FAIL
    proof fn test_result_halved() {
        reveal(ckeyhashmap_max_serialized_size);
        assert(ckeyhashmap_max_serialized_size() == 0x80000usize);
    }

    // Test 4: Mutate to a common "wrong constant" (power of 10 instead of power of 2)
    // SHOULD FAIL
    proof fn test_result_wrong_constant() {
        reveal(ckeyhashmap_max_serialized_size);
        assert(ckeyhashmap_max_serialized_size() == 1000000usize);
    }
}
