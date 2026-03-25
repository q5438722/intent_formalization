# Adversarial Test Summary: `DelegationMap::set`

## Target Specification

`DelegationMap::set(lo, hi, dst)` updates a delegation map so that all keys in `[lo, hi)` map to `dst`, while keys outside that range remain unchanged.

**Preconditions**: `old(self).valid()`, `dst@.valid_physical_address()`
**Postconditions**: `self.valid()`, keys in range → `dst@`, keys outside range → unchanged

---

## Results: All 15 adversarial tests FAIL verification ✅

The specification correctly rejects all invalid inputs, incorrect behaviors, and unintended logical reasoning.

### Boundary Tests (5/5 FAIL ✅)

| Test | Target | Result |
|------|--------|--------|
| B1: Address at limit (0x100000) | `valid_physical_address` rejects `len == 0x100000` | FAIL ✅ |
| B2: End iterator lt self | `lt_spec(None, None)` is false | FAIL ✅ |
| B3: Between with equal bounds | `between(ki, ki, ki)` is false (reflexivity) | FAIL ✅ |
| B4: Zero lt zero | `zero.cmp_spec(zero).lt()` is false (eq holds) | FAIL ✅ |
| B5: Hi excluded from range | `between(lo, hi, hi)` is false (exclusive upper bound) | FAIL ✅ |

### Behavioral Mutation Tests (5/5 FAIL ✅)

| Test | Target | Result |
|------|--------|--------|
| M1: In-range key → wrong value | Spec constrains in-range keys to `dst` | FAIL ✅ |
| M2: Out-of-range key changed | Spec preserves out-of-range keys | FAIL ✅ |
| M3: Valid map is invalid | Direct contradiction of `valid()` | FAIL ✅ |
| M4: Lo boundary → old value | `lo` is in `[lo, hi)`, must map to `dst` | FAIL ✅ |
| M5: Empty address is invalid | `len(0) < 0x100000` holds, so address IS valid | FAIL ✅ |

### Logical Tests (5/5 FAIL ✅)

| Test | Target | Result |
|------|--------|--------|
| L1: Set not deterministic | Postconditions fully determine output → deterministic | FAIL ✅ |
| L2: Empty range changes key | `lo >= hi` → between is vacuously false → no changes | FAIL ✅ |
| L3: Valid → all keys same | `valid()` doesn't imply uniform mapping | FAIL ✅ |
| L4: Set affects all keys | Only `[lo, hi)` is affected, not all keys | FAIL ✅ |
| L5: lt_spec not total | `cmp_properties` guarantees totality for distinct keys | FAIL ✅ |

## Conclusion

The specification for `DelegationMap::set` is **consistent** with respect to all 15 adversarial queries:
- **Boundary completeness**: Preconditions correctly reject invalid edge cases
- **Behavioral precision**: Postconditions precisely constrain both in-range and out-of-range behavior
- **Logical soundness**: No unintended properties (non-determinism, universal effects, incomparability) are derivable

No spec weaknesses were detected.
