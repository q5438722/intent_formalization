# Test Summary: `pages_with_mappings_are_mapped`

## Target Specification

```rust
pub proof fn pages_with_mappings_are_mapped(&self, page_ptr: PagePtr)
    requires
        self.wf(),
        page_ptr_valid(page_ptr),
        self.page_mappings(page_ptr).len() > 0,
    ensures
        self.page_is_mapped(page_ptr) == true,
```

**Semantics**: If the allocator is well-formed, the page pointer is valid, and the page has at least one mapping entry, then the page must appear in one of the mapped page sets (4k/2m/1g).

---

## Results Overview

| File | Tests | All Failed (as expected) |
|------|-------|------------------------|
| `boundary_tests.rs` | 4 | ✅ Yes (4 errors) |
| `behavioral_mutation_tests.rs` | 4 | ✅ Yes (4 errors) |
| `logical_tests.rs` | 4 | ✅ Yes (4 errors) |

**Total: 12/12 tests correctly rejected by Verus.**

---

## Boundary Tests (4/4 FAIL ✅)

| # | Test | Violated Precondition | Result |
|---|------|-----------------------|--------|
| 1 | `test_boundary_no_wf` | Missing `self.wf()` | FAIL ✅ |
| 2 | `test_boundary_unaligned_ptr` | `page_ptr % 0x1000 != 0` | FAIL ✅ |
| 3 | `test_boundary_zero_mappings` | `page_mappings().len() == 0` | FAIL ✅ |
| 4 | `test_boundary_ptr_out_of_range` | `page_ptr / 0x1000 >= NUM_PAGES` | FAIL ✅ |

**Interpretation**: All three preconditions (`wf()`, `page_ptr_valid`, `mappings.len() > 0`) are necessary — removing or violating any one prevents the postcondition from being provable.

---

## Behavioral Mutation Tests (4/4 FAIL ✅)

| # | Test | Mutated Property | Result |
|---|------|-----------------|--------|
| 5 | `test_mutation_not_mapped` | Negated postcondition (`== false`) | FAIL ✅ |
| 6 | `test_mutation_must_be_4k` | Strengthened to `mapped_pages_4k` only | FAIL ✅ |
| 7 | `test_mutation_refcount_equals_mappings_only` | Ignored `io_mappings` in ref_count | FAIL ✅ |
| 8 | `test_mutation_state_must_be_mapped4k` | Fixed state to `Mapped4k` | FAIL ✅ |

**Interpretation**: The spec correctly rejects negated outputs and overly-specific mutations. The postcondition is a disjunction over three mapped sets; narrowing to one is correctly rejected. The ref_count invariant properly accounts for both `mappings` and `io_mappings`.

---

## Logical Tests (4/4 FAIL ✅)

| # | Test | Tested Property | Result |
|---|------|----------------|--------|
| 9 | `test_logical_converse_mapped_implies_mappings` | Converse: mapped → mappings > 0 | FAIL ✅ |
| 10 | `test_logical_mapped_implies_specific_container` | Mapped → specific container | FAIL ✅ |
| 11 | `test_logical_different_pages_different_sets` | Two mapped pages can't share a set | FAIL ✅ |
| 12 | `test_logical_mappings_excludes_io_mappings` | mappings > 0 → io_mappings == 0 | FAIL ✅ |

**Interpretation**:
- **Test 9**: The converse is correctly not entailed — a page can be mapped via `io_mappings` alone with zero `page_mappings`.
- **Test 10**: The spec doesn't determine *which* container owns a mapped page from `page_mappings` alone.
- **Test 11**: Multiple pages can coexist in the same mapped set — the spec doesn't enforce exclusivity.
- **Test 12**: The spec allows both `mappings` and `io_mappings` to be simultaneously non-empty.

---

## Conclusion

The specification for `pages_with_mappings_are_mapped` is **consistent** with respect to all 12 adversarial queries:
- It correctly **rejects** all invalid inputs (boundary violations).
- It correctly **rejects** all mutated/incorrect behaviors.
- It correctly **refuses to entail** unintended logical properties.

No specification weaknesses were detected in this evaluation.
