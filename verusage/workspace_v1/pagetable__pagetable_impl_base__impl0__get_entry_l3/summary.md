# Adversarial Proof Test Summary

**Target**: `get_entry_l3` in `pagetable__pagetable_impl_base__impl0__get_entry_l3.rs`

## Overview

15 adversarial proof tests were generated across 3 categories to probe the semantic boundary of the `get_entry_l3` specification and its supporting spec functions (`spec_resolve_mapping_l3`, `spec_resolve_mapping_l4`, `spec_resolve_mapping_1g_l3`, etc.).

**Result: All 15 tests FAILED verification as expected.** The specification correctly rejects all tested invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Boundary Tests (5/5 FAILED ✓)

| # | Test | What it probes | Result |
|---|------|---------------|--------|
| 1 | `boundary_l4i_out_of_range` | L4 index 512 (OOB) — spec cannot resolve out-of-bounds access | FAILED ✓ |
| 2 | `boundary_l3i_out_of_range` | L3 index 512 (OOB) — spec cannot resolve out-of-bounds access | FAILED ✓ |
| 3 | `boundary_no_wf_l3_points_to_l2` | Missing `wf()` — structural invariants unavailable | FAILED ✓ |
| 4 | `boundary_kernel_l4_has_l3` | Kernel L4 index (0) — not guaranteed to have L3 backing | FAILED ✓ |
| 5 | `boundary_l4_none_implies_l3_some` | L4=None yet L3=Some — contradicts cascading resolution | FAILED ✓ |

## Behavioral Mutation Tests (5/5 FAILED ✓)

| # | Test | What it probes | Result |
|---|------|---------------|--------|
| 1 | `mutation_ps_entry_gives_l3` | Hugepage (ps=true) entry yielding regular L3 result | FAILED ✓ |
| 2 | `mutation_wrong_l3_addr` | L3 resolved entry forced to addr=0 | FAILED ✓ |
| 3 | `mutation_1g_and_l3_coexist` | Both 1G and regular L3 mappings simultaneously Some | FAILED ✓ |
| 4 | `mutation_4k_exists_when_l3_none` | 4K mapping exists when L3 returns None | FAILED ✓ |
| 5 | `mutation_not_present_gives_some` | Resolved L3 entry has present=false | FAILED ✓ |

## Logical Tests (5/5 FAILED ✓)

| # | Test | What it probes | Result |
|---|------|---------------|--------|
| 1 | `logical_l3_always_resolves` | L3 resolution always succeeds (too strong) | FAILED ✓ |
| 2 | `logical_same_result_different_l3i` | Different L3 indices give identical results (determinism) | FAILED ✓ |
| 3 | `logical_cr3_is_zero` | CR3 constrained to specific value 0 | FAILED ✓ |
| 4 | `logical_kernel_l4_end_is_one` | kernel_l4_end constrained to specific value 1 | FAILED ✓ |
| 5 | `logical_l3_equals_1g_l3` | Regular L3 and 1G L3 resolutions are equal | FAILED ✓ |

---

## Conclusion

The specification for `get_entry_l3` and its supporting definitions correctly:
- **Rejects out-of-bounds inputs** (L4/L3 indices ≥ 512)
- **Guards structural invariants** behind `wf()` preconditions
- **Distinguishes kernel vs user L4 entries** (kernel entries lack L3 guarantees)
- **Enforces cascading None propagation** (L4→L3→L2→L1)
- **Maintains mutual exclusivity** of hugepage (1G) vs regular L3 mappings
- **Does not over-constrain** parametric values (cr3, kernel_l4_end)
- **Does not assume uniformity** across different L3 indices

No specification weaknesses were detected by these adversarial tests.
