# Test Summary: `keys_in_index_range_agree` Specification Consistency

**Target**: `delegation_map_v__impl3__keys_in_index_range_agree.rs`
**Function under test**: `StrictlyOrderedMap::keys_in_index_range_agree`
**Spec**: Requires `self.valid() ∧ 0 ≤ lo ≤ hi < keys.len()`. Ensures `ret.0 ↔ ∀i∈[lo,hi]. map[keys[i]]@ == v@`, and when `¬ret.0`: `ret.1 ↔ (map[keys[hi]]@ ≠ v@ ∧ ∀i∈[lo,hi). map[keys[i]]@ == v@)`.

---

## Results: ALL 9 TESTS FAILED (as expected ✅)

The specification correctly rejects all adversarial queries.

### Boundary Tests (precondition violations) — 3/3 FAILED ✅

| Test | Failure Mode | Result | Error |
|------|-------------|--------|-------|
| `test_boundary_lo_gt_hi` | lo=2 > hi=1 → violates `lo ≤ hi` | **FAILED** | precondition not satisfied |
| `test_boundary_hi_eq_len` | hi=3 == len → violates `hi < len` | **FAILED** | precondition not satisfied |
| `test_boundary_empty_seq` | empty seq, hi=0 ≥ len=0 → violates `hi < len` | **FAILED** | precondition not satisfied |

### Behavioral Mutation Tests (negated postconditions) — 3/3 FAILED ✅

| Test | Failure Mode | Result | Error |
|------|-------------|--------|-------|
| `test_behavioral_negate_agreement` | All vals match v, assert ¬ret.0 (negated agreement) | **FAILED** | assertion failed |
| `test_behavioral_false_agreement` | vals[hi]≠v, assert ret.0 (false positive agreement) | **FAILED** | assertion failed |
| `test_behavioral_negate_almost` | "Almost" holds (only hi disagrees), assert ¬ret.1 | **FAILED** | assertion failed |

### Logical Tests (unguaranteed properties) — 3/3 FAILED ✅

| Test | Failure Mode | Result | Error |
|------|-------------|--------|-------|
| `test_logical_agreement_extends` | agree([lo,hi]) ⇒ agree([lo,hi+1]) (range extension) | **FAILED** | assertion failed |
| `test_logical_ret1_unspecified_when_agree` | ret.0 ⇒ ret.1 (ret.1 unconstrained when ret.0 is true) | **FAILED** | assertion failed |
| `test_logical_cross_function_agree` | values_agree ≡ keys_in_index_range_agree on different vals | **FAILED** | assertion failed |

---

## Conclusion

The `keys_in_index_range_agree` specification is **consistent** with respect to all tested adversarial queries:

1. **Input boundaries are enforced**: Invalid calls (reversed range, off-by-one, empty sequence) are correctly rejected by the precondition `0 ≤ lo ≤ hi < keys.len()`.
2. **Output relations are tight**: The postconditions correctly constrain both `ret.0` (full agreement) and `ret.1` (almost-agreement) — negating or inverting either is rejected.
3. **No unintended logical entailments**: The spec does not over-commit:
   - It does not guarantee range extension (agreeing on [lo,hi] says nothing about hi+1).
   - It leaves `ret.1` unconstrained when `ret.0` is true (the guard `¬ret.0` is necessary).
   - It does not equate `values_agree` and `keys_in_index_range_agree` without the `map_valid` invariant bridging direct value access and map-based key lookups.
