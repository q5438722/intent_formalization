# Bitmap Spec Completeness Report v2

**Date:** 2026-03-30
**Target:** `nanvix/src/libs/bitmap/` (8 exec functions)
**Pipeline:** Step2 brainstorm → Step3 formalize → Step4 entailment → Step5 critic
**Method:** Assume-in-body entailment with state refinement

---

## Pipeline Summary

| Stage | Count |
|---|---|
| Brainstorm properties | 15 (with state scenarios) |
| Formalized φ | 55 (incl. state variants) |
| Entailment: verified (incomplete) | 18 |
| Entailment: failed (complete) | 37 |
| Critic: True Positive | 14 |
| Critic: False Positive | 4 |
| **Distinct spec gaps** | **3** |

---

## Spec Gaps Found

### Gap 1: `new` — Missing Liveness Guarantee
**Severity:** High
**φ:** phi_1_new_err_valid_input (+ 3 state variants: minimal, typical, near_max), phi_4_new_liveness (+ 2 variants)

The spec defines when `new` must return `Err`:
```
number_of_bits == 0 ==> result is Err
number_of_bits >= u32::MAX ==> result is Err
number_of_bits % 8 != 0 ==> result is Err
```
But it never guarantees `Ok` for valid inputs. The ensures clause only says *if* `Ok` then certain properties hold — it never says valid input *must produce* `Ok`.

**Impact:** Callers cannot rely on `Bitmap::new(64)` succeeding, even though the body always succeeds for valid inputs.

**Suggested fix:** Add `number_of_bits > 0 && number_of_bits < u32::MAX && number_of_bits % 8 == 0 ==> result is Ok`.

**Nuance:** Very large allocations could plausibly fail due to memory exhaustion. The spec might want a conditional liveness guarantee or document that Err is possible for resource reasons.

---

### Gap 2: `set` — Missing Liveness Guarantee
**Severity:** High
**φ:** phi_5_set_err_valid_clear (+ 2 state variants: index0, last_index)

The spec's Err branch is `Err(_) => true` — it places no constraints on when Err can occur. Combined with the Ok branch requiring `!pre@.is_bit_set(index)`, this means:
- If `index < num_bits` and the bit is clear, the body will succeed
- But the spec allows returning Err anyway

**Impact:** Callers cannot rely on `set(valid_index)` succeeding even when the bit is available.

**Suggested fix:** Tighten the Err branch: `Err(_) ==> index >= pre@.num_bits || pre@.is_bit_set(index as int)`.

---

### Gap 3: `alloc` — Missing Frame Condition
**Severity:** Critical
**φ:** phi_4_alloc_flips_other_bits (+ 3 state variants: empty, in_use, almost_full)

The spec's Ok branch only constrains the allocated bit:
```
Ok(index) => {
    !pre@.is_bit_set(index)      // was clear
    post@.is_bit_set(index)      // now set
    post@.num_bits == pre@.num_bits
}
```
It says **nothing about other bits remaining unchanged**. The spec permits `alloc` to flip, clear, or set arbitrary other bits as a side effect.

**Impact:** After `alloc()`, previously allocated resources could be silently invalidated. This is the most dangerous gap — callers cannot trust that their existing allocations survive a new allocation.

**Suggested fix:** Add frame condition:
```
forall|i: int| 0 <= i < post@.num_bits && i != index as int
    ==> post@.is_bit_set(i) == pre@.is_bit_set(i)
```

---

## False Positives (4)

| φ | Reason | Critic Filter |
|---|---|---|
| phi_3_alloc_always_zero | Spec intentionally abstracts over allocation order — returning any free bit is valid | generality |
| phi_3_alloc_always_zero_empty | Same — returning index 0 from empty bitmap is a valid choice | generality |
| phi_3_alloc_always_zero_others_free | Same — deliberate non-determinism over allocation policy | generality |
| phi_5_set_err_valid_clear_already_set | Control case — when bit is already set, Ok is logically impossible, Err is correct | incompleteness |

---

## State Refinement Analysis

The state refinement feature (testing φ under specific object states) provided additional insight but did not discover gaps that the universal version missed in this case:

- All 3 gaps were detected by both universal and state-specific variants
- State variants helped **confirm** the gap scope (e.g., alloc frame condition is dangerous across all states: empty, in-use, almost-full)
- The `alloc_always_zero_bit0_set` variant was the only state-specific φ that verified while its universal version failed — but critic correctly marked it as FP (intentional non-determinism)

---

## Comparison with v1 (Manual)

| | v1 (manual, 6 φ) | v2 (pipeline, 55 φ) |
|---|---|---|
| Gaps found | 2 (new liveness, alloc fairness) | 3 (new liveness, set liveness, **alloc frame condition**) |
| Precision (after critic) | 33% (2/6) | 25% (14/55) or 78% (14/18 post-entailment) |
| New finding | — | **alloc frame condition** (critical severity) |
| False positive rate | 67% | 22% (post-entailment) |

The pipeline discovered the **alloc frame condition gap** that manual testing missed — a critical spec defect where `alloc` is allowed to corrupt other bits.

---

## Files

- Brainstorm: `nanvix/workspace/bitmap/brainstorm_v2.json`
- Candidates: `nanvix/workspace/bitmap/candidates_v2.json`
- Entailment results: `nanvix/workspace/bitmap/entailment_results.json`
- Critic raw: `nanvix/workspace/bitmap/critic_v2_raw.txt`
- Generated tests: `nanvix/workspace/bitmap/lib.test.generated.rs`
- Entailment tests (deployed): `nanvix/src/libs/bitmap/src/lib.test.rs`
