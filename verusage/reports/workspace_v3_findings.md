# Spec Consistency Findings — workspace_v3

**Date:** 2025-03-25
**Pipeline:** Generate φ → Entailment Check (Verus) → LLM Critic
**Scope:** 313 tasks from verusage corpus (atmosphere, ironkv, memory-allocator, nrkernel)

## Pipeline Stats

| Metric | Count |
|---|---|
| Tasks total | 313 |
| Tasks with entailment results | 279 |
| Tasks with ≥1 verified φ | 31 |
| φ generated | 1395 |
| φ entailed (verified by Verus) | 63 (4.5%) |
| Critic: true positive (pre-audit) | 36 |
| Critic: false positive (pre-audit) | 27 |
| Critic precision (pre-audit) | 57.1% |

## Post-Audit Results

After manual review of all 36 TPs with Tianyu:

| Category | Count | Notes |
|---|---|---|
| **Confirmed TP** | 4 | All from `next_run` |
| **FP — ghost/spec generality** | ~24 | nrkernel `Arch::inv()`, alignment, `calculate_page_block_at` |
| **FP — opaque invariant** | 2 | ironkv `is_marshalable`, atmosphere `helper_kernel_kill_proc_root` |
| **Borderline** | 2 | atmosphere `va_range_none` (empty input), `calculate_page_block_at` (one-past-end) |
| **Duplicate** | 4 | nrkernel alignment = same finding × 3, next_run gap①×2 |

**Audited precision: ~4/36 (11%)**

---

## Confirmed Finding: `CommitMask::next_run`

**File:** `memory-allocator/verified/commit_mask/commit_mask__impl__next_run.rs`
**Severity:** Medium (functional correctness, not safety)
**Verified φ:** 4

Scans a 512-bit bitmap from position `idx` to find the next contiguous run of committed slices. The spec only guarantees the returned range consists of set bits:

```rust
ensures {
  next_idx + count <= 512
  && forall |t| next_idx <= t < next_idx + count ==> self@.contains(t)
}
```

Three missing constraints, each demonstrated by a verified φ:

**① No forward-scan guarantee.** `next_idx >= idx` not required. Returning `(0, 0)` with `idx=100` is legal. Source has a commented-out fix (`forall |t| idx <= t < next_idx ==> !self@.contains(t)`), but it doesn't help: when `next_idx=0 < idx=100`, the range `100..0` is empty → vacuously true.

**② No completeness.** `(512, 0)` satisfies the spec for any input — "not found" is always valid. Implementation can skip all scanning.

**③ No maximality.** Can report `count=1` when the actual run is longer. Source comments acknowledge this.

Developers explicitly noted these gaps aren't needed for safety, but they leave functional correctness unverified — callers iterating committed regions could miss memory, loop forever, or make redundant syscalls.

---

## False Positive Analysis

### FP Pattern 1: Ghost/spec-level generality (24 TPs)

The dominant failure mode. The critic flagged spec-only (ghost) data structures for admitting degenerate values:

- **nrkernel `Arch::inv()`** (~20 TPs): Allows zero layers, 1-byte entry sizes. But `Arch` is a ghost struct — never constructed at runtime. The actual architecture is fixed via `x86_arch_spec()`. Proving lemmas for a more general `inv()` is **strictly stronger**, not weaker.
- **nrkernel `Directory::well_formed()`** (3 TPs): Commented-out alignment check. `Directory` is also ghost. Missing alignment doesn't affect proof soundness.
- **memory-allocator `calculate_page_block_at`** (1 TP): `block_size == 0` not excluded. But `PageId` is a ghost struct, `block_start_at` is a spec fn. The proof holds for a more general case; actual callers always pass nonzero.

**Root cause:** The critic doesn't distinguish ghost vs exec code. In Verus, ghost structs are proof annotations — they can't be incorrectly constructed at runtime. A more general ghost invariant is a feature, not a weakness.

### FP Pattern 2: Tautological φ — not spec-dependent (2 TPs)

- **atmosphere `helper_kernel_kill_proc_root`**: The φ says "if children list doesn't change, and old list contains ptr, then new list contains ptr." This is pure logic (`A == B ∧ B.contains(x) → A.contains(x)`) — holds regardless of the spec. The generator produced a tautology, not a spec query.
- **ironkv `is_marshalable`**: `usize <= u64::MAX` is tautologically true on 64-bit. The spec correctly reflects platform semantics; the critic misidentified it as a weakness.

**Root cause:** The generator sometimes produces φ that are logically valid independent of the spec. The entailment check can't filter these — they verify, but they're not testing the spec. The critic should check whether φ depends on the spec at all.

### Borderline Cases (2 TPs)

- **atmosphere `get_address_space_va_range_none`**: Empty `VaRange4K` (len=0) vacuously passes the check. Whether this matters depends on callers validating length beforehand.
- **memory-allocator `calculate_page_block_at`** (one-past-end): `<=` allows result at `segment_start + SEGMENT_SIZE`. Could be intentional (C-style one-past-end) or off-by-one.

---

## Observations

1. **Audited precision is 11%, not 57%.** The critic's self-reported precision is wildly optimistic. Most TPs collapse under domain knowledge.

2. **Ghost/exec distinction is critical.** 24/36 TPs (67%) are FP because the critic doesn't understand Verus's ghost code. Teaching it this distinction would eliminate the majority of false positives.

3. **Heavy deduplication needed.** The `Arch::inv()` weakness was reported ~20 times across different lemmas. Dedup at the invariant/predicate level would reduce noise dramatically.

4. **The one real finding is good.** `next_run` has genuine spec completeness gaps that developers themselves acknowledged in comments. The pipeline successfully found and formalized these.

5. **Commented-out specs are the strongest signal.** The `next_run` finding was partly confirmed by commented-out ensures clauses in the source. A targeted scan for commented-out spec clauses could be a high-precision heuristic.
