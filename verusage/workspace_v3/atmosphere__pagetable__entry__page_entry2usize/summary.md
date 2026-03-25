# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/atmosphere__pagetable__entry__page_entry2usize/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: mem_valid_vacuous → `page_entry2usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If MEM_valid is unsatisfiable, all page_entry2usize postconditions hold vacuously and the spec is meaningless

### φ2: present_result_even → `page_entry2usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A present page entry must have bit 0 set (odd result); proving the result even would indicate the present bit is not encoded or the precondition is vacuous

### φ3: ps_not_injective → `page_entry2usize`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Entries differing only in the PS flag must encode to different usize values; equal results would mean the PS bit position is lost

### φ4: all_perms_true_vacuous → `page_entry2usize`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** If there is no valid non-zero address that can carry all permission bits, the spec is vacuously satisfied for the most critical case of a fully-permissioned entry

### φ5: addr_zeroed_when_present → `page_entry2usize`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** ORing the present bit into the address must not corrupt the physical address field; a zeroed PA would mean the address-permission encoding is destructive

