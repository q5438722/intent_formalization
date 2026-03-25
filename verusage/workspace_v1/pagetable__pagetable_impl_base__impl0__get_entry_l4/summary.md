# Test Summary: `get_entry_l4`

**Target**: `pagetable__pagetable_impl_base__impl0__get_entry_l4.rs`
**Function**: `PageTable::get_entry_l4(target_l4i: L4Index) -> Option<PageEntry>`

## Specification Under Test

- **Preconditions**: `self.wf()`, `self.kernel_l4_end <= target_l4i < 512`
- **Postconditions**:
  1. Result equals `spec_resolve_mapping_l4(target_l4i)`
  2. `None` result cascades: no 4k mappings exist under that L4 index

## Results

All **15 tests** across 3 files **FAILED verification** as expected (5 errors each, 19 verified items from the base code). The spec correctly rejects all adversarial queries.

### Boundary Tests (5/5 failed ✓)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `boundary_l4i_out_of_range` | L4 index 512 (OOB array access) | FAIL ✓ |
| `boundary_kernel_entry_must_be_present` | Kernel entry present bit not guaranteed | FAIL ✓ |
| `boundary_no_wf_assert_none` | No wf() — cannot determine resolution | FAIL ✓ |
| `boundary_kernel_index_has_valid_l3` | Kernel index L3 table not guaranteed | FAIL ✓ |
| `boundary_no_wf_assert_cascade` | No wf() — 4k cascade not derivable | FAIL ✓ |

### Behavioral Mutation Tests (5/5 failed ✓)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `mutation_always_some` | Resolution doesn't always succeed | FAIL ✓ |
| `mutation_resolved_addr_is_zero` | Resolved entry addr not constrained to 0 | FAIL ✓ |
| `mutation_some_but_not_present` | Some in user range implies present=true | FAIL ✓ |
| `mutation_none_entry_has_nonzero_addr` | None entry is empty (addr=0) via present_or_zero | FAIL ✓ |
| `mutation_some_implies_all_4k_mapped` | Some at L4 doesn't imply all 4k mappings exist | FAIL ✓ |

### Logical Tests (5/5 failed ✓)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `logical_l4_always_resolves` | Resolution not guaranteed to succeed | FAIL ✓ |
| `logical_different_indices_different_results` | Different indices can map to same result | FAIL ✓ |
| `logical_cr3_is_zero` | cr3 not constrained to specific value | FAIL ✓ |
| `logical_kernel_l4_end_is_one` | kernel_l4_end not constrained to constant | FAIL ✓ |
| `logical_l4_some_implies_l3_some` | L4 resolve doesn't imply any L3 resolves | FAIL ✓ |

## Conclusion

The specification for `get_entry_l4` is **consistent** with respect to all tested adversarial queries. It correctly:

- **Rejects invalid inputs**: OOB indices, kernel-region assumptions, missing wf()
- **Rejects incorrect behaviors**: wrong addresses, wrong present bits, spurious mappings
- **Rejects unintended reasoning**: determinism assumptions, structural constraints on cr3/kernel_l4_end, cross-level implications

No spec weaknesses were detected in this test suite.
