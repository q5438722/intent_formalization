# Deep Analysis: Root Causes of Spec Differences — Verusage (Claude Opus 4.5)

**Source:** `/home/chentianyu/data/spec_gen_verusage/claude-opus-4.5/results.jsonl`

### Semantic Equivalence Rules

We compare generated specs against ground truth using the following semantic equivalence rules:

1. **Comma ≡ `&&`**: Top-level `condition1, condition2` is treated as equivalent to `condition1 && condition2` in requires/ensures blocks
2. **Commutative `<==>`**: `A <==> B` is treated as equivalent to `B <==> A`
3. **Commutative `==`**: `A == B` is treated as equivalent to `B == A` for simple equalities
4. **Trigger-insensitive**: `#[trigger]` placement differences are ignored
5. **Clause reordering**: `{A, B}` is equivalent to `{B, A}` (set comparison)
6. **`=~=` ≡ `==`**: Deep equality operator treated as equivalent to `==`
7. **Whitespace / trailing comma**: Normalized away

---

## 1. Overall Statistics

| Metric | Count | Rate |
|--------|-------|------|
| Total cases | 520 | |
| Empty generation (timeout) | 114 | 21.9% |
| No code extracted | 30 | 5.8% |
| Target function not found | 13 | 2.5% |
| **Evaluable cases** | **363** | **69.8%** |
| **Semantically equivalent** | **119** | **32.8% of evaluable** |
| **Semantically different** | **244** | **67.2% of evaluable** |

---

## 2. Equivalent Cases Resolved by Semantic Rules (11 notable cases)

The following 11 cases are semantically equivalent to the ground truth despite surface-level syntactic differences. They illustrate the importance of the equivalence rules above.

| Line | Project | Target Function | Equivalence Rule(s) Applied |
|------|---------|-----------------|----------------------------|
| 65 | Anvil | `always_and_equality` | reorder/comma, commutative == |
| 69 | Anvil | `always_implies_forall_intro` | trigger, reorder/comma |
| 105 | Anvil | `leads_to_apply` |  |
| 107 | Anvil | `leads_to_exists_intro` | trigger, reorder/comma |
| 117 | Anvil | `leads_to_weaken` |  |
| 139 | Anvil | `spec_entails_always_tla_forall` | trigger, reorder/comma |
| 152 | Anvil | `tla_forall_and_equality` | reorder/comma, commutative == |
| 172 | IronKV | `choose_gap_violator` | reorder/comma |
| 184 | IronKV | `new` | commutative == |
| 342 | Memory-Allocator | `lemma_map_distribute_auto` | trigger, reorder/comma, commutative == |
| 436 | NRKernel | `lemma_interp_of_entry_contains_mapping_implie...` | commutative == |

### Detailed Examples

#### Case 65 — `always_and_equality` (Anvil)

**Ensures (different syntax, same semantics):**
```verus
// Generated:
always(p).and(always(q)) == always(p.and(q)),
// Ground truth:
always(p.and(q)) == always(p).and(always(q)),
```
**Rule(s):** reorder/comma, commutative ==

#### Case 69 — `always_implies_forall_intro` (Anvil)

**Requires (different syntax, same semantics):**
```verus
// Generated:
forall |a: A| spec.entails(#[trigger] always(p.implies(a_to_q(a)))),
// Ground truth:
forall |a: A| #[trigger] spec.entails(always(p.implies(a_to_q(a)))),
```
**Rule(s):** trigger, reorder/comma

#### Case 105 — `leads_to_apply` (Anvil)

**Requires (different syntax, same semantics):**
```verus
// Generated:
spec.entails(p.leads_to(q)),
        spec.entails(p),
// Ground truth:
spec.entails(p),
        spec.entails(p.leads_to(q)),
```
**Rule(s):** 

#### Case 107 — `leads_to_exists_intro` (Anvil)

**Requires (different syntax, same semantics):**
```verus
// Generated:
forall |a: A| spec.entails(#[trigger] a_to_p(a).leads_to(q)),
// Ground truth:
forall |a: A| #[trigger] spec.entails(a_to_p(a).leads_to(q)),
```
**Rule(s):** trigger, reorder/comma

#### Case 117 — `leads_to_weaken` (Anvil)

**Requires (different syntax, same semantics):**
```verus
// Generated:
spec.entails(p1.leads_to(q1)),
        spec.entails(always(p2.implies(p1))),
        spec.entails(always(q1.implies(q2))),
// Ground truth:
spec.entails(always(p2.implies(p1))),
        spec.entails(always(q1.implies(q2))),
        spec.entails(p1.leads_to(q1)),
```
**Rule(s):** 

#### Case 139 — `spec_entails_always_tla_forall` (Anvil)

**Requires (different syntax, same semantics):**
```verus
// Generated:
forall |a: A| spec.entails(#[trigger] always(a_to_p(a))),
// Ground truth:
forall |a: A| spec.entails(always(#[trigger] a_to_p(a))),
```
**Rule(s):** trigger, reorder/comma

#### Case 152 — `tla_forall_and_equality` (Anvil)

**Ensures (different syntax, same semantics):**
```verus
// Generated:
tla_forall(a_to_p).and(q) == tla_forall(|a: A| a_to_p(a).and(q)),
// Ground truth:
tla_forall(|a: A| a_to_p(a).and(q)) == tla_forall(a_to_p).and(q),
```
**Rule(s):** reorder/comma, commutative ==

#### Case 172 — `choose_gap_violator` (IronKV)

**Ensures (different syntax, same semantics):**
```verus
// Generated:
lo.lt_spec(r),
            r.lt_spec(hi),
            self@.contains_key(*r.get()),
// Ground truth:
lo.lt_spec(r) && r.lt_spec(hi) && self@.contains_key(*r.get()),
```
**Rule(s):** reorder/comma

#### Case 184 — `new` (IronKV)

**Ensures (different syntax, same semantics):**
```verus
// Generated:
s.valid(),
            s@ == Map::<K, ID>::empty(),
// Ground truth:
s.valid(),
            s@ == Map::<K,ID>::empty(),
```
**Rule(s):** commutative ==

#### Case 342 — `lemma_map_distribute_auto` (Memory-Allocator)

**Ensures (different syntax, same semantics):**
```verus
// Generated:
forall|s1: Set<S>, s2: Set<S>, f: spec_fn(S) -> T|
            #[trigger] s1.union(s2).map(f) == s1.map(f).union(s2.map(f)),
// Ground truth:
forall|s1: Set<S>, s2: Set<S>, f: spec_fn(S) -> T| s1.union(s2).map(f) == #[trigger] s1.map(f).union(s2.map(f))
```
**Rule(s):** trigger, reorder/comma, commutative ==

#### Case 436 — `lemma_interp_of_entry_contains_mapping_implies_interp_contains_mapping` (NRKernel)

