# Abstraction-Level Comparison: `bitmap_raw` vs `bitmap_new`

This document focuses specifically on differences in **abstraction level** between the two versions, setting aside the surface-level attribute syntax differences (`verus!{}` vs `#[verus_spec]`).

---

## 1. Where Specifications Live: Concrete vs Abstract

The most fundamental abstraction difference is **which type owns the spec functions and lemmas**.

### `bitmap_raw`: Specifications on the Concrete Type

In `bitmap_raw`, nearly all spec functions and lemmas are defined on `impl Bitmap` (the concrete struct):

```
bitmap_raw spec/proof ownership:
├── BitmapView (abstract model)
│   ├── number_of_bits()      — accessor
│   ├── usage()               — derived
│   ├── count_free()          — derived
│   ├── has_free_bit()        — predicate
│   ├── is_full()             — predicate
│   ├── is_empty()            — predicate
│   ├── is_bit_set()          — predicate
│   ├── range_set()           — helper
│   └── wf()                  — well-formedness
│
└── Bitmap (concrete type)  ← owns everything else
    ├── bit_at()                          — spec fn
    ├── is_bit_set()                      — spec fn (redefined)
    ├── all_bits_set_in_range()           — spec fn
    ├── all_bits_unset_in_range()         — spec fn
    ├── has_free_range_at()               — spec fn
    ├── exists_contiguous_free_range()    — spec fn
    ├── inv()                             — invariant
    ├── lemma_set_bits_finite()           — proof
    ├── lemma_range_set_finite()          — proof
    ├── lemma_range_set_len()             — proof
    ├── lemma_insert_preserves_usage_bound()
    ├── lemma_free_range_implies_usage_bound()
    ├── lemma_has_free_bit_implies_exists_free_range_1()
    ├── lemma_usage_equals_number_of_bits_implies_full()
    ├── lemma_usage_less_than_capacity_means_not_full()
    ├── lemma_unset_bit_implies_has_free_bit()
    ├── lemma_all_bits_set_means_full()
    ├── ... (18 more lemmas: bit-ops, composite, loop helpers)
    └── Total: ~40 spec/proof items on Bitmap
```

### `bitmap_new`: Specifications on the Abstract Model

In `bitmap_new`, range-related specs and all abstract reasoning lemmas are moved to `impl BitmapView`:

```
bitmap_new spec/proof ownership:
├── BitmapView (abstract model)  ← now owns abstract reasoning
│   ├── usage()
│   ├── has_free_bit()
│   ├── is_full()
│   ├── is_empty()
│   ├── is_bit_set()
│   ├── range_set()
│   ├── wf()
│   ├── all_bits_set_in_range()           — MOVED from Bitmap
│   ├── all_bits_unset_in_range()         — MOVED from Bitmap
│   ├── has_free_range_at()               — MOVED from Bitmap
│   ├── exists_contiguous_free_range()    — MOVED from Bitmap
│   ├── lemma_set_bits_finite()           — MOVED from Bitmap
│   ├── lemma_range_set_finite()          — MOVED from Bitmap
│   ├── lemma_range_set_len()             — MOVED from Bitmap
│   ├── lemma_free_range_implies_usage_bound()   — MOVED
│   ├── lemma_insert_preserves_usage_bound()     — MOVED
│   ├── lemma_usage_equals_number_of_bits_implies_full()  — MOVED
│   ├── lemma_usage_less_than_capacity_means_not_full()   — MOVED
│   ├── lemma_unset_bit_implies_has_free_bit()   — MOVED
│   ├── lemma_all_bits_set_means_full()          — MOVED
│   ├── lemma_has_free_bit_implies_exists_free_range_1()  — MOVED
│   ├── lemma_set_bits_equal_has_free_range_at_equal()    — MOVED
│   └── lemma_set_bits_equal_exists_free_range_equal()    — MOVED
│
└── Bitmap (concrete type)  ← only owns implementation-coupled proofs
    ├── bit_at()                          — spec fn (now private)
    ├── inv() = wf() + internal_inv()     — invariant (split)
    ├── internal_inv()                    — invariant (closed)
    ├── lemma_bit_or_effects()            — bit-level proof
    ├── lemma_bit_and_not_effects()       — bit-level proof
    ├── lemma_byte_or_reflects_in_view()  — representation coupling
    ├── lemma_byte_and_not_reflects_in_view()
    ├── lemma_zero_bytes_means_empty_set()
    ├── lemma_new_bitmap_inv()
    ├── lemma_set_bit_preserves_inv()
    ├── lemma_clear_bit_preserves_inv()
    ├── lemma_alloc_range_establishes_inv()
    ├── lemma_alloc_loop_step_inv()
    ├── lemma_no_free_range_when_size_exceeds()
    ├── lemma_no_free_range_when_usage_exceeds()
    ├── lemma_full_byte_no_free_range()
    ├── lemma_set_bit_blocks_free_range()
    ├── lemma_free_range_was_unset_in_old()
    ├── lemma_no_range_found_frame()
    ├── lemma_phase1_complete_no_free_range()
    ├── lemma_all_positions_no_free_range()
    └── alloc_range_{first,second,third}_loop_invariant() — loop spec fns
```

