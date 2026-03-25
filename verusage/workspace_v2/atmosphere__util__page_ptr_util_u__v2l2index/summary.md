# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/util/util__page_ptr_util_u__v2l2index.rs`
**Date:** 2026-03-24T08:22:10Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 3
- False positives: 1

## Summary

Three of the four candidates are true positives, all stemming from the same root cause: a copy-paste bug where `va_1g_valid` has `when_used_as_spec(spec_va_2m_valid)` instead of `when_used_as_spec(spec_va_1g_valid)`. This causes the 1GB validity check to silently degrade to a 2MB validity check in spec context, allowing misaligned addresses into 1GB page operations and weakening downstream preconditions like `v2l2index`. The kernel memory boundary constant is an intentional design choice and not a spec issue.

## True Positives (Spec Issues)

### va_1g_valid_returns_spec_va_2m_valid
- **Confidence:** high
- **Reasoning:** The `va_1g_valid` function has `#[verifier(when_used_as_spec(spec_va_2m_valid))]` — clearly a copy-paste bug from `va_2m_valid`. This means whenever `va_1g_valid` is used in spec context it resolves to `spec_va_2m_valid` instead of `spec_va_1g_valid`, silently accepting 2MB-aligned addresses as 1GB-valid.

### va_1g_valid_not_spec_va_1g_valid
- **Confidence:** high
- **Reasoning:** This is a direct consequence of the copy-paste bug: a 2MB-aligned but non-1GB-aligned address passes `va_1g_valid` because the spec resolves to `spec_va_2m_valid`. This would allow misaligned addresses into 1GB page table operations, a real soundness gap.

### v2l2index_accepts_non_1g_via_va_1g
- **Confidence:** high
- **Reasoning:** This demonstrates the downstream impact of the copy-paste bug: `v2l2index`'s precondition `va_1g_valid(va)` is weaker than intended, accepting any 2MB-valid address through the 1GB disjunct. While `v2l2index` itself computes correctly for any address, callers relying on the 1GB path to guarantee 1GB alignment would be unsound.

## All Candidates

### φ1: va_1g_valid_returns_spec_va_2m_valid
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `va_1g_valid` has `when_used_as_spec(spec_va_2m_valid)` instead of `spec_va_1g_valid` — a copy-paste bug that silently makes `va_1g_valid` behave as 2MB validation, weakening all 1GB page checks
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** The `va_1g_valid` function has `#[verifier(when_used_as_spec(spec_va_2m_valid))]` — clearly a copy-paste bug from `va_2m_valid`. This means whenever `va_1g_valid` is used in spec context it resolves to `spec_va_2m_valid` instead of `spec_va_1g_valid`, silently accepting 2MB-aligned addresses as 1GB-valid.

### φ2: va_1g_valid_not_spec_va_1g_valid
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** A 2MB-aligned address that is NOT 1GB-aligned would still pass `va_1g_valid` due to the wrong `when_used_as_spec` annotation, allowing misaligned addresses into 1GB page operations
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** This is a direct consequence of the copy-paste bug: a 2MB-aligned but non-1GB-aligned address passes `va_1g_valid` because the spec resolves to `spec_va_2m_valid`. This would allow misaligned addresses into 1GB page table operations, a real soundness gap.

### φ3: v2l2index_accepts_non_1g_via_va_1g
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `v2l2index` requires `va_1g_valid(va)` as one disjunct, but since `va_1g_valid` resolves to `spec_va_2m_valid`, any 2MB-valid address satisfies the precondition through the 1GB path even when it's not actually 1GB-aligned
- **Verdict:** TRUE_POSITIVE (high)
- **Reasoning:** This demonstrates the downstream impact of the copy-paste bug: `v2l2index`'s precondition `va_1g_valid(va)` is weaker than intended, accepting any 2MB-valid address through the 1GB disjunct. While `v2l2index` itself computes correctly for any address, callers relying on the 1GB path to guarantee 1GB alignment would be unsound.

### φ4: spec_va_4k_valid_implies_2m_implies_1g_chain
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Since `MEM_1g_MASK` zeroes more low bits than `MEM_2m_MASK`, 1GB-validity should imply 2MB-validity — but this containment relationship is never formally established and relies on bitwise reasoning about the mask constants

### φ5: kernel_mem_end_l4index_allows_low_addresses
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `KERNEL_MEM_END_L4INDEX = 1` means L4 index 1 and above are valid — with index 0 excluded, the lowest ~512GB of virtual address space is blocked, but index 1 starts at a very low address (512GB) which may still overlap with user-space mappings on some configurations
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `KERNEL_MEM_END_L4INDEX = 1` is an intentional design constant that excludes L4 index 0 (the lowest 512GB) from valid kernel addresses. The property simply restates this design decision. Whether index 1 overlaps with user-space is an OS-level policy choice, not a spec bug.