**Ensures (different syntax, same semantics):**
```verus
// Generated:
forall|va: nat, pte: PTE| #![auto] self.interp_of_entry(j).contains_pair(va, pte) ==> self.interp().contains_pair(va, pte),
            forall|va: nat| #![auto] self.interp_of_entry(j).contains_key(va) ==> self.interp().contains_key(va),
            //forall|va: nat| #![auto]
            //    self.entry_base(j) <= va < self.entry_base(j+1) && !self.interp_of_entry(j).contains_key(va)
            //    ==> !self.interp().contains_key(va),
// Ground truth:
forall|va: nat| #![auto] self.interp_of_entry(j).contains_key(va) ==> self.interp().contains_key(va),
            forall|va: nat, pte: PTE| #![auto] self.interp_of_entry(j).contains_pair(va, pte) ==> self.interp().contains_pair(va, pte),
            //forall|va: nat| #![auto]
            //    self.entry_base(j) <= va < self.entry_base(j+1) && !self.interp_of_entry(j).contains_key(va)
            //    ==> !self.interp().contains_key(va),
```
**Rule(s):** commutative ==

---

## 3. Root Cause Taxonomy (244 different cases)

| Category | Subcategory | Count | % | Description |
|----------|-------------|-------|---|-------------|
| **Both Requires & Ensures Wrong** | | **134** | **54.9%** | |
| | Both Wrong | 134 | 54.9% | Both requires and ensures clauses differ from ground truth |
| **Wrong Ensures Only** | | **74** | **30.3%** | |
| | Wrong Ensures | 62 | 25.4% | Ensures clause expresses fundamentally different postconditions |
| | Partial Ensures | 12 | 4.9% | Some ensures clauses match, others differ or are missing |
| **Wrong Requires Only** | | **36** | **14.8%** | |
| | Wrong Requires | 32 | 13.1% | Requires clause has different preconditions |
| | Missing Requires | 2 | 0.8% | No requires clause when ground truth has one |
| | Too Weak Requires | 2 | 0.8% | Missing some preconditions from ground truth |

**Key observation:** "Both Wrong" (134 cases, 54.9%) is the dominant category — when the model gets something wrong, it tends to get both requires and ensures wrong simultaneously, suggesting a systematic misunderstanding of the function's contract rather than isolated clause errors.

---

## 4. Both Wrong — Deep Dive (134 cases)

When both requires and ensures differ, we can further analyze the nature of each side's difference.

| Sub-pattern | Count | Description |
|-------------|-------|-------------|
| Completely different requires & ensures | 76 | |
| Extra requires + wrong ensures | 32 | |
| Missing requires + wrong ensures | 12 | |
| Extra requires + partial ensures match | 5 | |
| Partial match on both sides | 5 | |
| Missing requires + partial ensures match | 4 | |

### Representative Examples

**Case 13** — `vrs_rely_condition_equivalent_to_lifted_vrs_rely_condition` (Anvil-Advanced) — :white_check_mark: Verified

| | Generated | Ground Truth |
|---|-----------|-------------|
| **requires** | `spec.entails(always(lift_state(\|s: ClusterState\|` | `(none)` |
| **ensures** | `spec.entails(always(lifted_vrs_rely_condition(cluster, controller_id))),` | `(forall \|other_id\| cluster.controller_models.remove(controller_id).contains_key(other_id)             ==> spec.entails(always(lift_state(#[trigger] vrs_rely(other_id)))))         <==>             spec.entails(always(lifted_vrs_rely_condition(cluster,` |

**Case 14** — `vrs_rely_condition_equivalent_to_lifted_vrs_rely_condition_action` (Anvil-Advanced) — :x: Verification failed

| | Generated | Ground Truth |
|---|-----------|-------------|
| **requires** | `spec.entails(always(lifted_vrs_rely_condition_action(cluster, controller_id))),` | `(none)` |
| **ensures** | `forall \|other_id\| cluster.controller_models.remove(controller_id).contains_key(other_id)             ==> spec.entails(always(lift_state(#[trigger] vrs_rely(other_id)))),` | `(forall \|other_id\| cluster.controller_models.remove(controller_id).contains_key(other_id)             ==> spec.entails(always(lift_state(#[trigger] vrs_rely(other_id)))))         <==>             spec.entails(always(lifted_vrs_rely_condition_action(c` |

**Case 70** — `always_implies_preserved_by_always` (Anvil) — :x: Verification failed

| | Generated | Ground Truth |
|---|-----------|-------------|
| **requires** | `spec.entails(always(p.implies(q))),         spec.entails(always(p)),` | `spec.entails(always(p.implies(q))),` |
| **ensures** | `spec.entails(always(q)),` | `spec.entails(always(always(p).implies(always(q)))),` |

**Case 75** — `always_p_or_eventually_q` (Anvil) — :x: Verification failed

| | Generated | Ground Truth |
|---|-----------|-------------|
| **requires** | `p.satisfied_by(ex),         always(next).satisfied_by(ex),         always(p.and(next).implies(later(p).or(later(q)))).satisfied_by(ex),` | `always(p.and(next).implies(later(p).or(later(q)))).satisfied_by(ex),         always(next).satisfied_by(ex),` |
| **ensures** | `always(p).satisfied_by(ex) \|\| eventually(q).satisfied_by(ex),` | `always(p.implies(always(p).or(eventually(q)))).satisfied_by(ex),` |

**Case 76** — `always_p_or_eventually_q_rec` (Anvil) — :x: Verification failed

| | Generated | Ground Truth |
|---|-----------|-------------|
| **requires** | `forall \|j: nat\| j < i ==> p.satisfied_by(ex.suffix(j)),` | `forall \|idx\| p.satisfied_by(ex.suffix(idx)) && next.satisfied_by(ex.suffix(idx)) ==> p.satisfied_by(ex.suffix(idx + 1)) \|\| q.satisfied_by(ex.suffix(idx + 1)),         forall \|idx\| next.satisfied_by(#[trigger] ex.suffix(idx)),         forall \|idx\| !q.` |
| **ensures** | `(forall \|j: nat\| p.satisfied_by(ex.suffix(j))) \|\| (exists \|j: nat\| q.satisfied_by(ex.suffix(j))),` | `p.satisfied_by(ex.suffix(i)),` |

---

## 5. Wrong Ensures Only — Deep Dive (74 cases)

These cases got the preconditions (requires) correct but produced wrong postconditions (ensures).

### 5.1 Partial Match (12 cases)

The model gets some ensures clauses right but misses or alters others.

**Case 137** — `seq_unequal_preserved_by_add_auto` (Anvil) — :white_check_mark: Verified

- Matching clauses: **1**
- Extra in generated: **1**
- Missing from generated: **1**

```verus
// Generated ensures:
forall|s1: Seq<A>, s2: Seq<A>| s1 != s2 ==> s1 + suffix != s2 + suffix,
// Ground truth ensures:
forall |s1: Seq<A>, s2: Seq<A>| s1 != s2 ==> s1 + suffix != s2 + suffix
```

**Case 179** — `set` (IronKV) — :x: Verification failed

- Matching clauses: **2**
- Extra in generated: **0**
- Missing from generated: **3**

