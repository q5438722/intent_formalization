# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_interp_aux_between/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: aligned_zero_size → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If aligned(addr, 0) is provable, the spec is unsound on division-by-zero (nat % 0 is unspecified in SMT)

### φ2: upper_vaddr_equals_base → `upper_vaddr (Arch)`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** upper_vaddr equalling base would mean a layer maps zero address space, contradicting inv's positive entry_size and num_entries

### φ3: entry_next_entry_equal → `entry_base / next_entry_base (Arch)`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** entry_base equalling next_entry_base implies zero-sized entries, violating the positive entry_size invariant

### φ4: well_formed_zero_entries → `well_formed (Directory)`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** A well_formed directory with zero entries would be degenerate; arch.inv() requires num_entries > 0 so entries.len() should be > 0

### φ5: dir_upper_leq_base → `inv / upper_vaddr (Directory)`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A valid directory's upper_vaddr must exceed base_vaddr to map a non-empty region; if upper ≤ base the address range is empty or inverted

