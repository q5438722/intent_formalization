# Test Summary: `delegation_map_v__impl3__values_agree`

## Target Function
`StrictlyOrderedMap::values_agree(&self, lo: usize, hi: usize, v: &ID) -> (bool, bool)`

**Preconditions:** `self.valid()`, `0 <= lo <= hi < self.keys@.len()`

**Postconditions:**
- `ret.0 == forall |i| lo <= i <= hi ==> self.vals@[i]@ == v@`
- `!ret.0 ==> (ret.1 == (self.vals@[hi]@ != v@ && forall |i| lo <= i < hi ==> self.vals@[i]@ == v@))`

---

## Results: All 9 tests FAILED verification (as expected)

### Boundary Tests (3/3 failed ✅)

| Test | Violation | Result |
|------|-----------|--------|
| `test_boundary_lo_gt_hi` | `lo=1 > hi=0` violates `lo <= hi` | FAIL ✅ |
| `test_boundary_hi_at_len` | `hi == keys.len()` violates `hi < keys.len()` | FAIL ✅ |
| `test_boundary_not_valid` | `!self.valid()` violates `self.valid()` | FAIL ✅ |

**Conclusion:** All three preconditions are properly enforced. Invalid inputs are correctly rejected.

### Behavioral Mutation Tests (3/3 failed ✅)

| Test | Mutation | Result |
|------|----------|--------|
| `test_behavioral_ret0_always_true` | Assert `ret.0` is always true | FAIL ✅ |
| `test_behavioral_weaken_range` | Assert `ret.0` tracks `[lo, hi)` instead of `[lo, hi]` | FAIL ✅ |
| `test_behavioral_flip_ret1` | Assert `ret.1` means `vals[hi] == v` (flipped) | FAIL ✅ |

**Conclusion:** The postconditions are tight enough to reject mutated behaviors. The spec correctly distinguishes `<=` vs `<` ranges and the sign of `ret.1`.

### Logical Tests (3/3 failed ✅)

| Test | Property Tested | Result |
|------|----------------|--------|
| `test_logical_ret1_when_all_match` | `ret.0 ==> ret.1` (ret.1 constrained when all match) | FAIL ✅ |
| `test_logical_all_values_match` | `ret.0` implies ALL values match (not just range) | FAIL ✅ |
| `test_logical_ret1_implies_ret0` | `ret.1 ==> ret.0` (cross-field implication) | FAIL ✅ |

**Conclusion:** The spec does not over-specify. Notably:
- `ret.1` is intentionally unconstrained when `ret.0 == true` (the implementation returns `(true, true)` but the spec allows any `ret.1` value)
- The range-based guarantee correctly does not extend to the entire sequence
- `ret.1 == true` does not imply `ret.0 == true` (ret.1 can be true in the failing case when `vals[hi] != v`)

---

## Overall Assessment

The specification for `values_agree` is **well-constrained**:
- All preconditions are necessary and enforced
- Postconditions reject incorrect behavioral mutations
- No unintended logical properties are entailed

**One observation:** The spec intentionally leaves `ret.1` unconstrained when `ret.0 == true`. The implementation always returns `(true, true)` in this case, but the spec does not require this. This is a deliberate under-specification that could be tightened if the caller depends on `ret.1` being true when all values agree.
