# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/definitions_u/definitions_u__lemma_maxphyaddr_facts.rs`
**Date:** 2026-03-24T12:22:04Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 1
- False positives: 4

## Summary

One true positive: `MAX_PHYADDR_WIDTH` is an `external_body` constant with bounds asserted by an axiom function — the hardware-dependent value `[32, 52]` is trusted without proof. The other four candidates are false positives — they are correct consequences of the verified lemma and standard x86-64 physical address width constraints.

## True Positives (Spec Issues)

### max_phyaddr_width_external_body
- **Confidence:** medium
- **Reasoning:** `MAX_PHYADDR_WIDTH` is `external_body` with its bounds asserted by an `axiom fn`. The value 52 is hardcoded but opaque to the verifier — the range `[32, 52]` is trusted without proof. This is the intentional design for hardware-dependent constants, but it is an unverified trust assumption.

## All Candidates

### φ1: max_phyaddr_width_external_body
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `MAX_PHYADDR_WIDTH` is `external_body` and its bounds are asserted by an axiom function — the actual value is opaque and trusted
- **Verdict:** TRUE_POSITIVE (medium)
- **Reasoning:** `MAX_PHYADDR_WIDTH` is `external_body` with its bounds asserted by an `axiom fn`. The value 52 is hardcoded but opaque to the verifier — the range `[32, 52]` is trusted without proof. This is the intentional design for hardware-dependent constants, but it is an unverified trust assumption.

### φ2: max_phyaddr_lower_bound
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** MAX_PHYADDR is at least 4GB-1 — derived from the axiom that `MAX_PHYADDR_WIDTH >= 32`, meaning the spec assumes at least 32-bit physical addressing
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the verified `lemma_maxphyaddr_facts`. The 32-bit lower bound follows from the axiom `MAX_PHYADDR_WIDTH >= 32`, which correctly models x86-64's minimum physical address width.

### φ3: max_phyaddr_upper_bound
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** MAX_PHYADDR is at most 52-bit — the axiom trusts that hardware never exceeds 52-bit physical addressing, but future x86 extensions could widen this
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Direct consequence of the verified `lemma_maxphyaddr_facts`. The 52-bit upper bound is the x86-64 architectural maximum for physical addresses per Intel/AMD specs.

### φ4: max_phyaddr_fits_usize
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** MAX_PHYADDR fits in a u64/usize — follows from the 52-bit upper bound, but the entire chain depends on the unverified `external_body` constant and axiom
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Trivially follows from the 52-bit upper bound. A 52-bit value always fits in a 64-bit usize. The trust gap is already captured by φ1.

### φ5: max_phyaddr_spec_equals_exec
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The exec and spec constants are linked via `when_used_as_spec` — the spec definition `(1 << WIDTH) - 1` produces a bitmask, but the correspondence depends on the external_body axiom for overflow safety
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `when_used_as_spec` linkage is verified by the exec const's ensures clause and proof block, which calls the axiom and uses bit_vector reasoning to establish overflow safety. This is correct given the axiom.

