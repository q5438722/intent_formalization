# Test Summary: `spec_entails_always_tla_forall`

## Target
`source-projects/anvil-library/verified/temporal_logic/spec_entails_always_tla_forall.rs`

The specification encodes a TLA-style temporal logic rule:
> If `spec` entails `always(P(a))` for all `a`, then `spec` entails `always(∀a. P(a))`.

It relies on two `external_body` axioms: `spec_entails_tla_forall` (universal introduction for entailment) and `tla_forall_always_equality_variant` (commutativity of `always` and `∀`).

---

## Results: All 9 tests FAILED verification ✅

Every adversarial test was correctly rejected, indicating the specification's semantic boundaries are well-guarded.

### Boundary Tests (3/3 rejected)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_boundary_weakened_precondition` | Calls `spec_entails_always_tla_forall` with `spec.entails(P(a))` instead of `spec.entails(always(P(a)))` | ❌ precondition not satisfied |
| 2 | `test_boundary_partial_precondition` | Provides precondition only for `a=0`, not `forall a` | ❌ precondition not satisfied |
| 3 | `test_boundary_one_direction_only` | Calls `tla_forall_always_equality_variant` with one-directional entailment instead of bidirectional | ❌ precondition not satisfied |

### Behavioral Mutation Tests (3/3 rejected)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_mutation_drop_spec` | Asserts `valid(always(∀a. P(a)))` instead of `spec.entails(always(∀a. P(a)))` — drops spec constraint | ❌ assertion failed |
| 2 | `test_mutation_reverse_entailment` | Asserts `always(∀a. P(a)).entails(spec)` — reverses entailment direction | ❌ assertion failed |
| 3 | `test_mutation_upgrade_without_always` | From `spec.entails(P(a))` (no `always`), asserts `spec.entails(always(∀a. P(a)))` | ❌ assertion failed |

### Logical Tests (3/3 rejected)

| # | Test | Failure Mode | Result |
|---|------|-------------|--------|
| 1 | `test_logical_entails_not_symmetric` | Tests if `p.entails(q)` implies `q.entails(p)` — entailment symmetry | ❌ assertion failed |
| 2 | `test_logical_valid_from_entails` | Tests if `spec.entails(p)` implies `valid(p)` — stronger universal claim | ❌ assertion failed |
| 3 | `test_logical_no_always_promotion` | Tests if `spec.entails(p)` implies `spec.entails(always(p))` — temporal strengthening | ❌ assertion failed |

---

## Conclusion

The specification correctly:
- **Rejects invalid inputs**: Missing `always`, partial quantification, and one-directional equivalence are all caught.
- **Rejects incorrect behaviors**: Strengthening the conclusion (dropping `spec`, reversing direction, upgrading without `always`) all fail.
- **Rejects unintended reasoning**: Symmetry of entailment, deriving universal validity from conditional entailment, and temporal promotion without justification are all rejected.

No specification weaknesses were detected. The external_body axioms and the main function's specification appear consistent and appropriately constrained.
