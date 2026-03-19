# Bitmap Specification Comparison: `bitmap_raw` vs `bitmap_new`

This document compares the specifications in `bitmap_raw` (the original Verus-native version) and `bitmap_new` (the refactored version using attribute macros). Both implement the same bitmap allocator but differ significantly in specification style and organization.

---

## 1. `lib.spec.rs` Comparison

### 1.1 `BitmapView` Struct

Both versions define the same abstract model:

```rust
pub struct BitmapView {
    pub num_bits: int,
    pub set_bits: Set<int>,
}
```

### 1.2 `BitmapView` Methods — Shared (with minor differences)

| Method | `bitmap_raw` | `bitmap_new` | Difference |
|--------|-------------|-------------|------------|
| `usage()` | ✅ | ✅ | Identical |
| `has_free_bit()` | ✅ | ✅ | Identical |
| `is_full()` | ✅ | ✅ | Identical |
| `is_empty()` | `self.set_bits =~= Set::empty()` | `self.set_bits == Set::<int>::empty()` | Raw uses extensional equality (`=~=`), new uses standard equality (`==`) with explicit type annotation |
| `is_bit_set()` | ✅ | ✅ | Identical |
| `range_set()` | ✅ | ✅ | Identical |
| `wf()` | ✅ | ✅ | Identical |

### 1.3 Methods Only in `bitmap_raw` (removed from `BitmapView` in new)

| Method | Description |
|--------|-------------|
| `number_of_bits()` | Returns `self.num_bits`. New version accesses `num_bits` field directly instead. |
| `count_free()` | Returns `self.number_of_bits() - self.usage()`. Removed in new version. |

### 1.4 Methods Only in `bitmap_new` (moved from `Bitmap` to `BitmapView`)

The new version moves several spec functions **from `Bitmap` to `BitmapView`**, making them operate on the abstract model rather than the concrete struct:

| Method | Description |
|--------|-------------|
| `all_bits_set_in_range(start, end)` | Checks all bits in `[start, end)` are set |
| `all_bits_unset_in_range(start, end)` | Checks all bits in `[start, end)` are unset |
| `has_free_range_at(start, n)` | Checks contiguous free range starting at `start` |
| `exists_contiguous_free_range(n)` | Checks if any contiguous free range of size `n` exists |

### 1.5 Lemmas Only in `bitmap_new` (added to `BitmapView`)

The new version adds many proof lemmas directly to `BitmapView`:

| Lemma | Purpose |
|-------|---------|
| `lemma_set_bits_finite()` | Proves `set_bits` is finite when `wf()` holds |
| `lemma_range_set_finite(lo, hi)` | Proves `range_set(lo, hi)` is finite |
| `lemma_range_set_len(lo, hi)` | Proves `range_set(lo, hi).len() == hi - lo` |
| `lemma_free_range_implies_usage_bound(p, n)` | Free range of size `n` implies `usage <= num_bits - n` |
| `lemma_insert_preserves_usage_bound(x)` | Inserting one element preserves usage bound |
| `lemma_usage_equals_number_of_bits_implies_full()` | Full usage means all bits set |
| `lemma_usage_less_than_capacity_means_not_full()` | Sub-full usage means not full |
| `lemma_unset_bit_implies_has_free_bit(i)` | Unset bit implies `has_free_bit()` |
| `lemma_all_bits_set_means_full()` | All bits set implies `is_full()` |
| `lemma_has_free_bit_implies_exists_free_range_1()` | `has_free_bit` implies `exists_contiguous_free_range(1)` |
| `lemma_set_bits_equal_has_free_range_at_equal()` | Equal set_bits → equal `has_free_range_at` results |
| `lemma_set_bits_equal_exists_free_range_equal()` | Equal set_bits → equal `exists_contiguous_free_range` results |

In `bitmap_raw`, these lemmas presumably live in `lib.proof.rs` and are on `Bitmap` rather than `BitmapView`.

### 1.6 `View` Implementation for `Bitmap`

Both are identical in structure — mapping `Bitmap` to `BitmapView` via `bit_at`.

### 1.7 `Bitmap` Spec Functions

| Spec Function | `bitmap_raw` | `bitmap_new` | Difference |
|---------------|-------------|-------------|------------|
| `bit_at()` | `pub open spec fn` | `spec fn` (private) | New version makes `bit_at` private |
| `is_bit_set()` | On `Bitmap` | **Removed** (on `BitmapView` only) | New version uses `self@.is_bit_set()` instead of `self.is_bit_set()` |
| `all_bits_set_in_range()` | On `Bitmap` | **Moved to `BitmapView`** | Same |
| `all_bits_unset_in_range()` | On `Bitmap` | **Moved to `BitmapView`** | Same |
| `has_free_range_at()` | On `Bitmap` | **Moved to `BitmapView`** | Same |
| `exists_contiguous_free_range()` | On `Bitmap` | **Moved to `BitmapView`** | Same |

### 1.8 `Bitmap::inv()` — Invariant

