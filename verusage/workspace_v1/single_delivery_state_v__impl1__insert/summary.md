# Test Summary: `single_delivery_state_v__impl1__insert`

## Target Specification

`CTombstoneTable::insert(&mut self, src: &EndPoint, last_seqno: u64)`:
- **Requires**: `old(self).abstractable()` ∧ `src@.valid_physical_address()`
- **Ensures**: `self@ =~= old(self)@.insert(src@, last_seqno as nat)` ∧ `self.abstractable()`

Where `abstractable` = all keys have `id.len() < 0x100000`.

## Results

All **15/15** adversarial tests **FAILED verification** as expected, confirming the specification correctly rejects invalid inputs, incorrect behaviors, and unintended reasoning.

### Boundary Tests (5/5 FAIL ✅)

| Test | Property Queried | Result |
|------|-----------------|--------|
| B1 | Endpoint at exact limit (len == 0x100000) is valid | FAIL ✅ |
| B2 | Oversized endpoint (len == 0x200000) is valid | FAIL ✅ |
| B3 | Table with invalid key is abstractable | FAIL ✅ |
| B4 | Insert invalid src preserves abstractable | FAIL ✅ |
| B5 | Empty table is NOT abstractable | FAIL ✅ |

### Behavioral Mutation Tests (5/5 FAIL ✅)

| Test | Property Queried | Result |
|------|-----------------|--------|
| M1 | After insert, value at key ≠ inserted value | FAIL ✅ |
| M2 | After insert, key is not present | FAIL ✅ |
| M3 | After insert, spurious unrelated key appears | FAIL ✅ |
| M4 | After insert of new key, table unchanged | FAIL ✅ |
| M5 | After insert, existing key's value changed | FAIL ✅ |

### Logical Tests (5/5 FAIL ✅)

| Test | Property Queried | Result |
|------|-----------------|--------|
| L1 | Derive `false` from spec assumptions (soundness) | FAIL ✅ |
| L2 | Insert is non-deterministic | FAIL ✅ |
| L3 | Abstractable implies table is empty | FAIL ✅ |
| L4 | All valid endpoints have the same id | FAIL ✅ |
| L5 | Double insert preserves first value (not second) | FAIL ✅ |

## Conclusion

The specification for `CTombstoneTable::insert` is **consistent** across all tested dimensions:
- **Boundary**: Correctly enforces `valid_physical_address` strict bound and `abstractable` invariant.
- **Behavioral**: Correctly specifies insert semantics (value stored, key present, no side effects on other keys).
- **Logical**: Sound (no false derivable), deterministic, and does not admit overly strong structural assumptions.

No specification weaknesses were detected.
