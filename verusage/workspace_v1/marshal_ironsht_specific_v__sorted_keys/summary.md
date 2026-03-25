# Adversarial Proof Test Summary: `sorted_keys`

## Target Specification
- `ckeykvlt(a, b)`: strict key comparison (`a.k.ukey < b.k.ukey`)
- `spec_sorted_keys(v)`: all consecutive pairs satisfy `ckeykvlt`
- `sorted_keys(v)`: exec function, postcondition `res == spec_sorted_keys(*v)`

## Results: All 13 tests FAILED verification ✅

The specification correctly rejects all adversarial queries — no unintended properties are entailed.

---

### Boundary Tests (4/4 failed ✅)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| B1 | `test_boundary_equal_keys_sorted` | `ckeykvlt(kv, kv)` for all kv | FAIL ✅ — strict `<` correctly rejects reflexive comparison |
| B2 | `test_boundary_descending_pair` | `a.ukey > b.ukey ==> ckeykvlt(a, b)` | FAIL ✅ — descending order correctly rejected |
| B3 | `test_boundary_max_u64_sorted` | `a.ukey == MAX ==> ∃b. ckeykvlt(a, b)` | FAIL ✅ — no u64 value exceeds MAX |
| B4 | `test_boundary_two_equal_elements_sorted` | Equal-key 2-element vector is sorted | FAIL ✅ — equal keys violate strict `<` |

### Behavioral Mutation Tests (4/4 failed ✅)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| M1 | `test_mutation_reverse_ordering` | `ckeykvlt(a,b) ==> ckeykvlt(b,a)` | FAIL ✅ — strict `<` is antisymmetric |
| M2 | `test_mutation_geq_implies_lt` | `a.ukey >= b.ukey ==> ckeykvlt(a,b)` | FAIL ✅ — greater-or-equal does not imply less-than |
| M3 | `test_mutation_sorted_reported_unsorted` | Sorted 2-elem vector is NOT sorted | FAIL ✅ — correctly sorted vectors are recognized |
| M4 | `test_mutation_unsorted_claimed_sorted` | Descending 2-elem vector IS sorted | FAIL ✅ — unsorted vectors are rejected |

### Logical Tests (5/5 failed ✅)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| L1 | `test_logical_sorted_implies_nonempty` | `sorted(v) ==> len > 0` | FAIL ✅ — empty vec is vacuously sorted |
| L2 | `test_logical_reflexivity` | `ckeykvlt(a, a)` | FAIL ✅ — strict `<` is irreflexive |
| L3 | `test_logical_first_key_is_zero` | `sorted(v) ∧ len>0 ==> v[0].ukey == 0` | FAIL ✅ — no absolute value constraint |
| L4 | `test_logical_totality` | `ckeykvlt(a,b) ∨ ckeykvlt(b,a)` | FAIL ✅ — equal keys satisfy neither direction |
| L5 | `test_logical_sorted_implies_empty_values` | `sorted(v) ==> v[0].v is empty` | FAIL ✅ — sorting is key-only, no value constraint |

---

## Conclusion

The specification for `sorted_keys` is **consistent** across all three query categories:
- **Boundary**: Invalid inputs (equal keys, descending, MAX overflow) are properly rejected
- **Behavioral**: Mutated input-output relations are properly rejected
- **Logical**: Unintended inferences (reflexivity, totality, value constraints, absolutevalue constraints) are properly rejected

No specification weaknesses were detected. The `ckeykvlt` strict ordering and `spec_sorted_keys` consecutive-pair formulation correctly bound the semantic space.
