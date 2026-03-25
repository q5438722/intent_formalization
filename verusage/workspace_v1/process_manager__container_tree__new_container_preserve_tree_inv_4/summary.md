# Test Summary: `new_container_preserve_tree_inv_4`

## Target
`process_manager__container_tree__new_container_preserve_tree_inv_4.rs`
- **Function**: `new_container_preserve_tree_inv_4`
- **Requires**: `new_container_ensures(root, old_perms, new_perms, container_ptr, new_container_ptr)`
- **Ensures**: `container_subtree_set_wf(root, new_perms)`

---

## Results

| File | Tests | All Failed (as expected) |
|------|-------|------------------------|
| `boundary_tests.rs` | 5 | ✅ Yes (5 errors) |
| `behavioral_mutation_tests.rs` | 5 | ✅ Yes (5 errors) |
| `logical_tests.rs` | 5 | ✅ Yes (5 errors) |

**All 15 adversarial tests correctly fail verification**, confirming the specification rejects invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Boundary Tests (5/5 FAIL ✅)

| Test | Violated Precondition | Result |
|------|----------------------|--------|
| `boundary_test_no_preconditions` | No preconditions assumed at all | FAIL ✅ |
| `boundary_test_new_container_already_exists` | `new_container_ptr` already in old domain | FAIL ✅ |
| `boundary_test_parent_not_in_domain` | `container_ptr` not in old domain | FAIL ✅ |
| `boundary_test_children_full` | Children list at max capacity | FAIL ✅ |
| `boundary_test_depth_overflow` | Parent depth ≥ `usize::MAX` | FAIL ✅ |

## Behavioral Mutation Tests (5/5 FAIL ✅)

| Test | Mutated Property | Result |
|------|-----------------|--------|
| `behavioral_test_negated_postcondition` | Negation of `container_subtree_set_wf` | FAIL ✅ |
| `behavioral_test_wrong_depth` | New container depth = 0 (should be ≥ 1) | FAIL ✅ |
| `behavioral_test_full_tree_wf` | Full `container_tree_wf` (only `subtree_set_wf` guaranteed) | FAIL ✅ |
| `behavioral_test_perms_unchanged` | Old perms = new perms (domain grew) | FAIL ✅ |
| `behavioral_test_nonempty_children` | New container has children (should be empty) | FAIL ✅ |

## Logical Tests (5/5 FAIL ✅)

| Test | Unguaranteed Property | Result |
|------|----------------------|--------|
| `logical_test_depth_bound` | All containers have depth ≤ 1 | FAIL ✅ |
| `logical_test_children_parent_wf_not_guaranteed` | `container_childern_parent_wf` on new perms | FAIL ✅ |
| `logical_test_uppertree_wf_not_guaranteed` | `container_uppertree_seq_wf` on new perms | FAIL ✅ |
| `logical_test_determinism` | Two valid transitions produce identical results | FAIL ✅ |
| `logical_test_linkedlist_wf_not_guaranteed` | `containers_linkedlist_wf` on new perms | FAIL ✅ |

---

## Notable Findings During Testing

### 1. Unused Parameter in `container_subtree_set_wf`
The `root_container` parameter in `container_subtree_set_wf` is **not used** in its body. Initial tests asserting `container_subtree_set_wf(new_container_ptr, new_perms)` and `container_subtree_set_wf(container_ptr, new_perms)` both **passed** (unexpectedly), confirming the first argument is semantically irrelevant. This is a spec design observation — the function signature suggests root-dependence but the body is root-agnostic.

### 2. Implicitly Preserved Invariants
Initial tests showed that `container_root_wf` and `container_subtree_set_exclusive` **can be derived** from `new_container_ensures` + the postcondition, even though they are not explicitly stated as ensures. This means the function's specification is **stronger than its declared postcondition** — it implicitly preserves these additional tree invariants.

### 3. Non-Deterministic Specification
The `new_container_ensures` predicate does **not** fully determine `new_container_perms`. Fields like `owned_procs`, `owned_endpoints`, `owned_threads`, `quota`, `owned_cpus`, `scheduler`, `can_have_children`, and `root_process` are **unconstrained** by the transition spec. This is by design (they're orthogonal to tree structure) but means the spec is a **relation**, not a function.

### 4. Three Invariant Components NOT Preserved
The function only ensures `container_subtree_set_wf`. The following invariant components of `container_tree_wf` on `new_perms` are **not provable** from this function alone:
- `container_childern_parent_wf` — bidirectional parent-child consistency
- `container_uppertree_seq_wf` — uppertree sequence internal consistency  
- `containers_linkedlist_wf` — linked list structural consistency

These are presumably established by companion functions (`new_container_preserve_tree_inv_1`, `_2`, `_3`, etc.).
