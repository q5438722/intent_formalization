# Adversarial Proof Test Summary

**Target**: `pagetable__pagetable_impl_base__impl0__get_entry_l2.rs`
**Function under test**: `PageTable::get_entry_l2` — resolves an L2 page table entry given valid L4/L3/L2 indices and a matching L3 entry.

## Results: All 12 tests FAILED verification ✓

Every adversarial test was correctly rejected by the specification, indicating the spec is consistent within the tested semantic boundaries.

---

### Boundary Tests (`boundary_tests.rs`) — 4/4 FAILED ✓

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_kernel_space_cascade` | Assert `mapping_4k` exclusion for kernel-space index (l4i=0, below `kernel_l4_end`) | FAIL — spec only constrains user-space indices (l4i ≥ kernel_l4_end) |
| 2 | `test_boundary_l3i_overflow` | Assert `spec_resolve_mapping_l3(256, 512).is_None()` with l3i=512 out of [0,512) | FAIL — out-of-range index yields undefined spec behavior |
| 3 | `test_boundary_no_wf_cascade` | Assert `mapping_4k` cascade without `wf()` precondition | FAIL — without well-formedness, no structural guarantees exist |
| 4 | `test_boundary_kernel_l4_end_value` | Assert `kernel_l4_end == 0` (specific value) | FAIL — spec only says `kernel_l4_end < 512` |

### Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 4/4 FAILED ✓

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_ps_returns_some` | present+ps (huge page) → assert `Some` instead of `None` | FAIL — spec correctly returns `None` for PS entries at L2 |
| 2 | `test_mutation_not_present_returns_some` | not-present → assert `Some` instead of `None` | FAIL — spec correctly returns `None` for absent entries |
| 3 | `test_mutation_mapping_exists_when_none` | L2=None → assert `mapping_4k` DOES contain entry (inverted cascade) | FAIL — spec correctly excludes unmapped VAs from `mapping_4k` |
| 4 | `test_mutation_l2_equals_2m` | L2 resolves → assert 2M also resolves (mutually exclusive) | FAIL — L2 (non-hugepage, !ps) and 2M (hugepage, ps) are exclusive |

### Logical Tests (`logical_tests.rs`) — 4/4 FAILED ✓

| # | Test | Unintended Property | Result |
|---|------|-------------------|--------|
| 1 | `test_logical_determinism` | Two different wf page tables give same L2 resolution | FAIL — different page tables have different contents |
| 2 | `test_logical_totality` | All valid indices resolve to `Some` | FAIL — entries can be empty/not-present |
| 3 | `test_logical_mapping_injective` | Different VAs map to different PAs (injectivity) | FAIL — no `disjoint_l1`; shared physical pages are allowed |
| 4 | `test_logical_l2_implies_all_l1` | L2 success implies all 512 L1 entries present | FAIL — individual L1 entries can be empty independently |

---

## Conclusion

The specification for `get_entry_l2` correctly:
- **Rejects invalid inputs** — boundary violations at kernel-space indices, out-of-range indices, and missing preconditions are all rejected.
- **Rejects mutated behaviors** — incorrect present/ps logic inversions and cascade violations are rejected.
- **Rejects unintended reasoning** — determinism, totality, injectivity, and overly strong L1 completeness are not entailed.

The spec appears **consistent** with respect to the tested semantic queries. No spec weaknesses were found in this test campaign.
