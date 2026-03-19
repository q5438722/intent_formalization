# Deep Analysis: Root Causes of Spec Differences — Verusage (Claude Opus 4.5)

**Source:** `/home/chentianyu/data/spec_gen_verusage/claude-opus-4.5/results.jsonl`
**Scope:** 255 cases where generated spec's `requires`/`ensures` clauses semantically differ from ground truth

---

## 1. Root Cause Taxonomy

We classify each difference into a primary root cause based on how the generated spec deviates from the ground truth:

| Category | Subcategory | Count | % | Description |
|----------|-------------|-------|---|-------------|
| **Near-Equivalent (Minor Syntactic)** | | **8** | **3.1%** | |
| | Trigger Placement | 4 | 1.6% | Only difference is `#[trigger]` annotation placement in quantifiers |
| | Clause Reordering | 4 | 1.6% | Same clauses in requires/ensures but listed in different order |
| **Wrong Postcondition (ensures)** | | **77** | **30.2%** | |
| | Wrong Ensures | 57 | 22.4% | The ensures clause expresses a fundamentally different postcondition |
| | Partial Ensures Match | 14 | 5.5% | Some ensures clauses match, but others are different or missing |
| | Overly Strong Ensures | 1 | 0.4% | Ensures clause asserts more than ground truth (stricter) |
| | Too Weak Ensures | 4 | 1.6% | Ensures clause asserts less than ground truth (weaker) |
| | Trivial Ensures (`true`) | 1 | 0.4% | Model generated `ensures true` instead of meaningful postcondition |
| **Wrong Precondition (requires)** | | **34** | **13.3%** | |
| | Wrong Requires | 25 | 9.8% | Requires clause has different preconditions |
| | Too Weak Requires | 7 | 2.7% | Missing preconditions from ground truth (may allow invalid inputs) |
| | Missing Requires | 2 | 0.8% | No requires clause when ground truth has one |
| **Both Requires and Ensures Wrong** | | **134** | **52.5%** | |
| | Both Wrong | 52 | 20.4% | Both requires and ensures differ from ground truth |
| | Stronger Requires + Wrong Ensures | 46 | 18.0% | Added unnecessary preconditions and wrong postcondition |
| | Weaker Requires + Wrong Ensures | 31 | 12.2% | Missing preconditions and wrong postcondition |
| | Stronger Req + Weaker Ens | 5 | 2.0% | Over-constrained input with under-specified output |
| **Other** | | **2** | **0.8%** | |
| | Operator Mismatch | 2 | 0.8% | Differs only in operators (e.g., `!=` vs `>`, `<==>` vs `==`) |

## 2. Near-Equivalent Cases (Arguably Correct)

These 8 cases differ only in minor syntactic aspects that do not change the logical meaning of the spec.

### 2.1 Trigger Placement Differences

