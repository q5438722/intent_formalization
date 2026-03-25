# Adversarial Test Summary: `leads_to_apply`

## Target
`source-projects/anvil-library/verified/temporal_logic/leads_to_apply.rs`

## Specification Under Test
`leads_to_apply(spec, p, q)`: Given `spec ⊨ p` and `spec ⊨ p ~> q`, concludes `spec ⊨ ◇q`.

---

## Results: ALL 15 TESTS FAILED (as expected)

The specification correctly rejects all adversarial queries. No weaknesses found.

### Boundary Tests (5/5 rejected) — `boundary_tests.rs`

| # | Test | Missing Precondition | Verus Error |
|---|------|---------------------|-------------|
| 1 | `test_boundary_missing_entails_p` | `spec.entails(p)` | precondition not satisfied |
| 2 | `test_boundary_missing_leads_to` | `spec.entails(p.leads_to(q))` | precondition not satisfied |
| 3 | `test_boundary_implies_apply_missing_p` | `p.satisfied_by(ex)` | precondition not satisfied |
| 4 | `test_boundary_leads_to_unfold_no_prereq` | `p.leads_to(q).satisfied_by(ex)` | precondition not satisfied |
| 5 | `test_boundary_exec_equality_no_pointwise` | pointwise equality | precondition not satisfied |

**Conclusion**: All preconditions are necessary — removing any one causes verification failure.

### Behavioral Mutation Tests (5/5 rejected) — `behavioral_mutation_tests.rs`

| # | Test | Mutation | Verus Error |
|---|------|----------|-------------|
| 1 | `test_mutation_always_instead_of_eventually` | `◇q` → `□q` | postcondition not satisfied |
| 2 | `test_mutation_entails_q_directly` | `◇q` → `q` | postcondition not satisfied |
| 3 | `test_mutation_unrelated_predicate` | `◇q` → `◇r` (unrelated) | postcondition not satisfied |
| 4 | `test_mutation_valid_instead_of_entails` | `spec ⊨ ◇q` → `⊨ ◇q` | postcondition not satisfied |
| 5 | `test_mutation_always_eventually` | `◇q` → `□◇q` | postcondition not satisfied |

**Conclusion**: The specification is tight — strengthening, swapping, or generalizing the conclusion is rejected.

### Logical Tests (5/5 rejected) — `logical_tests.rs`

| # | Test | Unintended Property | Verus Error |
|---|------|---------------------|-------------|
| 1 | `test_logical_valid_from_entails` | `spec ⊨ p` ⇒ `⊨ p` (universal validity) | postcondition not satisfied |
| 2 | `test_logical_leads_to_symmetry` | `p ~> q` ⇒ `q ~> p` (symmetry) | postcondition not satisfied |
| 3 | `test_logical_always_from_entails` | `spec ⊨ p` ⇒ `spec ⊨ □p` (persistence) | postcondition not satisfied |
| 4 | `test_logical_immediate_from_eventual` | `spec ⊨ ◇q` ⇒ `spec ⊨ q` (immediacy) | postcondition not satisfied |
| 5 | `test_logical_determinism` | `spec(ex₁) ∧ spec(ex₂)` ⇒ `ex₁ = ex₂` | postcondition not satisfied |

**Conclusion**: No unintended logical properties are derivable — entailment is properly scoped, temporal modalities are not conflated, and no implicit structural assumptions leak.

---

## Overall Assessment

**The specification for `leads_to_apply` is consistent.** All 15 adversarial queries across three categories (boundary, behavioral mutation, logical) were correctly rejected by Verus. The specification:

1. **Requires all preconditions** — no precondition is redundant or bypassable
2. **Provides a tight postcondition** — no stronger conclusion is derivable
3. **Prevents unintended reasoning** — temporal operators, entailment scope, and execution identity are all properly constrained
