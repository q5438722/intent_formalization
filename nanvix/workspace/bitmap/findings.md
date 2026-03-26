# Bitmap Spec Completeness Findings

## Overview

Pipeline v4 identified **2 spec completeness gaps** in the Nanvix `Bitmap` module (`src/libs/bitmap/src/lib.rs`).

**Method:** For each hypothesized gap, we construct a `proof fn` where:
- `requires` encodes the function's preconditions **plus a bad scenario** (behavior that *should* be impossible).
- `ensures` is an **exact copy** of the function's spec postconditions.
- If Verus **verifies** the proof fn, the spec does not exclude the bad scenario → **spec incomplete**.
- If Verus **rejects** it, the spec successfully excludes the bad scenario → **spec complete**.

---

## Finding 1: `new` lacks a liveness guarantee

### Spec (current)

```rust
#[verus_spec(result =>
    ensures
        result matches Ok(bitmap) ==> {
            &&& bitmap.inv()
            &&& bitmap@.num_bits == number_of_bits as int
            &&& bitmap@.is_empty()
        },
        number_of_bits == 0 ==> result is Err,
        number_of_bits >= u32::MAX ==> result is Err,
        number_of_bits % (u8::BITS as usize) != 0 ==> result is Err,
)]
pub fn new(number_of_bits: usize) -> Result<Self, Error>
```

### Test case

```rust
proof fn phi_new_no_liveness(number_of_bits: usize, result: Result<Bitmap, Error>)
    requires
        // Valid input
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
        // Bad scenario: returns Err despite valid input
        result is Err,
    ensures
        // Exact copy of new()'s spec ensures
        result matches Ok(bitmap) ==> {
            &&& bitmap.inv()
            &&& bitmap@.num_bits == number_of_bits as int
            &&& bitmap@.is_empty()
        },
        number_of_bits == 0 ==> result is Err,
        number_of_bits >= u32::MAX as usize ==> result is Err,
        number_of_bits % (u8::BITS as usize) != 0 ==> result is Err,
{
}
```

**Result:** Verus verifies this — **spec incomplete**.

### Problem

The spec lists three conditions under which `new` *must* return `Err`, but never states that it *must* return `Ok` when those conditions are absent. All four ensures clauses are vacuously satisfied:

1. `result matches Ok(bitmap) ==> ...` — vacuously true because `result is Err`.
2. `number_of_bits == 0 ==> result is Err` — antecedent false (n > 0).
3. `number_of_bits >= u32::MAX ==> result is Err` — antecedent false (n < MAX).
4. `number_of_bits % 8 != 0 ==> result is Err` — antecedent false (n % 8 == 0).

A conforming implementation could reject every input. Callers cannot prove that `new(n)` ever returns `Ok`.

### Suggested fix

Add a liveness clause:

```rust
number_of_bits > 0 && number_of_bits < u32::MAX as usize
    && number_of_bits % (u8::BITS as usize) == 0
    ==> result is Ok,
```

---

## Finding 2: `alloc` lacks fairness / progress

### Spec (current)

```rust
#[verus_spec(result =>
    requires
        old(self).inv(),
    ensures
        self.inv(),
        match result {
            Ok(index) => {
                &&& 0 <= index < self@.num_bits
                &&& self@.num_bits == old(self)@.num_bits
                &&& !old(self)@.is_bit_set(index as int)
                &&& self@.is_bit_set(index as int)
                &&& forall|i: int|
                    0 <= i < self@.num_bits && i != index
                        ==> self@.is_bit_set(i) == old(self)@.is_bit_set(i)
                &&& self@.set_bits == old(self)@.set_bits.insert(index as int)
                &&& self@.usage() == old(self)@.usage() + 1
            },
            Err(_) => {
                &&& old(self)@.is_full()
                &&& self@ == old(self)@
            },
        },
)]
pub fn alloc(&mut self) -> Result<usize, Error>
```

### Test case

```rust
proof fn phi_alloc_always_zero(
    pre: Bitmap, post: Bitmap, result: Result<usize, Error>
)
    requires
        pre.inv(),
        pre@.num_bits > 1,
        !pre@.is_bit_set(0),
        !pre@.is_bit_set(1),
        // Bad scenario: always returns index 0
        result == Ok::<usize, Error>(0usize),
        post.inv(),
        post@.num_bits == pre@.num_bits,
        post@.is_bit_set(0),
        forall|i: int| 0 < i < post@.num_bits
            ==> post@.is_bit_set(i) == pre@.is_bit_set(i),
        post@.set_bits == pre@.set_bits.insert(0int),
        post@.usage() == pre@.usage() + 1,
    ensures
        // Exact copy of alloc()'s spec ensures
        post.inv(),
        match result {
            Ok(index) => {
                &&& 0 <= index < post@.num_bits
                &&& post@.num_bits == pre@.num_bits
                &&& !pre@.is_bit_set(index as int)
                &&& post@.is_bit_set(index as int)
                &&& forall|i: int|
                    0 <= i < post@.num_bits && i != index
                        ==> post@.is_bit_set(i) == pre@.is_bit_set(i)
                &&& post@.set_bits == pre@.set_bits.insert(index as int)
                &&& post@.usage() == pre@.usage() + 1
            },
            Err(_) => {
                &&& pre@.is_full()
                &&& post@ == pre@
            },
        },
{
}
```

**Result:** Verus verifies this — **spec incomplete**.

### Problem

The `Ok` branch ensures that `alloc` returns *some* free bit index with a correct frame condition, but it does not constrain *which* free bit is chosen. A conforming implementation could always return index 0 whenever bit 0 is free, starving all other positions.

In the body, `alloc` delegates to `alloc_range(1)`, which uses a next-fit strategy starting from `self.next_free`. This provides progress in practice, but the spec does not capture it. Callers performing sequential allocations cannot prove they will eventually fill the bitmap.

### Discussion

Whether this is a bug depends on the intended abstraction level. If `alloc` is meant to be a nondeterministic "pick any free bit" operation, the spec is correct by design. If callers need progress guarantees (e.g., to prove an allocator can serve N requests on an N-bit bitmap), the spec is insufficient.

A possible strengthening (if progress is desired):

```rust
// alloc returns the lowest-indexed free bit
Ok(index) => {
    &&& forall|j: int| 0 <= j < index ==> old(self)@.is_bit_set(j)
    ...
}
```

---

## Summary

| Finding | Function | Gap | Severity |
|---------|----------|-----|----------|
| 1 | `new` | No liveness: valid input may return `Err` | High — callers cannot prove construction succeeds |
| 2 | `alloc` | No fairness: may always return same index | Medium — depends on caller requirements |

## Verified complete (no gap found)

| Function | Tested scenario | Result |
|----------|----------------|--------|
| `alloc_range` | Free range exists → Err | Rejected (Err requires `!exists_contiguous_free_range`) |
| `alloc` | Non-full bitmap → Err | Rejected (Err requires `is_full()`) |
| `set` | Err + state mutated | Rejected (Err requires `*self == *old(self)`) |
| `clear` | Err + state mutated | Rejected (Err requires `*self == *old(self)`) |
