# Test Results: `commit_mask__lemma_obtain_bit_index_1.rs`

## File Under Test

The file defines:
- `is_bit_set(a, b)`: spec fn — checks if bit `b` is set in `a` via `a & (1 << b) == (1 << b)`
- `lemma_obtain_bit_index_1_aux(a, hi)`: external_body proof fn — given nonzero `a: u64` with `a >> hi == 0`, returns `i < hi` with bit `i` set
- `lemma_obtain_bit_index_1(a)`: proof fn — given nonzero `a: usize`, returns `b < 64` with `is_bit_set(a, b)`

## Correctness Testing

**File:** `correctness_tests.rs`
**Result:** ✅ 30 verified, 0 errors

### Parameterized Tests (6 tests)
| Test | Description | Result |
|------|-------------|--------|
| P1 | Arbitrary nonzero usize has a valid set bit | ✅ Pass |
| P2 | Result b <= 63 | ✅ Pass |
| P3 | Double call on different inputs both valid | ✅ Pass |
| P4 | Set bit implies value is nonzero | ✅ Pass |
| P5 | Aux with a!=0, hi=64 | ✅ Pass |
| P6 | Aux with smaller hi=32 | ✅ Pass |

### Concrete Tests (17 tests)
| Test | Input | Result |
|------|-------|--------|
| C1 | a = 1 | ✅ Pass |
| C2 | a = 2 | ✅ Pass |
| C3 | a = 4 | ✅ Pass |
| C4 | a = 8 | ✅ Pass |
| C5 | a = 0xFF | ✅ Pass |
| C6 | a = 0xFFFF | ✅ Pass |
| C7 | a = 0x8000_0000_0000_0000 (highest bit) | ✅ Pass |
| C8 | a = 0xFFFF_FFFF_FFFF_FFFF (all bits) | ✅ Pass |
| C9 | a = 0x10 (bit 4) | ✅ Pass |
| C10 | a = 0xDEAD_BEEF (mixed) | ✅ Pass |
| C11 | a = 0x1_0000_0000 (bit 32) | ✅ Pass |
| C12 | a = 0x4000_0000_0000_0000 (bit 62) | ✅ Pass |
| CA1 | aux: a=1, hi=1 | ✅ Pass |
| CA2 | aux: a=1, hi=64 | ✅ Pass |
| CA3 | aux: a=3, hi=2 | ✅ Pass |
| CA4 | aux: a=0xFF, hi=8 | ✅ Pass |
| CA5 | aux: a=0x8000..., hi=64 | ✅ Pass |

**Conclusion:** All correctness tests verify. The specs correctly capture the intended behavior: any nonzero value has at least one set bit.

---

## Completeness Testing

### Round 1: Precondition Violations
**File:** `completeness_round1.rs`
**Result:** ✅ 4/4 tests fail as expected (3 verified infrastructure, 4 errors)

| Test | Description | Expected | Actual |
|------|-------------|----------|--------|
| R1-1 | Call with a=0 | ❌ Fail | ❌ precondition `a != 0` |
| R1-2 | Aux with a=0 | ❌ Fail | ❌ precondition `a != 0` |
| R1-3 | Aux with hi=65 | ❌ Fail | ❌ precondition `hi <= 64` |
| R1-4 | Aux with a>>hi != 0 | ❌ Fail | ❌ precondition `a >> hi == 0` |

### Round 2: Overly Strong Postconditions
**File:** `completeness_round2.rs`
**Result:** ✅ 5/5 tests fail as expected

| Test | Description | Expected | Actual |
|------|-------------|----------|--------|
| R2-1 | Assert b < 32 | ❌ Fail | ❌ assertion failed |
| R2-2 | Assert b == 0 | ❌ Fail | ❌ assertion failed |
| R2-3 | Assert b < 8 | ❌ Fail | ❌ assertion failed |
| R2-4 | Assert b is even | ❌ Fail | ❌ assertion failed |
| R2-5 | Assert b == 7 for 0xFF | ❌ Fail | ❌ assertion failed |

