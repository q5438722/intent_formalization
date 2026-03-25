# Adversarial Proof Test Summary

## Target
`helper_kernel_kill_proc_root` — removes a root process (depth==0) from the kernel, freeing its page table and reclaiming its page.

## Results: All 15 tests FAILED verification ✅

Every adversarial test was correctly rejected by the specification, indicating the spec is **sufficiently strong** across the tested dimensions.

---

### Boundary Tests (5/5 FAILED ✅)

| # | Test | Violated Precondition | Result |
|---|------|----------------------|--------|
| 1 | `test_boundary_proc_not_in_domain` | `proc_ptr` not in `proc_dom()` | FAIL ✅ |
| 2 | `test_boundary_nonempty_threads` | `owned_threads` is non-empty | FAIL ✅ |
| 3 | `test_boundary_nonempty_children` | `children` is non-empty | FAIL ✅ |
| 4 | `test_boundary_nonzero_depth` | `depth > 0` (not root) | FAIL ✅ |
| 5 | `test_boundary_has_ioid` | `ioid.is_Some()` | FAIL ✅ |

**Conclusion:** All preconditions are enforced. Invalid inputs are properly rejected.

---

### Behavioral Mutation Tests (5/5 FAILED ✅)

| # | Test | Mutated Postcondition | Result |
|---|------|----------------------|--------|
| 1 | `test_mutation_proc_still_in_domain` | Assert removed proc still in domain | FAIL ✅ |
| 2 | `test_mutation_thread_dom_changed` | Assert thread domain changed | FAIL ✅ |
| 3 | `test_mutation_container_dom_changed` | Assert container domain changed | FAIL ✅ |
| 4 | `test_mutation_proc_dom_unchanged` | Assert proc domain unchanged (no removal) | FAIL ✅ |
| 5 | `test_mutation_children_changed` | Assert remaining proc's children changed | FAIL ✅ |

**Conclusion:** All postconditions correctly constrain the output. Incorrect behavioral mutations are rejected.

---

### Logical Tests (5/5 FAILED ✅)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_pcid_still_active` | pcid remains active after removal | FAIL ✅ |
| 2 | `test_logical_page_alloc_unchanged` | allocated_pages_4k completely unchanged | FAIL ✅ |
| 3 | `test_logical_extra_proc_removed` | Unrelated process also removed | FAIL ✅ |
| 4 | `test_logical_uppertree_changed` | Remaining proc uppertree_seq changed | FAIL ✅ |
| 5 | `test_logical_double_removal` | Idempotent double removal possible | FAIL ✅ |

**Conclusion:** The spec does not entail unintended logical properties. The semantic boundary is well-defined.

---

## Overall Assessment

The specification for `helper_kernel_kill_proc_root` is **consistent** across all three testing dimensions:
- **Boundary**: Invalid inputs are rejected (preconditions are necessary).
- **Behavioral**: Incorrect outputs are rejected (postconditions are precise).
- **Logical**: Unintended reasoning is not supported (no over-permissive entailment).

No specification weaknesses were detected in this round of testing.
