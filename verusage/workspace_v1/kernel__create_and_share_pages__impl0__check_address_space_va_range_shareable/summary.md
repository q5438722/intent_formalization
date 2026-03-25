# Adversarial Test Summary: `check_address_space_va_range_shareable`

## Target Function
`Kernel::check_address_space_va_range_shareable(target_proc_ptr, va_range) -> bool`

**Preconditions:** `self.wf()`, `self.proc_dom().contains(target_proc_ptr)`, `va_range.wf()`  
**Postcondition:** `ret == self.address_space_range_shareable(target_proc_ptr, va_range)`

The spec `address_space_range_shareable` checks two conditions for all indices j in [0, va_range.len):
1. The VA at index j is in the process's address space domain.
2. The physical page reference counter for that mapping is ≤ `usize::MAX - va_range.len`.

---

## Results

| File | Tests | All Failed (as expected) |
|------|-------|------------------------|
| `boundary_tests.rs` | 5 | ✅ Yes (5/5 errors) |
| `behavioral_mutation_tests.rs` | 5 | ✅ Yes (5/5 errors) |
| `logical_tests.rs` | 5 | ✅ Yes (5/5 errors) |
| `correctness_tests.rs` (combined) | 15 | ✅ Yes (15/15 errors) |

**Verification baseline:** 42 verified (existing definitions/specs), 15 errors (adversarial tests).

---

## Boundary Tests (5/5 FAILED ✅)

| ID | Description | Failure Mode | Result |
|----|-------------|-------------|--------|
| B1 | `proc_ptr` not in `proc_dom()` | Precondition violation — no conclusion derivable | ✅ FAIL |
| B2 | `va_range` not well-formed (`len=usize::MAX`) | Precondition violation — spec rejects | ✅ FAIL |
| B3 | `VA = 0` claimed as 4k-valid | Edge case — L4 index 0 < KERNEL_MEM_END_L4INDEX | ✅ FAIL |
| B4 | Kernel not well-formed | Precondition violation — no shareability derivable | ✅ FAIL |
| B5 | Zero-length range claimed as NOT shareable | Vacuous truth — empty universal quantifier is true | ✅ FAIL |

## Behavioral Mutation Tests (5/5 FAILED ✅)

| ID | Description | Failure Mode | Result |
|----|-------------|-------------|--------|
| M1 | Shareable ⇒ first VA NOT in domain (negated) | Contradicts spec's domain inclusion guarantee | ✅ FAIL |
| M2 | Shareable ⇒ ref counter EXCEEDS threshold (negated) | Contradicts spec's ref counter bound | ✅ FAIL |
| M3 | NOT shareable ⇒ claim shareable (flipped) | Direct negation of given precondition | ✅ FAIL |
| M4 | Stricter ref counter bound (`-1`) | Spec only guarantees `≤ MAX - len`, not `≤ MAX - len - 1` | ✅ FAIL |
| M5 | Unconditionally claim shareable | Spec allows false return; not always true | ✅ FAIL |

## Logical Tests (5/5 FAILED ✅)

| ID | Description | Failure Mode | Result |
|----|-------------|-------------|--------|
| L1 | Shareability transfers across processes | Different procs have different address spaces | ✅ FAIL |
| L2 | Shareability implies zero ref counters | Spec only bounds, not equates to 0 | ✅ FAIL |
| L3 | Subrange shareability implies full range | Extra VAs may be unmapped or exceed threshold | ✅ FAIL |
| L4 | Shareability extends to VAs outside range | Spec only covers VAs within va_range | ✅ FAIL |
| L5 | Different processes have disjoint address spaces | Processes may share physical pages | ✅ FAIL |

---

## Conclusion

All 15 adversarial tests were correctly **rejected** by the specification. The spec for `check_address_space_va_range_shareable`:

1. **Properly guards inputs** — invalid proc_ptr, malformed va_range, and non-wf kernels do not yield derivable conclusions.
2. **Correctly constrains outputs** — mutated behaviors (negated conditions, stricter bounds, unconditional truth) are all rejected.
3. **Does not over-entail** — the spec does not support unintended inferences such as cross-process transfer, zero-ref-counter guarantees, subrange-to-full-range extension, or address space disjointness.

The specification is **consistent** with respect to all tested semantic queries.
