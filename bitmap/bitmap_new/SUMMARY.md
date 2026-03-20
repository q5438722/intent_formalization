# Bitmap Verus Test Generation — Summary

## Overview

Generated and validated Verus verified test cases for the `bitmap` crate to assess
**correctness** (specs prove correct properties) and **completeness** (specs reject
incorrect assertions).

## Step 1: Spec Removal

Ran `remove_specs.py` on `bitmap/bitmap_new/lib.rs` to produce `lib.no_spec.rs`, stripping
`proof! {}` blocks while retaining `#[verus_spec(...)]` annotations for verification.

## Step 2–3: Correctness Tests

### Generated Test File: `lib.gen_test.rs`

**52 correct test functions** (35 parametric + 17 concrete) covering all public bitmap operations:

| Category              | Parametric | Concrete | Functions Covered                     |
|-----------------------|-----------|----------|---------------------------------------|
| Constructor           | 4         | 0        | `new` (valid, zero, non-mult-8, max)  |
| Getter                | 1         | 0        | `number_of_bits`                      |
| Set/Test/Clear        | 9         | 5        | `set`, `test`, `clear`, frame props   |
| Alloc                 | 4         | 2        | `alloc` (empty, two-allocs, unset)    |
| Alloc Range           | 6         | 4        | `alloc_range` (size=1,3,8, full)      |
| Usage Tracking        | 4         | 1        | Usage after set/clear/alloc/alloc_range|
| Constancy             | 1         | 1        | `num_bits` constant across operations |
| Index                 | 3         | 2        | `index(0/7/8)`, OOB                   |
| Alloc-Clear-Realloc   | 1         | 0        | clear + realloc cycle                 |
| Full Bitmap           | 2         | 0        | alloc on full, error frame            |
| Invariant             | 4         | 0        | `inv()` after set/clear/alloc/alloc_range |
| Double set/clear      | 0         | 2        | Double-set fails, double-clear fails  |

Concrete tests use specific values: `Bitmap::new(64)`, index=0/3/5/7/8/10 (byte boundaries),
size=1/3/4/8 for alloc_range. `index(0)→(0,0)`, `index(7)→(0,7)`, `index(8)→(1,0)`.

### Verification Result

```
verification results:: 128 verified, 0 errors
```

All 52 correct tests verified successfully.

## Step 4–5: Completeness Tests (Error Injection)

### Generated: 13 Incorrect Test Cases (All Concrete Inputs)

Each test uses concrete values (`Bitmap::new(64)`, specific bit indices) and injects a single
wrong assertion that should be **rejected** by the specs:

| ID     | Error Injected                                        | Input       | Result   |
|--------|-------------------------------------------------------|-------------|----------|
| err01  | New bitmap has bit 0 set                              | new(64)     | REJECTED ✓ |
| err02  | New bitmap usage is 1                                 | new(64)     | REJECTED ✓ |
| err03  | After set(5), bit 5 is NOT set                        | new(64), 5  | REJECTED ✓ |
| err04  | After clear(5), bit 5 is still set                    | new(64), 5  | REJECTED ✓ |
| err05  | Setting bit 0 also sets bit 1 (frame violation)       | new(64), 0,1| REJECTED ✓ |
| err06  | Usage unchanged after set(0)                          | new(64), 0  | REJECTED ✓ |
| err07  | Alloc returns index >= 64                             | new(64)     | REJECTED ✓ |
| err08  | alloc_range(4) gives usage 5                          | new(64), 4  | REJECTED ✓ |
| err09  | num_bits changes after alloc                          | new(64)     | REJECTED ✓ |
| err10  | Two allocs return same index                          | new(64)     | REJECTED ✓ |
| err11  | set(3)+clear(3) gives usage 2                         | new(64), 3  | REJECTED ✓ |
| err12  | alloc_range(4) range is all unset                     | new(64), 4  | REJECTED ✓ |
| err13  | Error mutates full bitmap state                       | new(8)      | REJECTED ✓ |

### Completeness Result

**13/13 incorrect assertions rejected** — the specifications are complete enough
to catch all tested categories of errors.

## Conclusions

1. **Correctness: PASS** — All 52 correct test cases verify (128 total items).
2. **Completeness: PASS** — All 13 error-injected test cases are correctly rejected.
3. The specifications cover:
   - Functional correctness (set/clear/alloc behavior)
   - Frame properties (non-interference, same-byte and cross-byte)
   - Usage tracking (increment/decrement accounting)
   - Structural invariant preservation (`inv()`)
   - Error behavior (no-mutation on failure)
   - Capacity/bounds (index range, num_bits constancy)

---

## Round 2: Deep Spec Completeness Analysis

### Approach

Studied each spec clause to identify structural weaknesses:
1. **Implication vs conjunction** — `new` uses `==>` (implication), `from_raw_array` uses `&&` (conjunction)
2. **Match-based error conditions** — `set`/`clear`/`alloc` use `match` with constraining error branches
3. **Set algebra automation** — Whether SMT can derive `{}.insert(i).remove(i) == {}` automatically
4. **Existential witness instantiation** — Whether SMT can find witnesses for `has_free_bit()` / `!is_full()`

### Round 2 Incorrect Tests (err14–err26): All 13 REJECTED ✓

