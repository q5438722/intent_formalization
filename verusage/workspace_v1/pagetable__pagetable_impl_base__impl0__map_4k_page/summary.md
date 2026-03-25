# Adversarial Test Results: `map_4k_page` Specification

**Target**: `pagetable__pagetable_impl_base__impl0__map_4k_page.rs`
**Result**: All 14 adversarial tests **FAILED verification** as expected → specification correctly rejects all tested invalid properties.

---

## (1) Boundary Tests — 5/5 FAILED ✓

| # | Test | What it checks | Result |
|---|------|----------------|--------|
| 1 | `test_boundary_l4_out_of_range` | `spec_resolve_mapping_l4(512)` should not guarantee `None` for out-of-range index | FAIL ✓ |
| 2 | `test_boundary_max_l4i_always_resolves` | `spec_resolve_mapping_l4(511)` is NOT always `Some` — entries can be non-present | FAIL ✓ |
| 3 | `test_boundary_resolve_without_l2_parent` | 4k mapping should NOT exist when L2 parent is absent | FAIL ✓ |
| 4 | `test_boundary_zero_addr_not_mapped` | Mapped physical address CAN be 0 (`page_ptr_valid(0)` is true) | FAIL ✓ |
| 5 | `test_boundary_mapped_addr_in_page_closure` | Leaf data pages are NOT in `page_closure` (only page table pages are) | FAIL ✓ |

## (2) Behavioral Mutation Tests — 4/4 FAILED ✓

| # | Test | What it checks | Result |
|---|------|----------------|--------|
| 1 | `test_mutation_wrong_mapping_addr` | `mapping_4k` address matches `spec_resolve_mapping_4k_l1` — inequality rejected | FAIL ✓ |
| 2 | `test_mutation_4k_2m_overlap` | 4k and 2m mappings cannot overlap (L2 `ps` flag distinguishes them) | FAIL ✓ |
| 3 | `test_mutation_write_perm_flipped` | Write permission in `mapping_4k` matches resolve result — flipping rejected | FAIL ✓ |
| 4 | `test_mutation_page_closure_is_singleton` | `page_closure` is NOT just `{cr3}` — it includes L3/L2/L1 table pages | FAIL ✓ |

## (3) Logical Tests — 5/5 FAILED ✓

| # | Test | What it checks | Result |
|---|------|----------------|--------|
| 1 | `test_logical_mapping_injective` | Different VAs can map to SAME physical address (shared memory is allowed) | FAIL ✓ |
| 2 | `test_logical_tlb_identical_across_cpus` | TLBs are submaps, NOT required to be identical across CPUs | FAIL ✓ |
| 3 | `test_logical_wf_implies_nonempty_mapping` | `wf()` does NOT require any mappings to exist | FAIL ✓ |
| 4 | `test_logical_l1_entries_disjoint` | No `disjoint_l1` in spec — L1 entries can share physical addresses | FAIL ✓ |
| 5 | `test_logical_pcid_determines_mapping` | Same PCID does NOT imply same mappings | FAIL ✓ |

---

## Analysis

The specification for `map_4k_page` appears **well-constructed**:

1. **Preconditions properly guard inputs** — out-of-range indices, absent parent mappings, and invalid page pointers are all rejected.
2. **Postconditions correctly constrain outputs** — mapping addresses, permissions, and the preservation of 2M/1G mappings are all enforced.
3. **No unintended entailments detected** — the spec doesn't accidentally prove injectivity, TLB equality, or other properties it shouldn't.

### Notable spec design observations:
- **No `disjoint_l1`**: The spec has `disjoint_l4`, `disjoint_l3`, `disjoint_l2` but NOT `disjoint_l1`. This is intentional — L1 entries map to data pages (physical frames), and shared memory legitimately allows multiple VAs to map to the same physical page.
- **`page_ptr_valid(0)` is true**: Address 0 is a valid page pointer by the spec (0 % 0x1000 == 0, 0 / 0x1000 < NUM_PAGES). This is a deliberate choice.
- **TLB is a submap**: Each CPU's TLB is a submap of the full mapping, allowing stale entries during concurrent updates.
