# Test Summary: `pass_endpoint`

## Target
`process_manager__impl_base__impl0__pass_endpoint.rs` — passes an endpoint reference from a source thread's descriptor to a destination thread's descriptor, incrementing the endpoint's reference count.

## Results

All **18 adversarial tests FAILED verification** as expected (6 per category). The specification correctly rejects all tested invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (6/6 failed ✅)
| Test | Violated Precondition | Result |
|------|----------------------|--------|
| B1 | `src_thread_ptr` not in `thread_dom` | FAIL ✅ |
| B2 | `src_thread_ptr == dst_thread_ptr` | FAIL ✅ |
| B3 | `src_endpoint_index == MAX_NUM_ENDPOINT_DESCRIPTORS` (off-by-one) | FAIL ✅ |
| B4 | Source endpoint descriptor is `None` | FAIL ✅ |
| B5 | `rf_counter == usize::MAX` | FAIL ✅ |
| B6 | Destination endpoint descriptor is `Some` | FAIL ✅ |

### Behavioral Mutation Tests (6/6 failed ✅)
| Test | Mutated Property | Result |
|------|-----------------|--------|
| M1 | dst descriptor still `None` after update | FAIL ✅ |
| M2 | `owning_threads` not updated (no insert) | FAIL ✅ |
| M3 | endpoint queue changed | FAIL ✅ |
| M4 | `thread_dom` changed | FAIL ✅ |
| M5 | some process modified | FAIL ✅ |
| M6 | some container modified | FAIL ✅ |

### Logical Tests (6/6 failed ✅)
| Test | Unguaranteed Property | Result |
|------|----------------------|--------|
| L1 | src descriptor removed after pass | FAIL ✅ |
| L2 | `rf_counter` incremented by exactly 1 | FAIL ✅ |
| L3 | `endpoint_dom` strictly grew | FAIL ✅ |
| L4 | dst thread state changed | FAIL ✅ |
| L5 | endpoint `owning_container` changed | FAIL ✅ |
| L6 | all endpoint `rf_counter`s unchanged | FAIL ✅ |

## Conclusion

The `pass_endpoint` specification is **consistent** across all three query dimensions:
- **Boundary**: Invalid inputs are correctly rejected by preconditions.
- **Behavioral**: Incorrect output mutations are rejected by postconditions.
- **Logical**: Unintended inferences (e.g., rf_counter increment, src descriptor removal, state changes) are not entailed by the specification.

**Notable observation**: The spec does not propagate `rf_counter` increment or `owning_container` preservation for the modified endpoint in its postconditions (tests L2 and L5). These properties are guaranteed internally by `endpoint_add_ref`, but not exposed in `pass_endpoint`'s ensures clause. This is a design choice — the spec is minimally sufficient rather than maximally informative.
