# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/storage/verified/pmem_subregion/subregion_new.rs`
**Date:** 2026-03-24T15:17:21Z
**Status:** COMPLETE

## Stats

- Candidates generated: 5
- Entailed (verified): 5
- True positives: 0
- False positives: 5

## Summary

All five candidates are false positives. They confirm correct properties: reflexivity of views_differ, subrange independence from outside bytes, permission check via reflexive instantiation, subrange-of-subrange idempotency, and symmetry of equality when nothing is writable. No spec gaps found in the `WriteRestrictedPersistentMemorySubregion::new` constructor or its supporting definitions.

## All Candidates

### φ1: new_ensures_inv_with_self_referential_perm
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** Any view differs from itself only where allowed (trivially true since no bytes differ) — but if the opaque_inv relied on non-trivial reasoning about the initial view equaling current, a reflexivity bug could propagate
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `views_differ_only_where_subregion_allows(v, v, ...)` is trivially true since `v.state[addr] == v.state[addr]` for all addresses. Reflexivity is a correct property of equality.

### φ2: subregion_view_independent_of_outside_bytes
- **Type:** behavioral
- **Entailed:** ✅
- **Why flagged:** The subregion view should depend only on bytes within [start, start+len) — if bytes outside the range leaked into the subregion, the write restriction would not properly isolate the subregion
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `get_subregion_view` uses `subrange(start, start+len)` which only depends on bytes in `[start, start+len)`. If those bytes are equal, the subranges are extensionally equal. Correct by the subrange axiom.

### φ3: perm_check_holds_for_current_crash_states
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** The current region's own crash states should satisfy the permission — if the identity view diff didn't trigger the precondition, the invariant would not cover the current state's crash consistency
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Instantiating the precondition's forall with `alt_region_view = wrpm@` works because `views_differ_only_where_subregion_allows(wrpm@, wrpm@, ...)` is trivially true (reflexivity). So `wrpm@.can_crash_as(crash_state)` triggers `perm.check_permission(crash_state)`. Correct by instantiation.

### φ4: get_subregion_view_idempotent
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** Taking a full-range subregion of a subregion should be idempotent — if double subranging changed the result, nested subregion creation would produce inconsistent views
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** `subrange(start, start+len).subrange(0, len)` equals `subrange(start, start+len)` by the vstd subrange-of-subrange axiom. Correct by definition.

### φ5: views_differ_symmetric_when_nothing_writable
- **Type:** logical
- **Entailed:** ✅
- **Why flagged:** When nothing is writable, views_differ should be symmetric — if one direction held but not the other, the subregion invariant would be asymmetrically constraining
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** With `is_writable = |_| false`, every address in `[start, start+len)` hits the `!is_writable_absolute_addr_fn(addr)` case, so `v1.state[addr] == v2.state[addr]` for all addresses in range. Equality is symmetric, so `v2.state[addr] == v1.state[addr]` holds too. Correct by symmetry of equality.

