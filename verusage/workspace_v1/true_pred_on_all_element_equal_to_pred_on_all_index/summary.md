# Adversarial Test Summary: `true_pred_on_all_element_equal_to_pred_on_all_index`

## Target
`true_pred_on_all_element_equal_to_pred_on_all_index<A>(s, pred)`: proves that for any sequence `s` and predicate `pred`, the containment-based universality `(∀ obj. s.contains(obj) → pred(obj))` is equivalent to the index-based universality `(∀ i. 0 ≤ i < s.len() → pred(s[i]))`.

- **Requires**: (none)
- **Ensures**: `(∀ obj. s.contains(obj) → pred(obj)) ⟺ (∀ i. 0 ≤ i < s.len() → pred(s[i]))`

## Results

All **10 adversarial tests FAILED verification** as expected, confirming the specification correctly rejects invalid inputs, mutated behaviors, and unintended logical inferences.

### Boundary Tests (3/3 rejected ✅)
| Test | Failure Mode | Result |
|------|-------------|--------|
| `test_boundary_empty_seq_universal_pred` | Empty seq makes biconditional vacuously true; tries to derive `∀x. pred(x)` | **assertion failed** ✅ |
| `test_boundary_reverse_containment_implication` | Tries to reverse implication: `pred(x) → s.contains(x)` | **assertion failed** ✅ |
| `test_boundary_false_pred_nonempty_seq` | Always-false pred on non-empty seq; tries to derive index-side is true | **assertion failed** ✅ |

### Behavioral Mutation Tests (3/3 rejected ✅)
| Test | Mutation | Result |
|------|----------|--------|
| `test_mutation_stronger_predicate` | Calls lemma with `x>0`, tries to derive `x>5` for all elements | **assertion failed** ✅ |
| `test_mutation_different_sequence` | Calls lemma with `s1`, tries to derive result for different `s2` containing `-1` | **assertion failed** ✅ |
| `test_mutation_break_biconditional` | Seq contains `-2` (fails pred); tries to assert index-side is true | **assertion failed** ✅ |

### Logical Tests (4/4 rejected ✅)
| Test | Unintended Property | Result |
|------|---------------------|--------|
| `test_logical_no_cross_sequence_transfer` | pred on `s` does NOT transfer to `s.push(-1)` | **assertion failed** ✅ |
| `test_logical_no_sequence_equality_from_pred` | Two sequences satisfying same pred are NOT necessarily equal | **assertion failed** ✅ |
| `test_logical_pred_not_universally_derivable` | pred on contained elements does NOT imply `pred(-1)` | **assertion failed** ✅ |
| `test_logical_no_pred_implication_across_calls` | Calling lemma with two preds on same seq does NOT yield `pred1 → pred2` | **assertion failed** ✅ |

## Conclusion

The specification for `true_pred_on_all_element_equal_to_pred_on_all_index` is **consistent** with respect to all tested adversarial queries:
- **Boundary conditions** are handled correctly: vacuous truth on empty sequences does not leak, and the containment guard cannot be reversed or bypassed.
- **Behavioral mutations** are detected: the equivalence cannot be transferred to different predicates or sequences, and the biconditional cannot be broken.
- **Logical boundaries** are sound: the spec does not entail cross-sequence transfer, sequence equality from predicate behavior, universal predicate claims, or inter-predicate implications.

No specification weaknesses were identified.
