# Test Summary: `choose_gap_violator` Specification Consistency

**Target**: `delegation_map_v__impl3__choose_gap_violator.rs`
**Function under test**: `StrictlyOrderedMap::choose_gap_violator`
**Spec**: Requires `!self.gap(lo, hi)` (∃ key between lo and hi in map). Ensures returned `r` satisfies `lo < r < hi ∧ map[r.key] ∈ dom(map)`.

---

## Results: ALL 9 TESTS FAILED (as expected ✅)

The specification correctly rejects all adversarial queries.

### Boundary Tests (precondition violations) — 3/3 FAILED ✅

| Test | Failure Mode | Result | Error |
|------|-------------|--------|-------|
| `test_boundary_gap_holds` | Gap explicitly holds → precondition `!gap` violated | **FAILED** | precondition not satisfied |
| `test_boundary_lo_is_end` | `lo = None` → `lt_spec(None, _)` always false → gap vacuously true | **FAILED** | precondition not satisfied |
| `test_boundary_empty_map` | Map has no keys → gap always true for any range | **FAILED** | precondition not satisfied |

### Behavioral Mutation Tests (negated postconditions) — 3/3 FAILED ✅

| Test | Failure Mode | Result | Error |
|------|-------------|--------|-------|
| `test_behavioral_not_in_map` | Assert key NOT in map (negated membership) | **FAILED** | assertion failed |
| `test_behavioral_not_after_lo` | Assert `¬(lo < r)` (negated lower bound) | **FAILED** | assertion failed |
| `test_behavioral_not_before_hi` | Assert `¬(r < hi)` (negated upper bound) | **FAILED** | assertion failed |

### Logical Tests (unguaranteed properties) — 3/3 FAILED ✅

| Test | Failure Mode | Result | Error |
|------|-------------|--------|-------|
| `test_logical_unique_violator` | Assert result equals arbitrary other violator (uniqueness) | **FAILED** | assertion failed |
| `test_logical_stronger_ordering` | Assert result < midpoint (specific positioning) | **FAILED** | assertion failed |
| `test_logical_gap_before_result` | Assert gap(lo, r) holds (minimality — r is first key after lo) | **FAILED** | assertion failed |

---

## Conclusion

The `choose_gap_violator` specification is **consistent** with respect to all tested adversarial queries:

1. **Input boundaries are enforced**: Invalid calls (gap holds, end iterators, empty maps) are correctly rejected by the precondition `!self.gap(lo, hi)`.
2. **Output relations are tight**: The postcondition correctly constrains the result — negating any conjunct (membership, lower bound, upper bound) is rejected.
3. **No unintended logical entailments**: The spec does not over-commit — it does not guarantee uniqueness of the violator, specific positioning within the range, or minimality (first key after lo).
