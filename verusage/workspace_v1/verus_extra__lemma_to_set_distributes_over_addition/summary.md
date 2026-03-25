# Adversarial Test Summary: `lemma_to_set_distributes_over_addition`

## Specification Under Test

```rust
pub proof fn lemma_to_set_distributes_over_addition<A>(s: Seq<A>, t: Seq<A>)
    ensures (s+t).to_set() == s.to_set() + t.to_set()
```

**Key characteristics**: No preconditions. The postcondition states that converting a concatenated sequence to a set equals the union of the individual sets.

---

## Test Results

All **9 adversarial tests** correctly **FAILED** verification, meaning the specification properly rejects all invalid queries.

### Boundary Tests (3/3 FAILED ✅)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `test_boundary_empty_concat_has_element` | Empty + empty → result contains element | FAIL ✅ |
| `test_boundary_nonempty_element_excluded` | Element from s is NOT in result | FAIL ✅ |
| `test_boundary_phantom_element` | Result contains element not in either seq | FAIL ✅ |

Since the spec has **no preconditions**, boundary tests probe edge-case inputs with false assertions. The spec correctly rejects all three: empty sets remain empty, elements from inputs are preserved, and phantom elements cannot appear.

### Behavioral Mutation Tests (3/3 FAILED ✅)

| Test | Mutation Applied | Result |
|------|-----------------|--------|
| `test_mutation_intersection_instead_of_union` | Replace union with intersection | FAIL ✅ |
| `test_mutation_negate_postcondition` | Negate the ensures clause | FAIL ✅ |
| `test_mutation_missing_element_from_t` | Assert element from t absent in result | FAIL ✅ |

The spec correctly distinguishes union from intersection, cannot be negated, and preserves all elements from both operands.

### Logical Tests (3/3 FAILED ✅)

| Test | Logical Property Queried | Result |
|------|-------------------------|--------|
| `test_logical_derive_false` | Derive `false` from the spec | FAIL ✅ |
| `test_logical_seq_commutativity` | Set equality ⇒ sequence equality | FAIL ✅ |
| `test_logical_union_collapses_to_one_operand` | Union equals only one operand's set | FAIL ✅ |

The spec is **sound** (cannot derive `false`), does **not** conflate set-level commutativity with sequence-level commutativity, and correctly requires both operands to contribute to the union.

---

## Conclusion

The specification for `lemma_to_set_distributes_over_addition` is **consistent** with respect to all tested adversarial queries. It:

1. **Rejects invalid boundary inputs**: Edge cases (empty seqs, phantom elements) are properly handled.
2. **Rejects behavioral mutations**: The postcondition is precise — intersection, negation, and element omission are all rejected.
3. **Rejects unintended logical inferences**: The spec does not allow deriving `false`, does not conflate set and sequence equality, and does not permit collapsing the union to a single operand.

**No specification weaknesses were detected.**
