# Test Summary: same_or_deeper_depth_imply_none_ancestor

## File Under Test

`process_manager__spec_proof__impl2__same_or_deeper_depth_imply_none_ancestor.rs`

Defines a `ProcessManager` with a container tree hierarchy. The main spec under test is:

**`same_or_deeper_depth_imply_none_ancestor`**: If container `ancestor_ptr` has depth ≥ `child_ptr`'s depth, then `ancestor_ptr`'s subtree does NOT contain `child_ptr`. This encodes the tree property that a node can only be an ancestor of nodes at strictly greater depth.

Two versions exist:
1. **Method**: `ProcessManager::same_or_deeper_depth_imply_none_ancestor` (requires `self.wf()`)
2. **Standalone**: `same_or_deeper_depth_imply_none_ancestor(root, perms, a_ptr, child_ptr)` (requires `container_perms_wf` + `container_tree_wf`)

---

## Correctness Results (correctness_tests.rs)

**Result: 32 verified, 0 errors ✅ — ALL PASS**

| Test | Description | Expected | Actual |
|------|-------------|----------|--------|
| `test_method_basic` | Basic method call with valid preconditions, check postcondition | PASS | ✅ PASS |
| `test_same_ptr` | Same pointer (c_ptr, c_ptr) — depth ≥ depth trivially | PASS | ✅ PASS |
| `test_strictly_deeper` | Ancestor strictly deeper than child | PASS | ✅ PASS |
| `test_equal_depth` | Equal depth boundary case | PASS | ✅ PASS |
| `test_postcondition_negated_form` | Postcondition via `!contains()` form | PASS | ✅ PASS |
| `test_standalone_basic` | Standalone function with valid preconditions | PASS | ✅ PASS |
| `test_standalone_same_ptr` | Standalone with same pointer | PASS | ✅ PASS |
| `test_standalone_strictly_deeper` | Standalone with strictly deeper ancestor | PASS | ✅ PASS |
| `test_use_in_reasoning` | Chain two calls, assert both results | PASS | ✅ PASS |

---

## Completeness Results

### Round 1: Precondition Violations (completeness_round1.rs)

**Result: 23 verified, 7 errors ✅ — ALL TESTS FAIL**

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| `test_missing_wf` | Call without `self.wf()` (only `container_perms_wf`) | FAIL | ✅ FAIL |
| `test_ancestor_not_in_domain` | `ancestor_ptr` not in container domain | FAIL | ✅ FAIL |
| `test_child_not_in_domain` | `child_ptr` not in container domain | FAIL | ✅ FAIL |
| `test_reversed_depth` | Depth reversed: `ancestor.depth < child.depth` | FAIL | ✅ FAIL |
| `test_neither_in_domain` | Neither pointer in container domain | FAIL | ✅ FAIL |
| `test_standalone_missing_perms_wf` | Standalone: missing `container_perms_wf` | FAIL | ✅ FAIL |
| `test_standalone_missing_tree_wf` | Standalone: missing `container_tree_wf` | FAIL | ✅ FAIL |

### Round 2: Overly Strong Postconditions (completeness_round2.rs)

**Result: 23 verified, 5 errors ✅ — ALL TESTS FAIL**

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| `test_subtree_empty` | Assert ancestor's subtree is completely empty | FAIL | ✅ FAIL |
| `test_reverse_subtree` | Assert child's subtree doesn't contain ancestor (not guaranteed) | FAIL | ✅ FAIL |
| `test_ptrs_different` | Assert `ancestor_ptr != child_ptr` (could be same) | FAIL | ✅ FAIL |
| `test_depth_strictly_greater` | Assert depth is strictly greater (spec allows equal) | FAIL | ✅ FAIL |
| `test_subtree_bounded` | Assert `ancestor.depth == 0` (not guaranteed) | FAIL | ✅ FAIL |

### Round 3: Negated/Contradicted Postconditions (completeness_round3.rs)

**Result: 23 verified, 5 errors ✅ — ALL TESTS FAIL**

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| `test_negated_contains` | Assert subtree DOES contain child (`== true`) | FAIL | ✅ FAIL |
| `test_negated_direct` | Assert `.contains(child_ptr)` directly | FAIL | ✅ FAIL |
| `test_assert_false` | Assert `false` after valid call | FAIL | ✅ FAIL |
| `test_self_in_subtree` | Same ptr: assert subtree contains itself | FAIL | ✅ FAIL |
| `test_standalone_negated` | Standalone: negate postcondition | FAIL | ✅ FAIL |

### Round 4: Wrong Specific Values (completeness_round4.rs)

**Result: 23 verified, 5 errors ✅ — ALL TESTS FAIL**

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| `test_ancestor_depth_zero` | Assert `ancestor.depth == 0` | FAIL | ✅ FAIL |
| `test_child_depth_one` | Assert `child.depth == 1` | FAIL | ✅ FAIL |
| `test_depths_equal` | Assert depths are exactly equal (spec only says ≥) | FAIL | ✅ FAIL |
| `test_ancestor_is_root` | Assert ancestor's parent is `None` | FAIL | ✅ FAIL |
| `test_ptrs_equal` | Assert `ancestor_ptr == child_ptr` | FAIL | ✅ FAIL |

### Round 5: Cross-Function Misuse & Edge Cases (completeness_round5.rs)

**Result: 23 verified, 5 errors ✅ — ALL TESTS FAIL**

| Test | What it tests | Expected | Actual |
|------|--------------|----------|--------|
| `test_reverse_not_in_subtree` | Assert ancestor not in child's subtree (reverse direction) | FAIL | ✅ FAIL |
| `test_disjoint_subtrees_at_same_depth` | Assert same-depth containers have disjoint subtrees | FAIL | ✅ FAIL |
| `test_third_container` | Assert unrelated 3rd container not in subtree | FAIL | ✅ FAIL |
| `test_child_no_children` | Assert child has no children | FAIL | ✅ FAIL |
| `test_uppertree_seq` | Assert child's uppertree_seq doesn't contain ancestor | FAIL | ✅ FAIL |

---

## Overall Assessment

### Correctness: ✅ PASS
All 9 correctness tests verify successfully. The spec correctly expresses that containers at the same or deeper depth cannot be ancestors. Valid usages with diverse depth relationships (equal, strictly greater, same pointer, multiple calls) all produce the expected postcondition.

### Completeness: ✅ PASS
All 27 completeness tests fail as expected. The specs are tight enough to reject:
- Missing preconditions (all 4 requires clauses are necessary)
- Overly strong claims (subtree emptiness, pointer inequality, reverse relationships)
- Contradictions of the postcondition
- Assertions about specific unguaranteed values
- Cross-function inferences not supported by this spec

### Spec Gaps Found: None
No spec gaps were detected. The specification is both correct and complete for its stated purpose.
