# Test Summary: `tla_forall_not_equality`

## Target Specification
Proves De Morgan's law for temporal predicates:
```
tla_forall(|a| not(p(a))) == not(tla_exists(p))
```
i.e., ∀a. ¬P(a) ≡ ¬(∃a. P(a))

Uses two trusted axioms: `tla_forall_unfold` (unfolds universal quantifier) and `temp_pred_equality` (mutual entailment ⟹ equality).

---

## Results

All **9 tests FAILED verification** as expected — the specification correctly rejects all adversarial queries.

### Boundary Tests (3/3 failed ✅)

| Test | Target | Result |
|------|--------|--------|
| `test_boundary_unfold_without_forall_satisfied` | Call `tla_forall_unfold` when forall is NOT satisfied | FAILED — precondition rejected |
| `test_boundary_equality_one_direction` | Call `temp_pred_equality` on contradictory predicates (true vs false) | FAILED — precondition rejected |
| `test_boundary_unfold_with_exists_not_forall` | Call `tla_forall_unfold` when only exists holds, not forall | FAILED — precondition rejected |

### Behavioral Mutation Tests (3/3 failed ✅)

| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_drop_inner_negation` | `tla_forall(p) == not(tla_exists(p))` — drop inner ¬ | FAILED — assertion rejected |
| `test_mutation_drop_outer_negation` | `tla_forall(\|a\| not(p(a))) == tla_exists(p)` — drop outer ¬ | FAILED — assertion rejected |
| `test_mutation_swap_quantifiers` | `tla_exists(\|a\| not(p(a))) == not(tla_forall(p))` — other De Morgan | FAILED — assertion rejected |

### Logical Tests (3/3 failed ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_logical_equality_without_lemma` | De Morgan equality without calling the proof lemma | FAILED — not derivable without lemma |
| `test_logical_soundness` | Derive `false` after valid lemma use | FAILED — axioms are sound |
| `test_logical_forall_not_valid` | Derive `valid(tla_forall(\|a\| not(p(a))))` from equality alone | FAILED — equality ≠ validity |

---

## Conclusion

The specification is **consistent**: it rejects all tested boundary violations, behavioral mutations, and unintended logical inferences. The preconditions on the trusted axioms are sufficiently strong, the proven equality is precise (no unintended equalities hold), and the proof does not leak validity or unsoundness.
