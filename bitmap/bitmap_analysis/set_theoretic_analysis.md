# Deep Analysis: Set-Theoretic Abstraction in `bitmap_new`

This document analyzes the deeper consequences of `bitmap_new`'s design choice to treat `BitmapView` as a **self-contained set-theoretic object** rather than just a passive data carrier.

---

## 1. The Core Design Decision

Both versions define `BitmapView = (num_bits: int, set_bits: Set<int>)`. But they use it very differently:

- **`bitmap_raw`**: `BitmapView` is a *passive view* — a data record that `Bitmap` projects onto. All meaningful predicates and lemmas live on `Bitmap`. `BitmapView` is just the "return type of `view()`."

- **`bitmap_new`**: `BitmapView` is an *active specification model* — a first-class mathematical object with its own predicates, invariants, and proof library. `Bitmap` is merely one *implementation* of this model.

This is the difference between "the view is a projection of the struct" and "the struct is a realization of the model."

---

## 2. Advantage 1: Representation Independence

### The Problem in `bitmap_raw`

In `bitmap_raw`, predicates like `has_free_range_at` are on `Bitmap`:

```rust
// bitmap_raw — on Bitmap
pub open spec fn has_free_range_at(&self, start: int, n: int) -> bool {
    &&& 0 <= start
    &&& start + n <= self@.number_of_bits()
    &&& self.all_bits_unset_in_range(start, start + n)
}
```

This looks representation-independent (it goes through `self@`), but the predicate is **bound to the `Bitmap` type**. Every lemma that uses it requires a `&Bitmap`, even for purely mathematical reasoning. If you change the implementation — say from `RawArray<u8>` to `Vec<u64>`, or from a byte array to a hierarchical bitmap — you must:

1. Re-define every predicate.
2. Re-prove every lemma (even abstract ones like "free range implies usage bound").
3. Update every caller that names `Bitmap::has_free_range_at`.

### The Solution in `bitmap_new`

```rust
// bitmap_new — on BitmapView
pub open spec fn has_free_range_at(&self, start: int, n: int) -> bool {
    &&& 0 <= start
    &&& start + n <= self.num_bits
    &&& self.all_bits_unset_in_range(start, start + n)
}
```

This predicate is on the abstract `BitmapView`. If you swap the `Bitmap` implementation:
- All `BitmapView` predicates survive unchanged.
- All `BitmapView` lemmas survive unchanged.
- You only re-prove the representation-coupling lemmas (`lemma_byte_or_reflects_in_view`, etc.).
- Callers' specs remain valid — they reference `self@.has_free_range_at()`, which doesn't mention `Bitmap`.

**Concrete impact**: Of the ~40 proof obligations in `bitmap_raw`, roughly 15 are pure set-theory (finiteness, cardinality, usage bounds, range properties). In `bitmap_new`, these 15 are on `BitmapView` and survive any implementation change.

---

## 3. Advantage 2: Weaker Preconditions Enable Mid-Proof Reasoning

### The Problem in `bitmap_raw`

Abstract lemmas require `self.inv()`:

```rust
// bitmap_raw
proof fn lemma_free_range_implies_usage_bound(&self, p: int, n: int)
    requires
        self.inv(),         // Full concrete invariant needed
        self.has_free_range_at(p, n),
        n > 0,
    ensures
        self@.usage() <= self@.number_of_bits() - n,
```

`self.inv()` includes concrete state: `bits@.len() * 8 == number_of_bits`, `usage == set_bits.len()`, `next_free <= number_of_bits`, etc. This means you **cannot call this lemma in intermediate states** where the concrete invariant is temporarily broken — for example, after modifying `bits` but before updating `usage`.

### The Solution in `bitmap_new`

```rust
// bitmap_new
proof fn lemma_free_range_implies_usage_bound(&self, p: int, n: int)
    requires
        self.wf(),          // Only set-theoretic well-formedness
        self.has_free_range_at(p, n),
        n > 0,
    ensures
        self.usage() <= self.num_bits - n,
```

