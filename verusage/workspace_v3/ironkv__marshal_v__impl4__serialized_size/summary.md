# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/workspace_v3/ironkv__marshal_v__impl4__serialized_size/original.rs`
**Date:** 2026-03-24T22:51:18Z

## Stats

- Candidates generated: 5
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: empty_vec_zero_size → `serialized_size`
- **Type:** boundary
- **Entailed:** ❌
- **Why flagged:** Empty vec should still have a nonzero length-prefix; zero size would mean the length header is absent from serialization.

### φ2: size_bounded_by_header_only → `serialized_size`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A non-empty vec's serialized size must exceed just the length prefix; this would mean element payloads contribute zero bytes.

### φ3: no_length_prefix_in_size → `serialized_size`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** Serialized size should include the length-prefix header; omitting it would produce an incorrect, shorter encoding.

### φ4: longer_vec_no_larger → `serialized_size`
- **Type:** behavioral
- **Entailed:** ❌
- **Why flagged:** A vec with strictly more elements of the same fixed-size type must serialize to a strictly larger blob; equal-or-smaller would be wrong.

### φ5: single_elem_equals_elem_size → `serialized_size`
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** A single-element vec's serialization must be larger than just the element itself because the length prefix is missing from this accounting.

