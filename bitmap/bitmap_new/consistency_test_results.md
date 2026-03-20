# Consistency Test Results for Bitmap Specification

## Framework

Following the **Consistency via Entailment-Guided Querying** framework (consistency_v1.md), we evaluate the bitmap specification by generating three types of queries (φ) and checking whether the specification S correctly entails or rejects them:

> **A correct specification should entail all intended properties, and entail none of the unintended ones.**

| Query Type | What φ Encodes | Purpose |
|---|---|---|
| **Behavioral** | Input-output relations | Mutate expected outputs → spec should REJECT |
| **Boundary** | Input validity | Probe precondition edges → spec should correctly classify |
| **Logical** | Arbitrary semantic claims | Test higher-level properties → detect logical gaps |

---

## 1. Behavioral Inconsistency Tests (18 tests)

**File:** `behavioral_inconsistency_tests.rs`
**Expected:** ALL should be REJECTED (spec should refuse to entail incorrect behaviors)

| Test | Description | Result |
|---|---|---|
| BEH01 | After `set(0)`, assert bit NOT set (negated postcondition) | REJECTED ✓ |
| BEH02 | After `clear(0)`, assert bit still set (negated postcondition) | REJECTED ✓ |
| BEH03 | After `alloc()`, assert returned index's bit NOT set | REJECTED ✓ |
| BEH04 | After `alloc_range(8)`, assert not all bits in range set | REJECTED ✓ |
| BEH05 | After `test(0)` on empty bitmap, assert returns true | REJECTED ✓ |
| BEH06 | After `set(0)`, assert usage is 2 (should be 1) | REJECTED ✓ |
| BEH07 | After `alloc_range(4)`, assert usage is 3 (should be 4) | REJECTED ✓ |
| BEH08 | After `set(0); set(1)`, assert usage is 1 (should be 2) | REJECTED ✓ |
| BEH09 | After `set(0); set(1); clear(0)`, assert usage is 0 (should be 1) | REJECTED ✓ |
| BEH10 | After `new(64)`, assert num_bits is 32 (should be 64) | REJECTED ✓ |
| BEH11 | After `set(0)`, assert bit 1 also set (frame violation) | REJECTED ✓ |
| BEH12 | After `clear(0)`, assert bit 1 changed (frame violation) | REJECTED ✓ |
| BEH13 | After `alloc()`, assert num_bits changed (frame violation) | REJECTED ✓ |
| BEH14 | After `alloc_range(4)`, assert bit outside range set (frame violation) | REJECTED ✓ |
| BEH15 | Two consecutive allocs return same index (non-overlap mutation) | REJECTED ✓ |
| BEH16 | Clear already-clear bit, assert succeeds (error path mutation) | REJECTED ✓ |
| BEH17 | Set already-set bit, assert succeeds (error path mutation) | REJECTED ✓ |
| BEH18 | After failed operation, assert state was mutated | REJECTED ✓ |

**Summary:** 18/18 REJECTED ✓ — **No behavioral inconsistencies detected.**

The specification correctly rejects all mutated output assertions across:
- Output negation (5 tests)
- Value substitution (5 tests)
- Frame violation (5 tests)
- Error path mutation (3 tests)

---

## 2. Boundary Inconsistency Tests (20 tests)

**File:** `boundary_inconsistency_tests.rs`
**Expected:** SHOULD-REJECT tests → REJECTED; SHOULD-PASS tests → PASSED