| ID     | Error Injected                                              | Result   |
|--------|-------------------------------------------------------------|----------|
| err14  | alloc_range(3) off-by-one: assert bit start+3 is set       | REJECTED ✓ |
| err15  | set(0) then alloc: assert alloc returned 0 (already set)   | REJECTED ✓ |
| err16  | set(0)+set(1)+clear(0): assert bit 1 is unset (frame)      | REJECTED ✓ |
| err17  | set(0)+set(1)+clear(0): assert usage is 0 (should be 1)    | REJECTED ✓ |
| err18  | alloc_range(4): assert bit outside range is set (frame)     | REJECTED ✓ |
| err19  | Three allocs: assert usage is 2 (should be 3)              | REJECTED ✓ |
| err20  | Two alloc_range(2): assert ranges overlap                   | REJECTED ✓ |
| err21  | Clear unset bit: assert success (should be Err)             | REJECTED ✓ |
| err22  | alloc_range(2): assert usage is 1 (should be 2)            | REJECTED ✓ |
| err23  | After alloc: assert returned bit is NOT set                 | REJECTED ✓ |
| err24  | alloc_range(4): assert start+size > num_bits                | REJECTED ✓ |
| err25  | index(9): assert word=0 (should be 1)                      | REJECTED ✓ |
| err26  | index(9): assert bit=2 (should be 1)                       | REJECTED ✓ |

### Gap-Probing Tests: Spec Completeness Gaps Found

These tests assert **correct** properties. FAILED = spec can't prove the property (gap).

| ID     | Property Tested                                      | Result                           |
|--------|------------------------------------------------------|----------------------------------|
| gap01  | `new(8) is Ok` (liveness)                            | **FAILED — SPEC GAP**            |
| gap02  | `set(0)+clear(0) → is_empty()`                       | **FAILED — AUTOMATION GAP**      |
| gap03  | `alloc_range(8) on 8-bit → is_full()`                | PASSED ✓                         |
| gap04  | `alloc() on empty bitmap is Ok`                       | **FAILED — SPEC GAP**            |
| gap05  | `set(0) on empty bitmap is Ok`                        | PASSED ✓                         |
| gap06  | `alloc() on 7/8-filled bitmap is Ok`                  | **FAILED — SPEC GAP**            |
| gap07  | `clear(0) after set(0) is Ok`                         | PASSED ✓                         |
| gap08  | `alloc() on empty bitmap is Ok` (with lemma)          | PASSED ✓ (gap04 fixed w/ lemma)  |
| gap09  | `alloc_range(1) on empty bitmap is Ok`                | **FAILED — SPEC GAP**            |
| gap10  | `!is_full()` on empty bitmap                          | **FAILED — SPEC GAP**            |
| gap11  | `!is_empty()` after set(0)                            | PASSED ✓                         |
| gap12  | `set_bits =~= Set::empty()` after set+clear           | PASSED ✓ (gap02 fixed w/ `=~=`)  |
| gap13  | `!is_full()` on empty bitmap (with lemma)             | PASSED ✓ (gap10 fixed w/ lemma)  |
| gap14  | `alloc_range(1)` liveness (with lemma)                | PASSED ✓ (gap09 fixed w/ lemma)  |
| gap15  | `is_empty()` after set+clear (with `=~=` trigger)     | PASSED ✓ (gap02 fixed w/ `=~=`)  |
| gap16  | `has_free_bit()` after set(0) on 64-bit               | **FAILED — AUTOMATION GAP**      |

### Root Cause Analysis

**3 distinct spec/automation gaps identified:**

#### Gap A: `new()` lacks liveness guarantee (gap01)
- **Root cause**: `new` spec uses `result matches Ok(bitmap) ==> {...}` (implication), not `&&` (conjunction)
- **Impact**: Can't prove `new(valid_input)` succeeds, even though it always does
- **Contrast**: `from_raw_array` uses `result matches Ok(bitmap) && {...}` — guarantees success
- **Fix**: Change `==>` to `&&` in `new` spec, or add `ensures valid_input ==> result is Ok`

#### Gap B: `!is_full()` / `has_free_bit()` not automatically derivable (gap04, gap06, gap09, gap10, gap16)
- **Root cause**: Proving `!is_full()` requires finding a witness `i` where `!set_bits.contains(i)`.
  SMT solvers don't automatically instantiate existentials from `usage() < num_bits`.
- **Impact**: `alloc()` / `alloc_range()` liveness can't be proved without manually calling
  `lemma_usage_less_than_capacity_means_not_full()` (gap08, gap13, gap14 show this fixes it)
- **Fix**: The lemma exists but is not auto-triggered. Could add `#[verifier::trigger]` annotations
  or make the spec auto-apply the lemma.

#### Gap C: `is_empty()` requires extensional equality trigger (gap02)
- **Root cause**: `is_empty()` is defined as `set_bits == Set::empty()`. After `set+clear`,
  `set_bits` is `{}.insert(i).remove(i)`, which equals `{}` but SMT needs `=~=` to derive it.
- **Impact**: Can't prove bitmap returns to empty state after set+clear cycle
- **Fix**: Use `=~=` in the assert (gap12, gap15 show this works), or define `is_empty()` using
  `forall|i| !set_bits.contains(i)` instead of set equality.

## Updated Totals

- **Incorrect tests**: 26 total (err01–err26), all **26/26 REJECTED** ✓
- **Gap-probing tests**: 16 total (gap01–gap16)
  - 9 PASSED (spec is complete)
  - 4 FAILED → fixable with existing lemmas (automation gaps)
  - 3 FAILED → genuine spec gaps (new liveness, is_full derivation, has_free_bit)

## Files

- `bitmap/bitmap_new/lib.gen_test.rs` — Correct test suite (52 tests: 35 parametric + 17 concrete)
- `bitmap/bitmap_new/incorrect_tests/err01–err26.rs` — Incorrect test cases (26 total)
- `bitmap/bitmap_new/incorrect_tests/gap01–gap16.rs` — Gap-probing test cases (16 total)
- `bitmap/bitmap_new/incorrect_tests/results.txt` — Raw test results
- `nanvix/src/libs/bitmap/src/lib.gen_test.rs` — Deployed correct test suite