`self.wf()` is purely `forall|i| set_bits.contains(i) ==> 0 <= i < num_bits` — a set-theoretic constraint that holds at **all** intermediate states, because modifying `bits` doesn't change the *view's* `wf()` until the view itself changes.

**Concrete example**: In `lemma_no_free_range_when_usage_exceeds`, the bitmap_new version calls:

```rust
// bitmap_new: calls abstract lemma on the VIEW
self@.lemma_free_range_implies_usage_bound(p, size);
```

This works because `self@` (the view) always satisfies `wf()` as long as the `view()` function's definition is well-formed. No need to re-establish `inv()` first.

In `bitmap_raw`, the same call is:

```rust
// bitmap_raw: calls lemma on SELF (requiring inv())
self.lemma_free_range_implies_usage_bound(p, size);
```

This works only because `inv()` is maintained. But it creates a **proof coupling**: the abstract mathematical argument is entangled with the concrete invariant.

---

## 4. Advantage 3: Elimination of the `is_bit_set` Impedance Mismatch

### The Problem in `bitmap_raw`

`bitmap_raw` defines `is_bit_set` on *both* types with **different semantics**:

```rust
// BitmapView::is_bit_set — pure containment
pub open spec fn is_bit_set(&self, index: int) -> bool {
    self.set_bits.contains(index)
}

// Bitmap::is_bit_set — containment WITH bounds check
pub open spec fn is_bit_set(&self, bit_index: int) -> bool {
    &&& 0 <= bit_index < self@.number_of_bits()
    &&& self@.set_bits.contains(bit_index)
}
```

`Bitmap::is_bit_set(i)` bundles `0 <= i < num_bits` into the predicate. This means:
- `Bitmap::is_bit_set(i)` ≠ `BitmapView::is_bit_set(i)` in general.
- You need a bridging lemma:

```rust
// bitmap_raw only
proof fn lemma_is_bit_set_equals_view(&self, i: int)
    requires self.inv(), 0 <= i < self@.number_of_bits(),
    ensures self.is_bit_set(i) == self@.is_bit_set(i),
```

- Every time you want to convert between the two levels, you pay a proof obligation.
- Loop invariants naturally want one or the other. Using the wrong one forces quantifier rewrites.

### The Solution in `bitmap_new`

`bitmap_new` defines `is_bit_set` only on `BitmapView`. Everywhere — contracts, invariants, proofs, tests — uses `self@.is_bit_set(i)`:

```rust
// Test in bitmap_raw uses Bitmap::is_bit_set
assert(bitmap.is_bit_set(index as int));

// Test in bitmap_new uses BitmapView::is_bit_set
assert(bitmap@.is_bit_set(index as int));
```

No bridging lemma. No semantic gap. One definition for one concept.

**This matters for SMT efficiency too**: the solver sees one symbol `BitmapView::is_bit_set` instead of two symbols that need constant equating. Fewer matching loops, more predictable trigger behavior.

---

## 5. Advantage 4: Composable Set-Algebraic Reasoning

Because all predicates in `bitmap_new` are on `BitmapView` (which is `(int, Set<int>)`), they compose via standard set algebra. Consider how lemmas chain:

```
BitmapView::exists_contiguous_free_range(n)
  └─ defined via has_free_range_at(start, n)
       └─ defined via all_bits_unset_in_range(start, start+n)
            └─ defined via is_bit_set(i)
                 └─ defined via set_bits.contains(i)
```

Every layer reduces to `Set<int>` operations. This means:

1. **vstd set_lib is directly applicable**: Lemmas like `lemma_int_range`, `lemma_len_subset`, `lemma_set_disjoint_lens` apply directly to `set_bits` without unwinding through a concrete type.

2. **New lemmas compose freely**: If someone adds a new BitmapView operation (`find_first_free`, `count_free_in_range`, etc.), they can prove its properties using existing BitmapView lemmas — no need to touch `Bitmap` or its proofs.

3. **Hypothetical reasoning**: You can construct a `BitmapView` value in a proof and reason about it without ever creating a `Bitmap`:

