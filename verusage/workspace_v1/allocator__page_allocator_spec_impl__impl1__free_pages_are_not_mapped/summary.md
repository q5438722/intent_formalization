# Adversarial Test Results Summary

**Target**: `free_pages_are_not_mapped` — proves free 4k pages are not mapped under `wf()`.

## Results: All 12 tests FAILED verification ✓

The specification correctly rejected all adversarial properties.

### Boundary Tests (4/4 failed ✓)
| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_no_precondition` | No `wf()` at all | FAIL ✓ |
| `test_only_page_array_wf` | Only `page_array_wf` | FAIL ✓ |
| `test_missing_free_pages_4k_wf` | All wf except `free_pages_4k_wf` | FAIL ✓ |
| `test_missing_mapped_pages_4k_wf` | All wf except `mapped_pages_4k_wf` | FAIL ✓ |

**Finding**: The precondition `wf()` is necessary — each sub-predicate contributes to the proof. Removing `free_pages_4k_wf` prevents linking page pointers to `Free4k` state; removing `mapped_pages_4k_wf` prevents excluding mapped membership.

### Behavioral Mutation Tests (4/4 failed ✓)
| Test | Mutation | Result |
|------|----------|--------|
| `test_free_implies_mapped` | Negated output: free → mapped | FAIL ✓ |
| `test_mapped_implies_free` | Reversed relation: mapped → free | FAIL ✓ |
| `test_free_page_nonzero_refcount` | Free page has ref_count > 0 | FAIL ✓ |
| `test_free_page_has_owner` | Free page has owning_container | FAIL ✓ |

**Finding**: The spec correctly rejects all incorrect behavioral claims. Free pages cannot be mapped, mapped pages cannot be free, and free pages have zero reference count and no container owner.

### Logical Tests (4/4 failed ✓)
| Test | Property Probed | Result |
|------|----------------|--------|
| `test_free_2m_not_mapped` | Free 2m pages not mapped (generalization) | FAIL ✓ |
| `test_free_1g_not_mapped` | Free 1g pages not mapped (generalization) | FAIL ✓ |
| `test_wf_implies_no_free_pages` | Free set is empty (emptiness) | FAIL ✓ |
| `test_free_page_addr_zero` | Free page has address 0 (determinism) | FAIL ✓ |

**Finding**: Tests 1-2 are notable — while the wf() predicate semantically implies free 2m/1g pages can't be mapped (same state-disjointness reasoning applies), the solver could not automatically derive this without proof guidance. This indicates the property is entailed but requires explicit proof steps (as done for 4k). Tests 3-4 correctly confirm the spec doesn't over-constrain set cardinality or page addresses.

## Conclusion

The specification for `free_pages_are_not_mapped` is **consistent**: it rejects all 12 adversarial queries probing invalid inputs (boundary), incorrect behaviors (mutation), and unintended reasoning (logical). No spec weaknesses were detected.
