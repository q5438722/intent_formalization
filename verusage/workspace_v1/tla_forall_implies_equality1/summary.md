# Test Results Summary: `tla_forall_implies_equality1`

## Target
Temporal logic theorem: `∀a.(p(a) ⟹ q) == (∃a.p(a)) ⟹ q`  
File: `source-projects/anvil-library/verified/temporal_logic/tla_forall_implies_equality1.rs`

## Results

| Category | Tests | All Failed (as expected) |
|---|---|---|
| Boundary Tests | 5 | ✅ Yes (5/5 rejected) |
| Behavioral Mutation Tests | 5 | ✅ Yes (5/5 rejected) |
| Logical Tests | 7 | ✅ Yes (7/7 rejected) |
| **Total** | **17** | ✅ **All 17 correctly rejected** |

## Boundary Tests (`boundary_tests.rs`)
| ID | Description | Result |
|---|---|---|
| BT1 | One-direction entailment (p≠q but only one direction) | ✅ FAILED |
| BT2 | One-direction pointwise function entailment | ✅ FAILED |
| BT3 | Structural vs semantic equality without axiom call | ✅ FAILED |
| BT4 | Contradictory predicates: `true.entails(false)` | ✅ FAILED |
| BT5 | `valid(tla_forall(p))` for unsatisfiable universal | ✅ FAILED |

## Behavioral Mutation Tests (`behavioral_mutation_tests.rs`)
| ID | Mutation | Result |
|---|---|---|
| BMT1 | RHS: `tla_exists → tla_forall` | ✅ FAILED |
| BMT2 | Body: swap `p(a)⟹q` to `q⟹p(a)` | ✅ FAILED |
| BMT3 | LHS: `tla_forall → tla_exists` | ✅ FAILED |
| BMT4 | `tla_forall_or_equality`: RHS `forall → exists` | ✅ FAILED |
| BMT5 | De Morgan: LHS `forall → exists` | ✅ FAILED |

## Logical Tests (`logical_tests.rs`)
| ID | Property Tested | Result |
|---|---|---|
| LT1 | Arbitrary predicates not equal | ✅ FAILED |
| LT2 | Cannot derive `false` (axiom consistency) | ✅ FAILED |
| LT3 | `valid(p)` does not imply `valid(not(p))` | ✅ FAILED |
| LT4 | `entails` is not symmetric | ✅ FAILED |
| LT5 | Quantifier type mismatch rejected | ✅ FAILED |
| LT6 | Cannot cross-instantiate theorem with different q | ✅ FAILED |
| LT7 | No implicit function extensionality | ✅ FAILED |

## Conclusion

The specification is **consistent** against all 17 adversarial queries:
- **Precondition boundaries** are properly enforced — invalid inputs are rejected.
- **Behavioral mutations** (wrong quantifiers, reversed implications) are all caught.
- **Logical soundness** holds — no contradictions derivable, no unintended entailments, no spurious equalities.

The `external_body` axioms (`temp_pred_equality`, `a_to_temp_pred_equality`, `tla_forall_not_equality`, `tla_forall_or_equality`) appear sound within the tested scope. The proven theorem `tla_forall_implies_equality1` correctly establishes its equality without admitting any of the tested unintended properties.
