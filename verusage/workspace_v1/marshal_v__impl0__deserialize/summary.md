# Adversarial Proof Test Summary: `marshal_v__impl0__deserialize`

## Target
`source-projects/ironkv/verified/marshal_v/marshal_v__impl0__deserialize.rs`

Specifications tested: `Marshalable` trait (`is_marshalable`, `ghost_serialize`, `deserialize` postcondition) and implementations for `u64`, `usize`, `Option<T>`, `(T, U)`.

---

## Results Overview

| File | Tests | All Failed (as expected) |
|---|---|---|
| `boundary_tests.rs` | 5 | ✅ Yes (0 verified, 5 errors) |
| `behavioral_mutation_tests.rs` | 5 | ✅ Yes (0 verified, 5 errors) |
| `logical_tests.rs` | 5 | ✅ Yes (0 verified, 5 errors) |

**Total: 15/15 tests correctly rejected by the specification.**

---

## Boundary Tests (5/5 FAILED ✅)

| # | Test | What it asserts (incorrectly) | Failure mode |
|---|---|---|---|
| 1 | `test_boundary_u64_zero_serialize_length_zero` | `u64(0).ghost_serialize().len() == 0` | Edge case: zero value still needs 8 bytes |
| 2 | `test_boundary_u64_max_serialize_short` | `u64::MAX.ghost_serialize().len() < 8` | Edge case: max value still needs 8 bytes |
| 3 | `test_boundary_option_none_not_marshalable` | `!Option::None.is_marshalable()` | None is always marshalable |
| 4 | `test_boundary_pair_not_marshalable` | `!(0u64, 0u64).is_marshalable()` | Valid pair is marshalable |
| 5 | `test_boundary_option_some_serialize_tag_only` | `Some(0u64).ghost_serialize().len() == 1` | Tag + payload = 9 bytes, not 1 |

## Behavioral Mutation Tests (5/5 FAILED ✅)

| # | Test | What it asserts (incorrectly) | Failure mode |
|---|---|---|---|
| 1 | `test_mutation_option_none_wrong_tag` | `None.gs() =~= seq![1]` | Tag byte is 0, not 1 |
| 2 | `test_mutation_option_some_missing_tag` | `Some(v).gs() =~= v.gs()` | Missing seq![1] prefix |
| 3 | `test_mutation_pair_reversed_order` | `(1,2).gs() =~= 2.gs() + 1.gs()` | Serialization order is first then second |
| 4 | `test_mutation_u64_same_serialize_different_values` | `0u64.gs() =~= 1u64.gs()` | Different values ≠ same bytes |
| 5 | `test_mutation_usize_differs_from_u64` | `usize(42).gs() ≠ u64(42).gs()` | usize delegates to u64; they're equal |

## Logical Tests (5/5 FAILED ✅)

| # | Test | What it asserts (incorrectly) | Failure mode |
|---|---|---|---|
| 1 | `test_logical_derive_false` | `false` after valid operations | Soundness check: spec doesn't entail false |
| 2 | `test_logical_u64_serialize_length_varies` | `0.gs().len() != 1.gs().len()` | Fixed-width: always 8 bytes |
| 3 | `test_logical_none_some_same_length` | `None.gs().len() == Some(0).gs().len()` | 1 ≠ 9 |
| 4 | `test_logical_pair_length_first_only` | `(1,2).gs().len() == 1.gs().len()` | Length is sum, not first only |
| 5 | `test_logical_pair_serialize_commutative` | `(1,2).gs() =~= (2,1).gs()` | Serialization is not commutative |

---

## Conclusion

The specification is **strong enough** to reject all 15 adversarial queries:
- **Boundary**: Invalid edge cases (zero-length serialization, invalid marshalability) are properly rejected.
- **Behavioral**: Mutated outputs (wrong tags, reversed order, non-injective claims) are properly rejected.
- **Logical**: Unintended properties (false derivation, variable-length u64, commutativity) are properly rejected.

No spec weaknesses were detected — the specification correctly constrains the semantic space for the tested properties.
