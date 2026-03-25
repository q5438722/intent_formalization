# Adversarial Test Summary: `vec_filter` (vec_lib.rs)

## Specification Under Test

```rust
fn vec_filter<V>(v: Vec<V>, f: impl Fn(&V)->bool, f_spec: spec_fn(V)->bool) -> (r: Vec<V>)
    requires
        forall|v: V| f.requires((&v,)),
        forall|v:V, r:bool| f.ensures((&v,), r) ==> f_spec(v) == r,
    ensures
        r@.to_multiset() =~= v@.to_multiset().filter(f_spec)
```

The postcondition guarantees **multiset equality** between the result and the filtered input — correct element counts but **not** element ordering.

---

## Results: All 9 tests FAILED verification ✅

Every adversarial property was correctly rejected by the spec.

### Boundary Tests (3/3 rejected)

| # | Test | Assertion | Result |
|---|------|-----------|--------|
| 1 | Empty input → non-empty result | `r.len() > 0` | ❌ FAIL |
| 2 | Result longer than input | `r.len() > v.len()` | ❌ FAIL |
| 3 | All-rejecting predicate → non-empty result | `r.len() > 0` | ❌ FAIL |

**Conclusion**: The spec correctly constrains result size at all boundaries.

### Behavioral Mutation Tests (3/3 rejected)

| # | Test | Assertion | Result |
|---|------|-----------|--------|
| 1 | All match but claim empty result | `r.len() == 0` | ❌ FAIL |
| 2 | Partial match but claim all kept | `r.len() == 3` (should be 2) | ❌ FAIL |
| 3 | Wrong element multiplicity | `r.to_multiset().count(5) == 2` (should be 3) | ❌ FAIL |

**Conclusion**: The spec correctly rejects mutated output behaviors (wrong lengths, wrong counts).

### Logical Tests (3/3 rejected)

| # | Test | Assertion | Result |
|---|------|-----------|--------|
| 1 | Order preservation | `r =~= v.filter(pred)` | ❌ FAIL |
| 2 | Determinism (two calls equal) | `r1 =~= r2` | ❌ FAIL |
| 3 | First element determined | `r[0] == 3` | ❌ FAIL |

**Conclusion**: The spec correctly does NOT entail order preservation, determinism, or index-level guarantees — all of which are unintended stronger properties.

---

## Spec Weakness Analysis

The logical tests reveal that the specification is **intentionally weaker** than a typical sequence-level filter:

- **No order preservation**: The multiset postcondition admits any permutation of matching elements.
- **No determinism**: Multiple valid results exist for the same inputs.
- **No index guarantees**: Individual element positions are unconstrained.

These are **design choices**, not bugs — the multiset formulation is sufficient for many use cases and is easier to verify. However, if the developer intends order-preserving behavior (as the implementation actually provides), the spec is **incomplete**: it allows permuted results that the implementation would never produce.
