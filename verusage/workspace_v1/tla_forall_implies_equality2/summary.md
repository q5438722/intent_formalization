# Adversarial Test Summary: `tla_forall_implies_equality2`

## Target
`tla_forall_implies_equality2` — proves that universal quantification distributes over implication:
`∀a.(P → Q(a)) ≡ P → (∀a.Q(a))`

The specification relies on three `external_body` axioms:
1. `temp_pred_equality` — extensionality for temporal predicates
2. `a_to_temp_pred_equality` — extensionality for predicate-valued functions
3. `tla_forall_or_equality` — ∀ distributes over ∨ when one disjunct is constant

---

## Results: All 12 tests FAILED verification ✅

### Boundary Tests (4/4 rejected)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `boundary_test_contradictory_predicates` | Call `temp_pred_equality(true, false)` — violates both entailment directions | ✅ Precondition rejected |
| `boundary_test_one_way_entailment` | Call with `p.entails(q)` true but `q.entails(p)` false | ✅ Precondition rejected |
| `boundary_test_nonequivalent_function_families` | Call `a_to_temp_pred_equality` with always-true vs always-false | ✅ Precondition rejected |
| `boundary_test_nonuniversal_entailment` | Call with `state(0)==0` vs `state(0)>=0` — only one-way entailment | ✅ Precondition rejected |

### Behavioral Mutation Tests (4/4 rejected)

| Test | Mutation | Result |
|------|----------|--------|
| `mutation_swap_implies_direction` | `∀a.(Q(a)→P) == P→(∀a.Q(a))` — swapped implies direction | ✅ Postcondition rejected |
| `mutation_forall_to_instance` | `∀a.(P→Q(a)) == P→Q(0)` — replaced ∀ with single instance | ✅ Postcondition rejected |
| `mutation_negate_antecedent` | `∀a.(P→Q(a)) == ¬P→(∀a.Q(a))` — negated antecedent on RHS | ✅ Postcondition rejected |
| `mutation_implies_to_or` | `∀a.(P∨Q(a)) == P→(∀a.Q(a))` — replaced → with ∨ on LHS | ✅ Postcondition rejected |

### Logical Tests (4/4 rejected)

| Test | Property Tested | Result |
|------|----------------|--------|
| `logical_test_implies_not_commutative` | `P→Q == Q→P` — implies is NOT commutative | ✅ Postcondition rejected |
| `logical_test_one_way_entailment_not_equality` | `P⊨Q ⟹ P==Q` — one-way entailment ≠ equality | ✅ Postcondition rejected |
| `logical_test_forall_not_instance` | `∀a.Q(a) == Q(0)` — ∀ ≠ single instance | ✅ Postcondition rejected |
| `logical_test_valid_not_contradiction` | `valid(P) ⟹ valid(¬P)` — validity of negation | ✅ Postcondition rejected |

---

## Conclusion

The specification is **consistent** with respect to all tested adversarial properties:

- **Preconditions are tight**: Invalid inputs to `temp_pred_equality` and `a_to_temp_pred_equality` are correctly rejected. Both directions of entailment are enforced.
- **Behavioral integrity holds**: The main lemma's equality cannot be replaced by semantically incorrect variants (swapped direction, single instance, negated antecedent, wrong operator).
- **No unintended logical consequences**: The spec does not allow unsound reasoning such as implies-commutativity, one-way equality, or ∀-to-instance collapse.

No weaknesses were found in the specification under these 12 adversarial queries.
