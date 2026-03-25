# Adversarial Test Summary: `kernel__create_and_map_pages__impl0__range_alloc_and_map`

## Target Functions
- `create_entry_and_alloc_and_map` — allocates up to 4 pages and maps a single VA
- `range_alloc_and_map` — iterates over a VA range, calling the above for each entry
- `page_ptr2page_index` / `page_index2page_ptr` — page pointer ↔ index conversions
- `usize2page_entry` / `usize2page_entry_perm` / `usize2pa` — raw value decoders
- `va_4k_valid` — VA validity check

## Results Overview

| Category | Tests | Failed (Expected) | Passed (Unexpected) |
|---|---|---|---|
| Boundary | 12 | 12 | 0 |
| Behavioral Mutation | 12 | 12 | 0 |
| Logical | 12 | 12 | 0 |
| **Total** | **36** | **36** | **0** |

All 36 adversarial tests were correctly **rejected** by the specification.

## Boundary Tests (12/12 FAILED ✓)
Tests that violate `requires` clauses or use edge-case values:

| # | Test | Violated Precondition |
|---|---|---|
| 1 | Unaligned page pointer (ptr=1) | `ptr % 0x1000 == 0` |
| 2 | Page index at max (i=NUM_PAGES) | `i < NUM_PAGES` |
| 3 | Page index overflow (usize::MAX) | `i < NUM_PAGES` |
| 4 | Insufficient quota (mem_4k=3) | `mem_4k >= 4` |
| 5 | Insufficient free pages (3) | `free_pages >= 4` |
| 6 | VA already mapped | `dom.contains(va) == false` |
| 7 | Range quota shortfall (39 < 40) | `mem_4k >= 4 * len` |
| 8 | Range free pages shortfall (19 < 20) | `free_pages >= 4 * len` |
| 9 | Range length overflow | `len * 4 < usize::MAX` |
| 10 | Partial range overlap | VA already in domain |
| 11 | Proc not in domain | `proc_dom.contains(ptr)` |
| 12 | VA=0 not valid | `spec_va_4k_valid(0)` fails |

## Behavioral Mutation Tests (12/12 FAILED ✓)
Tests that mutate expected outputs/relations:

| # | Test | Mutated Property |
|---|---|---|
| 1 | Zero entry has present=true | Should be false |
| 2 | Zero entry has addr≠0 | Should be 0 |
| 3 | Zero perm has write=true | Should be false |
| 4 | Zero perm has execute_disable=true | Should be false |
| 5 | Zero perm has user=true | Should be false |
| 6 | Claim ret.0 > 4 | Ensures ret.0 ≤ 4 |
| 7 | Quota mem_2m also changes | Only mem_4k changes |
| 8 | Other proc's addr space changes | Must be preserved |
| 9 | ptr↔index roundtrip fails | Roundtrip is identity |
| 10 | usize2pa(0) not MEM_valid | Always MEM_valid |
| 11 | Mapped VA not in space | Must be in space |
| 12 | Page diff length ≠ range length | Must be equal |

## Logical Tests (12/12 FAILED ✓)
Tests for properties NOT explicitly guaranteed:

| # | Test | Unwarranted Claim |
|---|---|---|
| 1 | usize2pa is injective | Masking is lossy |
| 2 | Always uses exactly 4 pages | Only ≤ 4 guaranteed |
| 3 | Always uses ≥ 1 page | 0 is possible per spec |
| 4 | ptr→idx→ptr identity for unaligned | Only valid for aligned |
| 5 | Deterministic page allocation | Not guaranteed |
| 6 | ret.0 == 4*len exactly | Not specified |
| 7 | Quota subtraction == 4*len | Only subtract by ret.0 |
| 8 | Allocated pages are contiguous | Not guaranteed |
| 9 | va_4k_valid ⟹ page_ptr_valid | Different domains |
| 10 | Cross-call page uniqueness | Not guaranteed |
| 11 | Returned pages are sorted | Not guaranteed |
| 12 | Old VA entries removed | They are preserved |

## Conclusion

The specification for `range_alloc_and_map` and related functions is **robust** against all 36 adversarial queries:
- **Boundary**: All preconditions are tight enough to reject invalid inputs.
- **Behavioral**: All postconditions are strong enough to reject mutated outputs.
- **Logical**: The spec does not entail unwarranted properties (determinism, injectivity, contiguity, exact page counts, cross-call uniqueness, or sorting).

No specification weaknesses were found in this analysis.
