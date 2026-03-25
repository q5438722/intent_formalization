# Adversarial Test Summary: `unpack_conditions_from_spec`

## Target Specification
```
unpack_conditions_from_spec<T>(spec, c, p, q):
  requires: valid(stable(spec)), spec.and(c).entails(p.leads_to(q))
  ensures:  spec.entails(p.and(c).leads_to(q))
```

## Results: ALL 9 TESTS FAILED (as expected)

The specification correctly rejects all adversarial queries across all three categories.

---

### Boundary Tests (3/3 FAILED ✓)

| Test | What it probes | Result |
|------|---------------|--------|
| `boundary_test_missing_stability` | Calls function without `valid(stable(spec))` | FAILED — precondition not satisfied |
| `boundary_test_missing_entailment` | Calls function without `spec.and(c).entails(p.leads_to(q))` | FAILED — precondition not satisfied |
| `boundary_test_both_missing` | Asserts postcondition with zero preconditions | FAILED — assertion failed |

**Conclusion**: Both preconditions are independently necessary. The spec does not admit invalid inputs.

---

### Behavioral Mutation Tests (3/3 FAILED ✓)

| Test | What it probes | Result |
|------|---------------|--------|
| `mutation_test_drop_c_from_antecedent` | Asserts `spec.entails(p.leads_to(q))` — drops `c` from the leads_to antecedent | FAILED — assertion failed |
| `mutation_test_swap_p_q` | Asserts `spec.entails(q.and(c).leads_to(p))` — swaps p and q | FAILED — assertion failed |
| `mutation_test_strengthen_consequent` | Asserts `spec.entails(p.and(c).leads_to(q.and(c)))` — strengthens consequent with `c` | FAILED — assertion failed |

**Conclusion**: The spec precisely characterizes the output. It does not allow dropping conditions, reversing the leads_to direction, or strengthening the consequent.

---

### Logical Tests (3/3 FAILED ✓)

| Test | What it probes | Result |
|------|---------------|--------|
| `logical_test_converse` | From conclusion `spec ⊨ (p∧c)~>q`, tries to derive premise `(spec∧c) ⊨ p~>q` | FAILED — assertion failed |
| `logical_test_arbitrary_target` | Tries to substitute an arbitrary predicate `r` for `q` in the conclusion | FAILED — assertion failed |
| `logical_test_stronger_temporal` | Asserts `spec.entails(p.and(c).leads_to(always(q)))` — q holds forever, not just eventually | FAILED — assertion failed |

**Conclusion**: The spec does not support converse reasoning, arbitrary substitution, or temporal strengthening. No unintended logical inferences are possible.

---

## Overall Assessment

**The specification is consistent.** All 9 adversarial tests were correctly rejected:
- **Boundary**: Invalid inputs are properly guarded by preconditions.
- **Behavioral**: Incorrect output mutations are not derivable.
- **Logical**: No unintended properties (converse, universality, temporal strengthening) are entailed.

No spec weaknesses were found.