```rust
// Possible in bitmap_new: reason about an abstract state
let hypothetical = BitmapView { num_bits: 64, set_bits: old_view.set_bits.insert(x) };
hypothetical.lemma_insert_preserves_usage_bound(y);
```

In `bitmap_raw`, this requires `Bitmap::inv()`, which requires actual `bits`, `usage`, `next_free` fields — you can't just conjure a `Bitmap` in a proof context.

---

## 6. Advantage 5: Cleaner Proof Architecture

The split creates a natural **proof stratification**:

```
Layer 3: Client code (tests, callers)
    │   Interacts only with: BitmapView predicates + Bitmap::inv()
    │   Example: assert(bitmap@.all_bits_set_in_range(start, end))
    │
Layer 2: Representation coupling (lib.proof.rs on Bitmap)
    │   Proves: byte operations ↔ set_bits changes
    │   Example: OR-ing a byte → set_bits.insert(idx)
    │   Calls: Layer 1 lemmas on the view
    │
Layer 1: Abstract specification (lib.spec.rs on BitmapView)
    │   Proves: set-theoretic properties
    │   Example: insert preserves usage bound
    │   Uses: vstd set_lib only
    │
Layer 0: vstd (set_lib, set axioms)
```

In `bitmap_raw`, layers 1 and 2 are interleaved in the same `impl Bitmap` block. There's no clear boundary between "this lemma is about sets" and "this lemma is about bytes." This makes the proof harder to:
- **Read**: You must trace each lemma to understand whether it's abstract or concrete.
- **Maintain**: Changing a byte-level detail risks breaking an abstract lemma (even if logically independent).
- **Extend**: Adding a new operation requires understanding the full ~1169-line proof file.

In `bitmap_new`, the boundary is physical: `lib.spec.rs` (Layer 1) vs `lib.proof.rs` (Layer 2). You can read and modify each independently.

---

## 7. Advantage 6: The Invariant Split Enables Spec-Level Reasoning Without Unfolding

In `bitmap_raw`:

```rust
pub closed spec fn inv(&self) -> bool { /* everything */ }
```

Since `inv()` is `closed`, the SMT solver can never see its contents. When a lemma requires `inv()` as a precondition and wants to conclude `wf()`, the solver needs a separate lemma or assertion to extract that fact.

In `bitmap_new`:

```rust
pub open spec fn inv(&self) -> bool {
    &&& self@.wf()
    &&& self.internal_inv()
}
```

Since `inv()` is `open`, the solver can immediately deduce `self@.wf()` from `self.inv()`. This means:
- Any caller with `self.inv()` can invoke BitmapView lemmas that require `wf()` **without extra proof steps**.
- The solver doesn't need to unfold `internal_inv()` for abstract reasoning — it only needs `wf()`, which is already visible.

This is a **proof-engineering win**: fewer manual lemma invocations, faster solver times, and less risk of solver timeout on large invariant unfoldings.

---

## 8. Summary Table

| Property | `bitmap_raw` | `bitmap_new` |
|----------|-------------|-------------|
| Can swap representation without re-proving abstract lemmas? | No | **Yes** |
| Can call abstract lemmas in intermediate states? | No (requires `inv()`) | **Yes** (requires only `wf()`) |
| Single definition of `is_bit_set`? | No (two definitions + bridging lemma) | **Yes** |
| Can reason about hypothetical BitmapViews? | No (need concrete Bitmap) | **Yes** |
| vstd set_lib directly usable? | Indirectly (through Bitmap wrappers) | **Directly** |
| Clear boundary between abstract/concrete proofs? | No (interleaved in one file) | **Yes** (spec vs proof files) |
| Solver can extract `wf()` from `inv()` automatically? | No (`inv` is closed) | **Yes** (`inv` is open) |

The set-theoretic abstraction in `bitmap_new` is not just an organizational preference — it's a **verification architecture** that separates what a bitmap *means* (a bounded set of integers) from how it's *implemented* (a byte array with bookkeeping). This separation pays off in proof reuse, maintainability, solver efficiency, and extensibility.
