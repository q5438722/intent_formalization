# Test Summary: `alloc_and_map_io`

**Target**: `kernel__create_and_map_pages__impl0__alloc_and_map_io.rs`  
**Function under test**: `Kernel::alloc_and_map_io` (and related helpers)  
**Total tests**: 36 (12 boundary + 12 behavioral mutation + 12 logical)  
**All tests FAILED verification as expected** — the specification correctly rejects all adversarial queries.

---

## Boundary Tests (12/12 FAIL ✅)

| # | Test | Violated Precondition |
|---|------|-----------------------|
| 1 | Zero free pages | `get_num_of_free_pages() >= 1` |
| 2 | Proc not in domain | `proc_dom().contains(target_proc_ptr)` |
| 3 | Zero container quota | `get_container_quota(...).mem_4k >= 1` |
| 4 | VA = 0 (kernel region) | `va_4k_valid(target_va)` |
| 5 | VA = 1 (not aligned) | `va_4k_valid(target_va)` |
| 6 | VA already in IO space | `get_io_space(...).dom().contains(target_va) == false` |
| 7 | Proc has no IOMMU table | `get_proc_has_iommu_table(target_proc_ptr)` |
| 8 | Unaligned page ptr | `page_ptr2page_index` requires `ptr % 0x1000 == 0` |
| 9 | Page index at NUM_PAGES | `page_index2page_ptr` requires `i < NUM_PAGES` |
| 10 | Invalid VA for va2index | `va2index` requires valid VA |
| 11 | IOMMU L2 resolution None | L2 resolve `is_Some()` |
| 12 | Quota underflow (0 - 1) | `spec_subtract_mem_4k` arithmetic underflow |

## Behavioral Mutation Tests (12/12 FAIL ✅)

| # | Test | Mutated Postcondition |
|---|------|-----------------------|
| 1 | Free pages unchanged | `get_num_of_free_pages()` decreases by 1 |
| 2 | IO mapping is empty | `page_io_mappings(ret)` contains `(ioid, va)` |
| 3 | IO space unchanged | `get_io_space` gets new entry inserted |
| 4 | Proc domain grows | `proc_dom()` is preserved |
| 5 | Address space changed | `get_address_space` is preserved |
| 6 | Other proc IO space changed | Other procs' IO spaces preserved |
| 7 | Quota decremented by 2 | Quota decremented by exactly 1 |
| 8 | Return write is false | MapEntry has `write: true` |
| 9 | Return execute_disable is true | MapEntry has `execute_disable: false` |
| 10 | Container domain shrinks | `container_dom()` is preserved |
| 11 | Other container pages changed | Other containers' pages preserved |
| 12 | Container pages grow by 2 | Target container pages grow by exactly 1 |

## Logical Tests (12/12 FAIL ✅)

| # | Test | Unintended Property Queried |
|---|------|-----------------------------|
| 1 | Allocation determinism | Same inputs → same page (not guaranteed) |
| 2 | VA determines page | Different VAs → different pages (not guaranteed) |
| 3 | Ret addr equals VA | Page address relates to VA (not guaranteed) |
| 4 | Valid ptr implies free | `page_ptr_valid` ≠ "was free" |
| 5 | Distinct IOids → distinct pages | No such injectivity guarantee |
| 6 | Quota always positive after alloc | Quota can reach 0 |
| 7 | Page index < NUM_PAGES/2 | Only guaranteed < NUM_PAGES |
| 8 | IO mapping in address space | IO space ≠ address space |
| 9 | Double alloc same VA | Second alloc precondition fails |
| 10 | Container → unique proc | Multiple procs can share a container |
| 11 | Roundtrip for arbitrary values | Only valid for valid page ptrs |
| 12 | Total pages constant (wrong direction) | mapped + free doesn't increase |

---

## Conclusion

The specification for `alloc_and_map_io` is **consistent** with respect to all 36 adversarial queries:
- All **boundary violations** are correctly rejected by preconditions.
- All **behavioral mutations** are correctly rejected by postconditions.
- All **logical overreach** properties are correctly not entailed.

No specification weaknesses were detected.
