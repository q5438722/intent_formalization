# Adversarial Test Execution Summary

**Target**: `in_subtree_imply_exist_in_child` — proves that if `s_ptr` is in the subtree of `c_ptr`, then either `s_ptr` is a direct child, or some child of `c_ptr` has `s_ptr` in its subtree.

**Result**: All 15 adversarial tests **FAILED verification** as expected → specification correctly rejects all invalid queries.

| Verification | Count |
|---|---|
| Verified (spec + stub) | 6 |
| Errors (adversarial tests) | 15 |

---

## Boundary Tests (5/5 rejected ✅)

| Test | Violation | Error Type |
|---|---|---|
| B1: Missing `container_perms_wf` | Omit permissions well-formedness | precondition not satisfied |
| B2: Missing `container_tree_wf` | Omit tree well-formedness | precondition not satisfied |
| B3: `c_ptr` not in domain | `!container_perms.dom().contains(c_ptr)` | precondition not satisfied |
| B4: `s_ptr` not in subtree | `!subtree_set@.contains(s_ptr)` | precondition not satisfied |
| B5: All preconditions missing | Bare call with no assumptions | precondition not satisfied |

**Conclusion**: All four preconditions are independently enforced. No invalid input state can bypass the specification.

---

## Behavioral Mutation Tests (5/5 rejected ✅)

| Test | Mutation | Error Type |
|---|---|---|
| M1: Always direct child | Assert `children@.contains(s_ptr)` unconditionally | assertion failed |
| M2: Never direct child | Assert `!children@.contains(s_ptr)` | assertion failed |
| M3: Negate conclusion | Deny both disjuncts | assertion failed (×2) |
| M4: Child equals s_ptr | Assert witness child IS s_ptr when not direct child | assertion failed |
| M5: In all children's subtrees | Assert s_ptr in EVERY child's subtree | assertion failed |

**Conclusion**: The disjunctive postcondition is tight — neither branch can be universally forced or denied, and the existential witness cannot be constrained to an incorrect identity.

---

## Logical Tests (5/5 rejected ✅)

| Test | Property Tested | Error Type |
|---|---|---|
| L1: Subtree symmetry | `s_ptr.subtree_set.contains(c_ptr)` | assertion failed |
| L2: Equal depth | `depth(s_ptr) == depth(c_ptr)` | assertion failed |
| L3: c_ptr is root | `c_ptr == root_container` | assertion failed |
| L4: Depth exactly +1 | `depth(s_ptr) == depth(c_ptr) + 1` | assertion failed |
| L5: Not in root's subtree | `!root.subtree_set.contains(s_ptr)` when `c_ptr == root` | assertion failed |

**Conclusion**: The specification does not entail spurious structural claims — subtree membership is asymmetric, depth relationships are not over-constrained, and the lemma applies to any ancestor (not just root).

---

## Overall Assessment

The specification for `in_subtree_imply_exist_in_child` is **consistent**: it correctly rejects all 15 adversarial queries spanning boundary violations, behavioral mutations, and logical over-claims. No unintended entailments were detected.