**4 cases** differ only in where `#[trigger]` is placed in quantified expressions. This does not change the logical semantics of the spec itself (though it may affect Verus's proof search strategy).

**Case 69** — `always_implies_forall_intro` (Anvil)

```
// Generated requires:
forall |a: A| spec.entails(#[trigger] always(p.implies(a_to_q(a)))),
// Ground truth requires:
forall |a: A| #[trigger] spec.entails(always(p.implies(a_to_q(a)))),
```

**Case 107** — `leads_to_exists_intro` (Anvil)

```
// Generated requires:
forall |a: A| spec.entails(#[trigger] a_to_p(a).leads_to(q)),
// Ground truth requires:
forall |a: A| #[trigger] spec.entails(a_to_p(a).leads_to(q)),
```

**Case 139** — `spec_entails_always_tla_forall` (Anvil)

```
// Generated requires:
forall |a: A| spec.entails(#[trigger] always(a_to_p(a))),
// Ground truth requires:
forall |a: A| spec.entails(always(#[trigger] a_to_p(a))),
```

**Case 342** — `lemma_map_distribute_auto` (Memory-Allocator)

```
// Generated ensures:
forall|s1: Set<S>, s2: Set<S>, f: spec_fn(S) -> T|
            #[trigger] s1.union(s2).map(f) == s1.map(f).union(s2.map(f)),
// Ground truth ensures:
forall|s1: Set<S>, s2: Set<S>, f: spec_fn(S) -> T| s1.union(s2).map(f) == #[trigger] s1.map(f).union(s2.map(f))
```

### 2.2 Clause Reordering

**4 cases** have the same set of clauses but in a different order. This is semantically equivalent since `requires` and `ensures` clauses are conjunctive.

**Case 105** — `leads_to_apply` (Anvil)

```
// Generated requires:
spec.entails(p.leads_to(q)),
        spec.entails(p),
// Ground truth requires:
spec.entails(p),
        spec.entails(p.leads_to(q)),
```

**Case 117** — `leads_to_weaken` (Anvil)

```
// Generated requires:
spec.entails(p1.leads_to(q1)),
        spec.entails(always(p2.implies(p1))),
        spec.entails(always(q1.implies(q2))),
// Ground truth requires:
spec.entails(always(p2.implies(p1))),
        spec.entails(always(q1.implies(q2))),
        spec.entails(p1.leads_to(q1)),
```

**Case 184** — `new` (IronKV)

```
// Generated ensures:
s.valid(),
            s@ == Map::<K, ID>::empty(),
// Ground truth ensures:
s.valid(),
            s@ == Map::<K,ID>::empty(),
```

**Case 436** — `lemma_interp_of_entry_contains_mapping_implies_interp_contains_mapping` (NRKernel)

```
// Generated ensures:
forall|va: nat, pte: PTE| #![auto] self.interp_of_entry(j).contains_pair(va, pte) ==> self.interp().contains_pair(va, pte),
            forall|va: nat| #![auto] self.interp_of_entry(j).contains_key(va) ==> self.interp().contains_key(va),
            //forall|va: nat| #![auto]
            //    self.
// Ground truth ensures:
forall|va: nat| #![auto] self.interp_of_entry(j).contains_key(va) ==> self.interp().contains_key(va),
            forall|va: nat, pte: PTE| #![auto] self.interp_of_entry(j).contains_pair(va, pte) ==> self.interp().contains_pair(va, pte),
            //forall|va: nat| #![auto]
            //    self.
```

## 3. Wrong Postcondition (ensures) — Deep Dive

**77 cases** have correct requires but wrong ensures. This is the most common class of error (requires matched but ensures didn't).

### 3.1 Common Patterns in Ensures Differences

| Sub-pattern | Count | Description |
|-------------|-------|-------------|
| Involves quantifiers | 30 | Differences in `forall`/`exists` structure |
| Missing ensures clauses | 76 | Ground truth has clauses not in generated |
| Extra ensures clauses | 73 | Generated has clauses not in ground truth |

### 3.2 Representative Examples

#### Partial Match (some clauses correct, others wrong) (14 cases)

**Case 95** — `filtered_size_is_one_means_only_one_such_value` (Anvil) — :x: Verification failed

Generated ensures:
```verus
(m.filter(f).len() == 1) ==> (exists |v: V| #[trigger] m.contains(v) && f(v) && m.filter(f).count(v) == 1),
```
Ground truth ensures:
```verus
(m.filter(f).len() == 1) ==
```
- **Extra in generated:** 1
- **Missing from generated:** 1

**Case 159** — `true_pred_on_all_element_equal_to_pred_on_all_index` (Anvil) — :x: Verification failed

Generated ensures:
```verus
forall |a: A| s.contains(a) ==> pred(a)
        <==>
        forall |i: int| 0 <= i < s.len() ==> pred(s[i]),
```
Ground truth ensures:
```verus
(forall |obj: A| #[trigger] s.contains(obj) ==> pred(obj)) <==> (forall |i: int| 0 <= i < s.len() ==> pred(s[i]))
```
- **Extra in generated:** 1
- **Missing from generated:** 1

**Case 172** — `choose_gap_violator` (IronKV) — :x: Verification failed

Generated ensures:
```verus
lo.lt_spec(r),
            r.lt_spec(hi),
            self@.contains_key(*r.get()),
```
Ground truth ensures:
```verus
lo.lt_spec(r) && r.lt_spec(hi) && self@.contains_key(*r.get()),
```
- **Extra in generated:** 3
- **Missing from generated:** 1

#### Completely Wrong Ensures (57 cases)

**Case 12** — `only_interferes_with_itself_equivalent_to_lifted_only_interferes_with_itself_action` (Anvil-Advanced) — :x: Verification failed

Generated ensures:
```verus
spec.entails(always(lifted_vrs_reconcile_request_only_interferes_with_itself_action(controller_id)))
        <==>
        forall |vrs: VReplicaSetView| spec.entails(always(lift_state(vrs_reconcile_request_only_interferes_with_itself(controller_id, vrs)))),
```
Ground truth ensures:
```verus
spec.entails(always(tla_forall(|vrs: VReplicaSetView| 
            lift_state(vrs_reconcile_request_only_interferes_with_itself(controller_id, vrs)))))
        <==>
            spec.entails(always(lifted_vrs_reconcile_request_only_interferes_with_itself_action(controller_id)))
```
- **Extra in generated:** 1
- **Missing from generated:** 1

**Case 65** — `always_and_equality` (Anvil) — :x: Verification failed

Generated ensures:
```verus
always(p).and(always(q)) == always(p.and(q)),
```
Ground truth ensures:
```verus
always(p.and(q)) == always(p).and(always(q)),
```
- **Extra in generated:** 1
- **Missing from generated:** 1

**Case 66** — `always_distributed_by_and` (Anvil) — :white_check_mark: Verified

Generated ensures:
```verus
valid(always(p).and(always(q)).implies(always(p.and(q)))),
```
Ground truth ensures:
```verus
valid(always(p.and(q)).implies(always(p).and(always(q)))),
```
- **Extra in generated:** 1
- **Missing from generated:** 1

## 4. Wrong Precondition (requires) — Deep Dive

**36 cases** have wrong requires (with correct ensures). The model often struggles with identifying the exact preconditions needed.

### 4.1 Overly Strong Requires (Extra Preconditions)

**0 cases.** The model adds preconditions not in the ground truth. This makes the spec more restrictive than necessary — the proof would work for fewer inputs.

### 4.2 Too Weak Requires (Missing Preconditions)

**7 cases.** The model omits necessary preconditions from the ground truth. This could lead to unsound specs.

**Case 104** — `leads_to_always_tla_forall` (Anvil) — :x: Verification failed

Generated requires:
```verus
forall |a: A| domain.contains(a) ==> spec.entails(p.leads_to(always(a_to_p(a)))),
```
Ground truth requires:
```verus
forall |a: A| spec.entails(p.leads_to(always(#[trigger] a_to_p(a)))),
        domain.finite(),
        domain.len() > 0,
        forall |a: A| #[trigger] domain.contains(a),
```

**Case 121** — `next_preserves_inv_rec` (Anvil) — :x: Verification failed

Generated requires:
```verus
inv.satisfied_by(ex),
        forall |idx: nat| next.satisfied_by(ex.suffix(idx)),
        forall |any_ex: Execution<T>| inv.satisfied_by(any_ex) && next.satisfied_by(any_ex) ==> inv.satisfied_by(any_ex.suffix(1)),
```
Ground truth requires:
```verus
inv.satisfied_by(ex),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| inv.satisfied_by(#[trigger] ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx))
            ==> inv.satisfied_by(ex.suffix(idx + 1)),
```

**Case 142** — `strengthen_next` (Anvil) — :x: Verification failed

Generated requires:
```verus
spec.entails(always(lift_action(next))),
        spec.entails(always(lift_state(inv))),
        forall |s, s_next| inv(s) && #[trigger] next(s, s_next) ==> next_and_inv(s, s_next),
```
Ground truth requires:
```verus
spec.entails(always(lift_action(next))),
        spec.entails(always(lift_state(inv))),
        lift_action(next_and_inv).entails(lift_action(next).and(lift_state(inv))),
        lift_action(next).and(lift_state(inv)).entails(lift_action(next_and_inv)),
```

### 4.3 Wrong Requires (Different Conditions)

**25 cases.** The requires clause has different conditions entirely.

**Case 64** — `a_to_temp_pred_equality` (Anvil) — :x: Verification failed

Generated requires:
```verus
forall |a: A| #[trigger] p(a).entails(q(a)),
        forall |a: A| #[trigger] q(a).entails(p(a)),
```
Ground truth requires:
```verus
forall |a: A| #[trigger] p(a).entails(q(a)) && q(a).entails(p(a)),
```

**Case 81** — `always_weaken` (Anvil) — :x: Verification failed

Generated requires:
```verus
spec.entails(always(p)),
        p.entails(q),
```
Ground truth requires:
```verus
valid(p.implies(q)),
        spec.entails(always(p)),
```

**Case 90** — `entails_implies_leads_to` (Anvil) — :white_check_mark: Verified

Generated requires:
```verus
valid(p.implies(q)),
```
Ground truth requires:
```verus
p.entails(q),
```

## 5. Both Requires and Ensures Wrong — Deep Dive

**134 cases** have both requires and ensures differing from ground truth. This is the largest category.

### 5.1 Subcategory Breakdown

| Subcategory | Count | Description |
|-------------|-------|-------------|
| Both wrong (no clear pattern) | 52 | |
| Stronger requires + wrong ensures | 46 | |
| Weaker requires + wrong ensures | 31 | |
| Over-constrained input, under-specified output | 5 | |

### 5.2 Representative Examples

**Case 13** — `vrs_rely_condition_equivalent_to_lifted_vrs_rely_condition` (Anvil-Advanced) — :white_check_mark: Verified — _stronger_req_weaker_ens_

Generated requires:
```verus
spec.entails(always(lift_state(|s: ClusterState|
```
Ground truth requires:
```verus
// (none)
```
Generated ensures:
```verus
spec.entails(always(lifted_vrs_rely_condition(cluster, controller_id))),
```
Ground truth ensures:
```verus
(forall |other_id| cluster.controller_models.remove(controller_id).contains_key(other_id)
            ==> spec.entails(always(lift_state(#[trigger] vrs_rely(other_id)))))
        <==>
            spec.entails(always(lifted_vrs_rely_condition(cluster, controller_id))),
```

**Case 14** — `vrs_rely_condition_equivalent_to_lifted_vrs_rely_condition_action` (Anvil-Advanced) — :x: Verification failed — _wrong_both_stronger_req_

Generated requires:
```verus
spec.entails(always(lifted_vrs_rely_condition_action(cluster, controller_id))),
```
Ground truth requires:
```verus
// (none)
```
Generated ensures:
```verus
forall |other_id| cluster.controller_models.remove(controller_id).contains_key(other_id)
            ==> spec.entails(always(lift_state(#[trigger] vrs_rely(other_id)))),
```
Ground truth ensures:
```verus
(forall |other_id| cluster.controller_models.remove(controller_id).contains_key(other_id)
            ==> spec.entails(always(lift_state(#[trigger] vrs_rely(other_id)))))
        <==>
            spec.entails(always(lifted_vrs_rely_condition_action(cluster, controller_id))),
```

**Case 70** — `always_implies_preserved_by_always` (Anvil) — :x: Verification failed — _wrong_both_stronger_req_

Generated requires:
```verus
spec.entails(always(p.implies(q))),
        spec.entails(always(p)),
```
Ground truth requires:
```verus
spec.entails(always(p.implies(q))),
```
Generated ensures:
```verus
spec.entails(always(q)),
```
Ground truth ensures:
```verus
spec.entails(always(always(p).implies(always(q)))),
```

**Case 75** — `always_p_or_eventually_q` (Anvil) — :x: Verification failed — _wrong_both_stronger_req_

Generated requires:
```verus
p.satisfied_by(ex),
        always(next).satisfied_by(ex),
        always(p.and(next).implies(later(p).or(later(q)))).satisfied_by(ex),
```
Ground truth requires:
```verus
always(p.and(next).implies(later(p).or(later(q)))).satisfied_by(ex),
        always(next).satisfied_by(ex),
```
Generated ensures:
```verus
always(p).satisfied_by(ex) || eventually(q).satisfied_by(ex),
```
Ground truth ensures:
```verus
always(p.implies(always(p).or(eventually(q)))).satisfied_by(ex),
```

**Case 76** — `always_p_or_eventually_q_rec` (Anvil) — :x: Verification failed — _wrong_both_weaker_req_

Generated requires:
```verus
forall |j: nat| j < i ==> p.satisfied_by(ex.suffix(j)),
```
Ground truth requires:
```verus
forall |idx| p.satisfied_by(ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx)) ==> p.satisfied_by(ex.suffix(idx + 1)) || q.satisfied_by(ex.suffix(idx + 1)),
        forall |idx| next.satisfied_by(#[trigger] ex.suffix(idx)),
        forall |idx| !q.satisfied_by(#[trigger] ex.suffix(idx)),
        p.satisfied_by(ex),
```
Generated ensures:
```verus
(forall |j: nat| p.satisfied_by(ex.suffix(j))) || (exists |j: nat| q.satisfied_by(ex.suffix(j))),
```
Ground truth ensures:
```verus
p.satisfied_by(ex.suffix(i)),
```

## 6. Root Cause Distribution by Project

### Anvil (55 cases)

| Root Cause | Count | % |
|------------|-------|---|
| wrong_ensures | 17 | 31% |
| wrong_requires | 11 | 20% |
| wrong_both_stronger_req | 8 | 15% |
| wrong_both | 6 | 11% |
| too_weak_requires | 5 | 9% |
| trigger_placement | 3 | 5% |
| partial_ensures | 2 | 4% |
| clause_reorder | 2 | 4% |
| wrong_both_weaker_req | 1 | 2% |

### Anvil-Advanced (3 cases)

| Root Cause | Count | % |
|------------|-------|---|
| wrong_ensures | 1 | 33% |
| stronger_req_weaker_ens | 1 | 33% |
| wrong_both_stronger_req | 1 | 33% |

### IronKV (75 cases)

| Root Cause | Count | % |
|------------|-------|---|
| wrong_ensures | 24 | 32% |
| wrong_both_stronger_req | 14 | 19% |
| wrong_both | 10 | 13% |
| wrong_both_weaker_req | 9 | 12% |
| partial_ensures | 6 | 8% |
| wrong_requires | 4 | 5% |
| stronger_req_weaker_ens | 3 | 4% |
| too_weak_ensures | 2 | 3% |
| clause_reorder | 1 | 1% |
| missing_requires | 1 | 1% |
| overly_strong_ensures | 1 | 1% |

### Memory-Allocator (63 cases)

| Root Cause | Count | % |
|------------|-------|---|
| wrong_both | 16 | 25% |
| wrong_both_stronger_req | 11 | 17% |
| wrong_ensures | 10 | 16% |
| wrong_both_weaker_req | 9 | 14% |
| wrong_requires | 7 | 11% |
| partial_ensures | 4 | 6% |
| too_weak_requires | 2 | 3% |
| trivial_ensures | 1 | 2% |
| operator_mismatch | 1 | 2% |
| trigger_placement | 1 | 2% |
| stronger_req_weaker_ens | 1 | 2% |

### NRKernel (38 cases)

| Root Cause | Count | % |
|------------|-------|---|
| wrong_both | 12 | 32% |
| wrong_both_stronger_req | 7 | 18% |
| wrong_both_weaker_req | 7 | 18% |
| wrong_ensures | 5 | 13% |
| wrong_requires | 2 | 5% |
| operator_mismatch | 1 | 3% |
| missing_requires | 1 | 3% |
| partial_ensures | 1 | 3% |
| clause_reorder | 1 | 3% |
| too_weak_ensures | 1 | 3% |

### Node-Replication (21 cases)

| Root Cause | Count | % |
|------------|-------|---|
| wrong_both | 8 | 38% |
| wrong_both_weaker_req | 5 | 24% |
| wrong_both_stronger_req | 5 | 24% |
| partial_ensures | 1 | 5% |
| wrong_requires | 1 | 5% |
| too_weak_ensures | 1 | 5% |

## 7. Cross-Cutting Patterns

### 7.1 Quantifier Handling Issues

**115 out of 255 differing cases (45%)** involve quantified expressions (`forall`/`exists`). Common sub-issues:

- **Trigger placement:** The model places `#[trigger]` on different sub-expressions than the ground truth
- **Quantifier scope:** The model sometimes uses different variable bindings or quantifier nesting
- **tla_forall vs explicit forall:** In temporal logic specs, the model sometimes uses `tla_forall` where explicit `forall` is expected, or vice versa

### 7.2 Strength Calibration

- **Overly strong specs:** 52 cases — the model adds unnecessary constraints. This is "safe" but limits applicability.
- **Too weak specs:** 49 cases — the model omits necessary constraints. This is potentially unsound.
- The model tends to be slightly more often overly strong than too weak on requires, but the dominant issue is generating entirely different conditions.

### 7.3 Verification Correlation

- **wrong_ensures:** 17/57 verified OK (30%)
- **partial_ensures:** 5/14 verified OK (36%)
- **wrong_requires:** 5/25 verified OK (20%)
- **wrong_both:** 12/52 verified OK (23%)

## 8. Summary & Conclusions

Out of 520 total cases, 255 had semantically different specs (after extracting code and comparing `requires`/`ensures` clauses). The root causes break down as follows:

| Root Cause Group | Count | % of All Diffs |
|------------------|-------|----------------|
| Near-equivalent (trigger/reorder) | 8 | 3.1% |
| Wrong ensures only | 77 | 30.2% |
| Wrong requires only | 36 | 14.1% |
| Both wrong | 134 | 52.5% |
| Other | 0 | 0.0% |

### Key Takeaways

1. **The dominant failure mode is wrong postconditions (ensures):** Whether alone or combined with wrong requires, the ensures clause is wrong in the vast majority of cases. The model often captures part of the postcondition correctly but misses or alters key clauses.
2. **Near-equivalents are rare:** Only 8 cases (3.1%) are near-equivalent (differing only in trigger placement or clause ordering). Including these with the 108 semantically equivalent cases brings the effective equivalence rate to 32.0%.
3. **Quantifier handling is a major challenge:** 45% of all differing cases involve quantified expressions, suggesting the model struggles with the complex logical structure of Verus specs.
4. **The model tends toward overly strong requires:** When the requires clause differs, the model more often adds extra preconditions (52 cases) than omits them (49 cases). While this is safer, it indicates the model is being conservative.
5. **Project-specific challenges:** Anvil has the highest success rate for spec generation, likely because its temporal logic patterns are more formulaic. IronKV and Memory-Allocator have more diverse spec patterns that the model handles less consistently.
6. **Verification is a poor proxy for correctness:** Even among the 255 cases with wrong specs, 67 (26%) passed verification, because a wrong-but-provable spec is still verifiable. Conversely, a correct spec with incomplete proof body will fail verification.

## Appendix: All 255 Differing Cases (Case-by-Case)

| # | Line | Project | Target Function | Primary Root Cause | Req Diff | Ens Diff | Verified |
|---|------|---------|-----------------|-------------------|----------|----------|----------|
| 1 | 12 | Anvil-Advanced | `only_interferes_with_itself_equivalent_t...` | wrong_ensures |  | Yes | No |
| 2 | 13 | Anvil-Advanced | `vrs_rely_condition_equivalent_to_lifted_...` | stronger_req_weaker_ens | Yes | Yes | Yes |
| 3 | 14 | Anvil-Advanced | `vrs_rely_condition_equivalent_to_lifted_...` | wrong_both_stronger_req | Yes | Yes | No |
| 4 | 64 | Anvil | `a_to_temp_pred_equality` | wrong_requires | Yes |  | No |
| 5 | 65 | Anvil | `always_and_equality` | wrong_ensures |  | Yes | No |
| 6 | 66 | Anvil | `always_distributed_by_and` | wrong_ensures |  | Yes | Yes |
| 7 | 69 | Anvil | `always_implies_forall_intro` | trigger_placement | Yes |  | No |
| 8 | 70 | Anvil | `always_implies_preserved_by_always` | wrong_both_stronger_req | Yes | Yes | No |
| 9 | 72 | Anvil | `always_lift_action_unfold` | wrong_ensures |  | Yes | Yes |
| 10 | 73 | Anvil | `always_lift_state_unfold` | wrong_ensures |  | Yes | No |
| 11 | 75 | Anvil | `always_p_or_eventually_q` | wrong_both_stronger_req | Yes | Yes | No |
| 12 | 76 | Anvil | `always_p_or_eventually_q_rec` | wrong_both_weaker_req | Yes | Yes | No |
| 13 | 81 | Anvil | `always_weaken` | wrong_requires | Yes |  | No |
| 14 | 82 | Anvil | `commutativity_of_seq_map_and_filter` | wrong_both | Yes | Yes | No |
| 15 | 85 | Anvil | `eliminate_always` | wrong_ensures |  | Yes | No |
| 16 | 86 | Anvil | `empty_filter_implies_seq_pred_false_on_a...` | wrong_ensures |  | Yes | No |
| 17 | 90 | Anvil | `entails_implies_leads_to` | wrong_requires | Yes |  | Yes |
| 18 | 95 | Anvil | `filtered_size_is_one_means_only_one_such...` | partial_ensures |  | Yes | No |
| 19 | 96 | Anvil | `filtered_size_is_zero_means_no_such_valu...` | wrong_ensures |  | Yes | No |
| 20 | 97 | Anvil | `finite_set_to_seq_contains_all_set_eleme...` | wrong_ensures |  | Yes | Yes |
| 21 | 99 | Anvil | `init_invariant` | wrong_requires | Yes |  | No |
| 22 | 100 | Anvil | `init_invariant_rec` | wrong_both | Yes | Yes | No |
| 23 | 102 | Anvil | `leads_to_always_combine` | wrong_both | Yes | Yes | No |
| 24 | 103 | Anvil | `leads_to_always_enhance` | wrong_requires | Yes |  | No |
| 25 | 104 | Anvil | `leads_to_always_tla_forall` | too_weak_requires | Yes |  | No |
| 26 | 105 | Anvil | `leads_to_apply` | clause_reorder | Yes |  | No |
| 27 | 106 | Anvil | `leads_to_by_borrowing_inv` | wrong_requires | Yes |  | No |
| 28 | 107 | Anvil | `leads_to_exists_intro` | trigger_placement | Yes |  | No |
| 29 | 110 | Anvil | `leads_to_rank_step_one_help` | wrong_both | Yes | Yes | No |
| 30 | 112 | Anvil | `leads_to_rank_step_one_usize_help` | wrong_requires | Yes |  | No |
| 31 | 114 | Anvil | `leads_to_shortcut_temp` | wrong_both | Yes | Yes | No |
| 32 | 115 | Anvil | `leads_to_stable` | wrong_requires | Yes |  | No |
| 33 | 117 | Anvil | `leads_to_weaken` | clause_reorder | Yes |  | No |
| 34 | 118 | Anvil | `len_is_zero_means_count_for_each_value_i...` | wrong_both_stronger_req | Yes | Yes | Yes |
| 35 | 119 | Anvil | `map_values_to_set_eq_to_set_mk_map_value...` | wrong_ensures |  | Yes | No |
| 36 | 121 | Anvil | `next_preserves_inv_rec` | too_weak_requires | Yes |  | No |
| 37 | 126 | Anvil | `pack_conditions_to_spec` | wrong_both_stronger_req | Yes | Yes | No |
| 38 | 127 | Anvil | `push_filter_and_filter_push` | wrong_ensures |  | Yes | No |
| 39 | 129 | Anvil | `seq_equal_preserved_by_add` | wrong_both_stronger_req | Yes | Yes | Yes |
| 40 | 130 | Anvil | `seq_equal_preserved_by_add_prefix` | wrong_both_stronger_req | Yes | Yes | Yes |
| 41 | 132 | Anvil | `seq_filter_is_a_subset_of_original_seq` | wrong_ensures |  | Yes | No |
| 42 | 134 | Anvil | `seq_pred_false_on_all_elements_implies_e...` | wrong_requires | Yes |  | Yes |
| 43 | 135 | Anvil | `seq_pred_false_on_all_elements_is_equiva...` | wrong_ensures |  | Yes | Yes |
| 44 | 137 | Anvil | `seq_unequal_preserved_by_add_auto` | wrong_ensures |  | Yes | Yes |
| 45 | 138 | Anvil | `simplify_predicate` | wrong_both_stronger_req | Yes | Yes | Yes |
| 46 | 139 | Anvil | `spec_entails_always_tla_forall` | trigger_placement | Yes |  | No |
| 47 | 142 | Anvil | `strengthen_next` | too_weak_requires | Yes |  | No |
| 48 | 145 | Anvil | `tla_exists_equality` | wrong_ensures |  | Yes | No |
| 49 | 148 | Anvil | `tla_forall_a_p_leads_to_q_a_is_stable` | wrong_both_stronger_req | Yes | Yes | No |
| 50 | 150 | Anvil | `tla_forall_always_equality_variant` | wrong_requires | Yes |  | No |
| 51 | 152 | Anvil | `tla_forall_and_equality` | wrong_ensures |  | Yes | No |
| 52 | 153 | Anvil | `tla_forall_implies_equality1` | wrong_ensures |  | Yes | No |
| 53 | 156 | Anvil | `tla_forall_not_equality` | wrong_ensures |  | Yes | No |
| 54 | 159 | Anvil | `true_pred_on_all_element_equal_to_pred_o...` | partial_ensures |  | Yes | No |
| 55 | 161 | Anvil | `unpack_conditions_from_spec` | wrong_both | Yes | Yes | No |
| 56 | 163 | Anvil | `vacuous_leads_to` | too_weak_requires | Yes |  | No |
| 57 | 165 | Anvil | `wf1` | too_weak_requires | Yes |  | No |
| 58 | 166 | Anvil | `wf1_variant_temp` | wrong_requires | Yes |  | No |
| 59 | 172 | IronKV | `choose_gap_violator` | partial_ensures |  | Yes | No |
| 60 | 173 | IronKV | `erase` | wrong_both_weaker_req | Yes | Yes | No |
| 61 | 175 | IronKV | `greatest_lower_bound_index` | wrong_both | Yes | Yes | Yes |
| 62 | 176 | IronKV | `keys_in_index_range_agree` | partial_ensures |  | Yes | Yes |
| 63 | 177 | IronKV | `mind_the_gap` | wrong_ensures |  | Yes | Yes |
| 64 | 178 | IronKV | `new` | wrong_both_weaker_req | Yes | Yes | Yes |
| 65 | 179 | IronKV | `set` | too_weak_ensures |  | Yes | No |
| 66 | 180 | IronKV | `values_agree` | wrong_both | Yes | Yes | Yes |
| 67 | 181 | IronKV | `all_keys_agree` | wrong_both | Yes | Yes | No |
| 68 | 182 | IronKV | `almost_all_keys_agree` | wrong_both | Yes | Yes | No |
| 69 | 183 | IronKV | `empty_key_range_is_consistent` | wrong_requires | Yes |  | No |
| 70 | 184 | IronKV | `new` | clause_reorder |  | Yes | Yes |
| 71 | 186 | IronKV | `range_consistent_subset` | wrong_requires | Yes |  | No |
| 72 | 187 | IronKV | `set` | partial_ensures |  | Yes | No |
| 73 | 188 | IronKV | `delegate_for_key_range_is_host_impl` | wrong_ensures |  | Yes | Yes |
| 74 | 189 | IronKV | `vec_erase` | wrong_both | Yes | Yes | No |
| 75 | 191 | IronKV | `effect_of_delegation_map_set` | wrong_both_weaker_req | Yes | Yes | No |
| 76 | 200 | IronKV | `process_received_packet_next` | wrong_both_stronger_req | Yes | Yes | No |
| 77 | 201 | IronKV | `real_init_impl` | missing_requires | Yes |  | No |
| 78 | 204 | IronKV | `make_empty_event_results` | too_weak_ensures |  | Yes | Yes |
| 79 | 205 | IronKV | `make_send_only_event_results` | wrong_requires | Yes |  | No |
| 80 | 207 | IronKV | `deserialize` | wrong_ensures |  | Yes | No |
| 81 | 208 | IronKV | `lemma_same_views_serialize_the_same` | stronger_req_weaker_ens | Yes | Yes | No |
| 82 | 209 | IronKV | `lemma_serialization_is_not_a_prefix_of` | wrong_both_stronger_req | Yes | Yes | No |
| 83 | 213 | IronKV | `deserialize` | wrong_ensures |  | Yes | No |
| 84 | 214 | IronKV | `lemma_serialization_is_not_a_prefix_of` | wrong_both_stronger_req | Yes | Yes | No |
| 85 | 216 | IronKV | `serialize` | wrong_ensures |  | Yes | No |
| 86 | 217 | IronKV | `serialized_size` | wrong_ensures |  | Yes | No |
| 87 | 218 | IronKV | `lemma_serialization_is_not_a_prefix_of` | wrong_both_stronger_req | Yes | Yes | No |
| 88 | 220 | IronKV | `lemma_serialization_is_not_a_prefix_of` | wrong_both_stronger_req | Yes | Yes | No |
| 89 | 222 | IronKV | `serialize` | wrong_ensures |  | Yes | No |
| 90 | 223 | IronKV | `lemma_same_views_serialize_the_same` | wrong_both_stronger_req | Yes | Yes | No |
| 91 | 224 | IronKV | `lemma_serialization_is_not_a_prefix_of` | wrong_both_stronger_req | Yes | Yes | No |
| 92 | 226 | IronKV | `lemma_view_equal_symmetric` | wrong_both_stronger_req | Yes | Yes | No |
| 93 | 227 | IronKV | `serialize` | wrong_both_stronger_req | Yes | Yes | Yes |
| 94 | 228 | IronKV | `deserialize` | wrong_ensures |  | Yes | No |
| 95 | 230 | IronKV | `lemma_same_views_serialize_the_same` | stronger_req_weaker_ens | Yes | Yes | No |
| 96 | 231 | IronKV | `lemma_serialization_is_not_a_prefix_of` | wrong_both_stronger_req | Yes | Yes | No |
| 97 | 233 | IronKV | `lemma_view_equal_symmetric` | wrong_both_stronger_req | Yes | Yes | No |
| 98 | 234 | IronKV | `serialize` | wrong_ensures |  | Yes | No |
| 99 | 235 | IronKV | `serialized_size` | wrong_ensures |  | Yes | No |
| 100 | 236 | IronKV | `lemma_same_views_serialize_the_same` | wrong_both_stronger_req | Yes | Yes | No |
| 101 | 237 | IronKV | `lemma_serialization_is_not_a_prefix_of` | wrong_both_stronger_req | Yes | Yes | No |
| 102 | 239 | IronKV | `lemma_view_equal_symmetric` | wrong_both_stronger_req | Yes | Yes | No |
| 103 | 244 | IronKV | `sht_marshal_data_injective` | wrong_ensures |  | Yes | Yes |
| 104 | 246 | IronKV | `endpoints_contain` | wrong_ensures |  | Yes | Yes |
| 105 | 248 | IronKV | `test_unique` | wrong_ensures |  | Yes | Yes |
| 106 | 249 | IronKV | `maybe_ack_packet_impl` | wrong_both | Yes | Yes | Yes |
| 107 | 250 | IronKV | `receive_ack_impl` | wrong_ensures |  | Yes | No |
| 108 | 251 | IronKV | `receive_impl` | wrong_both_weaker_req | Yes | Yes | No |
| 109 | 253 | IronKV | `retransmit_un_acked_packets_for_dst` | wrong_both_weaker_req | Yes | Yes | No |
| 110 | 254 | IronKV | `send_single_cmessage` | wrong_both_weaker_req | Yes | Yes | No |
| 111 | 255 | IronKV | `same_view_same_marshalable` | overly_strong_ensures |  | Yes | No |
| 112 | 257 | IronKV | `lemma_seqno_in_un_acked_list` | wrong_ensures |  | Yes | No |
| 113 | 258 | IronKV | `truncate` | stronger_req_weaker_ens | Yes | Yes | No |
| 114 | 259 | IronKV | `insert` | wrong_both_weaker_req | Yes | Yes | No |
| 115 | 260 | IronKV | `un_acked_messages_extend` | wrong_both_weaker_req | Yes | Yes | Yes |
| 116 | 262 | IronKV | `lemma_filter_skip_rejected` | wrong_both | Yes | Yes | No |
| 117 | 263 | IronKV | `lemma_flatten_set_seq_spec` | wrong_ensures |  | Yes | No |
| 118 | 264 | IronKV | `lemma_fold_left_append_merge` | partial_ensures |  | Yes | No |
| 119 | 266 | IronKV | `lemma_if_everything_in_seq_satisfies_fil...` | wrong_requires | Yes |  | No |
| 120 | 267 | IronKV | `lemma_if_nothing_in_seq_satisfies_filter...` | wrong_both | Yes | Yes | No |
| 121 | 268 | IronKV | `lemma_map_set_singleton_auto` | wrong_ensures |  | Yes | Yes |
| 122 | 269 | IronKV | `lemma_seq_fold_left_append_len_int` | partial_ensures |  | Yes | No |
| 123 | 270 | IronKV | `lemma_seq_fold_left_append_len_int_le` | wrong_both | Yes | Yes | No |
| 124 | 271 | IronKV | `lemma_seq_fold_left_sum_le` | wrong_both | Yes | Yes | No |
| 125 | 272 | IronKV | `lemma_seq_fold_left_sum_len_int_positive` | wrong_ensures |  | Yes | No |
| 126 | 275 | IronKV | `lemma_to_set_distributes_over_addition` | wrong_ensures |  | Yes | No |
| 127 | 276 | IronKV | `lemma_to_set_singleton_auto` | wrong_ensures |  | Yes | No |
| 128 | 277 | IronKV | `lemma_to_set_union_auto` | wrong_ensures |  | Yes | Yes |
| 129 | 280 | IronKV | `map_fold_ok` | wrong_ensures |  | Yes | No |
| 130 | 281 | IronKV | `map_set_finite_auto` | wrong_ensures |  | Yes | Yes |
| 131 | 282 | IronKV | `set_map_union` | wrong_ensures |  | Yes | No |
| 132 | 283 | IronKV | `set_map_union_auto` | partial_ensures |  | Yes | Yes |
| 133 | 284 | IronKV | `some_differing_index_for_unequal_seqs` | wrong_both_weaker_req | Yes | Yes | No |
| 134 | 286 | Memory-Allocator | `bin_size_result_mul8` | wrong_both_weaker_req | Yes | Yes | No |
| 135 | 287 | Memory-Allocator | `bounds_for_smallest_bin_fitting_size` | wrong_requires | Yes |  | No |
| 136 | 289 | Memory-Allocator | `div2` | trivial_ensures | Yes | Yes | Yes |
| 137 | 290 | Memory-Allocator | `idx_in_range_has_bin_size` | wrong_requires | Yes |  | No |
| 138 | 291 | Memory-Allocator | `idx_out_of_range_has_different_bin_size` | wrong_ensures |  | Yes | No |
| 139 | 292 | Memory-Allocator | `leading_zeros_between` | wrong_both | Yes | Yes | Yes |
| 140 | 293 | Memory-Allocator | `leading_zeros_between_powers_of_2` | wrong_both | Yes | Yes | No |
| 141 | 294 | Memory-Allocator | `leading_zeros_monotonic` | wrong_both_stronger_req | Yes | Yes | No |
| 142 | 295 | Memory-Allocator | `leading_zeros_powers_of_2` | wrong_both | Yes | Yes | No |
| 143 | 297 | Memory-Allocator | `lemma_div_is_ordered` | wrong_requires | Yes |  | Yes |
| 144 | 298 | Memory-Allocator | `log2` | partial_ensures |  | Yes | Yes |
| 145 | 299 | Memory-Allocator | `mul_assoc` | wrong_ensures |  | Yes | No |
| 146 | 300 | Memory-Allocator | `mul_ordering` | wrong_both_weaker_req | Yes | Yes | No |
| 147 | 301 | Memory-Allocator | `out_of_small_range` | wrong_both_weaker_req | Yes | Yes | No |
| 148 | 305 | Memory-Allocator | `pow2_properties` | wrong_ensures |  | Yes | Yes |
| 149 | 306 | Memory-Allocator | `pow2_subtracts` | wrong_both | Yes | Yes | No |
| 150 | 307 | Memory-Allocator | `result2_idx_in_range_has_bin_size` | partial_ensures |  | Yes | No |
| 151 | 308 | Memory-Allocator | `result2_idx_out_of_range_has_different_b...` | wrong_ensures |  | Yes | No |
| 152 | 309 | Memory-Allocator | `result_bin` | wrong_both | Yes | Yes | No |
| 153 | 310 | Memory-Allocator | `result_bounds_for_smallest_bitting_size` | wrong_both_stronger_req | Yes | Yes | No |
| 154 | 311 | Memory-Allocator | `result_idx_in_range_has_bin_size` | wrong_both_stronger_req | Yes | Yes | No |
| 155 | 312 | Memory-Allocator | `result_idx_out_of_range_has_different_bi...` | wrong_both_stronger_req | Yes | Yes | No |
| 156 | 313 | Memory-Allocator | `result_sbin` | wrong_both_stronger_req | Yes | Yes | No |
| 157 | 314 | Memory-Allocator | `result_sbin_bounds` | wrong_both_stronger_req | Yes | Yes | Yes |
| 158 | 315 | Memory-Allocator | `result_sbin_idx_smallest_sbin_fitting_si...` | wrong_both_stronger_req | Yes | Yes | No |
| 159 | 316 | Memory-Allocator | `result_smallest_bin_fitting_size_size_of...` | wrong_both_stronger_req | Yes | Yes | No |
| 160 | 317 | Memory-Allocator | `shift_is_div` | wrong_both | Yes | Yes | No |
| 161 | 318 | Memory-Allocator | `size_gt_8_implies_idx_gt_1` | wrong_requires | Yes |  | Yes |
| 162 | 319 | Memory-Allocator | `size_le_8_implies_idx_eq_1` | wrong_requires | Yes |  | Yes |
| 163 | 320 | Memory-Allocator | `size_of_bin_bounds` | wrong_ensures |  | Yes | No |
| 164 | 321 | Memory-Allocator | `size_of_bin_bounds_not_huge` | wrong_both | Yes | Yes | Yes |
| 165 | 322 | Memory-Allocator | `size_of_bin_mult_word_size` | wrong_ensures |  | Yes | No |
| 166 | 324 | Memory-Allocator | `valid_sbin_idx_smallest_sbin_fitting_siz...` | wrong_requires | Yes |  | No |
| 167 | 326 | Memory-Allocator | `any_set` | operator_mismatch |  | Yes | No |
| 168 | 328 | Memory-Allocator | `create` | too_weak_requires | Yes |  | No |
| 169 | 330 | Memory-Allocator | `create_full` | wrong_ensures |  | Yes | Yes |
| 170 | 334 | Memory-Allocator | `is_full` | wrong_both_weaker_req | Yes | Yes | No |
| 171 | 335 | Memory-Allocator | `lemma_change_one_entry` | wrong_both_weaker_req | Yes | Yes | No |
| 172 | 336 | Memory-Allocator | `lemma_view` | partial_ensures |  | Yes | Yes |
| 173 | 337 | Memory-Allocator | `next_run` | wrong_both | Yes | Yes | No |
| 174 | 339 | Memory-Allocator | `lemma_bitmask_to_is_bit_set` | wrong_both_weaker_req | Yes | Yes | Yes |
| 175 | 340 | Memory-Allocator | `lemma_is_bit_set` | partial_ensures |  | Yes | Yes |
| 176 | 342 | Memory-Allocator | `lemma_map_distribute_auto` | trigger_placement |  | Yes | Yes |
| 177 | 344 | Memory-Allocator | `lemma_obtain_bit_index_1_aux` | wrong_both | Yes | Yes | No |
| 178 | 345 | Memory-Allocator | `lemma_obtain_bit_index_2` | wrong_both | Yes | Yes | Yes |
| 179 | 347 | Memory-Allocator | `lemma_obtain_bit_index_3_aux` | wrong_both | Yes | Yes | No |
| 180 | 348 | Memory-Allocator | `set_int_range_commit_size` | wrong_both_weaker_req | Yes | Yes | Yes |
| 181 | 350 | Memory-Allocator | `bitand_with_mask_gives_rounding` | wrong_both | Yes | Yes | Yes |
| 182 | 351 | Memory-Allocator | `block_ptr_aligned_to_word` | wrong_both | Yes | Yes | No |
| 183 | 352 | Memory-Allocator | `block_size_ge_word` | stronger_req_weaker_ens | Yes | Yes | Yes |
| 184 | 353 | Memory-Allocator | `block_start_at_diff` | wrong_both_stronger_req | Yes | Yes | No |
| 185 | 355 | Memory-Allocator | `align_down` | wrong_both_stronger_req | Yes | Yes | No |
| 186 | 356 | Memory-Allocator | `align_up` | wrong_ensures |  | Yes | No |
| 187 | 363 | Memory-Allocator | `segment_start_mult_commit_size` | wrong_ensures |  | Yes | Yes |
| 188 | 364 | Memory-Allocator | `sub_distribute` | wrong_ensures |  | Yes | No |
| 189 | 365 | Memory-Allocator | `two_mul_with_bit0` | wrong_both | Yes | Yes | Yes |
| 190 | 366 | Memory-Allocator | `two_mul_with_bit1` | wrong_both_weaker_req | Yes | Yes | No |
| 191 | 368 | Memory-Allocator | `nat_set_size` | wrong_both | Yes | Yes | No |
| 192 | 369 | Memory-Allocator | `pigeonhole_missing_idx_implies_double` | wrong_both_stronger_req | Yes | Yes | No |
| 193 | 370 | Memory-Allocator | `pigeonhole_missing_idx_implies_double_he...` | wrong_both | Yes | Yes | No |
| 194 | 371 | Memory-Allocator | `pigeonhole_too_many_elements_implies_dou...` | wrong_requires | Yes |  | No |
| 195 | 372 | Memory-Allocator | `set_mismatch` | wrong_both_weaker_req | Yes | Yes | Yes |
| 196 | 373 | Memory-Allocator | `singleton_set_unique_elt` | too_weak_requires | Yes |  | No |
| 197 | 374 | Node-Replication | `rids_match_add_none` | wrong_both_weaker_req | Yes | Yes | Yes |
| 198 | 375 | Node-Replication | `rids_match_add_rid` | partial_ensures |  | Yes | No |
| 199 | 376 | Node-Replication | `rids_match_pop` | wrong_both_stronger_req | Yes | Yes | Yes |
| 200 | 377 | Node-Replication | `log_entry_alive_value_wrap_around` | wrong_both_stronger_req | Yes | Yes | No |
| 201 | 378 | Node-Replication | `log_entry_alive_wrap_around_helper` | wrong_both | Yes | Yes | Yes |
| 202 | 379 | Node-Replication | `map_min_value_smallest` | wrong_both | Yes | Yes | No |
| 203 | 380 | Node-Replication | `pop_rid` | wrong_both_weaker_req | Yes | Yes | No |
| 204 | 381 | Node-Replication | `state_at_version_preserves` | wrong_both_weaker_req | Yes | Yes | No |
| 205 | 383 | Node-Replication | `LogRangeMatchesQueue_append` | wrong_both | Yes | Yes | No |
| 206 | 384 | Node-Replication | `LogRangeMatchesQueue_append_other` | wrong_both | Yes | Yes | No |
| 207 | 385 | Node-Replication | `LogRangeMatchesQueue_append_other_augmen...` | wrong_both | Yes | Yes | No |
| 208 | 386 | Node-Replication | `LogRangeMatchesQueue_update_change` | wrong_both | Yes | Yes | No |
| 209 | 387 | Node-Replication | `LogRangeMatchesQueue_update_change_2` | wrong_both | Yes | Yes | No |
| 210 | 388 | Node-Replication | `LogRangeNoNodeId_append_other` | wrong_both_weaker_req | Yes | Yes | No |
| 211 | 390 | Node-Replication | `combiner_request_ids_not_contains` | wrong_both_stronger_req | Yes | Yes | No |
| 212 | 391 | Node-Replication | `compute_nrstate_at_version_preserves` | wrong_both_stronger_req | Yes | Yes | Yes |
| 213 | 392 | Node-Replication | `concat_LogRangeNoNodeId_LogRangeMatchesQ...` | wrong_requires | Yes |  | No |
| 214 | 395 | Node-Replication | `max_of_set` | wrong_both_stronger_req | Yes | Yes | No |
| 215 | 396 | Node-Replication | `state_at_version_refines` | wrong_both_weaker_req | Yes | Yes | No |
| 216 | 397 | Node-Replication | `int_mod_less_than_same` | wrong_both | Yes | Yes | Yes |
| 217 | 398 | Node-Replication | `map_new_rec_dom_finite` | too_weak_ensures |  | Yes | Yes |
| 218 | 406 | NRKernel | `lemma_entry_sizes_aligned_auto` | wrong_both | Yes | Yes | No |
| 219 | 407 | NRKernel | `lemma_entry_sizes_increase` | operator_mismatch | Yes |  | No |
| 220 | 408 | NRKernel | `lemma_maxphyaddr_facts` | wrong_ensures |  | Yes | Yes |
| 221 | 409 | NRKernel | `lemma_new_seq` | wrong_ensures |  | Yes | No |
| 222 | 410 | NRKernel | `aligned_transitive` | wrong_requires | Yes |  | No |
| 223 | 411 | NRKernel | `aligned_transitive_auto` | missing_requires | Yes |  | Yes |
| 224 | 412 | NRKernel | `assert_maps_equal_contains_pair` | wrong_both | Yes | Yes | Yes |
| 225 | 413 | NRKernel | `lemma_aligned_iff_eq_mul_div` | wrong_ensures |  | Yes | No |
| 226 | 416 | NRKernel | `mod_add_zero` | wrong_both | Yes | Yes | No |
| 227 | 417 | NRKernel | `mod_mult_zero_implies_mod_zero` | wrong_both | Yes | Yes | No |
| 228 | 418 | NRKernel | `subtract_mod_eq_zero` | wrong_both | Yes | Yes | No |
| 229 | 419 | NRKernel | `lemma_entry_base_from_index` | wrong_both_stronger_req | Yes | Yes | No |
| 230 | 421 | NRKernel | `lemma_index_from_base_and_addr` | wrong_both_stronger_req | Yes | Yes | No |
| 231 | 422 | NRKernel | `lemma_empty_implies_interp_aux_empty` | wrong_both | Yes | Yes | Yes |
| 232 | 424 | NRKernel | `lemma_entries_interp_equal_implies_inter...` | wrong_both | Yes | Yes | No |
| 233 | 426 | NRKernel | `lemma_entries_interp_insert_implies_inte...` | wrong_both | Yes | Yes | No |
| 234 | 428 | NRKernel | `lemma_entries_interp_remove_implies_inte...` | wrong_both | Yes | Yes | No |
| 235 | 429 | NRKernel | `lemma_interp_aux_between` | wrong_both_stronger_req | Yes | Yes | No |
| 236 | 430 | NRKernel | `lemma_interp_aux_contains_implies_interp...` | wrong_both_stronger_req | Yes | Yes | No |
| 237 | 432 | NRKernel | `lemma_interp_entries_insert_implies_inte...` | wrong_requires | Yes |  | No |
| 238 | 433 | NRKernel | `lemma_interp_entries_remove_implies_inte...` | wrong_both_stronger_req | Yes | Yes | No |
| 239 | 434 | NRKernel | `lemma_interp_of_entry_between` | wrong_both_weaker_req | Yes | Yes | No |
| 240 | 435 | NRKernel | `lemma_interp_of_entry_contains_mapping_i...` | partial_ensures |  | Yes | No |
| 241 | 436 | NRKernel | `lemma_interp_of_entry_contains_mapping_i...` | clause_reorder |  | Yes | No |
| 242 | 437 | NRKernel | `lemma_interp_of_entry_disjoint_mappings` | wrong_both | Yes | Yes | No |
| 243 | 442 | NRKernel | `lemma_new_empty_dir` | too_weak_ensures |  | Yes | Yes |
| 244 | 443 | NRKernel | `lemma_nonempty_implies_interp_contains` | wrong_both_weaker_req | Yes | Yes | No |
| 245 | 444 | NRKernel | `address` | wrong_both_weaker_req | Yes | Yes | Yes |
| 246 | 445 | NRKernel | `lemma_addr_mask_when_hp_pat_is_zero` | wrong_both | Yes | Yes | No |
| 247 | 446 | NRKernel | `lemma_new_entry_addr_mask_is_address` | wrong_both | Yes | Yes | No |
| 248 | 448 | NRKernel | `lemma_zero_entry_facts` | wrong_both_weaker_req | Yes | Yes | No |
| 249 | 450 | NRKernel | `lemma_aligned_addr_mask_facts` | wrong_both_weaker_req | Yes | Yes | Yes |
| 250 | 452 | NRKernel | `lemma_bitvector_facts_simple` | wrong_ensures |  | Yes | No |
| 251 | 453 | NRKernel | `lemma_page_aligned_implies_mask_dir_addr...` | wrong_both_stronger_req | Yes | Yes | Yes |
| 252 | 461 | NRKernel | `interp_vmem_subrange` | wrong_both_weaker_req | Yes | Yes | No |
| 253 | 462 | NRKernel | `interp_vmem_update_range` | wrong_both_weaker_req | Yes | Yes | Yes |
| 254 | 465 | NRKernel | `lemma_inflight_vaddr_implies_hl_unmap_or...` | wrong_ensures |  | Yes | No |
| 255 | 468 | NRKernel | `monotonic_candidate_mapping_overlaps_exi...` | wrong_both_stronger_req | Yes | Yes | Yes |
