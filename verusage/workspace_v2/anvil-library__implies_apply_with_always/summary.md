# Spec Consistency Report

**Source:** `/home/chentianyu/intent_formalization/verusage/source-projects/anvil-library/verified/temporal_logic/implies_apply_with_always.rs`
**Date:** 2026-03-24T04:00:54Z
**Status:** NO_PHIS_VERIFIED

## Stats

- Candidates generated: 1
- Entailed (verified): 0
- True positives: 0
- False positives: 0

## All Candidates

### φ1: execution_equality_forces_fn_equality
- **Type:** logical
- **Entailed:** ❌
- **Why flagged:** execution_equality uses extensional equality (=~=) to collapse pointwise-equal spec_fn into intensional equality — structurally distinct closures become equal

