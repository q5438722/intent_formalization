# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/vstd_exd/map_lib/injective_finite_map_implies_dom_len_is_equal_to_values_len.rs`
**Date:** 2026-03-24T04:44:40Z
**Status:** COMPLETE

## Stats

- Candidates generated: 3
- Entailed (verified): 2
- True positives: 0
- False positives: 2

## Summary

Both candidates are false positives — standard boundary-case consequences of the injective map lemma applied to empty and singleton maps. No spec issues found.

## All Candidates

### φ1: injective_map_values_finite
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** the lemma ensures len equality which implicitly requires values().finite() — if values could be infinite with a defined len, this would be a gap

### φ2: empty_map_zero_values
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** empty map is trivially injective — tests whether the spec correctly handles the base case without producing nonsensical len for empty values set
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** Empty map has empty domain (len 0) and empty values set (len 0). This is a correct and expected base case property.

### φ3: singleton_map_values_len_one
- **Type:** boundary
- **Entailed:** ✅
- **Why flagged:** tests singleton boundary — if the recursive decomposition mishandles single-element maps, the values len could be wrong
- **Verdict:** FALSE_POSITIVE (high)
- **Reasoning:** A singleton map is injective with dom().len() == 1, so values().len() == 1 follows directly from the lemma. Correct and expected.

