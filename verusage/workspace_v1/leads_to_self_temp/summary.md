# Adversarial Test Summary: `leads_to_self_temp.rs`

## Target Specification

The specification defines temporal logic primitives (`always`, `eventually`, `valid`) over infinite execution traces, plus two axioms (`eventually_proved_by_witness`, `execution_equality`) and a theorem (`leads_to_self_temp`: every predicate leads-to itself).

## Results

**All 9 adversarial tests FAILED verification as expected** — the specification correctly rejects every invalid property.

| # | File | Test | Type | Result |
|---|------|------|------|--------|
| 1 | `boundary_tests.rs` | `test_eventually_no_witness` | Boundary | ✅ REJECTED — cannot prove `eventually(p)` without a witness |
| 2 | `boundary_tests.rs` | `test_always_from_single_point` | Boundary | ✅ REJECTED — `p` at one suffix does not imply `always(p)` |
| 3 | `boundary_tests.rs` | `test_valid_always_arbitrary` | Boundary | ✅ REJECTED — `valid(always(p))` not provable for arbitrary `p` |
| 4 | `mutation_tests.rs` | `test_leads_to_different_predicate` | Mutation | ✅ REJECTED — `p ~> q` not provable for arbitrary distinct `q` |
| 5 | `mutation_tests.rs` | `test_leads_to_always_self` | Mutation | ✅ REJECTED — strengthened `p ~> always(p)` not provable |
| 6 | `mutation_tests.rs` | `test_implies_always_instead_of_eventually` | Mutation | ✅ REJECTED — `p ⟹ always(p)` is stronger than `p ⟹ eventually(p)` |
| 7 | `logical_tests.rs` | `test_all_executions_equal` | Logical | ✅ REJECTED — arbitrary executions are not equal |
| 8 | `logical_tests.rs` | `test_eventually_implies_always` | Logical | ✅ REJECTED — `eventually(p) ⟹ always(p)` is invalid |
| 9 | `logical_tests.rs` | `test_leads_to_symmetric` | Logical | ✅ REJECTED — `leads_to` is not symmetric |

## Conclusion

The specification is **consistent** with respect to all tested semantic boundaries:
- **Boundary**: Invalid inputs (missing witnesses, insufficient preconditions) are properly rejected.
- **Behavioral**: Mutated postconditions (wrong target, strengthened operators) are rejected.
- **Logical**: Unintended structural/global properties (execution equality, symmetry, eventually⟹always) are rejected.

No specification weaknesses were detected.
