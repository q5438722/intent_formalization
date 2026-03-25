# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/nrkernel/verified/spec_t_mmu__pt_mem/spec_t__mmu__pt_mem__impl0__lemma_write_seq.rs`
**Date:** 2026-03-24T13:27:31Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. `PTMem` is a low-level spec abstraction modeling page table memory as a map with a fixed root pointer. All flagged properties — PML4 preservation, silent overwrites, empty identity, domain expansion, and order dependence — are correct by design. Invariants on valid addresses, concurrency, and structural constraints are enforced at higher abstraction layers in the refinement stack.

## All Candidates

### φ1: write_preserves_pml4
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** `write` never modifies `pml4` — even writing to the address that stores the PML4 pointer leaves the `pml4` field unchanged, allowing the PML4 base to diverge from physical memory
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is correct by design. `PTMem` is a spec-level abstraction where `pml4` is a fixed base address field, not a memory location. Writes modify the page table memory contents (`mem`), not the root pointer. The PML4 base address is set at initialization and should not change.

### φ2: write_overwrites_silently
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Sequential writes to the same address silently overwrite — no write-ordering or consistency check, so a concurrent writer could clobber page table entries without detection
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is standard map semantics — `Map::insert` overwrites existing keys. Concurrency concerns are handled at higher abstraction layers (rl1/rl2/rl3 with store buffers and TLBs), not at the PTMem level which models a single logical memory state.

### φ3: write_seq_empty_identity
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** `write_seq` with an empty sequence is identity — tests the base case of `fold_left`
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** This is the correct base case for `fold_left` over an empty sequence. An empty write sequence should produce no changes — this is expected and desirable behavior.

### φ4: write_expands_domain
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** Writing to an unmapped address creates a new entry — there's no guard preventing writes to addresses outside the allocated page table region, allowing arbitrary memory expansion
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `PTMem.mem` is a `Map<usize, usize>` — writing to a new key naturally adds it to the domain. Bounds checking on which addresses are valid page table locations is enforced at higher layers (directory invariants, allocated regions), not at this low-level memory abstraction.

### φ5: write_seq_order_matters
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** `write_seq` is order-dependent via `fold_left` — the last write to a given address wins, meaning reordering the write sequence changes the final state
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Order-dependent writes are correct and expected — `write_seq` models sequential application of writes via `fold_left`. In page table operations, write order matters (e.g., directory entries must be written before leaf entries). This is the intended semantics.

