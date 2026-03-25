# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__kernel__create_and_map_pages__impl0__check_address_space_va_range_free/original.rs`
**Date:** 2026-03-24T21:29:43Z

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 5
- False positives: 0

## Summary

All five properties verify, including the mutually contradictory pair φ1/φ2, which conclusively demonstrates that the preconditions involving `os.wf()`, `os.proc_dom().contains(p)`, and `va_range.wf()` are jointly unsatisfiable for any non-trivial (len > 0) range—and possibly for all inputs. The root cause is almost certainly one or more of the `#[verifier::external_body] closed spec fn wf()` stubs (visible on `OS`, `PageTable`, `PageAllocator`, `StaticLinkedList`) being unimplemented and thus defaulting to an unsatisfiable or overly restrictive predicate. This renders the entire spec for `check_address_space_va_range_free` vacuously correct: it "satisfies" every possible postcondition because no valid input exists. All five properties are true positives—they collectively expose that the specification framework cannot express or verify any meaningful property about address-space free-range checking until the well-formedness predicates are properly defined.

## True Positives

### always_free
- **Confidence:** high
- **Reasoning:** This directly contradicts φ2 (`never_free_nonempty`). Both verifying simultaneously means the preconditions are vacuously unsatisfiable—likely `OS::wf()` or `VaRange4K::wf()` admits no valid instances (or only the degenerate `len==0` case). Either way, the spec for `check_address_space_va_range_free` is untestable and provides no real guarantee, which is a genuine spec weakness.

### never_free_nonempty
- **Confidence:** high
- **Reasoning:** The only way this and φ1 both verify is if `va_range.wf() ∧ va_range.len > 0` is unreachable under the current `wf()` definitions, making this vacuously true. This reveals that the spec's well-formedness predicates are too strong, preventing any non-empty range from being expressible—a real spec issue that would silently block all mapping operations from being verified.

### proc_independent_result
- **Confidence:** medium
- **Reasoning:** If the preconditions are vacuous (as implied by φ1∧φ2), this verifies trivially. Even if only the `len==0` case is reachable (empty ranges are trivially free for all processes), the property still exposes that the spec cannot distinguish per-process address spaces for any meaningful range. A correctly specified system must allow process-dependent free-range checks for non-trivial ranges.

### free_but_first_va_mapped
- **Confidence:** high
- **Reasoning:** This claims a "free" range simultaneously has its first VA mapped—a direct semantic contradiction. It verifies because the precondition `va_range.len > 0 ∧ address_space_range_free(...)` is unsatisfiable under the current spec (as established by φ1∧φ2). This confirms the vacuous-precondition issue: the spec never reaches a state where a non-empty free range exists, so any conclusion follows.

### single_page_always_free
- **Confidence:** high
- **Reasoning:** If `VaRange4K::wf()` permits `len==1`, this would expose a real off-by-one or vacuous-quantifier bug in the free-range check. If `wf()` forbids `len==1` entirely, then it verifies vacuously, confirming the overly restrictive `wf()` problem. Either way it reveals a genuine spec defect—single-page allocations are the most basic operation and must be checkable.

## All Candidates

### φ1: always_free → `check_address_space_va_range_free`
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** If provable, the spec vacuously treats every VA range as free, making the check useless and allowing double-mapping
- **Verdict:** TRUE_POSITIVE (high)

### φ2: never_free_nonempty → `check_address_space_va_range_free`
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** If provable, the spec says no non-empty range is ever free, blocking all new mappings even in a pristine address space
- **Verdict:** TRUE_POSITIVE (high)

### φ3: proc_independent_result → `check_address_space_va_range_free`
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If provable, different processes always agree on which ranges are free, violating address-space isolation
- **Verdict:** TRUE_POSITIVE (medium)

### φ4: free_but_first_va_mapped → `check_address_space_va_range_free`
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** If provable, a range declared free actually has its first VA mapped, meaning the spec's notion of free contradicts the address-space contents
- **Verdict:** TRUE_POSITIVE (high)

### φ5: single_page_always_free → `check_address_space_va_range_free`
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** If provable, single-page ranges are always free regardless of mappings, suggesting the spec has an off-by-one or vacuous quantifier for len==1
- **Verdict:** TRUE_POSITIVE (high)

