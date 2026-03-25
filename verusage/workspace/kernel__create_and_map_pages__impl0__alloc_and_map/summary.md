# Adversarial Test Summary: `alloc_and_map`

## Target
`kernel__create_and_map_pages__impl0__alloc_and_map.rs` — Kernel page allocation and mapping.

## Results

| Test File | Total Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| `boundary_tests.rs` | 12 | 12 ✅ | 0 |
| `behavioral_mutation_tests.rs` | 12 | 12 ✅ | 0 |
| `logical_tests.rs` | 12 | 12 ✅ | 0 |
| **Total** | **36** | **36** | **0** |

## Interpretation

All 36 adversarial tests were **correctly rejected** by the specification. This means:

- **Boundary tests**: Invalid inputs (zero free pages, missing proc, zero quota, invalid VAs, unaligned pointers, out-of-range indices, pages already in closure) are all properly rejected by preconditions.
- **Behavioral mutation tests**: Incorrect output claims (unchanged free pages, missing VA in address space, unchanged quota, corrupted page mappings, wrong entry fields) are all contradicted by postconditions.
- **Logical tests**: Unintended properties (determinism, specific return values, VA=PA equality, injectivity of masking, contiguity, sorted ordering, preserved root_process, total owned page count) are all correctly not entailed by the spec.

## Conclusion

The specification for `alloc_and_map` (and `alloc_and_map_4k`) is **consistent** with respect to all 36 tested semantic queries. No evidence of specification weakness was found in these tests — the spec correctly rejects invalid inputs, incorrect behaviors, and unintended logical inferences.
