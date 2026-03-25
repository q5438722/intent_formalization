# Adversarial Test Summary: `delegation_map_v__impl3__new`

## Target Specification

`StrictlyOrderedVec::new()` and `StrictlyOrderedMap::new()` constructors with postconditions ensuring empty views and validity (`sorted` + `no_duplicates` for keys, empty `Map` for the map).

## Results: All 9 tests FAILED verification ✅

All adversarial properties were correctly rejected by the specification.

### Boundary Tests (3/3 rejected)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `bt_reversed_pair_is_sorted` | Reversed elements (cmp = Greater) satisfy `sorted` | ❌ Rejected |
| `bt_equal_pair_is_sorted` | Equal elements (cmp = Equal) satisfy `sorted` | ❌ Rejected |
| `bt_empty_seq_has_positive_length` | Empty sequence has len > 0 | ❌ Rejected |

### Behavioral Mutation Tests (3/3 rejected)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `bmt_empty_map_contains_key` | Empty map contains an arbitrary key | ❌ Rejected |
| `bmt_empty_map_has_element` | Empty map has at least one element (∃k) | ❌ Rejected |
| `bmt_two_empty_maps_differ` | Two empty maps are not extensionally equal | ❌ Rejected |

### Logical Tests (3/3 rejected)

| Test | Property Queried | Result |
|------|-----------------|--------|
| `lt_sorted_implies_no_duplicates` | `sorted` alone implies `no_duplicates` | ❌ Rejected |
| `lt_cmp_transitivity` | `cmp_spec` is transitive (not axiomatized) | ❌ Rejected |
| `lt_sorted_does_not_bound_length` | `sorted` + `no_duplicates` bounds length < 100 | ❌ Rejected |

## Conclusion

The specification correctly rejects all tested adversarial properties:
- **Boundary**: Invalid inputs (non-sorted, non-strict orderings) are properly rejected by `sorted`.
- **Behavioral**: The `Map::empty()` postcondition prevents false claims about map contents.
- **Logical**: The spec does not entail unintended properties—`cmp_spec` transitivity is not assumed, `sorted` does not imply `no_duplicates`, and no spurious length bounds exist.

No spec weaknesses were detected by these tests.