| Test | Description | Expected | Result | Match? |
|---|---|---|---|---|
| BND01 | `new(0)` succeeds | REJECT | REJECTED | ✓ |
| BND02 | `new(7)` succeeds (non-aligned) | REJECT | REJECTED | ✓ |
| BND03 | `new(u32::MAX)` succeeds | REJECT | REJECTED | ✓ |
| BND04 | `new(8)` works (min valid size) | PASS | PASSED | ✓ |
| BND05 | `new(4294967288)` (just below max) | PASS | PASSED | ✓ |
| BND06 | `test(64)` on 64-bit bitmap succeeds (OOB) | REJECT | REJECTED | ✓ |
| BND07 | `set(64)` on 64-bit bitmap succeeds (OOB) | REJECT | REJECTED | ✓ |
| BND08 | `clear(64)` on 64-bit bitmap succeeds (OOB) | REJECT | REJECTED | ✓ |
| BND09 | `test(63)` on 64-bit bitmap returns false | PASS | PASSED | ✓ |
| BND10 | `set(63)` on 64-bit bitmap works | PASS | PASSED | ✓ |
| BND11 | `test(1000)` on 64-bit bitmap succeeds (far OOB) | REJECT | REJECTED | ✓ |
| BND12 | Double `set(0)` succeeds | REJECT | REJECTED | ✓ |
| BND13 | `clear(0)` on unset bit succeeds | REJECT | REJECTED | ✓ |
| BND14 | `alloc()` on full 8-bit bitmap returns Err | PASS | PASSED | ✓ |
| BND15 | `alloc_range(0)` with `requires false` | PASS | PASSED | ✓ |
| BND16 | `alloc_range(8)` on half-full 8-bit bitmap returns Err | PASS | PASSED | ✓ |
| BND17 | `set(7)` at byte boundary, bit 8 unaffected | PASS | PASSED | ✓ |
| BND18 | `set(8)` at second byte start, bit 7 unaffected | PASS | PASSED | ✓ |
| BND19 | `alloc_range(4)` crossing byte boundary | PASS | PASSED | ✓ |
| BND20 | `alloc_range(8)` fills entire 8-bit bitmap | PASS | PASSED | ✓ |

**Summary:** 20/20 match expectations — **No boundary inconsistencies detected.**

- 9 SHOULD-REJECT tests: all correctly REJECTED ✓
- 11 SHOULD-PASS tests: all correctly PASSED ✓

The specification correctly:
- Rejects invalid constructor arguments (zero, non-aligned, overflow)
- Rejects out-of-bounds index access
- Rejects state-dependent precondition violations (double-set, clear-unset)
- Accepts valid boundary operations (last index, byte boundaries, full-size ranges)

---

## 3. Logical Inconsistency Tests (22 tests)

**File:** `logical_inconsistency_tests.rs`
**Expected:** SHOULD-PASS tests → PASSED; SHOULD-REJECT tests → REJECTED

| Test | Description | Expected | Result | Match? |
|---|---|---|---|---|
| LOG01 | `set(i); clear(i)` restores set_bits | PASS | PASSED | ✓ |
| LOG02 | `set(i); clear(i)` restores usage | PASS | PASSED | ✓ |
| LOG03 | `set(i); clear(i)` yields is_empty() | PASS | PASSED | ✓ |
| LOG04 | `alloc(); clear()` frees bit | PASS | PASSED | ✓ |
| LOG05 | `set(0); set(1)` — both bits set, usage == 2 | PASS | PASSED | ✓ |
| LOG06 | `set(1); set(0)` — same result as LOG05 | PASS | PASSED | ✓ |
| LOG07 | `clear(0); clear(1)` is commutative on result | PASS | PASSED | ✓ |
| LOG08 | Empty bitmap is not full (with lemma) | PASS | PASSED | ✓ |
| **LOG09** | **Full bitmap is not empty** | **PASS** | **REJECTED** | **✗ SPEC GAP** |
| LOG10 | Empty AND has-set-bit is contradiction | REJECT | REJECTED | ✓ |
| LOG11 | Usage exceeds capacity | REJECT | REJECTED | ✓ |
| LOG12 | `set` increases usage by exactly 1 | PASS | PASSED | ✓ |
| LOG13 | `clear` decreases usage by exactly 1 | PASS | PASSED | ✓ |
| LOG14 | `set` decreases usage (wrong monotonicity) | REJECT | REJECTED | ✓ |
| LOG15 | `clear` increases usage (wrong monotonicity) | REJECT | REJECTED | ✓ |
| LOG16 | `alloc()` on empty bitmap must succeed | PASS | PASSED | ✓ |
| LOG17 | `alloc()` after clear must succeed | PASS | PASSED | ✓ |
| **LOG18** | **`alloc_range(8)` on empty 64-bit bitmap must succeed** | **PASS** | **REJECTED** | **✗ SPEC GAP** |
| LOG19 | alloc always returns index 0 (false determinism) | REJECT | REJECTED | ✓ |
| LOG20 | Two alloc_ranges must be adjacent (false) | REJECT | REJECTED | ✓ |
| LOG21 | set_bits equality implies view equality (false) | REJECT | REJECTED | ✓ |
| LOG22 | alloc vs alloc_range(1) must differ (false) | REJECT | REJECTED | ✓ |

