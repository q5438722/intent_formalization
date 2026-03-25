# Adversarial Test Summary: `leads_to_always_enhance`

## Target
`source-projects/anvil-library/verified/temporal_logic/leads_to_always_enhance.rs`

The function `leads_to_always_enhance` proves a temporal logic rule:
Given `spec ⊨ □inv`, `spec ⊨ p ↝ □q1`, and `q1∧inv ⊨ q2`, conclude `spec ⊨ p ↝ □q2`.

## Results: 12/12 tests FAILED verification (as expected)

All adversarial queries were correctly rejected by the specification.

### Boundary Tests (4/4 FAILED ✅)
| Test | What was dropped/weakened | Result |
|------|--------------------------|--------|
| `boundary_drop_invariant` | Removed `spec ⊨ □inv` | FAIL ✅ |
| `boundary_drop_leads_to` | Removed `spec ⊨ p ↝ □q1` | FAIL ✅ |
| `boundary_drop_entailment` | Removed `q1∧inv ⊨ q2` | FAIL ✅ |
| `boundary_weaken_leads_to_no_always` | `p ↝ q1` instead of `p ↝ □q1` | FAIL ✅ |

**Conclusion**: All three preconditions are necessary; weakening the inner `always` also breaks the proof.

### Behavioral Mutation Tests (4/4 FAILED ✅)
| Test | Mutation applied | Result |
|------|-----------------|--------|
| `mutation_reverse_conclusion` | Reversed leads_to direction: `□q2 ↝ p` | FAIL ✅ |
| `mutation_swap_entailment` | Reversed entailment: `q2 ⊨ q1∧inv` | FAIL ✅ |
| `mutation_drop_spec_guard` | Global validity: `valid(p ↝ □q2)` | FAIL ✅ |
| `mutation_replace_inv_with_p` | Replaced `inv` with `p`: `q1∧p ⊨ q2` | FAIL ✅ |

**Conclusion**: The conclusion cannot be reversed, the entailment direction matters, the spec guard is essential, and the invariant cannot be replaced by a non-global predicate.

### Logical Tests (4/4 FAILED ✅)
| Test | Property tested | Result |
|------|----------------|--------|
| `logical_always_eventually_not_implies_eventually_always` | `□◇p → ◇□p` (non-theorem) | FAIL ✅ |
| `logical_leads_to_not_symmetric` | `p ↝ q → q ↝ p` (symmetry) | FAIL ✅ |
| `logical_eventually_conjunction_invalid` | `◇p ∧ ◇q → ◇(p∧q)` | FAIL ✅ |
| `logical_witness_not_deterministic` | `choose` returns specific index | FAIL ✅ |

**Conclusion**: The specification does not admit standard temporal logic non-theorems, does not assume leads_to symmetry, and does not expose deterministic witness selection.

## Overall Assessment

The specification of `leads_to_always_enhance` is **tight and well-formed**:
- **No missing preconditions detected** — each precondition is independently necessary.
- **No incorrect behaviors admitted** — mutated conclusions are all rejected.
- **No unintended logical consequences** — non-theorems and structural assumptions are rejected.

No specification weaknesses were found.
