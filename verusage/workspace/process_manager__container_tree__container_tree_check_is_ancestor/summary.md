# Summary: container_tree_check_is_ancestor Specification Testing

## File Under Test

`process_manager__container_tree__container_tree_check_is_ancestor.rs` defines:
- **`container_tree_check_is_ancestor`** (exec fn): Checks if container `a_ptr` is an ancestor of `child_ptr` in a container tree by walking up the parent chain. Returns `true` iff `a_ptr` appears in `child_ptr`'s `uppertree_seq`.
- **`seq_push_lemma`** (proof fn, external_body): Provides three properties about `Seq::push` and `Seq::contains`.
- **`container_perms_wf`** (open spec fn): Well-formedness of container permission map (init, addr, children wf/unique, no self-children, uppertree no-dups, subtree finite, depth == uppertree len).
- **`container_tree_wf`** (open spec fn): Conjunction of 7 closed sub-predicates defining tree structure.
- 7 closed spec fns: `container_root_wf`, `container_childern_parent_wf`, `containers_linkedlist_wf`, `container_childern_depth_wf`, `container_subtree_set_wf`, `container_uppertree_seq_wf`, `container_subtree_set_exclusive`.

---

## Correctness Results (should all PASS)

**Result: 17 verified, 0 errors ✅**

| Test | Description | Expected | Actual |
|------|-------------|----------|--------|
| `test_seq_push_contains_pushed_value` | Pushed value is contained in result | PASS | ✅ PASS |
| `test_seq_push_empty_seq` | Push on empty seq contains the value | PASS | ✅ PASS |
| `test_seq_push_preserves_existing` | Existing element preserved after push | PASS | ✅ PASS |
| `test_seq_push_absent_stays_absent` | Non-existing element stays absent after push of different value | PASS | ✅ PASS |
| `test_seq_push_param_existing` | Parameterized: existing elements preserved | PASS | ✅ PASS |
| `test_seq_push_param_absent` | Parameterized: absent elements stay absent | PASS | ✅ PASS |
| `test_container_perms_wf_empty_map` | Empty map satisfies container_perms_wf (vacuously true) | PASS | ✅ PASS |
| `test_tree_wf_implies_subpredicates` | container_tree_wf implies all 7 sub-predicates | PASS | ✅ PASS |
| `test_seq_push_usize` | seq_push_lemma works for usize type | PASS | ✅ PASS |
| `test_seq_push_multiple` | Multiple sequential pushes preserve all elements | PASS | ✅ PASS |
| `test_wf_consistency` | container_perms_wf properties hold for any c_ptr in domain | PASS | ✅ PASS |
| `test_seq_push_same_value` | Pushing duplicate value still contains it | PASS | ✅ PASS |

---

## Completeness Results (should all FAIL)

### Round 1: Precondition Violations
**Result: 5 verified (definitions), 5 errors ✅**

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_fail_assert_init_without_wf` | Assert is_init without container_perms_wf | FAIL | ✅ FAIL |
| `test_fail_assert_addr_without_wf` | Assert addr == c_ptr without wf | FAIL | ✅ FAIL |
| `test_fail_assert_children_wf_without_perms_wf` | Assert children.wf() without perms wf | FAIL | ✅ FAIL |
| `test_fail_assert_root_wf_without_tree_wf` | Assert container_root_wf without tree_wf | FAIL | ✅ FAIL |
| `test_fail_no_duplicates_without_wf` | Assert no_duplicates without wf | FAIL | ✅ FAIL |

### Round 2: Overly Strong Postconditions
**Result: 5 verified (definitions), 5 errors ✅**

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_fail_push_same_length` | push preserves length (wrong: it increases by 1) | FAIL | ✅ FAIL |
| `test_fail_push_only_element` | push makes seq length 1 | FAIL | ✅ FAIL |
| `test_fail_empty_map_has_elements` | Empty wf map has elements in domain | FAIL | ✅ FAIL |
| `test_fail_tree_wf_implies_depth_zero` | Every node has depth 0 under tree_wf | FAIL | ✅ FAIL |
| `test_fail_push_guarantees_unique` | Push guarantees no_duplicates | FAIL | ✅ FAIL |

### Round 3: Negated/Contradicted Postconditions
**Result: 5 verified (definitions), 5 errors ✅**

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_fail_push_not_contained` | Pushed value NOT contained (negation) | FAIL | ✅ FAIL |
| `test_fail_existing_not_preserved` | Existing element NOT preserved (negation) | FAIL | ✅ FAIL |
| `test_fail_absent_element_appears` | Absent element appears after unrelated push | FAIL | ✅ FAIL |
| `test_fail_negate_empty_map_wf` | Negate container_perms_wf for empty map | FAIL | ✅ FAIL |
| `test_fail_negate_tree_wf_implies_root_wf` | tree_wf does NOT imply root_wf (negation) | FAIL | ✅ FAIL |

### Round 4: Wrong Specific Values
**Result: 5 verified (definitions), 5 errors ✅**

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_fail_push_wrong_order` | Push produces wrong element order | FAIL | ✅ FAIL |
| `test_fail_push_wrong_length` | Push gives wrong length (2 instead of 4) | FAIL | ✅ FAIL |
| `test_fail_push_wrong_index` | Wrong element at index after push | FAIL | ✅ FAIL |
| `test_fail_push_empty_wrong_value` | Wrong value at index 0 after push to empty | FAIL | ✅ FAIL |
| `test_fail_wrong_containment` | Wrong containment claim on concrete seq | FAIL | ✅ FAIL |

### Round 5: Cross-Function Misuse & Edge Cases
**Result: 5 verified (definitions), 5 errors ✅**

| Test | What it tests | Expected | Actual |
|------|---------------|----------|--------|
| `test_fail_perms_wf_implies_tree_wf` | container_perms_wf alone implies tree_wf | FAIL | ✅ FAIL |
| `test_fail_tree_wf_implies_perms_wf` | tree_wf alone implies container_perms_wf | FAIL | ✅ FAIL |
| `test_fail_single_pred_implies_tree_wf` | Single sub-predicate implies full tree_wf | FAIL | ✅ FAIL |
| `test_fail_push_reverses` | Push changes first element (it doesn't) | FAIL | ✅ FAIL |
| `test_fail_two_roots` | Two different roots both satisfy tree_wf | FAIL | ✅ FAIL |

---

## Overall Assessment

### Correctness: ✅ PASS
All 12 correctness tests verify successfully. The specifications of `seq_push_lemma`, `container_perms_wf`, and `container_tree_wf` are consistent and correctly describe their intended properties.

### Completeness: ✅ PASS
All 25 completeness tests fail as expected. The specifications are tight enough to reject:
- Using properties without establishing preconditions
- Claiming overly strong postconditions
- Negating established properties
- Asserting wrong concrete values
- Cross-function misuse and invalid structural claims

### Spec Gaps Found: None
No spec gaps were identified. The specifications correctly reject all tested invalid assertions.

### Notes
- `container_tree_check_is_ancestor` is an exec function requiring `Tracked` resources, making it impractical to call directly from proof tests. Testing focused on its spec predicates and the helper `seq_push_lemma`.
- Many sub-predicates of `container_tree_wf` are `closed spec fn`, limiting external reasoning to their interface (as intended by the design).
- `container_tree_wf` and `container_perms_wf` are independent predicates — neither implies the other, which the completeness tests confirm.
