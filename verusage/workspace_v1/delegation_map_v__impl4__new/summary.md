# Adversarial Test Summary: `delegation_map_v__impl4__new`

## Target
`DelegationMap::new(k_zero, id_zero)` — constructs a delegation map where every key maps to the same endpoint.

**Preconditions**: `k_zero == K::zero_spec()`, `id_zero@.valid_physical_address()` (id length < 0x100000)
**Postconditions**: `s.valid()`, `s@ == Map::total(|k| id_zero@)`

## Results

All 9 tests **failed verification** as expected, confirming the specification correctly rejects each unintended property.

### Boundary Tests (3/3 FAIL ✓)

| Test | Property Challenged | Result |
|------|-------------------|--------|
| `bt_address_at_exact_boundary` | `valid_physical_address` holds at len == 0x100000 | FAIL ✓ |
| `bt_address_above_boundary` | `valid_physical_address` holds at len == 0x200000 | FAIL ✓ |
| `bt_total_map_invalid_endpoint` | Total map with invalid endpoint yields valid addresses | FAIL ✓ |

### Behavioral Mutation Tests (3/3 FAIL ✓)

| Test | Property Challenged | Result |
|------|-------------------|--------|
| `bmt_wrong_value` | Total map lookup returns a different endpoint | FAIL ✓ |
| `bmt_nonuniform_mapping` | Two keys in constant total map yield different values | FAIL ✓ |
| `bmt_domain_missing_key` | Total map's domain does not contain a given key | FAIL ✓ |

### Logical Tests (3/3 FAIL ✓)

| Test | Property Challenged | Result |
|------|-------------------|--------|
| `lt_different_maps_equal` | Total maps with distinct endpoints are extensionally equal | FAIL ✓ |
| `lt_stronger_address_bound` | `valid_physical_address` implies a much stronger bound (< 0x100) | FAIL ✓ |
| `lt_total_map_finite_domain` | Total map has a finite domain | FAIL ✓ |

## Conclusion

The specification of `DelegationMap::new` is **consistent** with respect to all tested adversarial queries:
- **Boundary**: The `valid_physical_address` guard correctly enforces the strict `< 0x100000` bound and rejects boundary-violating endpoints.
- **Behavioral**: The `Map::total` postcondition correctly ensures uniform mapping, full domain, and correct values.
- **Logical**: The spec does not accidentally entail stronger bounds, finite domains, or equality of distinct maps.

No specification weaknesses were detected.