**Summary:** 20/22 match expectations — **2 spec gaps detected.**

- 12 SHOULD-PASS tests: 10 PASSED ✓, **2 REJECTED ✗**
- 10 SHOULD-REJECT tests: all correctly REJECTED ✓

### Detected Spec Gaps

#### GAP 1: LOG09 — Full bitmap cannot be proved non-empty

**Test:** After setting all 8 bits, call `lemma_usage_equals_number_of_bits_implies_full()` and assert `is_full()` then `!is_empty()`.

**Failure point:** `assert(bitmap@.is_full())` — the verifier cannot connect the lemma's postcondition (`forall|i| ... ==> is_bit_set(i)`) to the `is_full()` spec function, even though they are semantically identical.

**Root cause:** Automation gap. The lemma's `ensures` clause uses `is_bit_set(i)` which expands to `set_bits.contains(i)`, which is exactly the definition of `is_full()`. However, Verus' quantifier instantiation may not trigger the connection automatically. This is an **automation gap** rather than a true spec incompleteness — the property holds semantically but the proof machinery cannot establish it without an additional explicit assertion bridging quantifier triggers.

#### GAP 2: LOG18 — alloc_range liveness on empty bitmap

**Test:** On an empty 64-bit bitmap, assert that `alloc_range(8)` succeeds.

**Failure point:** `assert(a is Ok)` — the verifier cannot prove the existence of a contiguous free range of size 8.

**Root cause:** The `alloc_range` spec's error condition requires `!exists_contiguous_free_range(size)`. To prove Ok, we must demonstrate `exists_contiguous_free_range(8)`, which requires providing an existential witness (e.g., the range [0, 8) is free). The `lemma_usage_less_than_capacity_means_not_full()` only proves `!is_full()`, which is insufficient — it proves a single free bit exists, not a contiguous range. This is a **spec gap**: there is no lemma bridging "empty bitmap" to "exists contiguous free range of any size ≤ num_bits."

---

## Overall Summary

| Test Category | Total | PASSED | REJECTED | Matches Expected | Gaps Found |
|---|---|---|---|---|---|
| **Behavioral** | 18 | 0 | 18 | 18/18 (100%) | 0 |
| **Boundary** | 20 | 11 | 9 | 20/20 (100%) | 0 |
| **Logical** | 22 | 12 | 10 | 20/22 (91%) | 2 |
| **Total** | **60** | **23** | **37** | **58/60 (97%)** | **2** |

### Consistency Assessment

The bitmap specification is **highly consistent** across behavioral and boundary queries:
- **Behavioral consistency (100%):** All 18 mutated output assertions were correctly rejected. The spec entails no unintended input-output relationships.
- **Boundary consistency (100%):** All 20 boundary tests matched expectations. The spec correctly classifies valid and invalid inputs at all boundaries.
- **Logical consistency (91%):** 20 of 22 logical property tests matched expectations. The spec correctly rejects false stronger-than-intended claims (determinism, adjacency, structural equality). Two gaps were found in liveness/completeness properties.

### Spec Gaps Identified

Both gaps are in the **logical query** category, specifically in **liveness properties**:

1. **full ⟹ ¬empty** bridge (LOG09): Cannot prove `is_full() ==> !is_empty()` due to automation gap in quantifier triggers.
2. **empty ⟹ exists_contiguous_free_range(n)** bridge (LOG18): Missing lemma to establish that an empty bitmap has contiguous free ranges of arbitrary valid size.

These gaps are consistent with findings in the existing `incorrect_tests/results.txt` (e.g., gap04, gap09, gap10, gap16), confirming that the spec's primary weakness is in **liveness lemma coverage**, not in behavioral or boundary correctness.

### Interpretation via Consistency Framework

> *"Specification completeness is not defined by what a specification can prove, but by what it refuses to entail."*

The spec **refuses to entail all 37 incorrect/adversarial queries** tested. The 2 gaps are not inconsistencies (the spec does not entail wrong properties) but rather **incompleteness**: the spec fails to entail 2 correct properties. These represent cases where the semantic space is under-covered, not where it over-admits.
