# Weekly Progress (2026-10-20)

Snapshot: **2026-10-20 11:06 UTC**.

## Takeaway Messages

1. **Spec generation: cleaned up the artifacts, ported them to the latest Verus, and submitted two reviewable cases.** After freezing the Slice and Vec results, we ported complete semantic API families to `upstream/main@775e2d403`, integrating 54 Slice bindings and 45 Vec bindings. The Slice case (54 bindings) and Vec PartialEq case (22 new bindings) have been pushed to the [`slice-vec-1020` branch](https://github.com/q5438722/verus/tree/slice-vec-1020), at commit [`318e45c56`](https://github.com/q5438722/verus/commit/318e45c56afbd37559361d946a2839499bf3dc46). 
2. **Spec generation: added 151 specification bindings this week, increasing the total from 92 to 243.** Slice added 98 (78 to 176), and Vec added 53 (14 to 67).
3. **Wrapping up for spec generation** Noticing that waiting for all generated results to receive human review is consuming, we would like to discuss whether this would be sufficient to wrap up the current phase.
4. **Spec provability: preliminary results** The first rule-based experiment completed 783 proof sessions. Paired comparisons include both variant-only and original-only successes, but no individual rule shows a significant improvement after multiple-comparison correction, and time/token statistics show no consistent advantage.
5. **Spec provability: exploring more rules and LLM-generated equivalent variants.** We are exploring 10 additional rule-based rewrites and LLM-written variants.

## 1. Spec Generation for the Verus Standard Library

### This Week's Progress

After fixing or explicitly skipping the issues discussed last week, we continued generating specifications for Rust standard-library APIs, still focusing primarily on **Slice and Vec**.

- **Addressed the previous blockers.** For issues such as callback functions, we continue generating specs where faithful modeling is possible with existing support. Unsupported cases are recorded and skipped so that a few difficult APIs do not block the rest. Skipping a case does not mean the issue is solved, and skipped cases do not count toward completed coverage.
- **Generated additional specifications.** Based on a mechanical count of the active campaign's canonical bindings, Slice increased from 78 to 176, and Vec from 14 to 67, adding **151 bindings this week**. This measures the specification bindings currently collected by the generation pipeline, not the number of APIs that have passed correctness checks, completeness checks, and human review.

### Current Spec-Generation Progress

| Module | Total APIs | Canonical bindings, Oct. 13 | Canonical bindings, Oct. 20 | Added this week | Pushed in this submission |
|---|---:|---:|---:|---:|---:|
| Slice | 276 | 78 | 176 | +98 | 54 |
| Vec | 104 | 14 | 67 | +53 | 22 |
| **Total** | **380** | **92** | **243** | **+151** | **76** |

### The Two Submitted Cases

| Case | Submitted content | New bindings | Current scope |
|---|---|---:|---|
| [Slice](https://github.com/q5438722/verus/tree/slice-vec-1020/source/vstd/std_specs/slice) | `arrays`: 27; `ascii`: 7; `chunk_by`: 11; `windows`: 9 | 54 | Four complete semantic families; each binding has a corresponding client test |
| [Vec](https://github.com/q5438722/verus/tree/slice-vec-1020/source/vstd/std_specs/vec) | `partial_eq` | 22 | `eq`/`ne` for 11 heterogeneous type combinations; reuses two existing upstream Vec-to-Vec bindings, giving family coverage of 24/24 |

### Current Bottlenecks

After fixing or working around the previous blockers, the following two categories remain. Based on last week's discussion, these can be left out of scope for now.

| Bottleneck | Representative case | Current handling |
|---|---|---|
| The latest Verus trait-impl external-spec constraints conflict with the existing iterator abstraction | The Slice `Chunks` family had 69 bindings under the previous model. The latest Verus requires overridden methods from the same impl to be registered together, but adding `next`/`next_back` creates a circular dependency with `IteratorSpec::remaining`. | Do not submit a partial family; omit `Chunks` as a whole for now, retaining the issue and evidence. |
| Rust 1.99 API/impl identities do not match the current external-specification mechanism | The impl identity for `Vec::push_mut` does not match; `[T; N]::try_from(Vec<T, A>)` has become a const impl. | Do not work around these issues by weakening contracts or changing the calling domain; omit the affected families for now. |

### Key Discussion: What Is Sufficient to Wrap Up?

If it is difficult to find enough reviewers for all generated results, could we define the goal for this phase as **a thorough review of the defined Slice and Vec scopes, using them as cases to demonstrate the method's effectiveness, rather than waiting for the entire standard library to be reviewed?**

Possible completion criteria are:

1. **Fix the evaluation scope.** Define the target API set for each module, distinguishing completed items, reused upstream specs, temporarily skipped items, and unresolved cases.
2. **Retain reproducible evidence.** Provide the corresponding source, correctness and completeness results, and necessary limitations for each included specification.
3. **Complete human review within that scope.** Ask reviewers to assess whether the specs match API semantics and whether the modeling and assumptions underlying the automated checks are reasonable; record review findings and fixes.

## 2. Spec Provability: Do Equivalent Formulations Affect Proof Generation?

### Research Question and Method

This direction asks: **without changing a spec's semantics, can changes to its formulation improve proof-generation success rates or reduce the time and token cost of generating proofs?**

Based on the equivalence-checking method previously proposed by MSRI, we first implemented a **spec equivalence checker**. It keeps preconditions and the calling domain unchanged and compares the original and rewritten specs over shared inputs and outputs/poststates:

```text
forall input i, output/poststate o:
    requires(i) ==> (spec_original(i, o) <==> spec_variant(i, o))
```

This separates differences due to formulation from those due to weakening or otherwise changing the specification's semantics. Rewrites affect only the target function's `ensures`; they leave `requires`, the function body, and the remaining context unchanged. Only candidates that pass equivalence checking enter the first-round proof-generation comparison.

### First Experiment: 5 Basic Equivalence-Preserving Rewrites

We implemented and froze 5 basic AST-based rewrite rules, then completed candidate generation and equivalence checking across 849 single-file tasks in VeruSAGE-Bench. Each rule produces at most one nontrivial candidate per task.

| Rule | Rewrite | Candidates generated | Confirmed equivalent |
|---|---|---:|---:|
| `reorder_ensures` | Reverse the order of multiple top-level `ensures` conditions | 87 | 27 |
| `commute_boolean` | Swap the operands of Boolean conjunction or disjunction | 50 | 2 |
| `implication_to_disjunction` | Rewrite `A ==> B` as `!A \|\| B` | 74 | 14 |
| `de_morgan_or_double_negation` | Apply De Morgan's laws or eliminate double negation | 0 | 0 |
| `orient_symmetric_relation` | Swap the sides of an equality/disequality, or swap comparison operands and reverse the comparison direction | 146 | 48 |

For tasks where multiple rules passed equivalence checking, we also combined the rewrites in a fixed order, obtaining another 26 confirmed-equivalent combined candidates for secondary analysis.

We then generated proofs for the original and equivalent rewritten versions using the same **`gpt-5.6-sol-fast` model, prompt settings, tools, and budget**:

**Original spec -> AST rewrite -> equivalence checking -> proof generation under consistent settings -> paired comparison.**

The experiment ran 666 original conditions and 117 equivalent transformed conditions, totaling **783 proof sessions, with one run per condition**. All sessions have ended, with zero provider, toolchain, or runner infrastructure failures.

### Rule-Based Rewrite Results

The original conditions achieved an overall first-attempt verified success rate of **182/666 = 27.33%**. `Variant-only` means that only the rewritten version succeeded; `Original-only` means that only the original version succeeded.

| Rule | Paired tasks | Original success | Variant success | Variant-only | Original-only | Holm-adjusted p-value |
|---|---:|---:|---:|---:|---:|---:|
| `reorder_ensures` | 27 | 9/27 = 33.33% | 6/27 = 22.22% | 3 | 6 | 1.0 |
| `commute_boolean` | 2 | 1/2 = 50.00% | 1/2 = 50.00% | 0 | 0 | 1.0 |
| `implication_to_disjunction` | 14 | 4/14 = 28.57% | 4/14 = 28.57% | 0 | 0 | 1.0 |
| `orient_symmetric_relation` | 48 | 14/48 = 29.17% | 14/48 = 29.17% | 4 | 4 | 1.0 |
| `combined` | 26 | 8/26 = 30.77% | 9/26 = 34.62% | 3 | 2 | NA |

**We have not found an individual rule that consistently improves overall success rates.** No individual rule shows a significant improvement after multiple-comparison correction. The success-rate point estimate decreases by 11.11 percentage points for `reorder_ensures` and increases by 3.85 percentage points for `combined`; however, the latter is a secondary analysis with a wide confidence interval, so it does not establish a consistent benefit.

### Key Discussion: Is This Direction Worth Exploring Further?

The first experiment observed success/failure differences between equivalent versions, but has not identified **a reproducible, predictable rewrite strategy with a net benefit**. This leaves two possible directions:

1. Expand rule coverage and introduce more flexible LLM-generated candidates, while continuing to enforce semantic equivalence through checking. **[Ongoing]**
2. Explore equivalent rewriting as a fallback after proof-generation failure: if direct proof generation fails, rewrite the spec equivalently and try again, rather than continuing to attempt proofs against the same formulation.
