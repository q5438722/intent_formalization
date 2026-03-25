# Adversarial Proof Test Summary

## Target: `marshal_v__impl3__lemma_serialize_injective.rs`

### Specification Under Test

The `Marshalable` trait defines serialization injectivity:
- **Preconditions**: `self.is_marshalable()`, `other.is_marshalable()`, `self.ghost_serialize() == other.ghost_serialize()`
- **Postcondition**: `self.view_equal(other)`

Implementations: `u64`, `usize`, `Option<T>`, `(T, U)`

---

## Results Summary

| File | Tests | All Failed? | Spec Weakness Found? |
|------|-------|-------------|---------------------|
| `boundary_tests.rs` | 5 | ✅ Yes (5/5 errors) | No |
| `behavioral_mutation_tests.rs` | 5 | ✅ Yes (5/5 errors) | No |
| `logical_tests.rs` | 5 | ✅ Yes (5/5 errors) | No |

**Total: 15/15 tests correctly rejected** — the specification is consistent with respect to the tested properties.

---

## Boundary Tests (5 tests — all FAIL ✅)

| Test | Description | Failure Mode |
|------|-------------|-------------|
| B1 | `0u64` vs `1u64` — different serializations | Precondition violated: `ghost_serialize` equality |
| B2 | `None` vs `Some(0u64)` — different serializations | Precondition violated: `ghost_serialize` equality |
| B3 | `Some(100u64)` vs `Some(200u64)` — different inner values | Precondition violated: `is_marshalable` (cannot verify `ghost_serialize().len()`) |
| B4 | `(1,2)` vs `(3,4)` — different tuples | Precondition violated: `is_marshalable` (cannot verify `ghost_serialize().len()`) |
| B5 | `0u64` vs `u64::MAX` — extreme boundary values | Precondition violated: `ghost_serialize` equality |

**Observation**: B3/B4 fail on `is_marshalable()` rather than `ghost_serialize` equality because Verus cannot automatically resolve `spec_u64_to_le_bytes(...).len()` to prove the length bound in `Option`/tuple's `is_marshalable`. This is a consequence of the trait-level `ghost_serialize` being `external_body` — even though the impls are `open spec fn`, Verus struggles to compose the length reasoning through the trait boundary.

---

## Behavioral Mutation Tests (5 tests — all FAIL ✅)

| Test | Description | Failure Mode |
|------|-------------|-------------|
| M1 | Negate `view_equal` for equal u64s | Assertion failed: `!a.view_equal(&b)` |
| M2 | Negate `view_equal` for `None`/`None` | Assertion failed: `!a.view_equal(&b)` |
| M3 | Claim `None` serializes as `[1]` instead of `[0]` | Assertion failed: wrong serialization |
| M4 | Claim `Some` tag byte is `0` instead of `1` | Assertion failed: wrong tag byte |
| M5 | Claim equal u64 views differ (`a@ !== b@`) | Assertion failed: views are equal |

---

## Logical Tests (5 tests — all FAIL ✅)

| Test | Description | Failure Mode |
|------|-------------|-------------|
| L1 | Assert `0u64.view_equal(1u64)` (false equality) | Assertion failed |
| L2 | Assert `None.view_equal(Some(0))` (cross-variant) | Assertion failed |
| L3 | Assert `0u64` and `1u64` have same serialization | Assertion failed |
| L4 | Derive `view_equal` from serialization equality without calling lemma (Option) | Assertion failed |
| L5 | Derive component serialization equality from tuple serialization equality without lemma | Assertion failed |

**Key finding (L4, L5)**: The specification correctly requires explicit invocation of `lemma_serialize_injective` — Verus cannot derive injectivity automatically from the `open spec fn` definitions alone. This confirms the lemma serves a genuine proof obligation.

---

## Conclusions

1. **Precondition enforcement**: All invalid inputs (different serializations, non-marshalable values) are correctly rejected.
2. **Postcondition integrity**: All mutations of the ensures clause (`view_equal`) are correctly rejected.
3. **Logical boundaries**: No unintended reasoning is derivable — injectivity cannot be proved without explicitly invoking the lemma.
4. **No spec weakness detected**: The specification is consistent across all 15 adversarial queries.

### Note on `is_marshalable` Verification

Tests B3, B4 revealed that Verus cannot automatically prove `is_marshalable()` for `Option<u64>` or `(u64, u64)` in proof mode, because the proof requires knowing `spec_u64_to_le_bytes(x).len() == 8`, which is not directly available without additional axioms or lemmas about `spec_u64_to_le_bytes`. This is not a spec *weakness* (it doesn't admit invalid reasoning), but rather a gap in automation — the spec is *safe* but potentially harder to use than intended.
