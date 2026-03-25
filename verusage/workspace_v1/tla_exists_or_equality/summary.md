# Adversarial Test Summary: `tla_exists_or_equality.rs`

## Target Specification
The target proves that existential quantification distributes over disjunction in temporal logic:
```
tla_exists(|a: A| a_to_p(a).or(q)) == tla_exists(a_to_p).or(q)
```
It relies on two `external_body` axioms:
- `tla_exists_proved_by_witness`: witness satisfaction → existential satisfaction
- `temp_pred_equality`: mutual entailment → structural equality

## Test Results

**Total: 11 tests, 11 correctly rejected (FAILED verification)**

### Boundary Tests (3/3 rejected ✅)

| ID   | Description | Result |
|------|-------------|--------|
| BT-1 | False witness violates `tla_exists_proved_by_witness` requires | ✅ Rejected (precondition not satisfied) |
| BT-2 | One-way entailment violates `temp_pred_equality` requires | ✅ Rejected (precondition not satisfied) |
| BT-3 | Assert existence from always-false predicate | ✅ Rejected (assertion failed) |

### Behavioral Mutation Tests (3/3 rejected ✅)

| ID   | Description | Result |
|------|-------------|--------|
| BM-1 | Drop `.or(q)` from RHS of main theorem | ✅ Rejected (assertion failed) |
| BM-2 | Equate `tla_exists(p)` with `tla_exists(|a| p(a).or(q))` | ✅ Rejected (assertion failed) |
| BM-3 | Assert `valid(tla_exists(always_false))` | ✅ Rejected (assertion failed) |

### Logical Tests (5/5 rejected ✅)

| ID   | Description | Result |
|------|-------------|--------|
| LT-1 | Equality of clearly different predicates without mutual entailment | ✅ Rejected (assertion failed) |
| LT-2 | `tla_exists` distributes over `implies` (invalid law) | ✅ Rejected (assertion failed) |
| LT-3 | Existence implies specific non-witness satisfies predicate | ✅ Rejected (assertion failed) |
| LT-4 | Structural or-idempotence without calling `temp_pred_equality` | ✅ Rejected (assertion failed) |
| LT-5 | `choose_witness` outside `recommends` scope yields valid witness | ✅ Rejected (assertion failed + recommendation not met) |

## Conclusion

The specification is **robust against all 11 adversarial queries**:

1. **Preconditions are tight**: Both `external_body` axioms correctly reject calls that violate their `requires` clauses. Invalid witnesses and incomplete entailment proofs are properly blocked.

2. **Behavioral mutations are rejected**: The spec does not admit mutated versions of the main theorem. Dropping the `or(q)` term or asserting validity of always-false existentials are properly caught.

3. **Unintended logical inferences are blocked**: The spec does not allow:
   - Deriving equality without mutual entailment proof
   - Distributing `tla_exists` over `implies` (a logically invalid law)
   - Concluding a specific value satisfies a predicate from mere existence
   - Structural equality without invoking the extensionality axiom
   - Extracting meaningful witnesses when no witness exists

**No spec weaknesses detected.** The specification correctly constrains its semantic boundary across all three test categories.
