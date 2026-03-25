# Test Summary: `empty_key_range_is_consistent`

## Specification Under Test

```rust
proof fn empty_key_range_is_consistent(&self, lo, hi, id)
    requires lo.geq_spec(*hi),       // range [lo, hi) is empty
    ensures  self.range_consistent(lo, hi, id),  // vacuously true
```

When `lo >= hi`, the range `[lo, hi)` contains no keys, so `range_consistent` holds trivially for any map and any destination `id`.

---

## Results: **9/9 tests correctly FAILED** ✅

The specification rejects all adversarial queries — no weaknesses detected.

### Boundary Tests (3/3 failed)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `boundary_test_lo_less_than_hi` | Call with `lo < hi` (explicit precondition violation) | ❌ precondition not satisfied |
| `boundary_test_some_vs_none` | Call with `lo=Some(k)`, `hi=None` (None is ∞, so lo < hi) | ❌ precondition not satisfied |
| `boundary_test_no_precondition` | Call with no guarantee on lo/hi ordering | ❌ precondition not satisfied |

### Behavioral Mutation Tests (3/3 failed)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `behavioral_test_negate_postcondition` | Assert `!range_consistent` after valid call | ❌ assertion failed |
| `behavioral_test_arbitrary_key_maps_to_id` | Assert `dm@[k] == id@` for arbitrary key k | ❌ assertion failed (+ recommendation: key not in domain) |
| `behavioral_test_nonempty_range_consistent` | Assert `range_consistent` for non-empty range | ❌ assertion failed |

### Logical Tests (3/3 failed)

| Test | Failure Mode | Result |
|------|-------------|--------|
| `logical_test_vacuous_id_uniqueness` | Assert `id1@ == id2@` from two vacuous calls | ❌ assertion failed |
| `logical_test_range_extension` | Extend empty-range consistency to larger range | ❌ assertion failed |
| `logical_test_map_totality` | Assert all keys exist in map domain | ❌ assertion failed |

---

## Conclusion

The specification for `empty_key_range_is_consistent` is **consistent** with respect to all tested adversarial properties:

- **Precondition guard is tight**: Invalid inputs (lo < hi) are correctly rejected at the call site.
- **Postcondition is not over-promising**: The vacuous `range_consistent` does not leak information about arbitrary key lookups.
- **No unintended logical entailments**: Vacuous truth from empty ranges cannot be exploited to derive ID equality, range extension, or map totality.