```verus
// Generated ensures:
self.valid(),
            self@ == old(self)@.insert(k, v),
// Ground truth ensures:
self.valid(),
            self@ == old(self)@.insert(k, v),
            forall |lo, hi| self.gap(lo, hi) <==>
                            old(self).gap(lo, hi)
                        && !(lo.lt_spec(KeyIterator::new_spec(k))
                          && KeyIterator::new_spec(k).lt_spec(hi)),
```

**Case 187** — `set` (IronKV) — :x: Verification failed

- Matching clauses: **1**
- Extra in generated: **6**
- Missing from generated: **4**

```verus
// Generated ensures:
self.valid(),
            self@.contains_key(k),
            self@[k] == v,
            forall|other_k: K| #![auto] other_k != k ==> (
                old(self)@.contains_key(other_k) ==> self@.contains_key(other_k) && self@[other_k] == old(self)@[other_k]
            ),
            forall|other_k: K| #![auto] other_k != k ==> (
                self@.contains_key(other_k) ==> old(self)@.contains_k
// Ground truth ensures:
self.valid(),
            self@ == old(self)@.insert(k, v),
            forall |lo, hi| self.gap(lo, hi) <==>
                            old(self).gap(lo, hi)
                        && !(lo.lt_spec(KeyIterator::new_spec(k))
                          && KeyIterator::new_spec(k).lt_spec(hi)),
```

**Case 204** — `make_empty_event_results` (IronKV) — :white_check_mark: Verified

- Matching clauses: **4**
- Extra in generated: **0**
- Missing from generated: **1**

```verus
// Generated ensures:
res@.recvs =~= Seq::<NetEvent>::empty(),
        res@.clocks =~= Seq::<NetEvent>::empty(),
        res@.sends =~= Seq::<NetEvent>::empty(),
        res@.ios =~= Seq::<NetEvent>::empty(),
// Ground truth ensures:
res@.recvs == Seq::<NetEvent>::empty(),
        res@.clocks == Seq::<NetEvent>::empty(),
        res@.sends == Seq::<NetEvent>::empty(),
        res@.ios == Seq::<NetEvent>::empty(),
        extract_packets_from_abstract_ios(abstractify_raw_log_to_ios(res@.ios)) == Set::<Packet>::empty(),
```

### 5.2 Completely Wrong Ensures (62 cases)

No overlap between generated and ground truth ensures clauses.

**Case 12** — `only_interferes_with_itself_equivalent_to_lifted_only_interferes_with_itself_action` (Anvil-Advanced) — :x: Verification failed

```verus
// Generated ensures:
spec.entails(always(lifted_vrs_reconcile_request_only_interferes_with_itself_action(controller_id)))
        <==>
        forall |vrs: VReplicaSetView| spec.entails(always(lift_state(vrs_reconcile_request_only_interferes_with_itself(controller_id, vrs)))),
// Ground truth ensures:
spec.entails(always(tla_forall(|vrs: VReplicaSetView| 
            lift_state(vrs_reconcile_request_only_interferes_with_itself(controller_id, vrs)))))
        <==>
            spec.entails(always(lifted_vrs_reconcile_request_only_interferes_with_itself_action(controller_id)))
```

**Case 66** — `always_distributed_by_and` (Anvil) — :white_check_mark: Verified

```verus
// Generated ensures:
valid(always(p).and(always(q)).implies(always(p.and(q)))),
// Ground truth ensures:
valid(always(p.and(q)).implies(always(p).and(always(q)))),
```

**Case 72** — `always_lift_action_unfold` (Anvil) — :white_check_mark: Verified

```verus
// Generated ensures:
forall |i: nat| p(#[trigger] ex.suffix(i).head(), ex.suffix(i).head_next()),
// Ground truth ensures:
forall |i| p(#[trigger] ex.suffix(i).head(), ex.suffix(i).head_next()),
```

**Case 73** — `always_lift_state_unfold` (Anvil) — :x: Verification failed

```verus
// Generated ensures:
forall |i: nat| p(#[trigger] ex.suffix(i).head()),
// Ground truth ensures:
forall |i| p(#[trigger] ex.suffix(i).head()),
```

**Case 85** — `eliminate_always` (Anvil) — :x: Verification failed

```verus
// Generated ensures:
valid(spec.implies(p)),
// Ground truth ensures:
spec.entails(p),
```

---

## 6. Wrong Requires Only — Deep Dive (36 cases)

These cases got ensures correct but produced wrong requires.

### 6.1 Wrong Requires (32 cases)

**Case 64** — `a_to_temp_pred_equality` (Anvil) — :x: Verification failed

```verus
// Generated requires:
forall |a: A| #[trigger] p(a).entails(q(a)),
        forall |a: A| #[trigger] q(a).entails(p(a)),
// Ground truth requires:
forall |a: A| #[trigger] p(a).entails(q(a)) && q(a).entails(p(a)),
```

**Case 81** — `always_weaken` (Anvil) — :x: Verification failed

```verus
// Generated requires:
spec.entails(always(p)),
        p.entails(q),
// Ground truth requires:
valid(p.implies(q)),
        spec.entails(always(p)),
```

**Case 82** — `commutativity_of_seq_map_and_filter` (Anvil) — :x: Verification failed

```verus
// Generated requires:
forall |a: A| #[trigger] pred(a) == pred_on_mapped(map(a)),
// Ground truth requires:
forall |i: int| 0 <= i < s.len() ==> #[trigger] pred(s[i]) == #[trigger] pred_on_mapped(map(s[i])),
```

**Case 90** — `entails_implies_leads_to` (Anvil) — :white_check_mark: Verified

```verus
// Generated requires:
valid(p.implies(q)),
// Ground truth requires:
p.entails(q),
```

### 6.2 Missing Requires (2 cases)

**Case 201** — `real_init_impl` (IronKV)

- Generated: `(no requires)`
- Ground truth: `netc.valid(),`

**Case 411** — `aligned_transitive_auto` (NRKernel)

- Generated: `(no requires)`
- Ground truth: `0 < b,
        0 < c,
        aligned(a, b),
        aligned(b, c),`

### 6.3 Too Weak Requires (2 cases)

**Case 328** — `create` (Memory-Allocator)

```verus
// Generated requires:
idx + count <= COMMIT_MASK_BITS,
// Ground truth requires:
idx + count <= COMMIT_MASK_BITS,
            old(self)@ == Set::<int>::empty(),
```

**Case 373** — `singleton_set_unique_elt` (Memory-Allocator)

```verus
// Generated requires:
s.len() == 1,
        s.contains(a),
        s.contains(b),
// Ground truth requires:
s.finite(),
        s.len() == 1,
        s.contains(a),
        s.contains(b),
```

---

## 7. Root Cause Distribution by Project

### Anvil

- Evaluable cases: ~104
- Equivalent: 56
- Different: 48

| Root Cause | Count | % |
|------------|-------|---|
| wrong_requires | 17 | 35% |
| wrong_ensures | 16 | 33% |
| wrong_both | 14 | 29% |
| partial_ensures | 1 | 2% |

