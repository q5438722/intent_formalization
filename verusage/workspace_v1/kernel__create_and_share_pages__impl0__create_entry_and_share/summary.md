# Adversarial Proof Test Summary

## Target: `create_entry_and_share`

This function shares a memory page mapping from a source process's virtual address to a target process's virtual address. It first creates a page table entry for the target, then shares the mapping.

---

## Results Overview

| Test Category          | Total | Failed (Expected) | Passed (Unexpected) |
|------------------------|-------|--------------------|---------------------|
| Boundary Tests         |     7 |                  7 |                   0 |
| Behavioral Mutations   |     7 |                  7 |                   0 |
| Logical Tests          |     7 |                  7 |                   0 |
| **Total**              |**21** |             **21** |               **0** |

**All 21 adversarial tests failed verification as expected.** The specification correctly rejects all tested invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Boundary Tests (7/7 Failed ✓)

| # | Test | Property Violated | Result |
|---|------|-------------------|--------|
| 1 | `test_boundary_src_proc_not_in_domain` | `src_proc_ptr ∉ proc_dom` | ✓ Failed |
| 2 | `test_boundary_target_proc_not_in_domain` | `target_proc_ptr ∉ proc_dom` | ✓ Failed |
| 3 | `test_boundary_insufficient_quota` | `quota.mem_4k < 3` | ✓ Failed |
| 4 | `test_boundary_insufficient_free_pages` | `free_pages < 3` | ✓ Failed |
| 5 | `test_boundary_target_va_already_mapped` | `target_va ∈ addr_space` | ✓ Failed |
| 6 | `test_boundary_src_va_not_mapped` | `src_va ∉ addr_space` | ✓ Failed |
| 7 | `test_boundary_ref_counter_overflow` | `ref_count > MAX - 1` | ✓ Failed |

**Conclusion:** The preconditions correctly guard all boundary cases.

---

## Behavioral Mutation Tests (7/7 Failed ✓)

| # | Test | Mutation Applied | Result |
|---|------|-----------------|--------|
| 1 | `test_mutation_ret_exceeds_bound` | Claim `ret > 3` | ✓ Failed |
| 2 | `test_mutation_proc_dom_changes` | Claim proc domain changed | ✓ Failed |
| 3 | `test_mutation_free_pages_wrong_count` | Claim wrong free page count | ✓ Failed |
| 4 | `test_mutation_target_va_not_added` | Claim target_va not in domain | ✓ Failed |
| 5 | `test_mutation_ref_counter_not_incremented` | Claim ref counter unchanged | ✓ Failed |
| 6 | `test_mutation_wrong_entry_shared` | Claim wrong entry at target_va | ✓ Failed |
| 7 | `test_mutation_other_proc_space_changed` | Claim other proc's space is empty | ✓ Failed |

**Conclusion:** The postconditions correctly reject all behavioral mutations.

---

## Logical Tests (7/7 Failed ✓)

| # | Test | Unintended Property Tested | Result |
|---|------|---------------------------|--------|
| 1 | `test_logical_ret_always_3` | Determinism: ret always 3 | ✓ Failed |
| 2 | `test_logical_ret_always_0` | Stronger bound: ret always 0 | ✓ Failed |
| 3 | `test_logical_src_target_must_differ` | Structural: src ≠ target forced | ✓ Failed |
| 4 | `test_logical_quota_unchanged` | Quota preservation (contradicts spec) | ✓ Failed |
| 5 | `test_logical_wf_not_preserved` | wf NOT preserved (negated) | ✓ Failed |
| 6 | `test_logical_page_mapping_domain_changes` | page_mapping domain changes | ✓ Failed |
| 7 | `test_logical_address_space_invariant` | Address space unchanged | ✓ Failed |

**Conclusion:** The specification does not entail any of these unintended logical properties.

---

## Key Findings

1. **Spec is consistent** across all three test categories — no unintended entailments detected.
2. **Self-sharing is allowed** — the spec permits `src_proc_ptr == target_proc_ptr` as long as `src_va != target_va` (Test L3 confirms this, as the preconditions `src_va ∈ dom` and `target_va ∉ dom` imply `src_va ≠ target_va`).
3. **Return value is non-deterministic** within `[0, 3]` — the spec correctly does not over-constrain the number of pages consumed.
4. **Frame conditions are well-specified** — unrelated processes, containers, threads, and endpoints are correctly preserved.
