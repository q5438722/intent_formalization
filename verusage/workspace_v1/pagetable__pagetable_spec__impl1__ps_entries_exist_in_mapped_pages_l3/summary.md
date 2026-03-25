# Test Execution Summary: `ps_entries_exist_in_mapped_pages_l3`

## Target Specification
The proof function `ps_entries_exist_in_mapped_pages_l3` on `PageTable` proves that under `wf_l4`, `wf_l3`, and `wf_mapping_1g`:
1. **PS entries are mapped**: L3 entries with `present && ps` have `page_not_mapped(addr) == false`
2. **Non-PS entries are in L2**: L3 entries with `present && !ps` have their addr in `l2_tables`

## Results Summary

| File | Tests | Failures | Status |
|------|-------|----------|--------|
| `boundary_tests.rs` | 5 | 5 | ✅ All rejected |
| `behavioral_mutation_tests.rs` | 5 | 5 | ✅ All rejected |
| `logical_tests.rs` | 5 | 5 | ✅ All rejected |
| `correctness_tests.rs` (combined) | 15 | 15 | ✅ All rejected |

## Boundary Tests (5/5 FAILED — as expected)

| # | Test | Failure Mode | Error Type |
|---|------|-------------|------------|
| 1 | `boundary_test_missing_wf_l4` | Omit `wf_l4()` precondition | precondition not satisfied |
| 2 | `boundary_test_missing_wf_l3` | Omit `wf_l3()` precondition | precondition not satisfied |
| 3 | `boundary_test_missing_wf_mapping_1g` | Omit `wf_mapping_1g()` precondition | precondition not satisfied |
| 4 | `boundary_test_missing_all_preconditions` | Omit all 3 preconditions | precondition not satisfied |
| 5 | `boundary_test_l2_ensures_without_wf_l3` | L2 conclusion without `wf_l3()` | precondition not satisfied |

## Behavioral Mutation Tests (5/5 FAILED — as expected)

| # | Test | Mutation | Error Type |
|---|------|----------|------------|
| 1 | `mutation_test_ps_entry_is_unmapped` | Negate: `page_not_mapped == true` | assertion failed |
| 2 | `mutation_test_non_ps_entry_not_in_l2` | Negate: `!l2_tables.contains(addr)` | assertion failed |
| 3 | `mutation_test_mapped_addr_differs` | Mutate: `mapping_1g[va].addr != entry.addr` | assertion failed |
| 4 | `mutation_test_ps_entry_addr_is_zero` | Mutate: `entry.addr == 0` | assertion failed |
| 5 | `mutation_test_ps_entry_in_l2_tables` | Swap: apply non-PS postcondition to PS | assertion failed |

## Logical Tests (5/5 FAILED — as expected)

| # | Test | Unintended Property | Error Type |
|---|------|---------------------|------------|
| 1 | `logical_test_ps_entry_also_in_mapping_4k` | 1G pages also in 4K mapping | assertion failed |
| 2 | `logical_test_all_l3_entries_present` | All L3 entries must be present | assertion failed |
| 3 | `logical_test_ps_entry_must_be_writable` | PS entries must be writable | assertion failed |
| 4 | `logical_test_ps_entries_unique_addrs` | PS entries have unique PAs | assertion failed |
| 5 | `logical_test_ps_entry_executable` | PS entries must be executable | assertion failed |

## Notable Finding

During initial testing, a logical test asserting **L3 reverse map injectivity** (`l3_rev_map@[p1] != l3_rev_map@[p2]` when `p1 != p2`) **passed verification**. This is because `wf_l3` requires `spec_resolve_mapping_l4(l3_rev_map@[p]).get_Some_0().addr == p`, making the mapping functionally invertible. Since `spec_resolve_mapping_l4` at a given index returns a single value, two distinct pointers must map to distinct indices. This is a **logically derived consequence** of the spec, not a weakness — the test was replaced with `logical_test_ps_entries_unique_addrs`.

## Conclusion

The specification is **consistent** across all three test dimensions:
- **Boundary**: All preconditions are necessary; none can be dropped
- **Behavioral**: Both postconditions are tight; negations and mutations are rejected
- **Logical**: The spec does not entail unintended properties (writability, executability, PA uniqueness, cross-mapping)
