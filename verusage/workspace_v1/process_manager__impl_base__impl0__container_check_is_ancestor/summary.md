# Adversarial Test Summary: `container_check_is_ancestor`

## Target Function
`container_check_is_ancestor` in `ProcessManager` — checks if `ancestor_ptr`'s subtree contains `child_ptr`, using container tree depth comparison and uppertree_seq/subtree_set membership.

## Results Overview

| Test File | Tests | Expected Failures | Actual Failures | Unexpected Passes |
|-----------|-------|-------------------|-----------------|-------------------|
| boundary_tests.rs | 6 | 6 | 6 | 0 |
| behavioral_mutation_tests.rs | 6 | 6 | 6 | 0 |
| logical_tests.rs | 6 | 6 | 6 | 0 |
| **Total** | **18** | **18** | **18** | **0** |

## Boundary Tests (6/6 FAIL ✅)

| Test | Violation | Result |
|------|-----------|--------|
| B1: ancestor not in domain | `!container_perms.dom().contains(a_ptr)` | FAIL ✅ |
| B2: child not in domain | `!container_perms.dom().contains(child_ptr)` | FAIL ✅ |
| B3: equal depth | `depth(a) == depth(child)` instead of `<` | FAIL ✅ |
| B4: ancestor deeper | `depth(a) > depth(child)` (reversed) | FAIL ✅ |
| B5: no tree_wf | Missing `container_tree_wf` precondition | FAIL ✅ |
| B6: no perms_wf | Missing `container_perms_wf` precondition | FAIL ✅ |

**Analysis**: The spec correctly rejects all invalid inputs. Preconditions on domain membership, depth ordering, and well-formedness invariants are all necessary and cannot be omitted.

## Behavioral Mutation Tests (6/6 FAIL ✅)

| Test | Mutation | Result |
|------|----------|--------|
| M1: deeper is ancestor | Claim `depth >=` implies ancestry | FAIL ✅ |
| M2: swap ancestor/child | Claim child is ancestor of parent | FAIL ✅ |
| M3: negate uppertree | subtree_set contains ⇏ uppertree_seq contains | FAIL ✅ |
| M4: equal depth = ancestor | Siblings are ancestors of each other | FAIL ✅ |
| M5: flip uppertree result | NOT in uppertree ⇒ IS in subtree | FAIL ✅ |
| M6: universal ancestry | Every pair has ancestry relation | FAIL ✅ |

**Analysis**: The spec correctly rejects all mutated postconditions. The bidirectional ensures (uppertree_seq ↔ subtree_set) prevents flipping either direction. Depth-based reasoning prevents invalid ancestry claims.

## Logical Tests (6/6 FAIL ✅)

| Test | Unintended Property | Result |
|------|---------------------|--------|
| L1: reflexivity | Container in own subtree_set | FAIL ✅ |
| L2: symmetry | Ancestry is symmetric | FAIL ✅ |
| L3: transitivity | Ancestry is transitive | FAIL ✅ |
| L4: uppertree indexing | Ancestor at specific index | FAIL ✅ |
| L5: sibling disjointness | Same-depth nodes have disjoint subtrees | FAIL ✅ |
| L6: leaf subtree empty | No children ⇒ empty subtree_set | FAIL ✅ |

**Analysis**: The spec does not entail any of these structural tree properties. This is primarily because key invariants (`container_subtree_set_wf`, `container_childern_parent_wf`, `container_subtree_set_exclusive`, etc.) are defined as `closed spec fn` with `external_body`, making their contents opaque to the verifier. Properties like transitivity and sibling disjointness are reasonable tree invariants but cannot be derived from the exposed specification surface.

## Conclusion

The specification for `container_check_is_ancestor` is **consistent** with respect to all 18 adversarial queries tested:
- **Boundary correctness**: All preconditions are necessary; removing any one prevents verification.
- **Behavioral correctness**: No incorrect input-output relations are admitted.
- **Logical containment**: No unintended structural properties are derivable.

The closed nature of the tree well-formedness specs (`container_subtree_set_wf`, etc.) limits what can be proven externally — this is a design choice that prevents unintended reasoning but also means properties like transitivity require explicit lemmas.
