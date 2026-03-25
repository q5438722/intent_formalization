# Adversarial Test Summary: `always_to_always_later.rs`

## Target
Temporal logic theorem: if `spec ⊨ □p` then `spec ⊨ □(◇p)` (always implies always-later).

## Results: 12/12 tests FAILED verification ✅

All tests correctly rejected — the specification is consistent across all probed dimensions.

---

### Boundary Tests (4/4 failed) — `boundary_tests.rs`

| Test | Failure Mode | Verus Error |
|------|-------------|-------------|
| `test_missing_entails_precondition` | Call `always_to_always_later` without `spec.entails(always(p))` | precondition not satisfied |
| `test_propagate_without_always` | Call `always_propagate_forwards` without `always(p).satisfied_by(ex)` | precondition not satisfied |
| `test_entails_trans_partial_precondition` | Call `entails_trans` missing `q.entails(r)` | precondition not satisfied |
| `test_execution_equality_no_evidence` | Call `execution_equality` without pointwise equality | precondition not satisfied |

**Conclusion**: All preconditions are necessary — no function can be invoked with invalid inputs.

---

### Behavioral Mutation Tests (4/4 failed) — `behavioral_mutation_tests.rs`

| Test | Mutation | Verus Error |
|------|----------|-------------|
| `test_reversed_entailment` | Reversed: `always(later(p)).entails(spec)` instead of `spec.entails(always(later(p)))` | postcondition not satisfied |
| `test_wrong_predicate` | Swapped predicate: `always(later(q))` instead of `always(later(p))` | postcondition not satisfied |
| `test_later_implies_always` | Claimed `later(p) ⊨ always(p)` (next step ≠ all steps) | postcondition not satisfied |
| `test_p_implies_always_p` | Claimed `p ⊨ always(p)` (now ≠ forever) | postcondition not satisfied |

**Conclusion**: The spec correctly distinguishes the precise entailment relationship — mutated outputs are all rejected.

---

### Logical Tests (4/4 failed) — `logical_tests.rs`

| Test | Property Tested | Verus Error |
|------|----------------|-------------|
| `test_converse_always_later_to_always` | Converse: `□◇p ⊨ □p` (fails: missing step 0) | postcondition not satisfied |
| `test_arbitrary_valid` | Universal validity: `valid(p)` for any `p` | postcondition not satisfied |
| `test_entails_does_not_lift_to_always` | Lifting: `p⊨q` does NOT imply `p⊨□q` | postcondition not satisfied |
| `test_suffix_position_independence` | Position transfer: `p@a` does NOT imply `p@b` | postcondition not satisfied |

**Conclusion**: The spec does not admit unintended logical inferences — no vacuous validity, no false lifting, no position confusion.

---

## Overall Assessment

The specification for `always_to_always_later` is **well-constrained**:
- **Preconditions** are tight — all required conditions are enforced
- **Postconditions** are precise — no over-approximation detected
- **Axioms** (`external_body` functions) do not introduce inconsistencies
- **Logical boundaries** are correct — converse, lifting, and independence properties are properly excluded
