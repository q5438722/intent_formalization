# Test Execution Summary: `get_entry_2m_l2`

## Target Function
`PageTable::get_entry_2m_l2(target_l4i, target_l3i, target_l2i, l3_entry) -> Option<PageEntry>`

Resolves a 2MB page entry at L2 level of a 4-level x86-64 page table hierarchy.

**Preconditions**: `wf()`, `kernel_l4_end <= l4i < 512`, `0 <= l3i < 512`, `0 <= l2i < 512`, L3 mapping resolves to `l3_entry`.

**Postcondition**: Result matches `spec_resolve_mapping_2m_l2(l4i, l3i, l2i)`.

---

## Results Overview

| Test File | Tests | All Failed (as expected) |
|-----------|-------|------------------------|
| `boundary_tests.rs` | 5 | ✅ Yes (5/5 errors) |
| `behavioral_mutation_tests.rs` | 5 | ✅ Yes (5/5 errors) |
| `logical_tests.rs` | 5 | ✅ Yes (5/5 errors) |

**All 15 adversarial tests failed verification**, confirming the specification correctly rejects invalid inputs, incorrect behaviors, and unintended reasoning.

---

## Boundary Tests (Precondition Violations)

| ID | Description | Result |
|----|-------------|--------|
| B1 | `l4i = 0` (below `kernel_l4_end = 1`) — kernel range | FAIL ✅ |
| B2 | `l3i` unbounded (missing `0 <= l3i < 512`) | FAIL ✅ |
| B3 | `l2i` unbounded (missing `0 <= l2i < 512`) | FAIL ✅ |
| B4 | Missing `wf()` — cannot derive `user_only` invariant | FAIL ✅ |
| B5 | `l3_entry` mismatch — wrong entry address vs. spec resolution | FAIL ✅ |

**Analysis**: The spec properly guards all preconditions. Without `wf()`, invariants like `user_only` cannot be derived. Without bounded indices, `wf_mapping_2m` cannot guarantee page alignment.

---

## Behavioral Mutation Tests (Mutated Outputs)

| ID | Description | Result |
|----|-------------|--------|
| M1 | Assert result is always `Some` (remove None case) | FAIL ✅ |
| M2 | Assert result is always `None` (remove Some case) | FAIL ✅ |
| M3 | Assert result addr is `l2_entry.addr + 1` (off-by-one) | FAIL ✅ |
| M4 | Assert `present && !ps` yields `Some` (flip PS condition) | FAIL ✅ |
| M5 | Assert write permission is negated (flip write bit) | FAIL ✅ |

**Analysis**: The spec correctly distinguishes between present/absent entries, PS/non-PS entries, and preserves exact address and permission values from L2 table entries.

---

## Logical Tests (Unintended Properties)

| ID | Description | Result |
|----|-------------|--------|
| L1 | Resolved 2m entry addr is always nonzero | FAIL ✅ |
| L2 | Resolved 2m entry always has write permission | FAIL ✅ |
| L3 | Different `l2i` indices yield same result | FAIL ✅ |
| L4 | 2m `None` implies L3 `None` (stronger backward implication) | FAIL ✅ |
| L5 | 2m `Some` implies all non-PS L2 resolutions are `None` (universal) | FAIL ✅ |

**Analysis**:
- **L1**: The spec does not guarantee `addr != 0` for present entries — a page can be mapped at physical address 0.
- **L2**: `rwx_upper_level_entries` only constrains non-PS L2 entries (directory entries), not PS entries (2MB huge pages). So 2MB page entries are not forced to have write=true.
- **L3**: Different L2 indices naturally access different entries in the page table.
- **L4**: 2m resolution being `None` does NOT imply L3 is `None` — L3 can resolve but the L2 entry may lack `present` or `ps` flags.
- **L5**: Having a 2m mapping at one index does NOT preclude non-PS L2 entries at other indices in the same table.

---

## Conclusion

The specification for `get_entry_2m_l2` is **consistent** with respect to all tested adversarial properties. It correctly:
1. **Rejects invalid inputs** — precondition violations prevent proving postcondition properties.
2. **Rejects incorrect behaviors** — mutated outputs are not entailed by the specification.
3. **Rejects unintended reasoning** — stronger-than-specified properties cannot be derived.

No specification weaknesses were found in this round of testing.
