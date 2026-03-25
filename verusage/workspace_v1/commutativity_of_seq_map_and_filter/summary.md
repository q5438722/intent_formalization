# Adversarial Test Summary: `commutativity_of_seq_map_and_filter`

## Specification Under Test

```rust
proof fn commutativity_of_seq_map_and_filter<A, B>(s, pred, pred_on_mapped, map)
    requires forall |i| 0 <= i < s.len() ==> pred(s[i]) == pred_on_mapped(map(s[i])),
    ensures  s.map_values(map).filter(pred_on_mapped) == s.filter(pred).map_values(map),
```

## Results: All 11 tests FAILED verification ✅

The specification correctly rejects all adversarial queries — no weaknesses detected.

---

### Boundary Tests (3/3 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `boundary_predicates_fully_disagree` | Predicates disagree on ALL elements (pred=positive, pred_on_mapped=negative) | ❌ precondition not satisfied |
| `boundary_partial_disagreement` | Predicates disagree on ONE element (map shifts values breaking compatibility) | ❌ precondition not satisfied |
| `boundary_map_negates_values` | Map negates values making predicate compatibility impossible | ❌ precondition not satisfied |

**Conclusion**: The `requires` clause correctly rejects incompatible predicate/map combinations at all call sites.

---

### Behavioral Mutation Tests (4/4 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `mutation_filter_is_noop` | Assert filter has no effect (map.filter == map) | ❌ assertion failed |
| `mutation_length_preserved` | Assert filtered length == original length | ❌ assertion failed |
| `mutation_different_map_same_result` | Assert result with map_fn×2 == result with map_fn×3 | ❌ assertion failed |
| `mutation_result_is_empty` | Assert result equals empty seq when all elements pass filter | ❌ assertion failed |

**Conclusion**: The postcondition is precise enough to reject mutated output relations — incorrect behaviors cannot be derived.

---

### Logical Tests (4/4 failed ✅)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `logical_commutativity_without_lemma` | Assert postcondition without calling the lemma | ❌ assertion failed |
| `logical_different_preds_same_filter` | Assert different predicates produce same filter result | ❌ assertion failed |
| `logical_extend_to_pushed_element` | Assert lemma result extends to a larger sequence without re-proving | ❌ assertion failed |
| `logical_map_not_injective` | Assert map is injective (1²≠(-1)² is false) | ❌ assertion failed |

**Conclusion**: The specification does not leak unintended logical consequences — commutativity is scoped to its proven instance and does not generalize beyond the precondition.

---

## Overall Assessment

The specification for `commutativity_of_seq_map_and_filter` is **consistent**:
- **Boundary**: Invalid inputs (incompatible predicates) are rejected by the precondition.
- **Behavioral**: Incorrect output mutations (no-filter, wrong-length, wrong-map, empty-result) are rejected.
- **Logical**: Unintended reasoning (no-proof commutativity, predicate equivalence, scope extension, injectivity) cannot be derived.
