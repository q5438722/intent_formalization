# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__defs/spec_t__mmu__defs__MAX_PHYADDR.rs`
**Date:** 2026-03-24T13:23:36Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 2
- True positives: 1
- False positives: 1

## Summary

One true positive: the `axiom_max_phyaddr_width_facts` external_body axiom is an unverified trust assumption that provides weaker bounds than the actual constant value. One false positive: the odd-value candidate is a tautology that doesn't actually probe the spec's behavior.

## True Positives (Spec Issues)

### axiom_max_phyaddr_width_external_body
- **Confidence:** medium
- **Reasoning:** `axiom_max_phyaddr_width_facts` is `external_body` with `unimplemented!()` body — this is a trusted axiom. While the actual constant is 52 (set in the `external_body` const), the axiom only constrains it to `[32, 52]`, making it weaker than the actual value. The axiom itself is an unverified trust assumption.

## All Candidates

### φ1: max_phyaddr_width_not_52
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `MAX_PHYADDR_WIDTH` is `external_body` constant set to 52 but the axiom only constrains it to `[32, 52]` — the SMT solver could assume it's not 52

### φ2: max_phyaddr_width_equals_32
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** The axiom allows `MAX_PHYADDR_WIDTH == 32`, which would make `MAX_PHYADDR` only 4GB — dramatically smaller than the intended 52-bit physical address space

### φ3: max_phyaddr_spec_smaller_than_expected
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If `MAX_PHYADDR_WIDTH < 48`, `MAX_PHYADDR_SPEC` would be smaller than the typical 48-bit virtual address space limit — the weak axiom allows this

### φ4: axiom_max_phyaddr_width_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `axiom_max_phyaddr_width_facts` is `external_body` — trusted without proof, the axiom could be unsound if `MAX_PHYADDR_WIDTH` is not actually constrained to `[32, 52]` at runtime
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `axiom_max_phyaddr_width_facts` is `external_body` with `unimplemented!()` body — this is a trusted axiom. While the actual constant is 52 (set in the `external_body` const), the axiom only constrains it to `[32, 52]`, making it weaker than the actual value. The axiom itself is an unverified trust assumption.

### φ5: max_phyaddr_width_odd_value
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** The axiom allows non-standard physical address widths (e.g., 33, 37, 41) that no real x86 CPU supports — only 36, 39, 46, 48, and 52 are used in practice
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This φ doesn't actually test anything about the spec — `w == w` is a tautology that holds for any `w`. The preconditions constrain a free parameter unrelated to `MAX_PHYADDR_WIDTH`. Even as a conceptual concern, the spec intentionally abstracts over the exact width to support different CPU generations, and constraining to exact hardware values would be over-specification.