| Aspect | `bitmap_raw` | `bitmap_new` |
|--------|-------------|-------------|
| Structure | Single `closed spec fn inv()` | Split: `pub open spec fn inv()` = `wf() && internal_inv()` |
| Visibility | `closed` | `inv()` is `open`, `internal_inv()` is `closed` |
| `bits.inv()` | Not checked | **Added**: `self.bits.inv()` is checked in `internal_inv()` |
| Field access | Uses `self@.number_of_bits()` accessor | Uses `self@.num_bits` field directly |

**`bitmap_raw` `inv()`:**
```rust
pub closed spec fn inv(&self) -> bool {
    &&& self@.number_of_bits() > 0
    &&& self@.number_of_bits() == self.bits@.len() * (u8::BITS as int)
    &&& self@.number_of_bits() < u32::MAX as int
    &&& self@.wf()
    &&& self@.set_bits.finite()
    &&& self@.usage() <= self@.number_of_bits()
    &&& self.number_of_bits as int == self@.number_of_bits()
    &&& self.usage as int == self@.usage()
    &&& self.next_free as int <= self@.number_of_bits()
}
```

**`bitmap_new` `inv()` + `internal_inv()`:**
```rust
pub open spec fn inv(&self) -> bool {
    &&& self@.wf()
    &&& self.internal_inv()
}

pub closed spec fn internal_inv(&self) -> bool {
    &&& self.bits.inv()                          // NEW
    &&& self@.num_bits > 0
    &&& self@.num_bits == self.bits@.len() * (u8::BITS as int)
    &&& self@.num_bits < u32::MAX as int
    &&& self@.wf()
    &&& self@.set_bits.finite()
    &&& self@.usage() <= self@.num_bits
    &&& self.number_of_bits as int == self@.num_bits
    &&& self.usage as int == self@.usage()
    &&& self.next_free as int <= self@.num_bits
}
```

---

## 2. `lib.rs` Comparison

### 2.1 Module and Crate Structure

| Aspect | `bitmap_raw` | `bitmap_new` |
|--------|-------------|-------------|
| Crate attributes | None | `#![cfg_attr(not(feature = "std"), no_std)]`, `#![cfg_attr(all(test, feature = "std"), feature(random))]`, proc_macro_hygiene attrs |
| Test module | None | `mod test` under `#[cfg(all(test, feature = "std"))]` |
| Imports | `use crate::libs::raw_array::RawArray` | `use ::raw_array::RawArray` (external crate) |
| Error imports | `use crate::libs::error::{Error, ErrorCode}` | `use ::sys::error::{Error, ErrorCode}` |
| Include guards | None | `#[cfg(verus_keep_ghost)]` on spec/proof includes |
| Deref impl | None | `impl Deref for Bitmap` under `#[cfg(test)]` |

### 2.2 Struct Definition

| Aspect | `bitmap_raw` | `bitmap_new` |
|--------|-------------|-------------|
| Context | Inside `verus!{}` block | Outside `verus!{}` block |
| Attributes | `#[verifier::ext_equal]` | `#[verus_verify]`, `#[verus_verify(external_derive)]`, `#[derive(Debug)]` |
| `ext_equal` | Active | Commented out (`// TODO - Restore when supported by Verus PR #2239`) |
| Fields | Identical | Identical |

### 2.3 Specification Style (Fundamental Difference)

This is the **most significant architectural difference** between the two versions:

| Aspect | `bitmap_raw` | `bitmap_new` |
|--------|-------------|-------------|
| Specification syntax | Inline Verus syntax: `pub fn f() -> (result: T) requires ... ensures ...` | Attribute macro: `#[verus_spec(result => requires ... ensures ...)]` |
| Proof blocks | `proof { ... }` | `proof! { ... }` macro |
| Ghost declarations | `let ghost x = ...;` | `proof_decl! { let ghost x = ...; }` |
| Loop invariants | Inline: `while cond invariant ... decreases ...` | Attribute: `#[cfg_attr(verus_keep_ghost, verus_spec(invariant ...))]` |
| Loop invariant bodies | Written inline | Factored into helper spec functions (e.g., `self.alloc_range_first_loop_invariant(...)`) |
| For loops | Not supported; uses `while` | Uses `for alloc_offset in 0..size` with `#[cfg_attr]` spec |
| Return type | Named: `-> (result: Result<Self, Error>)` | Standard: `-> Result<Self, Error>` |

### 2.4 Function-by-Function Specification Comparison

#### `new(number_of_bits)`

| Aspect | `bitmap_raw` | `bitmap_new` |
|--------|-------------|-------------|
| Ensures (Ok) | `bitmap@.number_of_bits() == number_of_bits as int`, `bitmap@.is_empty()`, explicit `forall\|i\| ... !bitmap.is_bit_set(i)` | `bitmap@.num_bits == number_of_bits as int`, `bitmap@.is_empty()` (no explicit forall) |
| Error conditions | Three separate `==> result is Err` clauses | Same three `==> result is Err` clauses |
| Pattern | `result is Ok ==> { let bitmap = result->Ok_0; ... }` | `result matches Ok(bitmap) ==> { ... }` |

#### `from_raw_array(array)`

