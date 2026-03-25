# Adversarial Test Summary: `ckeyhashmap_max_serialized_size_exec`

## Target Specification

- **Spec function**: `ckeyhashmap_max_serialized_size()` — opaque, returns `0x100000`
- **Exec function**: `ckeyhashmap_max_serialized_size_exec()` — ensures `r == ckeyhashmap_max_serialized_size()`

## Results

All **12/12** adversarial tests were **correctly rejected** by Verus (failed verification), indicating the specification is consistent with respect to the queried properties.

### Boundary Tests (4/4 correctly rejected)

| Test | Property (φ) | Result |
|------|-------------|--------|
| `test_result_is_zero` | result == 0 | ✗ REJECTED |
| `test_result_is_max` | result == usize::MAX | ✗ REJECTED |
| `test_off_by_one_below` | result == 0xFFFFF | ✗ REJECTED |
| `test_off_by_one_above` | result == 0x100001 | ✗ REJECTED |

### Behavioral Mutation Tests (4/4 correctly rejected)

| Test | Property (φ) | Result |
|------|-------------|--------|
| `test_result_negated` | result != 0x100000 | ✗ REJECTED |
| `test_result_doubled` | result == 0x200000 | ✗ REJECTED |
| `test_result_halved` | result == 0x80000 | ✗ REJECTED |
| `test_result_wrong_constant` | result == 1000000 (decimal) | ✗ REJECTED |

### Logical Tests (4/4 correctly rejected)

| Test | Property (φ) | Result |
|------|-------------|--------|
| `test_opaque_positive` | result > 0 (no reveal) | ✗ REJECTED |
| `test_opaque_exact_value` | result == 0x100000 (no reveal) | ✗ REJECTED |
| `test_opaque_upper_bound` | result < 0x200000 (no reveal) | ✗ REJECTED |
| `test_stronger_inequality` | result > 0x100000 (with reveal) | ✗ REJECTED |

## Analysis

The specification is **tight and well-formed**:

1. **Boundary robustness**: Incorrect constant values (zero, max, off-by-one) are all rejected.
2. **Behavioral correctness**: Mutated outputs (negated, doubled, halved, wrong base) are rejected.
3. **Logical consistency**: The `#[verifier::opaque]` attribute properly prevents information leakage — even true properties (like `> 0`) cannot be deduced without explicit `reveal`. Stronger-than-specified inequalities are also rejected.

**Conclusion**: No specification weakness detected. The opaque constant pattern provides both behavioral correctness (via `reveal`) and information hiding (via opacity).