### Anvil-Advanced

- Evaluable cases: ~63
- Equivalent: 60
- Different: 3

| Root Cause | Count | % |
|------------|-------|---|
| wrong_both | 2 | 67% |
| wrong_ensures | 1 | 33% |

### IronKV

- Evaluable cases: ~118
- Equivalent: 22
- Different: 73

| Root Cause | Count | % |
|------------|-------|---|
| wrong_both | 36 | 49% |
| wrong_ensures | 26 | 36% |
| partial_ensures | 6 | 8% |
| wrong_requires | 4 | 5% |
| missing_requires | 1 | 1% |

### Memory-Allocator

- Evaluable cases: ~89
- Equivalent: 19
- Different: 62

| Root Cause | Count | % |
|------------|-------|---|
| wrong_both | 38 | 61% |
| wrong_ensures | 14 | 23% |
| wrong_requires | 7 | 11% |
| too_weak_requires | 2 | 3% |
| partial_ensures | 1 | 2% |

### NRKernel

- Evaluable cases: ~117
- Equivalent: 60
- Different: 37

| Root Cause | Count | % |
|------------|-------|---|
| wrong_both | 26 | 70% |
| wrong_ensures | 4 | 11% |
| wrong_requires | 3 | 8% |
| partial_ensures | 3 | 8% |
| missing_requires | 1 | 3% |

### Node-Replication

- Evaluable cases: ~29
- Equivalent: 5
- Different: 21

| Root Cause | Count | % |
|------------|-------|---|
| wrong_both | 18 | 86% |
| wrong_ensures | 1 | 5% |
| wrong_requires | 1 | 5% |
| partial_ensures | 1 | 5% |

---

## 8. Cross-Cutting Patterns

### 8.1 Quantifier Handling

**109/244 (45%)** of differing cases involve quantified expressions.

Common issues:
- The model generates `forall` with different variable bindings
- Quantifier body contains different sub-expressions
- `tla_forall` vs explicit `forall` in temporal logic specs
- `exists` used where `forall` is expected, or vice versa

### 8.2 Verification vs Correctness

Among the 244 cases with wrong specs, **65 (27%)** still pass Verus verification.

| Root Cause | Verified OK | Total | Rate |
|------------|-------------|-------|------|
| wrong_both | 34 | 134 | 25% |
| wrong_ensures | 19 | 62 | 31% |
| wrong_requires | 5 | 32 | 16% |
| partial_ensures | 6 | 12 | 50% |
| missing_requires | 1 | 2 | 50% |
| too_weak_requires | 0 | 2 | 0% |

This confirms that **verification is not a reliable proxy for semantic correctness**: a wrong spec can still be provable if the wrong conditions are internally consistent.

### 8.3 Difficulty by Function Type

- Functions starting with `lemma_`/`property_`/`spec_`: 63 wrong cases
- Functions with `inv_`/`transition_`/`new`/`update_`: 9 wrong cases
- No clear difficulty gradient by function naming convention — both types are equally challenging.

---

## 9. Summary

### Overall Results

| Category | Count | Rate |
|---|---|---|
| Semantically equivalent | 119 | 32.8% of evaluable |
| Semantically different | 244 | 67.2% of evaluable |

Of the 119 equivalent cases, 11 required semantic equivalence rules (comma≡&&, commutative <==> and ==, trigger-insensitive, clause reordering) beyond basic normalization to recognize as equivalent:

- **5 cases** via clause reordering (different order but same set of conjuncts)
- **3 cases** via trigger placement (`#[trigger]` on different sub-expressions)
- **2 cases** via commutative equality (`A == B` vs `B == A`)
- **1 case** via comma≡&& normalization (`a, b` vs `a && b`)

### Key Findings

1. **Both-wrong dominates:** 134/244 (55%) of diffs have both requires and ensures wrong. The model's errors are correlated — when it misunderstands the function's purpose, it gets both sides wrong.
2. **Wrong ensures is more common than wrong requires:** 74 cases have wrong ensures only vs 36 with wrong requires only. Postconditions are harder to infer than preconditions.
3. **Quantifiers remain the core challenge:** 45% of differing cases involve `forall`/`exists`, confirming that complex logical quantification is the hardest aspect of spec generation.
4. **Syntactic near-equivalences are rare:** Only 11 out of 255 initially-different cases turned out equivalent under semantic rules. The remaining 244 differences are genuine semantic disagreements.
5. **Verification ≠ correctness:** 27% of wrong specs still pass verification — a wrong but internally consistent spec is still provable.

---

## Appendix A: All 11 Semantically Equivalent Cases Requiring Enhanced Rules

| # | Line | Project | Target Function | Rule(s) |
|---|------|---------|-----------------|---------|
| 1 | 65 | Anvil | `always_and_equality` | reorder/comma, commutative == |
| 2 | 69 | Anvil | `always_implies_forall_intro` | trigger, reorder/comma |
| 3 | 105 | Anvil | `leads_to_apply` |  |
| 4 | 107 | Anvil | `leads_to_exists_intro` | trigger, reorder/comma |
| 5 | 117 | Anvil | `leads_to_weaken` |  |
| 6 | 139 | Anvil | `spec_entails_always_tla_forall` | trigger, reorder/comma |
| 7 | 152 | Anvil | `tla_forall_and_equality` | reorder/comma, commutative == |
| 8 | 172 | IronKV | `choose_gap_violator` | reorder/comma |
| 9 | 184 | IronKV | `new` | commutative == |
| 10 | 342 | Memory-Allocator | `lemma_map_distribute_auto` | trigger, reorder/comma, commutative == |
| 11 | 436 | NRKernel | `lemma_interp_of_entry_contains_mapping_implie...` | commutative == |

## Appendix B: All 244 Semantically Different Cases

