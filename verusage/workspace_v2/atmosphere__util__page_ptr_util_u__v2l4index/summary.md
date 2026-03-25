# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/util/util__page_ptr_util_u__v2l4index.rs`
**Date:** 2026-03-24T08:24:16Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 3
- False positives: 2

## Summary

Three true positives all stem from the same root cause: `va_1g_valid` has `when_used_as_spec(spec_va_2m_valid)` instead of `when_used_as_spec(spec_va_1g_valid)`, a copy-paste bug from the adjacent `va_2m_valid` definition. This silently degrades 1GB validation to 2MB validation in spec context, weakens downstream preconditions like `v2l4index`, and admits misaligned addresses. Two false positives describe expected properties: the L4 index lower bound of 1 is intentional kernel-space design, and the 1GB-implies-2MB alignment relationship is a correct mathematical fact.

## True Positives (Spec Issues)

### va_1g_valid_resolves_to_spec_va_2m_valid
- **Confidence:** high
- **Reasoning:** `va_1g_valid` has `#[verifier(when_used_as_spec(spec_va_2m_valid))]` — clearly a copy-paste bug from `va_2m_valid`. In spec context, `va_1g_valid(va)` resolves to `spec_va_2m_valid(va)` instead of `spec_va_1g_valid(va)`, silently degrading all 1GB validation to 2MB validation.

### va_1g_accepts_2m_aligned_not_1g
- **Confidence:** high
- **Reasoning:** Direct consequence of the copy-paste bug: a 2MB-aligned but non-1GB-aligned address passes `va_1g_valid` in spec context, allowing misaligned addresses into 1GB page operations.

### v2l4index_precond_weakened_by_1g_bug
- **Confidence:** high
- **Reasoning:** Downstream impact of the same bug: `v2l4index`'s precondition disjunct `va_1g_valid(va)` resolves to `spec_va_2m_valid(va)`, making the precondition strictly weaker than intended and admitting addresses that aren't actually 1GB-aligned through the 1GB path.

## All Candidates

### φ1: va_1g_valid_resolves_to_spec_va_2m_valid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `va_1g_valid` has `when_used_as_spec(spec_va_2m_valid)` instead of `spec_va_1g_valid` — a copy-paste bug that makes 1GB validation silently use the 2MB check in spec context
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** `va_1g_valid` has `#[verifier(when_used_as_spec(spec_va_2m_valid))]` — clearly a copy-paste bug from `va_2m_valid`. In spec context, `va_1g_valid(va)` resolves to `spec_va_2m_valid(va)` instead of `spec_va_1g_valid(va)`, silently degrading all 1GB validation to 2MB validation.

### φ2: va_1g_accepts_2m_aligned_not_1g
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Due to the wrong `when_used_as_spec`, a 2MB-aligned but non-1GB-aligned address passes `va_1g_valid`, allowing misaligned addresses into 1GB page table operations
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Direct consequence of the copy-paste bug: a 2MB-aligned but non-1GB-aligned address passes `va_1g_valid` in spec context, allowing misaligned addresses into 1GB page operations.

### φ3: v2l4index_precond_weakened_by_1g_bug
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `v2l4index`'s precondition is weaker than intended — any 2MB-valid address satisfies the disjunction via the broken `va_1g_valid` path even when it's not truly 1GB-aligned
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** Downstream impact of the same bug: `v2l4index`'s precondition disjunct `va_1g_valid(va)` resolves to `spec_va_2m_valid(va)`, making the precondition strictly weaker than intended and admitting addresses that aren't actually 1GB-aligned through the 1GB path.

### φ4: v2l4index_lower_bound_is_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `KERNEL_MEM_END_L4INDEX = 1` means L4 index 0 is excluded — the entire first 512GB of virtual address space is blocked, but index 1 (starting at 512GB) may still be within user-space range on some OS configurations
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the intended design: `KERNEL_MEM_END_L4INDEX = 1` deliberately excludes L4 index 0 to restrict valid virtual addresses to kernel-space. The lower bound of 1 is a correct and desirable invariant, not a spec gap.

### φ5: spec_va_1g_valid_implies_2m_valid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** 1GB alignment implies 2MB alignment since the 1GB mask is stricter — this means the `va_1g_valid` copy-paste bug makes the 1GB and 2MB spec paths fully equivalent, completely hiding the missing 1GB check
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is a correct mathematical property: 1GB alignment (zeroing more low bits) strictly implies 2MB alignment. While it explains why the copy-paste bug is hard to detect, the implication itself is a desirable and true property of the alignment hierarchy, not a spec defect.

