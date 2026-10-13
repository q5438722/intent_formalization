# Progress: Rust Standard-Library Specification Generation

Snapshot: **2026-10-13 11:43 UTC**.

This report covers the current progress and acceptance criteria of the Slice
and Vec campaigns, together with the collected specification artifacts. It
distinguishes historically supported scopes, usable API bindings, and ongoing
implementation and completeness work. A completed report or successful client
check does not mean that an entire module has been verified.

## 1. Current specification-generation criteria and added rules

### 1.1 Basic requirements: correctness and completeness

| Track | Required evidence |
|---|---|
| Correctness | The actual source body, or an authorized faithfully corresponding proof copy, establishes the current contract under the original calling domain. |
| Completeness | Real specdet analysis of the same contract, views, and calling domain, retaining the actual comparison, selected observations, query, and saved-input `replay --reexecute`. |

### 1.2 Added and clarified rules

- **Specify meaningful selected value/content observations.** Pointer and
  ownership properties are **not required** in public specifications or
  completeness checks; their absence alone is not incompleteness. This does
  not exclude observable element updates, capacity values, Result payloads,
  or callback effects that change the selected result. Internal correctness
  proofs may need memory machinery, but must not turn it into additional
  public calling requirements. This prevents low-level memory properties that
  are irrelevant to the public semantics from becoming unnecessary proof
  obstacles.
- **Reuse existing vstd abstractions before adding models, following Chris's
  feedback.** Use the current iterator `remaining`, the actual `Item` types,
  applicable old/final writeback relations, and source-backed views. Connect
  constructor, consumer, and observer behavior. Do not use construction-time
  auxiliary sequences as consumption-time invariants without a proof.
  Unconstrained answer-selecting helpers and new unapproved axioms are not
  repairs; any task-specific zero-UF requirement remains in force.
- **Write guarantees directly in `ensures`, following Jay's feedback.** Do
  not hide the entire contract behind `ensures some_post(...)`. Expand the
  actual return, content, branch, and writeback relations. A single explicit
  equation such as `result@ == input@` is fine; this is not a physical
  line-count rule. Meaningful shared views, sequence operations, and existing
  protocol functions remain reusable.
- **Clarify the equality boundary and proof-only visibility adjustments,
  following Jay's feedback.** `T: PartialEq<U>` does not mean that equality
  conforms to the Verus model. When needed, use
  `<T as PartialEqSpec<U>>::obeys_eq_spec()` to guard the entire postcondition
  rather than adding it as an API `requires`. Private-field visibility may be
  widened only in isolated correctness-proof source copies, while all fields,
  type layout, generics, bodies, and dispatch must otherwise remain unchanged.
  Public specifications and completeness checks must still use the original
  library, with an independent proof of the correspondence between the copy
  and the original type, views, and source.

## 2. Current results, ongoing work, and bottlenecks

### 2.1 Collected specification artifacts

An independent frozen collection currently contains **98 actual API
declarations**, with direct `ensures` clauses. Another **17 represented API
identities** reuse existing vstd contracts without duplicate declarations.

| Module | From root `mod.rs` | From other source files | Collected declarations | Upstream-reused entries | Supported identities represented |
|---|---:|---:|---:|---:|---:|
| Slice | 23 | 39 | 62 | 10 | 72 |
| Vec | 7 | 29 | 36 | 7 | 43 |
| Total | 30 | 68 | 98 | 17 | 115 |

The source-file split follows the original Rust implementation/origin location,
not the `Self` type or the trait declaration. For example,
`&[T] != Vec<U,A>` belongs to the Vec non-`mod.rs` file because its
implementation is in `partial_eq.rs`. Common Slice views and types are stored
once and are not counted as API contracts.

### 2.2 Remaining work

Within the initial 380-item queue, the collected evidence includes 36 newly
supported Slice scopes and 25 Vec scopes:

| Module | Initial active queue | Previously reviewed support within that queue | Not closed under the earlier scoped accounting |
|---|---:|---:|---:|
| Slice | 276 | 36 | 240 |
| Vec | 104 | 25 | 79 |
| Total | 380 | 61 | 319 |

**319 is a reconciliation reference, not a remaining count freshly audited
under the October 13 criteria.** The 98 collected declarations also include
37 earlier project results that were already excluded from the initial queue;
subtracting 98 from 380 would therefore be incorrect.

The aggregate tables currently lag the actual work: Slice's
`inventory/coverage.tsv` still dates from October 12 09:58 UTC, and Vec's
`coverage/INDEX.tsv` from October 12 23:34 UTC. Their old `not_started`,
`pending`, and unbound-candidate labels must be interpreted using the newer
family records, code, and review conclusions. A precise current total of APIs
that are both verified and complete has **not yet been re-established** for
the entire updated queue.

### 2.3 Current bottlenecks

| Bottleneck | Simple example | Response |
|---|---|---|
| Verifier/source compatibility | The original Drain source expression `elt as *const _` triggers raw-borrow lowering that the current Verus frontend does not support. | Submit an upstream PR to Verus to add support for this class of source patterns. Until the PR is merged and included in a release, maintain a project-specific local Verus version for checking the current Rust 1.96 source instead of rewriting std execution logic merely to work around verifier limitations. |
| Generic trait and state behavior | An `FnMut` callback may execute `count += 1` on every call, so the same input can produce different results and states at different calls. | A proof must not assume that the callback is pure, stateless, or deterministic, or narrow the original API's legal calling domain. The specification must describe the state change across calls. |
| Completeness extraction and solving | For a method returning `&mut T`, the checker must compare not only the returned value but also the writeback to the original container after the borrow ends. The current extractor sometimes cannot generate that observation, and the query returns `unknown (incomplete quantifiers)`. | `UNKNOWN` must not be classified directly as complete or incomplete. If the result remains `UNKNOWN` after checking the inputs, observations, and solver configuration, the case should be handed to an LLM for semantic analysis, with the original query, reason for the unresolved result, LLM analysis, and final disposition recorded. |


### 2.4 Artifact and evidence locations

The main collected specification files have been pushed to the
[`argus-run` branch](https://github.com/q5438722/rust-std-spec/tree/argus-run):

- [Slice root-module specifications (`slice_mod.rs`)](https://github.com/q5438722/rust-std-spec/blob/argus-run/slice_mod.rs)
- [Slice specifications from other source files (`slice_non_mod.rs`)](https://github.com/q5438722/rust-std-spec/blob/argus-run/slice_non_mod.rs)
- [Vec root-module specifications (`vec_mod.rs`)](https://github.com/q5438722/rust-std-spec/blob/argus-run/vec_mod.rs)
- [Vec specifications from other source files (`vec_non_mod.rs`)](https://github.com/q5438722/rust-std-spec/blob/argus-run/vec_non_mod.rs)
