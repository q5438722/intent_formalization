# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/atmosphere/verified/memory_manager/memory_manager__spec_impl__impl0__create_pagetable_l4_entry.rs`
**Date:** 2026-03-24T07:09:30Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 3
- True positives: 0
- False positives: 0

## All Candidates

### φ1: usize2pa_mem_valid_unconditional
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** usize2pa's external_body unconditionally ensures MEM_valid(ret) for ANY input — arbitrary usize values produce "valid" physical addresses without proof

### φ2: zero_entry_is_empty
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** Relies on two independent external_body claims (usize2page_entry and usize2pa) both correctly handling zero — if either is wrong the combined property is unsound

### φ3: pci_bitmap_wf_domain_exact_size
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** PCIBitMap::wf uses biconditional so domain membership implies bounds — but u8 already constrains bus < 256, dev < 32, fun < 8 are NOT implied by u8 type alone, so wf silently accepts out-of-spec device/function numbers if ghost_map is constructed carelessly

### φ4: root_table_resolve_no_ensures
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** RootTable::resolve is a closed external_body spec with NO ensures clauses — it returns Option<(IOid, usize)> but nothing constrains the IOid to be valid or the usize to be meaningful

### φ5: page_closure_includes_l4_unconditionally
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** page_closure absorbs all l4_table domain pointers unconditionally — if l4_table domain overlaps with l3/l2/l1 domains, page_closure silently merges them with no disjointness check

