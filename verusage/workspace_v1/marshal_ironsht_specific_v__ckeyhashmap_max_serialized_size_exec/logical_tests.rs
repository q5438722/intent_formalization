extern crate verus_builtin_macros as builtin_macros;
use vstd::prelude::*;
use vstd::*;

fn main() {}

verus! {
    #[verifier::opaque]
    pub open spec fn ckeyhashmap_max_serialized_size() -> usize {
        0x100000
    }

    // Test 1: Without reveal, assert value is positive — tests opacity
    // SHOULD FAIL
    proof fn test_opaque_positive() {
        // Do NOT reveal — the verifier should not know the value
        assert(ckeyhashmap_max_serialized_size() > 0usize);
    }

    // Test 2: Without reveal, assert exact value — tests opacity
    // SHOULD FAIL
    proof fn test_opaque_exact_value() {
        // Do NOT reveal — the verifier should not know the value
        assert(ckeyhashmap_max_serialized_size() == 0x100000usize);
    }

    // Test 3: Without reveal, assert an upper bound — tests opacity
    // SHOULD FAIL
    proof fn test_opaque_upper_bound() {
        // Do NOT reveal — the verifier should not know the value
        assert(ckeyhashmap_max_serialized_size() < 0x200000usize);
    }

    // Test 4: With reveal, assert a strictly stronger inequality (> instead of ==)
    // SHOULD FAIL
    proof fn test_stronger_inequality() {
        reveal(ckeyhashmap_max_serialized_size);
        assert(ckeyhashmap_max_serialized_size() > 0x100000usize);
    }
}