**Key insight**: In `bitmap_new`, `BitmapView` is a self-contained abstract specification that can be reasoned about **independently of the concrete `Bitmap` representation**. In `bitmap_raw`, you must always work through `Bitmap` to access range predicates and abstract lemmas.

---

## 2. Invariant Encapsulation

### `bitmap_raw`: Fully Opaque Invariant

```rust
pub closed spec fn inv(&self) -> bool {
    // Everything hidden — callers know only that inv() holds
}
```

Callers cannot see any component of `inv()`. They cannot deduce that `inv()` implies `wf()`.

### `bitmap_new`: Two-Layer Invariant

```rust
pub open spec fn inv(&self) -> bool {
    &&& self@.wf()           // Visible: callers can use wf() directly
    &&& self.internal_inv()  // Hidden: implementation details sealed
}

pub closed spec fn internal_inv(&self) -> bool {
    // Hidden details: bits.inv(), bit-count alignment, usage tracking, etc.
}
```

This is a **higher level of abstraction** because:
- Callers can pattern-match on `inv()` and extract `wf()` without a lemma.
- The representation invariant (`internal_inv`) stays hidden.
- This enables callers to reason about `BitmapView` properties (which depend on `wf()`) without the verifier needing to unfold the full invariant.

---

## 3. Specification Interfaces: Concrete References vs Abstract References

### `bitmap_raw`: Specs Reference Concrete Self

In `bitmap_raw`, function contracts reference the concrete `Bitmap` type and its spec functions:

```rust
// bitmap_raw alloc_range ensures (on Ok)
self.all_bits_set_in_range(start, start + size)       // Bitmap method
old(self).all_bits_unset_in_range(start, start + size) // Bitmap method
old(self).exists_contiguous_free_range(size as int)     // Bitmap method
self.is_bit_set(i) == old(self).is_bit_set(i)          // Bitmap method
```

Callers must work with `Bitmap`-level predicates. To use these in abstract reasoning, they need bridging lemmas (like `lemma_is_bit_set_equals_view`) to connect `Bitmap::is_bit_set` to `BitmapView::is_bit_set`.

### `bitmap_new`: Specs Reference the Abstract View

In `bitmap_new`, function contracts reference `BitmapView` via the `@` operator:

```rust
// bitmap_new alloc_range ensures (on Ok)
self@.all_bits_set_in_range(start, start + size)        // BitmapView method
old(self)@.all_bits_unset_in_range(start, start + size) // BitmapView method
!old(self)@.exists_contiguous_free_range(size as int)    // BitmapView method
self@.is_bit_set(i) == old(self)@.is_bit_set(i)         // BitmapView method
```