### Round 3: Negated/Wrong Postconditions
**File:** `completeness_round3.rs`
**Result:** ✅ 6/6 tests fail as expected

| Test | Description | Expected | Actual |
|------|-------------|----------|--------|
| R3-1 | Assert bit NOT set | ❌ Fail | ❌ assertion failed |
| R3-2 | Assert b >= 64 | ❌ Fail | ❌ assertion failed |
| R3-3 | Assert complementary bit set | ❌ Fail | ❌ assertion failed |
| R3-4 | Different inputs, same result | ❌ Fail | ❌ assertion failed |
| R3-5 | Assert bit 0 always set | ❌ Fail | ❌ assertion failed |
| R3-6 | Aux negated postcondition | ❌ Fail | ❌ assertion failed |

### Round 4: Incorrect Value Assertions
**File:** `completeness_round4.rs`
**Result:** ✅ 7/7 tests fail as expected

| Test | Description | Expected | Actual |
|------|-------------|----------|--------|
| R4-1 | a=1: assert b != 0 | ❌ Fail | ❌ assertion failed |
| R4-2 | a=2: assert b == 0 | ❌ Fail | ❌ assertion failed |
| R4-3 | a=4: assert b == 1 | ❌ Fail | ❌ assertion failed |
| R4-4 | Assert b > 0 always | ❌ Fail | ❌ assertion failed |
| R4-5 | Assert a == (1 << b) | ❌ Fail | ❌ assertion failed |
| R4-6 | High bit: assert b < 63 | ❌ Fail | ❌ assertion failed |
| R4-7 | Aux: assert i == 0 always | ❌ Fail | ❌ assertion failed |

### Round 5: Cross-function & Edge Cases
**File:** `completeness_round5.rs`
**Result:** ✅ 8/8 tests fail as expected

| Test | Description | Expected | Actual |
|------|-------------|----------|--------|
| R5-1 | is_bit_set(0, 0) | ❌ Fail | ❌ assertion failed |
| R5-2 | is_bit_set(MAX, 64) | ❌ Fail | ❌ assertion failed |
| R5-3 | Assert a > (1 << b) | ❌ Fail | ❌ assertion failed |
| R5-4 | Determinism: two calls same result | ❌ Fail | ❌ assertion failed |
| R5-5 | Aux: assert i == 1 for a=1 | ❌ Fail | ❌ assertion failed |
| R5-6 | All lower bits set | ❌ Fail | ❌ assertion failed |
| R5-7 | Aux with hi=0 | ❌ Fail | ❌ precondition failed |
| R5-8 | Cross-function misuse | ❌ Fail | ❌ precondition failed |

---

## Overall Summary

| Category | Tests | Pass/Fail as Expected |
|----------|-------|-----------------------|
| Correctness | 30 | 30/30 ✅ |
| Completeness R1 (preconditions) | 4 | 4/4 ✅ |
| Completeness R2 (strong postconditions) | 5 | 5/5 ✅ |
| Completeness R3 (negated conditions) | 6 | 6/6 ✅ |
| Completeness R4 (wrong values) | 7 | 7/7 ✅ |
| Completeness R5 (edge cases) | 8 | 8/8 ✅ |
| **Total** | **60** | **60/60 ✅** |

### Assessment

**Correctness:** The specs are correct. All valid usages of the lemma verify successfully across diverse inputs (powers of 2, max values, mixed bit patterns, high/low bits).

**Completeness:** The specs are complete. All 30 incorrect tests across 5 rounds failed as expected:
- Preconditions (`a != 0`, `hi <= 64`, `a >> hi == 0`) are enforced
- Postconditions (`b < 64`, `is_bit_set(a, b)`) are neither too strong nor too weak
- The spec correctly does not over-promise: it doesn't guarantee determinism, specific bit indices, or uniqueness
- Cross-function property violations are caught

The specifications are well-designed: they capture exactly the intended property ("every nonzero integer has at least one set bit") without over-constraining the result.
