# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__pagetable__pagetable_impl_base__impl0__map_4k_page/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: existing_mapping_lost → `map_4k_page`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If map_4k_page silently removes a pre-existing 4k mapping at a different VA, it corrupts the address space

### φ2: 4k_2m_mapping_conflict → `map_4k_page`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If the same L2 entry simultaneously resolves as both a 2m large page and an L1 table pointer, the page table is incoherent

### φ3: write_permission_flipped → `map_4k_page`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the stored write permission is the opposite of what was requested, memory protection is silently violated

### φ4: spurious_mapping_appears → `map_4k_page`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** If mapping one 4k page causes unrelated VAs to become mapped, arbitrary memory becomes accessible

### φ5: mapped_addr_mismatch → `map_4k_page`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** If the physical address stored in the mapping differs from the requested address, the page table maps to wrong physical memory

