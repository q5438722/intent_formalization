# Adversarial Proof Test Summary: `proc_tree_root_inv`

## Target Function
```
proof fn proc_tree_root_inv(&self, proc_ptr: ProcPtr)
    requires self.wf(), self.proc_dom().contains(proc_ptr),
    ensures  depth == 0 ==> container.root_process.unwrap() == proc_ptr
```

## Results Overview

| Category | Total | Failed (expected) | Passed (unexpected) |
|----------|-------|--------------------|---------------------|
| Boundary | 6 | 6 ✅ | 0 |
| Mutation | 5 | 5 ✅ | 0 |
| Logical  | 5 | 5 ✅ | 0 |
| **Total** | **16** | **16** | **0** |

All 16 tests correctly **failed verification**, confirming the specification rejects invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Boundary Tests (6/6 failed ✅)

| # | Test | Violated Precondition | Result |
|---|------|----------------------|--------|
| 1 | `test_boundary_missing_wf` | `self.wf()` omitted | FAIL ✅ |
| 2 | `test_boundary_missing_membership` | `proc_dom().contains(proc_ptr)` omitted | FAIL ✅ |
| 3 | `test_boundary_no_preconditions` | Both preconditions omitted | FAIL ✅ |
| 4 | `test_boundary_helper_missing_perms_wf` | `proc_perms_wf` omitted from helper | FAIL ✅ |
| 5 | `test_boundary_helper_missing_tree_wf` | `proc_tree_wf` omitted from helper | FAIL ✅ |
| 6 | `test_boundary_helper_missing_dom_subset` | `proc_tree_dom_subset_of_proc_dom` omitted from helper | FAIL ✅ |

**Conclusion**: All preconditions are necessary; the spec correctly rejects invalid inputs.

---

## Behavioral Mutation Tests (5/5 failed ✅)

| # | Test | Mutation | Result |
|---|------|----------|--------|
| 1 | `test_mutation_negate_ensures` | Assert `root_process != proc_ptr` when depth==0 | FAIL ✅ |
| 2 | `test_mutation_nonzero_depth_is_root` | Assert depth>0 implies root_process==proc_ptr | FAIL ✅ |
| 3 | `test_mutation_root_process_none` | Assert root_process is None when depth==0 | FAIL ✅ |
| 4 | `test_mutation_helper_negate_root_depth` | Assert root has non-zero depth | FAIL ✅ |
| 5 | `test_mutation_helper_nonroot_depth_zero` | Assert non-root has depth==0 | FAIL ✅ |

**Conclusion**: All behavioral mutations are rejected; the spec correctly constrains input-output relationships.

---

## Logical Tests (5/5 failed ✅)

| # | Test | Unintended Property | Result |
|---|------|---------------------|--------|
| 1 | `test_logical_depth_zero_implies_no_parent` | depth==0 ⟹ parent is None | FAIL ✅ |
| 2 | `test_logical_root_always_some` | Every container has root_process | FAIL ✅ |
| 3 | `test_logical_depth_bounded` | depth < PROC_CHILD_LIST_LEN | FAIL ✅ |
| 4 | `test_logical_depth_zero_no_children` | depth==0 ⟹ no children | FAIL ✅ |
| 5 | `test_logical_depth_zero_in_root_container` | depth==0 ⟹ owning_container == root_container | FAIL ✅ |

**Conclusion**: The spec does not over-entail; none of these unstated properties are derivable.

---

## Notable Finding

During development, an initial test (`test_logical_unique_root_per_container`) that assumed two distinct depth-0 processes in the same container could coexist **passed verification** — correctly deriving a contradiction (`assert(false)`) from the postcondition. This confirms the spec is strong enough to guarantee uniqueness of root processes per container, which is a desirable and intended property.

---

## Overall Assessment

The specification for `proc_tree_root_inv` is **well-formed**:
- **Preconditions are tight**: every `requires` clause is necessary
- **Postconditions are precise**: mutations of the ensures clause are rejected
- **Entailment is controlled**: the spec does not admit unintended logical consequences