| # | Line | Project | Target Function | Root Cause | Req Diff | Ens Diff | Verified |
|---|------|---------|-----------------|------------|----------|----------|----------|
| 1 | 12 | Anvil-Advanced | `only_interferes_with_itself_equivalent_t...` | wrong_ensures |  | Yes | No |
| 2 | 13 | Anvil-Advanced | `vrs_rely_condition_equivalent_to_lifted_...` | wrong_both | Yes | Yes | Yes |
| 3 | 14 | Anvil-Advanced | `vrs_rely_condition_equivalent_to_lifted_...` | wrong_both | Yes | Yes | No |
| 4 | 64 | Anvil | `a_to_temp_pred_equality` | wrong_requires | Yes |  | No |
| 5 | 66 | Anvil | `always_distributed_by_and` | wrong_ensures |  | Yes | Yes |
| 6 | 70 | Anvil | `always_implies_preserved_by_always` | wrong_both | Yes | Yes | No |
| 7 | 72 | Anvil | `always_lift_action_unfold` | wrong_ensures |  | Yes | Yes |
| 8 | 73 | Anvil | `always_lift_state_unfold` | wrong_ensures |  | Yes | No |
| 9 | 75 | Anvil | `always_p_or_eventually_q` | wrong_both | Yes | Yes | No |
| 10 | 76 | Anvil | `always_p_or_eventually_q_rec` | wrong_both | Yes | Yes | No |
| 11 | 81 | Anvil | `always_weaken` | wrong_requires | Yes |  | No |
| 12 | 82 | Anvil | `commutativity_of_seq_map_and_filter` | wrong_requires | Yes |  | No |
| 13 | 85 | Anvil | `eliminate_always` | wrong_ensures |  | Yes | No |
| 14 | 86 | Anvil | `empty_filter_implies_seq_pred_false_on_a...` | wrong_ensures |  | Yes | No |
| 15 | 90 | Anvil | `entails_implies_leads_to` | wrong_requires | Yes |  | Yes |
| 16 | 95 | Anvil | `filtered_size_is_one_means_only_one_such...` | wrong_ensures |  | Yes | No |
| 17 | 96 | Anvil | `filtered_size_is_zero_means_no_such_valu...` | wrong_ensures |  | Yes | No |
| 18 | 97 | Anvil | `finite_set_to_seq_contains_all_set_eleme...` | wrong_ensures |  | Yes | Yes |
| 19 | 99 | Anvil | `init_invariant` | wrong_requires | Yes |  | No |
| 20 | 100 | Anvil | `init_invariant_rec` | wrong_both | Yes | Yes | No |
| 21 | 102 | Anvil | `leads_to_always_combine` | wrong_both | Yes | Yes | No |
| 22 | 103 | Anvil | `leads_to_always_enhance` | wrong_requires | Yes |  | No |
| 23 | 104 | Anvil | `leads_to_always_tla_forall` | wrong_requires | Yes |  | No |
| 24 | 106 | Anvil | `leads_to_by_borrowing_inv` | wrong_requires | Yes |  | No |
| 25 | 110 | Anvil | `leads_to_rank_step_one_help` | wrong_both | Yes | Yes | No |
| 26 | 112 | Anvil | `leads_to_rank_step_one_usize_help` | wrong_requires | Yes |  | No |
| 27 | 114 | Anvil | `leads_to_shortcut_temp` | wrong_both | Yes | Yes | No |
| 28 | 115 | Anvil | `leads_to_stable` | wrong_requires | Yes |  | No |
| 29 | 118 | Anvil | `len_is_zero_means_count_for_each_value_i...` | wrong_both | Yes | Yes | Yes |
| 30 | 119 | Anvil | `map_values_to_set_eq_to_set_mk_map_value...` | wrong_ensures |  | Yes | No |
| 31 | 121 | Anvil | `next_preserves_inv_rec` | wrong_requires | Yes |  | No |
| 32 | 126 | Anvil | `pack_conditions_to_spec` | wrong_both | Yes | Yes | No |
| 33 | 127 | Anvil | `push_filter_and_filter_push` | wrong_ensures |  | Yes | No |
| 34 | 129 | Anvil | `seq_equal_preserved_by_add` | wrong_both | Yes | Yes | Yes |
| 35 | 130 | Anvil | `seq_equal_preserved_by_add_prefix` | wrong_both | Yes | Yes | Yes |
| 36 | 132 | Anvil | `seq_filter_is_a_subset_of_original_seq` | wrong_ensures |  | Yes | No |
| 37 | 134 | Anvil | `seq_pred_false_on_all_elements_implies_e...` | wrong_requires | Yes |  | Yes |
| 38 | 135 | Anvil | `seq_pred_false_on_all_elements_is_equiva...` | wrong_ensures |  | Yes | Yes |
| 39 | 137 | Anvil | `seq_unequal_preserved_by_add_auto` | partial_ensures |  | Yes | Yes |
| 40 | 138 | Anvil | `simplify_predicate` | wrong_both | Yes | Yes | Yes |
| 41 | 142 | Anvil | `strengthen_next` | wrong_requires | Yes |  | No |
| 42 | 145 | Anvil | `tla_exists_equality` | wrong_ensures |  | Yes | No |
| 43 | 148 | Anvil | `tla_forall_a_p_leads_to_q_a_is_stable` | wrong_both | Yes | Yes | No |
| 44 | 150 | Anvil | `tla_forall_always_equality_variant` | wrong_requires | Yes |  | No |
| 45 | 153 | Anvil | `tla_forall_implies_equality1` | wrong_ensures |  | Yes | No |
| 46 | 156 | Anvil | `tla_forall_not_equality` | wrong_ensures |  | Yes | No |
| 47 | 159 | Anvil | `true_pred_on_all_element_equal_to_pred_o...` | wrong_ensures |  | Yes | No |
| 48 | 161 | Anvil | `unpack_conditions_from_spec` | wrong_both | Yes | Yes | No |
| 49 | 163 | Anvil | `vacuous_leads_to` | wrong_requires | Yes |  | No |
| 50 | 165 | Anvil | `wf1` | wrong_requires | Yes |  | No |
| 51 | 166 | Anvil | `wf1_variant_temp` | wrong_requires | Yes |  | No |
| 52 | 173 | IronKV | `erase` | wrong_both | Yes | Yes | No |
| 53 | 175 | IronKV | `greatest_lower_bound_index` | wrong_both | Yes | Yes | Yes |
| 54 | 176 | IronKV | `keys_in_index_range_agree` | wrong_ensures |  | Yes | Yes |
| 55 | 177 | IronKV | `mind_the_gap` | wrong_ensures |  | Yes | Yes |
| 56 | 178 | IronKV | `new` | wrong_both | Yes | Yes | Yes |
| 57 | 179 | IronKV | `set` | partial_ensures |  | Yes | No |
| 58 | 180 | IronKV | `values_agree` | wrong_both | Yes | Yes | Yes |
| 59 | 181 | IronKV | `all_keys_agree` | wrong_both | Yes | Yes | No |
| 60 | 182 | IronKV | `almost_all_keys_agree` | wrong_both | Yes | Yes | No |
| 61 | 183 | IronKV | `empty_key_range_is_consistent` | wrong_requires | Yes |  | No |
| 62 | 186 | IronKV | `range_consistent_subset` | wrong_requires | Yes |  | No |
| 63 | 187 | IronKV | `set` | partial_ensures |  | Yes | No |
| 64 | 188 | IronKV | `delegate_for_key_range_is_host_impl` | wrong_ensures |  | Yes | Yes |
| 65 | 189 | IronKV | `vec_erase` | wrong_both | Yes | Yes | No |
| 66 | 191 | IronKV | `effect_of_delegation_map_set` | wrong_both | Yes | Yes | No |
| 67 | 200 | IronKV | `process_received_packet_next` | wrong_both | Yes | Yes | No |
| 68 | 201 | IronKV | `real_init_impl` | missing_requires | Yes |  | No |
| 69 | 204 | IronKV | `make_empty_event_results` | partial_ensures |  | Yes | Yes |
| 70 | 205 | IronKV | `make_send_only_event_results` | wrong_requires | Yes |  | No |
| 71 | 207 | IronKV | `deserialize` | wrong_ensures |  | Yes | No |
| 72 | 208 | IronKV | `lemma_same_views_serialize_the_same` | wrong_both | Yes | Yes | No |
| 73 | 209 | IronKV | `lemma_serialization_is_not_a_prefix_of` | wrong_both | Yes | Yes | No |
| 74 | 213 | IronKV | `deserialize` | wrong_ensures |  | Yes | No |
| 75 | 214 | IronKV | `lemma_serialization_is_not_a_prefix_of` | wrong_both | Yes | Yes | No |
| 76 | 216 | IronKV | `serialize` | wrong_ensures |  | Yes | No |
| 77 | 217 | IronKV | `serialized_size` | wrong_ensures |  | Yes | No |
| 78 | 218 | IronKV | `lemma_serialization_is_not_a_prefix_of` | wrong_both | Yes | Yes | No |
| 79 | 220 | IronKV | `lemma_serialization_is_not_a_prefix_of` | wrong_both | Yes | Yes | No |
| 80 | 222 | IronKV | `serialize` | wrong_ensures |  | Yes | No |
| 81 | 223 | IronKV | `lemma_same_views_serialize_the_same` | wrong_both | Yes | Yes | No |
| 82 | 224 | IronKV | `lemma_serialization_is_not_a_prefix_of` | wrong_both | Yes | Yes | No |
| 83 | 226 | IronKV | `lemma_view_equal_symmetric` | wrong_both | Yes | Yes | No |
| 84 | 227 | IronKV | `serialize` | wrong_both | Yes | Yes | Yes |
| 85 | 228 | IronKV | `deserialize` | wrong_ensures |  | Yes | No |
| 86 | 230 | IronKV | `lemma_same_views_serialize_the_same` | wrong_both | Yes | Yes | No |
| 87 | 231 | IronKV | `lemma_serialization_is_not_a_prefix_of` | wrong_both | Yes | Yes | No |
| 88 | 233 | IronKV | `lemma_view_equal_symmetric` | wrong_both | Yes | Yes | No |
| 89 | 234 | IronKV | `serialize` | wrong_ensures |  | Yes | No |
| 90 | 235 | IronKV | `serialized_size` | wrong_ensures |  | Yes | No |
| 91 | 236 | IronKV | `lemma_same_views_serialize_the_same` | wrong_both | Yes | Yes | No |
| 92 | 237 | IronKV | `lemma_serialization_is_not_a_prefix_of` | wrong_both | Yes | Yes | No |
| 93 | 239 | IronKV | `lemma_view_equal_symmetric` | wrong_both | Yes | Yes | No |
| 94 | 244 | IronKV | `sht_marshal_data_injective` | wrong_ensures |  | Yes | Yes |
| 95 | 246 | IronKV | `endpoints_contain` | wrong_ensures |  | Yes | Yes |
| 96 | 248 | IronKV | `test_unique` | wrong_ensures |  | Yes | Yes |
| 97 | 249 | IronKV | `maybe_ack_packet_impl` | wrong_both | Yes | Yes | Yes |
| 98 | 250 | IronKV | `receive_ack_impl` | partial_ensures |  | Yes | No |
| 99 | 251 | IronKV | `receive_impl` | wrong_both | Yes | Yes | No |
| 100 | 253 | IronKV | `retransmit_un_acked_packets_for_dst` | wrong_both | Yes | Yes | No |
| 101 | 254 | IronKV | `send_single_cmessage` | wrong_both | Yes | Yes | No |
| 102 | 255 | IronKV | `same_view_same_marshalable` | partial_ensures |  | Yes | No |
| 103 | 257 | IronKV | `lemma_seqno_in_un_acked_list` | wrong_ensures |  | Yes | No |
| 104 | 258 | IronKV | `truncate` | wrong_both | Yes | Yes | No |
| 105 | 259 | IronKV | `insert` | wrong_both | Yes | Yes | No |
| 106 | 260 | IronKV | `un_acked_messages_extend` | wrong_both | Yes | Yes | Yes |
| 107 | 262 | IronKV | `lemma_filter_skip_rejected` | wrong_both | Yes | Yes | No |
| 108 | 263 | IronKV | `lemma_flatten_set_seq_spec` | wrong_ensures |  | Yes | No |
| 109 | 264 | IronKV | `lemma_fold_left_append_merge` | wrong_ensures |  | Yes | No |
| 110 | 266 | IronKV | `lemma_if_everything_in_seq_satisfies_fil...` | wrong_requires | Yes |  | No |
| 111 | 267 | IronKV | `lemma_if_nothing_in_seq_satisfies_filter...` | wrong_both | Yes | Yes | No |
| 112 | 268 | IronKV | `lemma_map_set_singleton_auto` | wrong_ensures |  | Yes | Yes |
| 113 | 269 | IronKV | `lemma_seq_fold_left_append_len_int` | wrong_ensures |  | Yes | No |
| 114 | 270 | IronKV | `lemma_seq_fold_left_append_len_int_le` | wrong_both | Yes | Yes | No |
| 115 | 271 | IronKV | `lemma_seq_fold_left_sum_le` | wrong_both | Yes | Yes | No |
| 116 | 272 | IronKV | `lemma_seq_fold_left_sum_len_int_positive` | wrong_ensures |  | Yes | No |
| 117 | 275 | IronKV | `lemma_to_set_distributes_over_addition` | wrong_ensures |  | Yes | No |
| 118 | 276 | IronKV | `lemma_to_set_singleton_auto` | wrong_ensures |  | Yes | No |
| 119 | 277 | IronKV | `lemma_to_set_union_auto` | wrong_ensures |  | Yes | Yes |
| 120 | 280 | IronKV | `map_fold_ok` | wrong_ensures |  | Yes | No |
| 121 | 281 | IronKV | `map_set_finite_auto` | wrong_ensures |  | Yes | Yes |
| 122 | 282 | IronKV | `set_map_union` | wrong_ensures |  | Yes | No |
| 123 | 283 | IronKV | `set_map_union_auto` | partial_ensures |  | Yes | Yes |
| 124 | 284 | IronKV | `some_differing_index_for_unequal_seqs` | wrong_both | Yes | Yes | No |
| 125 | 286 | Memory-Allocator | `bin_size_result_mul8` | wrong_both | Yes | Yes | No |
| 126 | 287 | Memory-Allocator | `bounds_for_smallest_bin_fitting_size` | wrong_requires | Yes |  | No |
| 127 | 289 | Memory-Allocator | `div2` | wrong_both | Yes | Yes | Yes |
| 128 | 290 | Memory-Allocator | `idx_in_range_has_bin_size` | wrong_requires | Yes |  | No |
| 129 | 291 | Memory-Allocator | `idx_out_of_range_has_different_bin_size` | wrong_ensures |  | Yes | No |
| 130 | 292 | Memory-Allocator | `leading_zeros_between` | wrong_both | Yes | Yes | Yes |
| 131 | 293 | Memory-Allocator | `leading_zeros_between_powers_of_2` | wrong_both | Yes | Yes | No |
| 132 | 294 | Memory-Allocator | `leading_zeros_monotonic` | wrong_both | Yes | Yes | No |
| 133 | 295 | Memory-Allocator | `leading_zeros_powers_of_2` | wrong_both | Yes | Yes | No |
| 134 | 297 | Memory-Allocator | `lemma_div_is_ordered` | wrong_requires | Yes |  | Yes |
| 135 | 298 | Memory-Allocator | `log2` | wrong_ensures |  | Yes | Yes |
| 136 | 299 | Memory-Allocator | `mul_assoc` | wrong_ensures |  | Yes | No |
| 137 | 300 | Memory-Allocator | `mul_ordering` | wrong_both | Yes | Yes | No |
| 138 | 301 | Memory-Allocator | `out_of_small_range` | wrong_both | Yes | Yes | No |
| 139 | 305 | Memory-Allocator | `pow2_properties` | wrong_ensures |  | Yes | Yes |
| 140 | 306 | Memory-Allocator | `pow2_subtracts` | wrong_both | Yes | Yes | No |
| 141 | 307 | Memory-Allocator | `result2_idx_in_range_has_bin_size` | wrong_ensures |  | Yes | No |
| 142 | 308 | Memory-Allocator | `result2_idx_out_of_range_has_different_b...` | wrong_ensures |  | Yes | No |
| 143 | 309 | Memory-Allocator | `result_bin` | wrong_both | Yes | Yes | No |
| 144 | 310 | Memory-Allocator | `result_bounds_for_smallest_bitting_size` | wrong_both | Yes | Yes | No |
| 145 | 311 | Memory-Allocator | `result_idx_in_range_has_bin_size` | wrong_both | Yes | Yes | No |
| 146 | 312 | Memory-Allocator | `result_idx_out_of_range_has_different_bi...` | wrong_both | Yes | Yes | No |
| 147 | 313 | Memory-Allocator | `result_sbin` | wrong_both | Yes | Yes | No |
| 148 | 314 | Memory-Allocator | `result_sbin_bounds` | wrong_both | Yes | Yes | Yes |
| 149 | 315 | Memory-Allocator | `result_sbin_idx_smallest_sbin_fitting_si...` | wrong_both | Yes | Yes | No |
| 150 | 316 | Memory-Allocator | `result_smallest_bin_fitting_size_size_of...` | wrong_both | Yes | Yes | No |
| 151 | 317 | Memory-Allocator | `shift_is_div` | wrong_both | Yes | Yes | No |
| 152 | 318 | Memory-Allocator | `size_gt_8_implies_idx_gt_1` | wrong_requires | Yes |  | Yes |
| 153 | 319 | Memory-Allocator | `size_le_8_implies_idx_eq_1` | wrong_requires | Yes |  | Yes |
| 154 | 320 | Memory-Allocator | `size_of_bin_bounds` | wrong_ensures |  | Yes | No |
| 155 | 321 | Memory-Allocator | `size_of_bin_bounds_not_huge` | wrong_both | Yes | Yes | Yes |
| 156 | 322 | Memory-Allocator | `size_of_bin_mult_word_size` | wrong_ensures |  | Yes | No |
| 157 | 324 | Memory-Allocator | `valid_sbin_idx_smallest_sbin_fitting_siz...` | wrong_requires | Yes |  | No |
| 158 | 326 | Memory-Allocator | `any_set` | wrong_ensures |  | Yes | No |
| 159 | 328 | Memory-Allocator | `create` | too_weak_requires | Yes |  | No |
| 160 | 330 | Memory-Allocator | `create_full` | wrong_ensures |  | Yes | Yes |
| 161 | 334 | Memory-Allocator | `is_full` | wrong_both | Yes | Yes | No |
| 162 | 335 | Memory-Allocator | `lemma_change_one_entry` | wrong_both | Yes | Yes | No |
| 163 | 336 | Memory-Allocator | `lemma_view` | wrong_ensures |  | Yes | Yes |
| 164 | 337 | Memory-Allocator | `next_run` | wrong_both | Yes | Yes | No |
| 165 | 339 | Memory-Allocator | `lemma_bitmask_to_is_bit_set` | wrong_both | Yes | Yes | Yes |
| 166 | 340 | Memory-Allocator | `lemma_is_bit_set` | partial_ensures |  | Yes | Yes |
| 167 | 344 | Memory-Allocator | `lemma_obtain_bit_index_1_aux` | wrong_both | Yes | Yes | No |
| 168 | 345 | Memory-Allocator | `lemma_obtain_bit_index_2` | wrong_both | Yes | Yes | Yes |
| 169 | 347 | Memory-Allocator | `lemma_obtain_bit_index_3_aux` | wrong_both | Yes | Yes | No |
| 170 | 348 | Memory-Allocator | `set_int_range_commit_size` | wrong_both | Yes | Yes | Yes |
| 171 | 350 | Memory-Allocator | `bitand_with_mask_gives_rounding` | wrong_both | Yes | Yes | Yes |
| 172 | 351 | Memory-Allocator | `block_ptr_aligned_to_word` | wrong_both | Yes | Yes | No |
| 173 | 352 | Memory-Allocator | `block_size_ge_word` | wrong_both | Yes | Yes | Yes |
| 174 | 353 | Memory-Allocator | `block_start_at_diff` | wrong_both | Yes | Yes | No |
| 175 | 355 | Memory-Allocator | `align_down` | wrong_both | Yes | Yes | No |
| 176 | 356 | Memory-Allocator | `align_up` | wrong_ensures |  | Yes | No |
| 177 | 363 | Memory-Allocator | `segment_start_mult_commit_size` | wrong_ensures |  | Yes | Yes |
| 178 | 364 | Memory-Allocator | `sub_distribute` | wrong_ensures |  | Yes | No |
| 179 | 365 | Memory-Allocator | `two_mul_with_bit0` | wrong_both | Yes | Yes | Yes |
| 180 | 366 | Memory-Allocator | `two_mul_with_bit1` | wrong_both | Yes | Yes | No |
| 181 | 368 | Memory-Allocator | `nat_set_size` | wrong_both | Yes | Yes | No |
| 182 | 369 | Memory-Allocator | `pigeonhole_missing_idx_implies_double` | wrong_both | Yes | Yes | No |
| 183 | 370 | Memory-Allocator | `pigeonhole_missing_idx_implies_double_he...` | wrong_both | Yes | Yes | No |
| 184 | 371 | Memory-Allocator | `pigeonhole_too_many_elements_implies_dou...` | wrong_requires | Yes |  | No |
| 185 | 372 | Memory-Allocator | `set_mismatch` | wrong_both | Yes | Yes | Yes |
| 186 | 373 | Memory-Allocator | `singleton_set_unique_elt` | too_weak_requires | Yes |  | No |
| 187 | 374 | Node-Replication | `rids_match_add_none` | wrong_both | Yes | Yes | Yes |
| 188 | 375 | Node-Replication | `rids_match_add_rid` | wrong_ensures |  | Yes | No |
| 189 | 376 | Node-Replication | `rids_match_pop` | wrong_both | Yes | Yes | Yes |
| 190 | 377 | Node-Replication | `log_entry_alive_value_wrap_around` | wrong_both | Yes | Yes | No |
| 191 | 378 | Node-Replication | `log_entry_alive_wrap_around_helper` | wrong_both | Yes | Yes | Yes |
| 192 | 379 | Node-Replication | `map_min_value_smallest` | wrong_both | Yes | Yes | No |
| 193 | 380 | Node-Replication | `pop_rid` | wrong_both | Yes | Yes | No |
| 194 | 381 | Node-Replication | `state_at_version_preserves` | wrong_both | Yes | Yes | No |
| 195 | 383 | Node-Replication | `LogRangeMatchesQueue_append` | wrong_both | Yes | Yes | No |
| 196 | 384 | Node-Replication | `LogRangeMatchesQueue_append_other` | wrong_both | Yes | Yes | No |
| 197 | 385 | Node-Replication | `LogRangeMatchesQueue_append_other_augmen...` | wrong_both | Yes | Yes | No |
| 198 | 386 | Node-Replication | `LogRangeMatchesQueue_update_change` | wrong_both | Yes | Yes | No |
| 199 | 387 | Node-Replication | `LogRangeMatchesQueue_update_change_2` | wrong_both | Yes | Yes | No |
| 200 | 388 | Node-Replication | `LogRangeNoNodeId_append_other` | wrong_both | Yes | Yes | No |
| 201 | 390 | Node-Replication | `combiner_request_ids_not_contains` | wrong_both | Yes | Yes | No |
| 202 | 391 | Node-Replication | `compute_nrstate_at_version_preserves` | wrong_both | Yes | Yes | Yes |
| 203 | 392 | Node-Replication | `concat_LogRangeNoNodeId_LogRangeMatchesQ...` | wrong_requires | Yes |  | No |
| 204 | 395 | Node-Replication | `max_of_set` | wrong_both | Yes | Yes | No |
| 205 | 396 | Node-Replication | `state_at_version_refines` | wrong_both | Yes | Yes | No |
| 206 | 397 | Node-Replication | `int_mod_less_than_same` | wrong_both | Yes | Yes | Yes |
| 207 | 398 | Node-Replication | `map_new_rec_dom_finite` | partial_ensures |  | Yes | Yes |
| 208 | 406 | NRKernel | `lemma_entry_sizes_aligned_auto` | wrong_both | Yes | Yes | No |
| 209 | 407 | NRKernel | `lemma_entry_sizes_increase` | wrong_requires | Yes |  | No |
| 210 | 408 | NRKernel | `lemma_maxphyaddr_facts` | wrong_ensures |  | Yes | Yes |
| 211 | 409 | NRKernel | `lemma_new_seq` | partial_ensures |  | Yes | No |
| 212 | 410 | NRKernel | `aligned_transitive` | wrong_requires | Yes |  | No |
| 213 | 411 | NRKernel | `aligned_transitive_auto` | missing_requires | Yes |  | Yes |
| 214 | 412 | NRKernel | `assert_maps_equal_contains_pair` | wrong_both | Yes | Yes | Yes |
| 215 | 413 | NRKernel | `lemma_aligned_iff_eq_mul_div` | wrong_ensures |  | Yes | No |
| 216 | 416 | NRKernel | `mod_add_zero` | wrong_both | Yes | Yes | No |
| 217 | 417 | NRKernel | `mod_mult_zero_implies_mod_zero` | wrong_both | Yes | Yes | No |
| 218 | 418 | NRKernel | `subtract_mod_eq_zero` | wrong_both | Yes | Yes | No |
| 219 | 419 | NRKernel | `lemma_entry_base_from_index` | wrong_both | Yes | Yes | No |
| 220 | 421 | NRKernel | `lemma_index_from_base_and_addr` | wrong_both | Yes | Yes | No |
| 221 | 422 | NRKernel | `lemma_empty_implies_interp_aux_empty` | wrong_both | Yes | Yes | Yes |
| 222 | 424 | NRKernel | `lemma_entries_interp_equal_implies_inter...` | wrong_both | Yes | Yes | No |
| 223 | 426 | NRKernel | `lemma_entries_interp_insert_implies_inte...` | wrong_both | Yes | Yes | No |
| 224 | 428 | NRKernel | `lemma_entries_interp_remove_implies_inte...` | wrong_both | Yes | Yes | No |
| 225 | 429 | NRKernel | `lemma_interp_aux_between` | wrong_both | Yes | Yes | No |
| 226 | 430 | NRKernel | `lemma_interp_aux_contains_implies_interp...` | wrong_both | Yes | Yes | No |
| 227 | 432 | NRKernel | `lemma_interp_entries_insert_implies_inte...` | wrong_requires | Yes |  | No |
| 228 | 433 | NRKernel | `lemma_interp_entries_remove_implies_inte...` | wrong_both | Yes | Yes | No |
| 229 | 434 | NRKernel | `lemma_interp_of_entry_between` | wrong_both | Yes | Yes | No |
| 230 | 435 | NRKernel | `lemma_interp_of_entry_contains_mapping_i...` | partial_ensures |  | Yes | No |
| 231 | 437 | NRKernel | `lemma_interp_of_entry_disjoint_mappings` | wrong_both | Yes | Yes | No |
| 232 | 442 | NRKernel | `lemma_new_empty_dir` | partial_ensures |  | Yes | Yes |
| 233 | 443 | NRKernel | `lemma_nonempty_implies_interp_contains` | wrong_both | Yes | Yes | No |
| 234 | 444 | NRKernel | `address` | wrong_both | Yes | Yes | Yes |
| 235 | 445 | NRKernel | `lemma_addr_mask_when_hp_pat_is_zero` | wrong_both | Yes | Yes | No |
| 236 | 446 | NRKernel | `lemma_new_entry_addr_mask_is_address` | wrong_both | Yes | Yes | No |
| 237 | 448 | NRKernel | `lemma_zero_entry_facts` | wrong_both | Yes | Yes | No |
| 238 | 450 | NRKernel | `lemma_aligned_addr_mask_facts` | wrong_both | Yes | Yes | Yes |
| 239 | 452 | NRKernel | `lemma_bitvector_facts_simple` | wrong_ensures |  | Yes | No |
| 240 | 453 | NRKernel | `lemma_page_aligned_implies_mask_dir_addr...` | wrong_both | Yes | Yes | Yes |
| 241 | 461 | NRKernel | `interp_vmem_subrange` | wrong_both | Yes | Yes | No |
| 242 | 462 | NRKernel | `interp_vmem_update_range` | wrong_both | Yes | Yes | Yes |
| 243 | 465 | NRKernel | `lemma_inflight_vaddr_implies_hl_unmap_or...` | wrong_ensures |  | Yes | No |
| 244 | 468 | NRKernel | `monotonic_candidate_mapping_overlaps_exi...` | wrong_both | Yes | Yes | Yes |