Callers interact **only with the abstract model**. No bridging lemmas needed — the contract is already expressed at the abstract level.

---

## 4. Proof Factoring: Abstract vs Concrete Reasoning

### `bitmap_raw`: Mixed Reasoning in `lib.proof.rs` (1169 lines)

All proofs live together in one `impl Bitmap` block:
- **Abstract set-theory lemmas** (finiteness, cardinality, subset bounds) mixed with
- **Bit-level proofs** (OR/AND-NOT effects) mixed with
- **Representation coupling proofs** (byte changes ↔ set_bits changes) mixed with
- **Loop invariant proofs** (alloc_range search correctness)

The abstract lemmas (e.g., `lemma_insert_preserves_usage_bound`, `lemma_free_range_implies_usage_bound`) are expressed in terms of `self.inv()` and `self.is_bit_set()` — tying them to the concrete type even though they reason about sets.

### `bitmap_new`: Separated Abstract and Concrete Proofs

| File | Content | Lines |
|------|---------|-------|
| `lib.spec.rs` | Abstract model + abstract lemmas on `BitmapView` | 394 |
| `lib.proof.rs` | Bit-level + representation-coupling + loop proofs on `Bitmap` | 716 |

**Abstract lemmas** (on `BitmapView`) require only `self.wf()` and `self.num_bits >= 0` — no `inv()`, no `bits`, no representation details. Example:

```rust
// bitmap_new: abstract lemma on BitmapView
pub proof fn lemma_insert_preserves_usage_bound(&self, x: int)
    requires
        self.wf(),                      // Only abstract well-formedness
        0 <= x < self.num_bits,         // Abstract bound
        !self.set_bits.contains(x),
    ensures
        self.set_bits.insert(x).len() <= self.num_bits,
```

vs `bitmap_raw`:

```rust
// bitmap_raw: same lemma on Bitmap
pub proof fn lemma_insert_preserves_usage_bound(&self, x: int)
    requires
        self.inv(),                         // Full concrete invariant
        0 <= x < self@.number_of_bits(),    // Through view accessor
        !self@.set_bits.contains(x),
    ensures
        self@.set_bits.insert(x).len() <= self@.number_of_bits(),
```

The `bitmap_new` version is strictly more abstract: it doesn't require `inv()` (which includes representation invariants), only `wf()` (which is a pure set constraint).

---

## 5. Eliminated Duplication

### `bitmap_raw`: Redundant Spec Functions

`bitmap_raw` defines `is_bit_set` on **both** `BitmapView` and `Bitmap`:

```rust
// On BitmapView
pub open spec fn is_bit_set(&self, index: int) -> bool {
    self.set_bits.contains(index)
}

// On Bitmap (adds bounds check)
pub open spec fn is_bit_set(&self, bit_index: int) -> bool {
    &&& 0 <= bit_index < self@.number_of_bits()
    &&& self@.set_bits.contains(bit_index)
}
```

These have subtly different semantics — `Bitmap::is_bit_set` bundles bounds-checking, `BitmapView::is_bit_set` does not. This requires a bridging lemma `lemma_is_bit_set_equals_view` in `lib.proof.rs`.

### `bitmap_new`: Single Definition

`bitmap_new` defines `is_bit_set` only on `BitmapView`. All contracts use `self@.is_bit_set()`. No duplication, no bridging lemma needed.

Similarly, `all_bits_set_in_range`, `all_bits_unset_in_range`, `has_free_range_at`, and `exists_contiguous_free_range` each exist only once (on `BitmapView`) instead of being duplicated.

---

## 6. Finiteness Proof Approach

### `bitmap_raw`: Manual Wrapping of vstd Axioms

`bitmap_raw` wraps every vstd set axiom as a standalone lemma on `Bitmap`:

```rust
proof fn lemma_empty_set_finite()     { /* axiom */ }
proof fn lemma_insert_finite(s, x)    { /* axiom */ }
proof fn lemma_remove_finite(s, x)    { /* axiom */ }
proof fn lemma_union_finite(s1, s2)   { /* axiom */ }
proof fn lemma_difference_finite(s1, s2) { /* axiom */ }
proof fn lemma_insert_len(s, x)       { /* axiom */ }
proof fn lemma_insert_same_len(s, x)  { ... }
proof fn lemma_disjoint_union_len(s1, s2) { ... }
proof fn lemma_remove_len(s, x)       { /* axiom */ }
proof fn lemma_remove_same_len(s, x)  { ... }
proof fn lemma_empty_len()            { /* axiom */ }
proof fn lemma_ext_equal_finite(s1, s2) { ... }
proof fn lemma_ext_equal_len(s1, s2)  { ... }
```

These are **thin wrappers** over vstd, adding ~150 lines. They're called by name throughout other proofs.

### `bitmap_new`: Direct Use + Structural Proofs

`bitmap_new` eliminates most wrapper lemmas. Instead:
- `BitmapView::lemma_set_bits_finite()` proves finiteness structurally via subset.
- `BitmapView::lemma_range_set_finite()` and `lemma_range_set_len()` call vstd directly.
- Composite lemmas (e.g., `lemma_set_bit_preserves_inv`) call `self@.lemma_insert_preserves_usage_bound()` — i.e., the abstract lemma on the view — instead of chaining through wrapper lemmas.

This reduces `lib.proof.rs` from **1169 lines to 716 lines**.

---

## 7. Error Specification Abstraction

### `bitmap_raw`: Separate Liveness Clauses

```rust
result is Ok ==> { ... },
result is Err ==> self@ == old(self)@,
old(self)@.has_free_bit() ==> result is Ok,   // separate liveness
```

The error case only says the state is unchanged. The caller must combine the liveness clause with knowledge of the precondition to determine when success is guaranteed.

### `bitmap_new`: Exhaustive Match with Error Characterization

```rust
match result {
    Ok(index) => { ... },
    Err(_) => {
        &&& old(self)@.is_full()    // Why it failed
        &&& self@ == old(self)@     // State unchanged
    },
}
```

The error case now **characterizes the failure reason** (`is_full()`). This is a higher level of abstraction because:
- The postcondition is self-contained — no need to combine with a separate clause.
- Callers can immediately conclude from `Err` that the bitmap was full, without needing to invoke a contrapositive argument on the liveness clause.

---

## 8. Summary

| Dimension | `bitmap_raw` | `bitmap_new` |
|-----------|-------------|-------------|
| **Spec functions** | On concrete `Bitmap` | On abstract `BitmapView` |
| **Lemmas** | All on `Bitmap`, require `inv()` | Abstract ones on `BitmapView` (require only `wf()`), representation ones on `Bitmap` |
| **Invariant** | Fully opaque `closed` | Open `wf()` + closed `internal_inv()` |
| **Function contracts** | Reference `self.is_bit_set()`, `self.has_free_range_at()` | Reference `self@.is_bit_set()`, `self@.has_free_range_at()` |
| **Duplication** | `is_bit_set` on both types + bridging lemma | Single definition on `BitmapView` |
| **Finiteness proofs** | 13 vstd wrapper lemmas | Direct calls + structural proofs |
| **Error specs** | State-unchanged + separate liveness | Exhaustive match with failure reason |
| **Proof lines** | 1169 | 716 (39% reduction) |
| **Total lines** | 2062 | 1832 (11% reduction) |

**Bottom line**: `bitmap_new` operates at a **higher level of abstraction** by making `BitmapView` a self-sufficient specification layer. Abstract reasoning (set properties, range predicates, usage bounds) is decoupled from representation reasoning (byte operations, invariant maintenance). This separation means callers reason purely about the abstract model, and the concrete proofs are smaller because they delegate abstract arguments to `BitmapView` lemmas.