| Aspect | `bitmap_raw` | `bitmap_new` |
|--------|-------------|-------------|
| Requires | No `array.inv()` | **Adds** `array.inv()` |
| Ensures pattern | `result is Ok ==> {...}` + separate liveness `result is Ok` | Single `result matches Ok(bitmap) && { ... }` (liveness folded in) |
| `Empty set proof` | `Self::lemma_empty_set_finite()` called | Not called (handled differently) |

#### `alloc()`

| Aspect | `bitmap_raw` | `bitmap_new` |
|--------|-------------|-------------|
| Ensures pattern | `result is Ok ==> {...}`, `result is Err ==> self@ == old(self)@` | `match result { Ok(index) => {...}, Err(_) => { old(self)@.is_full() && self@ == old(self)@ } }` |
| Liveness | Separate: `old(self)@.has_free_bit() ==> result is Ok` | Folded into error: `Err` implies `is_full()` |
| Proof call | `old(self).lemma_has_free_bit_implies_exists_free_range_1()` | `old(self)@.lemma_has_free_bit_implies_exists_free_range_1()` (called on view) |

#### `alloc_range(size)`

| Aspect | `bitmap_raw` | `bitmap_new` |
|--------|-------------|-------------|
| Ensures pattern | `result is Ok ==> {...}`, `result is Err ==> {...}` | `match result { Ok(start) => {...}, Err(_) => {...} }` |
| Ok: range references | `self.all_bits_set_in_range(...)` (on Bitmap) | `self@.all_bits_set_in_range(...)` (on BitmapView) |
| Error: liveness | Separate: `old(self).exists_contiguous_free_range(...) ==> result is Ok` | Folded: `Err(_) => { !old(self)@.exists_contiguous_free_range(...) && ... }` |
| Set equality | `=~=` (extensional) | `==` (standard) |
| Internal proof helpers | Inline lemma calls with `as int` casts | Lemma calls use raw `usize` values |
| Loop structure | `while alloc_offset < size` | `for alloc_offset in 0..size` |
| Loop invariants | Inline | Factored into spec functions |
| Debug assert | None | `#[cfg(not(verus_keep_ghost))] debug_assert_eq!(...)` |

#### `set(index)`

| Aspect | `bitmap_raw` | `bitmap_new` |
|--------|-------------|-------------|
| Ensures pattern | `result is Ok ==> {...}`, `result is Err ==> *self == *old(self)` | `match result { Ok(()) => {...}, Err(_) => { index >= ... \|\| is_bit_set && *self == *old(self) } }` |
| Liveness | Separate clause | Folded into error match arm |
| Set equality | `=~=` | `==` |

#### `clear(index)`

| Aspect | `bitmap_raw` | `bitmap_new` |
|--------|-------------|-------------|
| Requires | **Has precondition**: `index < num_bits && is_bit_set(index)` | **No precondition** |
| Error handling | Precondition prevents error; separate liveness `result is Ok` | Error arm: `index >= num_bits \|\| !is_bit_set(index)` |
| Set equality | `=~=` | `==` |

#### `test(index)`

| Aspect | `bitmap_raw` | `bitmap_new` |
|--------|-------------|-------------|
| Pattern | Separate `result is Ok ==>`, `result is Err ==>`, liveness clause | `match result { Ok(b) => {...}, Err(_) => {...} }` |

#### `index(index)` and `index_unchecked(index)`

Both are functionally identical, differing only in specification syntax (inline vs attribute macro).

---

## 3. Summary of Key Differences

### 3.1 Architectural

1. **Specification Location**: New version moves range-related specs (`all_bits_set_in_range`, `has_free_range_at`, etc.) from `Bitmap` to `BitmapView`, separating abstract specifications from concrete implementation.
2. **Invariant Structure**: New version splits `inv()` into an open `inv()` + closed `internal_inv()`, allowing callers to see that `wf()` is part of the invariant.
3. **Lemma Placement**: New version adds many proof lemmas directly to `BitmapView` in `lib.spec.rs`, rather than keeping all proofs in `lib.proof.rs`.

### 3.2 Syntax/Style

1. **Verus Syntax**: Raw uses native Verus syntax inside `verus!{}` blocks; new uses `#[verus_spec]` attribute macros with `proof!{}` and `proof_decl!{}` macros.
2. **Set Equality**: Raw uses `=~=` (extensional); new uses `==` (standard).
3. **Pattern Matching**: Raw uses `result is Ok ==>` / `result is Err ==>`; new uses `match result { ... }`.
4. **Loop Invariants**: Raw writes inline; new factors into helper spec functions.
5. **For Loops**: New supports `for x in 0..n` with attribute-based specs.

### 3.3 Semantic

1. **`clear()` preconditions**: Raw requires callers prove the bit is set; new handles it as a runtime error.
2. **Error specifications**: New version makes error conditions more explicit (e.g., `Err` implies `is_full()` for `alloc`).
3. **`bits.inv()`**: New version adds `self.bits.inv()` to the invariant, validating the underlying `RawArray`.
4. **`bit_at` visibility**: Raw is `pub open`; new is private `spec fn`.
5. **`from_raw_array` precondition**: New adds `array.inv()` requirement.
