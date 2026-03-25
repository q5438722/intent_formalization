# Adversarial Proof Test Summary

**Target**: `delegation_map_v__impl3__mind_the_gap.rs`
**Specification under test**: `mind_the_gap` — three postconditions about the `gap` predicate on `StrictlyOrderedMap`.

## Results: All 9 tests FAILED verification ✅ (as expected)

The specification correctly rejects all adversarial queries — no spec weaknesses detected.

---

### Boundary Tests (3/3 rejected)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `boundary_equal_bounds_not_empty` | `gap(a, a) ⟹ ¬contains_key(k)` | FAIL ✅ |
| `boundary_end_end_not_false` | `gap(end, end) ⟹ false` | FAIL ✅ |
| `boundary_reversed_bounds_not_empty` | `hi < lo ∧ gap(lo, hi) ⟹ ¬contains_key(k)` | FAIL ✅ |

**Analysis**: Vacuously true gaps (equal, reversed, or end-end bounds) correctly yield no information about map contents.

---

### Behavioral Mutation Tests (3/3 rejected)

| Test | Mutation Applied | Result |
|------|-----------------|--------|
| `mutation_drop_overlap` | Post 1 without overlap condition `y < x` | FAIL ✅ |
| `mutation_extend_gap_left` | Post 2 reversed: extend instead of narrow | FAIL ✅ |
| `mutation_negate_membership` | Post 3 negated: key present despite gap | FAIL ✅ |

**Analysis**: Each postcondition mutation is correctly rejected. Gap merging requires overlap, narrowing cannot extend, and key exclusion cannot be negated.

---

### Logical Tests (3/3 rejected)

| Test | Unstated Property Queried | Result |
|------|--------------------------|--------|
| `logical_adjacent_gaps_no_merge` | `gap(a,b) ∧ gap(b,c) ⟹ gap(a,c)` | FAIL ✅ |
| `logical_gap_not_global_empty` | `gap(lo,hi) ⟹ ¬contains_key(k)` for arbitrary k | FAIL ✅ |
| `logical_gap_not_implies_ordering` | `gap(lo,hi) ⟹ lo < hi` | FAIL ✅ |

**Analysis**: The specification does not entail unintended logical consequences. Adjacent gaps cannot merge (boundary key b may exist), a single gap does not imply global emptiness, and gap existence does not imply bound ordering.

---

## Conclusion

The `mind_the_gap` specification is **consistent** with respect to all tested semantic queries. It correctly:
- Rejects invalid input reasoning (boundary tests)
- Rejects mutated behavioral properties (mutation tests)
- Does not entail unstated logical properties (logical tests)
