# Adversarial Test Results Summary

**Target**: `proc_tree_check_is_ancestor` — checks if `a_ptr` is an ancestor of `child_ptr` in a process tree by walking the `uppertree_seq`.

**Postcondition**: `ret == proc_perms@[child_ptr].value().uppertree_seq@.contains(a_ptr)`

---

## Results: 12/12 tests FAILED verification ✅

All adversarial tests were correctly rejected by the specification.

### (1) Boundary Tests — 4/4 FAILED ✅

| Test | Violated Precondition | Assertion | Result |
|------|----------------------|-----------|--------|
| `boundary_test_equal_depth` | `depth(a) == depth(child)` instead of `<` | `uppertree_seq.contains(a_ptr)` | FAILED ✅ |
| `boundary_test_reversed_depth` | `depth(a) > depth(child)` instead of `<` | `uppertree_seq.contains(a_ptr)` | FAILED ✅ |
| `boundary_test_child_is_root` | `child_ptr == root_proc` | `root.uppertree_seq.contains(a_ptr)` | FAILED ✅ |
| `boundary_test_a_not_in_domain` | `a_ptr ∉ proc_tree_dom` | `uppertree_seq.contains(a_ptr)` | FAILED ✅ |

**Conclusion**: The specification correctly guards all preconditions. Invalid inputs do not lead to false ancestry conclusions.

### (2) Behavioral Mutation Tests — 4/4 FAILED ✅

| Test | Mutation | Assertion | Result |
|------|----------|-----------|--------|
| `behavioral_always_ancestor` | Claim all lesser-depth nodes are ancestors | `uppertree_seq.contains(a_ptr)` always | FAILED ✅ |
| `behavioral_never_ancestor` | Claim no node is ever an ancestor | `!uppertree_seq.contains(a_ptr)` always | FAILED ✅ |
| `behavioral_wrong_postcondition` | Equate ancestry with direct parentage | `ret == (parent == Some(a_ptr))` | FAILED ✅ |
| `behavioral_ancestor_implies_parent` | Ancestor must be direct parent | `parent == Some(a_ptr)` when ancestor | FAILED ✅ |

**Conclusion**: The specification correctly distinguishes between ancestry and parentage, and does not trivially force the result to always-true or always-false.

### (3) Logical Tests — 4/4 FAILED ✅

| Test | Unintended Property | Assertion | Result |
|------|---------------------|-----------|--------|
| `logical_test_symmetry` | Ancestry is symmetric | `a.uppertree_seq.contains(child)` | FAILED ✅ |
| `logical_test_immediate_depth` | Ancestor depth is exactly depth−1 | `depth(child) == depth(a) + 1` | FAILED ✅ |
| `logical_test_unique_depth` | Same depth implies same node | `a_ptr == b_ptr` | FAILED ✅ |
| `logical_test_subtree_implies_child` | Subtree member = direct child | `children.contains(child)` | FAILED ✅ |

**Notable finding**: An earlier transitivity test (`a ⊂ ancestors(b) ∧ b ⊂ ancestors(c) ⟹ a ⊂ ancestors(c)`) was verified by the spec — transitivity IS entailed. This is correct behavior for a well-formed tree, confirming the spec's `proc_uppertree_seq_wf` properly encodes uppertree prefix relationships.

**Conclusion**: The specification does not admit unintended logical consequences. It correctly rejects symmetry, stronger depth constraints, unique-depth assumptions, and subtree-to-child conflation.

---

## Overall Assessment

The specification for `proc_tree_check_is_ancestor` is **consistent**: it rejects all 12 adversarial properties across boundary, behavioral, and logical categories. No spec weaknesses were found.
