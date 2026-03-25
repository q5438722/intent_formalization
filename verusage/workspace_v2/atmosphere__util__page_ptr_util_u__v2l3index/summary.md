# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/util/util__page_ptr_util_u__v2l3index.rs`
**Date:** 2026-03-24T08:23:19Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 4
- False positives: 0

## Summary

All four candidates are true positives stemming from the same root cause: `va_1g_valid` has `when_used_as_spec(spec_va_2m_valid)` instead of `when_used_as_spec(spec_va_1g_valid)`, a copy-paste bug from the adjacent `va_2m_valid` definition. This causes all spec-level 1GB validation to silently degrade to 2MB validation, with cascading effects on downstream functions like `v2l3index` whose preconditions become weaker than intended.

## True Positives (Spec Issues)

### va_1g_valid_equals_spec_va_2m_valid
- **Confidence:** high
- **Reasoning:** `va_1g_valid` has `#[verifier(when_used_as_spec(spec_va_2m_valid))]` — a clear copy-paste bug from `va_2m_valid`. In spec context, `va_1g_valid(va)` resolves to `spec_va_2m_valid(va)` instead of `spec_va_1g_valid(va)`, making all 1GB validation checks silently use 2MB validation.

### va_1g_accepts_non_1g_aligned
- **Confidence:** high
- **Reasoning:** Direct consequence of the copy-paste bug: a 2MB-aligned but non-1GB-aligned address passes `va_1g_valid` in spec context, allowing misaligned addresses into 1GB page operations.

### v2l3index_weakened_precondition
- **Confidence:** high
- **Reasoning:** Downstream impact of the same bug: `v2l3index`'s precondition disjunction `va_1g_valid(va)` resolves to `spec_va_2m_valid(va)`, so any 2MB-valid address satisfies the precondition through the 1GB path regardless of actual 1GB alignment.

### v2l3index_1g_path_nonzero_result
- **Confidence:** high
- **Reasoning:** A 2MB-aligned address with nonzero L3 index is not 1GB-aligned, yet passes `va_1g_valid` and enters `v2l3index`. Callers relying on the 1GB path to guarantee specific L3 index properties (e.g., that it indexes into a valid 1GB page table entry) would get incorrect results.

## All Candidates

### φ1: va_1g_valid_equals_spec_va_2m_valid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `va_1g_valid` has `when_used_as_spec(spec_va_2m_valid)` instead of `spec_va_1g_valid` — copy-paste bug makes 1GB validation resolve to 2MB validation in spec context
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `va_1g_valid` has `#[verifier(when_used_as_spec(spec_va_2m_valid))]` — a clear copy-paste bug from `va_2m_valid`. In spec context, `va_1g_valid(va)` resolves to `spec_va_2m_valid(va)` instead of `spec_va_1g_valid(va)`, making all 1GB validation checks silently use 2MB validation.

### φ2: va_1g_accepts_non_1g_aligned
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A 2MB-aligned but non-1GB-aligned address passes `va_1g_valid` due to the wrong `when_used_as_spec` annotation, allowing misaligned addresses into 1GB page operations
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Direct consequence of the copy-paste bug: a 2MB-aligned but non-1GB-aligned address passes `va_1g_valid` in spec context, allowing misaligned addresses into 1GB page operations.

### φ3: v2l3index_weakened_precondition
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `v2l3index`'s precondition is weaker than intended because `va_1g_valid` resolves to `spec_va_2m_valid`, so any 2MB-valid address satisfies the disjunction through the 1GB path even without actual 1GB alignment
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Downstream impact of the same bug: `v2l3index`'s precondition disjunction `va_1g_valid(va)` resolves to `spec_va_2m_valid(va)`, so any 2MB-valid address satisfies the precondition through the 1GB path regardless of actual 1GB alignment.

### φ4: spec_va_1g_implies_2m
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** 1GB alignment implies 2MB alignment (stricter mask subsumes weaker mask), so the `va_1g_valid` bug makes the 1GB and 2MB paths fully equivalent — hiding that 1GB checks are never actually performed

### φ5: v2l3index_1g_path_nonzero_result
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** A 2MB-aligned address with nonzero L3 index (i.e. not 1GB-aligned) passes `va_1g_valid` and enters `v2l3index` — callers expecting `va_1g_valid` to guarantee L3 index is meaningful for 1GB pages get incorrect results
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** A 2MB-aligned address with nonzero L3 index is not 1GB-aligned, yet passes `va_1g_valid` and enters `v2l3index`. Callers relying on the 1GB path to guarantee specific L3 index properties (e.g., that it indexes into a valid 1GB page table entry) would get incorrect results.

