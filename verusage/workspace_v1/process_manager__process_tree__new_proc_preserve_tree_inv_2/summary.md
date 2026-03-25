# Adversarial Proof Test Summary

## Target
`process_manager__process_tree__new_proc_preserve_tree_inv_2.rs`

**Function under test**: `new_proc_preserve_tree_inv_2`
- **Requires**: `new_proc_ensures(root_proc, proc_tree_dom, old_proc_perms, new_proc_perms, proc_ptr, new_proc_ptr)`
- **Ensures**: `proc_childern_parent_wf(root_proc, proc_tree_dom.insert(new_proc_ptr), new_proc_perms)`

---

## Results

### Boundary Tests (`boundary_tests.rs`) — 3/3 FAILED ✅

| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_new_proc_already_in_tree` | `proc_tree_dom.contains(new_proc_ptr) == true` (should be false) | FAILED — precondition rejected |
| `test_boundary_parent_not_in_tree` | `!proc_tree_dom.contains(proc_ptr)` (parent must be in tree) | FAILED — precondition rejected |
| `test_boundary_children_at_capacity` | `children.len() >= PROC_CHILD_LIST_LEN` (must be <) | FAILED — precondition rejected |

**Conclusion**: The spec correctly rejects all tested boundary violations. Preconditions are well-guarded.

### Behavioral Mutation Tests (`behavioral_mutation_tests.rs`) — 3/3 FAILED ✅

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_wrong_domain` | Assert postcondition on `proc_tree_dom` (without `.insert(new_proc_ptr)`) | FAILED — wrong domain rejected |
| `test_mutation_old_perms` | Assert postcondition with `old_proc_perms` instead of `new_proc_perms` | FAILED — stale perms rejected |
| `test_mutation_negated_postcondition` | Assert `!proc_childern_parent_wf(...)` (negation) | FAILED — negation rejected |

**Conclusion**: The spec correctly distinguishes between the actual postcondition and mutated variants. Behavioral mutations are rejected.

### Logical Tests (`logical_tests.rs`) — 3/3 FAILED ✅

| Test | Unguaranteed Property | Result |
|------|----------------------|--------|
| `test_logical_uppertree_seq_wf_not_guaranteed` | `proc_uppertree_seq_wf` on new tree | FAILED — not derivable |
| `test_logical_subtree_set_wf_not_guaranteed` | `proc_subtree_set_wf` on new tree | FAILED — not derivable |
| `test_logical_full_tree_wf_not_guaranteed` | Full `proc_tree_wf` (7 sub-properties) | FAILED — not derivable |

**Conclusion**: The spec does not allow unintended reasoning for these three properties. They are not derivable from `proc_childern_parent_wf` alone.

---

## Notable Finding: Spec Entails More Than Stated

During testing, the following properties were found to be **unexpectedly derivable** from the preconditions (`new_proc_ensures`) combined with the postcondition (`proc_childern_parent_wf`):

1. **`proc_root_wf`** — Root well-formedness on the new tree is derivable. The root's depth, membership, and parent properties are preserved through `new_proc_ensures`.

2. **`procs_linkedlist_wf`** — Linked-list well-formedness on the new tree is derivable. The parent reverse pointer and children containment relationships carry through.

These are properties the function does NOT explicitly ensure (they are commented out in the original ensures clause), yet they can be automatically proven by the SMT solver from the available facts. This means the specification is **stronger than intended** in these dimensions — it implicitly guarantees properties it does not declare.

**Implication**: If these properties are intended consequences, the ensures clause could be strengthened to make them explicit. If unintended, the precondition (`new_proc_ensures`) may be over-constrained.

---

## Overall Summary

| Category | Tests | Failed (Expected) | Passed (Unexpected) |
|----------|-------|--------------------|---------------------|
| Boundary | 3 | 3 ✅ | 0 |
| Behavioral Mutation | 3 | 3 ✅ | 0 |
| Logical | 3 | 3 ✅ | 0 |
| **Total** | **9** | **9** | **0** |

All 9 adversarial tests correctly fail verification, confirming the specification properly rejects invalid inputs, incorrect behaviors, and most unintended logical inferences.
