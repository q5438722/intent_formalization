# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_os_invariant/spec_t__os_invariant__lemma_candidate_mapping_inflight_vmem_overlap_hl_implies_os.rs`
**Date:** 2026-03-24T14:42:53Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 4
- True positives: 0
- False positives: 4

## Summary

All four candidates are false positives. Idle threads not triggering inflight overlap follows from the match arm structure. Entry size chain is correct by the concrete constant definitions. The rl2→rl1 interp preserving writes is explicit in the interp function. And nat_keys membership preservation follows from the Map::new domain predicate and usize-to-nat round-tripping.

## All Candidates

### φ1: candidate_overlaps_inflight_vmem_idle_thread
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** An Idle thread should never cause inflight vmem overlap — tests that the Idle match arm in the existential correctly returns false; if Idle threads trigger overlap, no new mappings could ever be created
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `Idle` variant in the match arm of `candidate_mapping_overlaps_inflight_vmem` returns false (it doesn't match `Map` or `Unmap`). With only an `Idle` thread in the set, no witness satisfies the existential. Correct by construction.

### φ2: entry_size_layer_product_chain
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The entry_size_is_next_layer_size invariant chains L0 = 512*L1 = 512^2*L2 = 512^3*L3 — tests that the x86_arch_spec layers are correctly constructed; wrong sizes would break virtual address space partitioning
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `x86_arch_spec` is a concrete constant with layers defined as `[L0_ENTRY_SIZE, L1_ENTRY_SIZE, L2_ENTRY_SIZE, L3_ENTRY_SIZE]` with `num_entries: 512` at each level. The chain `L0 = 512*L1 = 512^2*L2 = 512^3*L3` follows directly from the constant definitions `L0 = 512*L1`, `L1 = 512*L2`, `L2 = 512*L3`.

### φ3: upper_vaddr_equals_2_pow_48
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** `upper_vaddr(0, 0)` should equal 2^48 = 256TB — the canonical x86-64 virtual address space; if wrong, `candidate_mapping_in_bounds` would accept or reject mappings at incorrect boundaries

### φ4: rl2_interp_preserves_writes_field
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The rl2→rl1 refinement should faithfully pass through the `writes` field — if the interp function altered writes, the writer core identity or TSO/nonpos tracking would diverge between abstraction levels
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** The `rl2::State::interp()` function explicitly sets `writes: self.writes` in the constructed `rl1::State`. The ensures is a direct restatement of the interp definition.

### φ5: nat_keys_preserves_membership
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `nat_keys` converts usize keys to nat — an existing usize key should always appear in the nat-keyed map; if the `k <= usize::MAX` guard in `Map::new` is wrong, valid keys could be dropped
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `nat_keys` defines its domain as `|k: nat| k <= usize::MAX && m.contains_key(k as usize)`. For any `k: usize`, `k as nat <= usize::MAX` holds, and `m.contains_key(k)` implies `m.contains_key((k as nat) as usize)`. Correct by Map::new axioms.

