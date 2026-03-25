# Adversarial Proof Test Summary: `create_entry`

**Target**: `kernel__mem_util__impl0__create_entry.rs` — `Kernel::create_entry(&mut self, proc_ptr, va) -> (usize, PageMapPtr)`

## Results

All **36 tests** across 3 files **FAILED verification** as intended — the specification correctly rejects all adversarial queries.

| File | Tests | All Failed? |
|------|-------|-------------|
| `boundary_tests.rs` | 12 | ✅ 12/12 failed |
| `behavioral_mutation_tests.rs` | 12 | ✅ 12/12 failed |
| `logical_tests.rs` | 12 | ✅ 12/12 failed |

## Boundary Tests (precondition violations)

| # | Property Tested | Result |
|---|----------------|--------|
| 1 | `proc_ptr` not in `proc_dom` | FAIL ✅ |
| 2 | Quota `mem_4k == 2` (needs ≥ 3) | FAIL ✅ |
| 3 | Free pages == 2 (needs ≥ 3) | FAIL ✅ |
| 4 | `va = 0` (kernel addr, not valid 4k VA) | FAIL ✅ |
| 5 | VA already in address space | FAIL ✅ |
| 6 | Quota == 0 (edge case) | FAIL ✅ |
| 7 | Free pages == 0 (edge case) | FAIL ✅ |
| 8 | Misaligned VA (`0x1001`) | FAIL ✅ |
| 9 | `ret.0 == 4` when spec says `ret.0 <= 3` | FAIL ✅ |
| 10 | Quota OK but free pages insufficient | FAIL ✅ |
| 11 | Non-aligned page pointer (`ptr = 7`) | FAIL ✅ |
| 12 | Page index == `NUM_PAGES` (off-by-one) | FAIL ✅ |

## Behavioral Mutation Tests (incorrect outputs)

| # | Property Tested | Result |
|---|----------------|--------|
| 1 | Claim `ret.0 > 3` (mutated upper bound) | FAIL ✅ |
| 2 | Free pages off-by-one (`ret.0 + 1`) | FAIL ✅ |
| 3 | `proc_dom` changes (should be preserved) | FAIL ✅ |
| 4 | `thread_dom` changes (should be preserved) | FAIL ✅ |
| 5 | `container_dom` changes (should be preserved) | FAIL ✅ |
| 6 | Quota `mem_2m` changes (only `mem_4k` should) | FAIL ✅ |
| 7 | Other process's address space changes | FAIL ✅ |
| 8 | `page_mapping` ghost state changes | FAIL ✅ |
| 9 | Zero entry has `present == true` | FAIL ✅ |
| 10 | `ptr ↔ index` roundtrip broken | FAIL ✅ |
| 11 | `endpoint_dom` changes (should be preserved) | FAIL ✅ |
| 12 | Ref counter changes for mapped pages | FAIL ✅ |

## Logical Tests (unintended reasoning)

| # | Property Tested | Result |
|---|----------------|--------|
| 1 | `ret.0 >= 1` always (not guaranteed) | FAIL ✅ |
| 2 | `ret.0 == 3` always (not guaranteed) | FAIL ✅ |
| 3 | `usize2pa` is injective (it's not, due to masking) | FAIL ✅ |
| 4 | `ptr ↔ index` roundtrip for non-aligned ptr | FAIL ✅ |
| 5 | Returned `PageMapPtr` is deterministic | FAIL ✅ |
| 6 | `va_4k_valid` implies `page_ptr_valid` | FAIL ✅ |
| 7 | Quota always decreases by exactly 3 | FAIL ✅ |
| 8 | Returned `PageMapPtr` is `page_ptr_valid` | FAIL ✅ |
| 9 | PCID activity is universally preserved | FAIL ✅ |
| 10 | Returned ptr not in existing set (uniqueness) | FAIL ✅ |
| 11 | VA becomes mapped in address space (only L2 resolved) | FAIL ✅ |
| 12 | Free pages always decrease by exactly 3 | FAIL ✅ |

## Conclusion

The specification for `create_entry` correctly rejects all 36 adversarial queries. The spec:
- **Properly guards** preconditions (quota, free pages, VA validity, domain membership)
- **Correctly bounds** outputs (`ret.0 <= 3`) without over-constraining
- **Preserves** domain invariants, address spaces, and page mappings
- **Does not entail** unintended properties like determinism, injectivity, or exact page counts

No spec weaknesses detected by this test suite.
