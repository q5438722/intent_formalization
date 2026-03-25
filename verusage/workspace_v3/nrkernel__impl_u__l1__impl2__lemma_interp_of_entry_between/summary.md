# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/nrkernel__impl_u__l1__impl2__lemma_interp_of_entry_between/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: aligned_zero_trivially_true → `aligned`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If aligned(x, 0) is provable, any address is "aligned" to zero, enabling vacuous reasoning through the alignment preconditions

### φ2: arch_upper_vaddr_equals_base → `upper_vaddr`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If upper_vaddr equals base for a valid arch, the mapped address region has zero size, making the entire layer useless

### φ3: entry_base_collision_different_idx → `entry_base`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If two distinct indices map to the same entry base, page table entries would alias and translations would be ambiguous

### φ4: well_formed_zero_entries → `well_formed`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If a well-formed directory can have zero entries, the invariant fails to enforce that page tables have meaningful structure

### φ5: directory_upper_leq_base → `upper_vaddr`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If a valid directory's upper_vaddr is at or below its base, the directory covers no address space or wraps around, breaking address containment

