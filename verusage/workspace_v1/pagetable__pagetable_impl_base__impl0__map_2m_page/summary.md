# Test Execution Summary: `map_2m_page`

## Target
`pagetable__pagetable_impl_base__impl0__map_2m_page.rs` — Maps a 2MB page into the x86-64 page table hierarchy.

## Results Overview

| Test File | Tests | All Failed (as expected) |
|-----------|-------|------------------------|
| boundary_tests.rs | 5 | ✅ Yes (5/5 errors) |
| behavioral_mutation_tests.rs | 5 | ✅ Yes (5/5 errors) |
| logical_tests.rs | 5 | ✅ Yes (5/5 errors) |

**Total: 15/15 tests correctly rejected by Verus** — The specification is consistent with respect to all queried properties.

---

## Boundary Tests (precondition violations / edge cases)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_boundary_kernel_l4_end_must_be_positive` | `kernel_l4_end > 0` under `wf()` | ❌ FAILED — spec allows `kernel_l4_end == 0` |
| 2 | `test_boundary_resolve_l4_at_512` | `spec_resolve_mapping_l4(512).is_None()` | ❌ FAILED — index 512 is out-of-range (`recommends`, not `requires`) |
| 3 | `test_boundary_max_l4i_always_resolves` | L4 index 511 always maps to Some | ❌ FAILED — empty entries are allowed |
| 4 | `test_boundary_pcid_always_some` | `pcid.is_Some()` under `wf()` | ❌ FAILED — XOR with ioid, could be None |
| 5 | `test_boundary_l3_tables_nonempty` | L3 tables must be non-empty | ❌ FAILED — no present L4 entries → no L3 tables |

## Behavioral Mutation Tests (incorrect postconditions)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_mutation_mapping_2m_unchanged` | `mapping_2m` is unchanged after insert | ❌ FAILED — insert adds new entry |
| 2 | `test_mutation_wrong_entry_addr` | Inserted entry has wrong address | ❌ FAILED — addr must match |
| 3 | `test_mutation_mapping_4k_gains_entry` | `mapping_4k` gains entry after 2M op | ❌ FAILED — 4K mapping is unchanged |
| 4 | `test_mutation_mapping_1g_changed` | `mapping_1g` changes after 2M op | ❌ FAILED — 1G mapping is unchanged |
| 5 | `test_mutation_page_closure_changed` | `page_closure` changes after 2M op | ❌ FAILED — page closure is unchanged |

## Logical Tests (unintended reasoning / unguaranteed properties)

| # | Test | Property Queried | Result |
|---|------|-----------------|--------|
| 1 | `test_logical_mapping_2m_injective` | 2M mapping is injective (no shared pages) | ❌ FAILED — shared physical pages allowed |
| 2 | `test_logical_tlb_2m_identical_across_cpus` | TLB 2M caches identical across CPUs | ❌ FAILED — only submaps, not equal |
| 3 | `test_logical_wf_implies_nonempty_mapping_2m` | `wf()` implies non-empty 2M mapping | ❌ FAILED — empty 2M mapping is valid |
| 4 | `test_logical_l1_entries_disjoint` | L1 entries within table have disjoint addrs | ❌ FAILED — no `disjoint_l1` in spec |
| 5 | `test_logical_pcid_determines_mapping` | Same PCID implies same mappings | ❌ FAILED — PCID is not a mapping key |

## Conclusion

The specification for `map_2m_page` is **consistent** with respect to all 15 adversarial queries:
- **Boundary**: Invalid inputs and edge cases are properly rejected.
- **Behavioral**: Incorrect output mutations are properly rejected.
- **Logical**: Unintended stronger properties are not entailed.

Notable spec design observations:
- No `disjoint_l1` property exists (unlike L2/L3/L4), allowing shared physical frames at L1 leaf level.
- TLB caches are intentionally weak (submaps only), reflecting real hardware behavior.
- `kernel_l4_end` can be 0, which is architecturally valid but may be unintended.
