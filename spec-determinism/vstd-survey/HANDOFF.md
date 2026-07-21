# vstd determinism study — handoff

Last updated: 2026-07-19

This document is the primary handoff for the vstd specification inventory and
determinism experiments under `spec-determinism/vstd-survey/`.

## 1. Executive summary

The current work has:

1. inventoried the visible vstd module/spec surface;
2. built a vstd-specific runner on top of the existing spec-determinism code;
3. added source-line-qualified extraction for same-named impl methods;
4. run determinism checks over the original 111 AST-visible public exec
   definitions with explicit postconditions in a matching Verus/vstd snapshot;
5. manually audited the original 27 `R0 = unknown` results;
6. fixed several general equal-fn/view-generation bugs found by the experiment.
7. normalized `verus_!` aliases in the scanner, exposing 26 additional public
   postcondition targets in the matching snapshot;
8. audited all 44 exec definitions with no local postcondition and ran guarded
   determinism-reward negative controls.
9. ran `gpt-5-mini` over all 44 targets with one feedback round; the model
   moved from 29 to 4 `add_spec` decisions, with zero raw or guarded reward.

The current effective experiment result is:

| Result | Count |
|---|---:|
| Complete | 87 |
| Remaining unknown | 20 |
| Unsupported mutable-reference returns | 4 |
| SMT-confirmed `sat` witnesses | 0 |
| Total tested definitions | 111 |

The 20 remaining unknowns are semantically audited:

| Audit category | Count |
|---|---:|
| Should be complete; tooling/proof gap | 7 |
| Intentional/permitted nondeterminism | 9 |
| Genuine semantic underconstraint | 4 |
| Unresolved | 0 |

**Critical scope warning:** the scanner now sees `verus_! { ... }` aliases, but
the 111-target determinism experiment predates that fix. The matching snapshot
now contains 137 visible public exec definitions with postconditions, so 26
newly visible targets remain untested. These include the non-deprecated
`vstd::cell::invcell`, `cell::pcell`, and `cell::pcell_maybe_uninit` APIs.
The deprecated `vstd::cell::InvCell` was tested; the replacement
`vstd::cell::invcell::InvCell` has only been included in the no-post audit so
far.

## 2. Repository and environment

### Main repository

```text
/home/chentianyu/intent_formalization
```

Project directory:

```text
/home/chentianyu/intent_formalization/spec-determinism
```

The worktree was already heavily modified before this vstd work. Do not assume
that the entire diff of `gen_det.py` or `extractor.py` belongs to this effort.
The vstd-specific additions are described in this document and covered by
targeted self-tests.

At handoff time, the entire `spec-determinism/vstd-survey/` directory is still
untracked (`?? vstd-survey/`). A reset, clean checkout, or environment loss
would remove the handoff, scripts, summaries, and artifacts. Before destructive
git operations, preserve the directory and decide what to commit. A reasonable
split is:

- commit `HANDOFF.md`, `README.md`, the two Python scripts, generated CSV/JSON
  summaries, and selected review documents;
- do not blindly commit all raw `verus_log/` artifacts until a large-artifact
  storage policy is chosen.

### Current upstream Verus source

```text
~/verus
```

Pinned commit:

```text
cf3b5c3fb937b9effa9478d4735b49743d8646eb
```

vstd source:

```text
~/verus/source/vstd
```

This source is used by the latest inventory report.

### Matching executable experiment snapshot

The determinism experiments use:

```text
/home/chentianyu/nanvix/toolchain/verus
```

Version:

```text
0.2026.05.17.e479cce
commit e479cce36490b8fa4b0fd7755aa742aec354372c
toolchain 1.95.0-x86_64-unknown-linux-gnu
```

Matching vstd source:

```text
/home/chentianyu/nanvix/toolchain/verus/vstd
```

The experiments deliberately use this older source because source and compiled
vstd metadata must match. `~/verus` is newer and currently has no matching
built Verus/vstd bundle.

Do not use `/home/chentianyu/intent_formalization/verus` for these experiments:
that bundle is missing required proc-macro shared libraries and cannot import
its bundled `libvstd.rlib` successfully.

## 3. What is being checked

For a function specification:

```text
requires P(x)
ensures  Q(x, y)
```

spec-determinism asks whether two outputs satisfying the same specification can
differ:

```text
P(x) && Q(x, y1) && Q(x, y2) ==> equal(y1, y2)
```

Interpretation:

- `R0 = unsat`: the specification determines the selected semantic output;
- `R0 = sat`: confirmed specification nondeterminism;
- `R0 = unknown`: no verdict; requires equality/proof/solver audit;
- compile/extraction errors: pipeline coverage issue, not a spec verdict.

The method checks uniqueness of the formal contract. It does **not** prove that
an `assume_specification` agrees with the real Rust implementation, and it does
not detect over-strong but deterministic specs.

Core conceptual references:

- [spec-determinism skill](../../skills/spec-determinism/SKILL.md)
- [pipeline reference](../docs/pipeline-2026-06-02.en.md)
- [determinism funnel](../docs/determinism-funnel-framework.md)
- [unknown-handling strategy](../docs/unknown-handling-strategy-2026-05-15.md)
- [Phase 2 summary](../docs/PHASE2_SUMMARY.en.md)
- [abstract/view-quotient determinism](../docs/abstract-determinism-plan-2026-06-04.en.md)

## 4. vstd-survey directory map

```text
vstd-survey/
├── HANDOFF.md                         # this document
├── README.md                          # inventory/big-picture report
├── scan_vstd.py                       # source-level inventory scanner
├── run_determinism.py                 # vstd experiment runner
├── run_missing_spec_reward.py         # no-post audit/reward experiment
├── run_llm_missing_spec_feedback.py   # LLM generation + feedback loop
├── generated/
│   ├── inventory.json                 # full current-upstream inventory
│   ├── modules.csv                    # one row per visible module
│   ├── groups.csv                     # aggregate module groups
│   └── exec_functions.csv             # visible exec definitions/signatures
└── experiments/
    ├── REVIEW-2026-07-14.md           # combined experiment review
    ├── UNKNOWN-AUDIT-2026-07-15.md    # semantic audit of original 27 unknowns
    ├── MISSING-SPEC-REWARD-2026-07-20.md # no-post reward conclusion
    ├── LLM-MISSING-SPEC-FEEDBACK-2026-07-20.md # gpt-5-mini batch review
    ├── LLM-MISSING-SPEC-MODEL-COMPARISON-2026-07-20.md # model comparison
    ├── SPECGEN-EXPERIMENT-RESULTS-2026-07-21.md # consolidated examples/results
    ├── pilot-2026-07-14/              # initial array/bytes pilot
    ├── public-free-2026-07-14/        # 34 public free definitions
    ├── raw-pointer-strict-2026-07-14/ # strict equality rerun for 6 pointer APIs
    ├── impl-methods-2026-07-14/       # 77 line-qualified impl methods
    ├── missing-spec-reward-2026-07-20/# 44 no-post definitions
    ├── llm-missing-spec-gpt5mini-2026-07-20/ # gpt-5-mini loop
    └── llm-missing-spec-gpt56sol-2026-07-20/ # gpt-5.6-sol loop
```

Primary reading order:

1. this handoff;
2. [README.md](README.md) for the module inventory and broad vstd structure;
3. [experiment review](experiments/REVIEW-2026-07-14.md);
4. [27-unknown audit](experiments/UNKNOWN-AUDIT-2026-07-15.md);
5. [no-post reward review](experiments/MISSING-SPEC-REWARD-2026-07-20.md);
6. [LLM feedback review](experiments/LLM-MISSING-SPEC-FEEDBACK-2026-07-20.md);
7. [model comparison](experiments/LLM-MISSING-SPEC-MODEL-COMPARISON-2026-07-20.md);
8. [consolidated generated-spec examples](experiments/SPECGEN-EXPERIMENT-RESULTS-2026-07-21.md);
9. per-run `SUMMARY.md` and per-target artifacts.

## 5. Inventory methodology

Implemented in [scan_vstd.py](scan_vstd.py).

### Counting categories

The scanner separates:

- exec definitions with bodies;
- signature-only exec declarations;
- exec postconditions;
- `assume_specification` sites;
- model `spec fn` declarations;
- proof functions;
- axiom functions;
- external type/trait specs;
- View/DeepView implementations;
- macros and parser-error modules.

The current-upstream report at `cf3b5c3` contains:

```text
125 visible modules (build.rs excluded)
52,715 source lines
3,405 specification-related declaration sites
553 contract sites
328 visible exec declarations
245 visible exec definitions with bodies
151 visible public exec definitions
135 visible public exec definitions with postconditions
16 visible public exec definitions without postconditions
```

These are **source declaration-site** counts:

- macro templates count once, not once per expansion;
- parse-recovery modules are marked;
- the numbers are not semantic coverage percentages.

### Scanner implementation

The scanner uses `tree-sitter-verus` for functions and a lexical fallback for:

- `assume_specification`;
- `returns`;
- `default_ensures`;
- attributes and macro counts.

It writes JSON/CSV and updates the generated section of
[README.md](README.md) between marker comments.

### `verus_!` alias normalization

The parser recognizes normal `verus! { ... }` blocks but does not directly
parse functions inside aliases such as:

```rust
use verus as verus_;
verus_! {
    ...
}
```

`scan_vstd.py` now normalizes `verus_!` to `verus!` in an in-memory parse copy,
preserving source line numbers and leaving source files unchanged. This adds
coverage for:

```text
cell/invcell.rs
cell/pcell.rs
cell/pcell_maybe_uninit.rs
std_specs/cmp.rs
std_specs/core.rs
std_specs/iter.rs
std_specs/vec.rs
```

Other normalized alias files include `map.rs`, `tokens.rs`,
`std_specs/slice.rs`, and `std_specs/maybe_uninit.rs`.

The remaining source-level inventory limitation is macro expansion:
`macro_rules!` templates count once and generated concrete items/types/views are
not enumerated.

Concrete example:

- tested deprecated type:
  `vstd::cell::InvCell<T>` in `vstd/cell.rs`;
- omitted replacement:
  `vstd::cell::invcell::InvCell<T, Pred>` in
  `vstd/cell/invcell.rs`.

Both versions use the same weak `replace`/`get` result contract:

```rust
ensures self.inv(result)
```

The alias-aware inventory now sees both versions, but only the deprecated
version appears in the historical 111-target determinism run.

## 6. Experiment runner architecture

Implemented in [run_determinism.py](run_determinism.py).

For each target:

1. resolve module file;
2. call `extract_spec(source, fn, source_line=...)`;
3. construct an `EqualPolicy`;
4. call `build_det_check_spec` with the vstd `ViewRegistry`;
5. enumerate schemas and render the guarded template;
6. build a standalone Verus harness importing precompiled vstd;
7. run Verus with SMT logging;
8. load the largest SMT2 query with `build_schema_ctx`;
9. run `run_schema_search`;
10. classify with `classify_ok`;
11. persist all artifacts and update a summary.

### Target identity

Targets use:

```text
module:function@source_line
```

This is a **source-location identifier**, not necessarily a public Rust API
path. Private implementation modules are included in the identifier.

For example:

```text
source target id:
  contrib::exec_spec::map:get_ref@24

public trait method:
  vstd::contrib::exec_spec::ToRef::get_ref

concrete impl:
  <&HashMap<K,V> as ToRef<&HashMap<K,V>>>::get_ref
```

`contrib::exec_spec::map` is private (`mod map; pub use map::*;`), so a public
rustdoc page named `vstd::contrib::exec_spec::map::get_ref` is not expected.

Examples:

```text
hash_map:new@43
hash_map:new@209
cell:new@178
cell:new@344
```

The line is required because many methods repeat within one source file.

### View equality

The runner builds one `ViewRegistry` over the matching vstd source and passes it
to `build_det_check_spec`. This is necessary for wrappers such as:

- `HashMapWithView`;
- `HashSetWithView`;
- `StringHashMap`;
- `StringHashSet`.

Raw wrapper equality frequently produces false unknowns.

### Runner-specific compatibility handling

The runner currently contains snapshot-specific behavior:

- `extern crate alloc`;
- extra imports for `raw_ptr`, `hash_map`, and `hash_set`;
- suppression of invalid `reveal(...)` calls for imported closed spec
  functions;
- May-snapshot API rewrites:
  - `simple_pptr`: `.ptr().addr()` to `.pptr().addr()`;
  - deprecated `cell`: `.ptr().addr()` to `.id()`, with unusable scalar
    narrowing guards removed;
- detection of trivial `true` equal functions;
- optional strict pointer comparison;
- optional ViewRegistry disable switch;
- explicit unsupported status for returned `&mut T`.

These rewrites should not silently migrate into a general runner without
version gating.

## 7. Core code changes

### `extract/extractor.py`

File:

[spec_determinism/extract/extractor.py](../spec_determinism/extract/extractor.py)

Added:

- `source_line` argument to `_extract_fn_chunk`;
- line-aware function candidate selection;
- `_find_enclosing_node_by_line`;
- `source_line` argument to `extract_spec`;
- exact same-name function/method selection;
- line-aware impl context recovery;
- regression test selecting the second of two same-named impl methods.

Important current locations:

```text
_extract_fn_chunk(... source_line ...)       around line 416
_find_enclosing_node_by_line                 around line 1152
extract_spec(... source_line ...)             around line 1236
same-name regression test                     near end of extractor self-tests
```

### `codegen/gen_det.py`

File:

[spec_determinism/codegen/gen_det.py](../spec_determinism/codegen/gen_det.py)

vstd-driven fixes include:

1. `PointsToRaw` opacity checks only the outer type; a tuple containing a
   `PointsToRaw` field no longer collapses entirely to `true`.
2. `Tracked<T>`/`Ghost<T>` with an inner `spec_view` compare through `@@`.
3. registered View equality is preferred over `#[verifier::ext_equal]` wrapper
   equality.
4. the L3 generic-bound gate only requires View-like bounds for generic
   parameters actually projected through `T::V`.
5. concrete-key views such as `StringHashMap<Value> ->
   Map<Seq<char>, Value>` no longer incorrectly require `Value: View`.
6. regression self-tests cover all of the above.

Important current locations:

```text
_is_points_to_raw_type                      around line 120
Tracked/Ghost equality branch               around line 1862
view-first struct equality                  around line 2083
_generic_l3_view_bounds_satisfied           around line 2297
vstd regression fixtures                    in _run_self_tests
```

### Worktree warning

Both core files had substantial pre-existing modifications. The current
`git diff --stat` is much larger than the changes listed here. Review changes
by symbol/test, not by assuming every diff hunk was introduced by this work.

## 8. Experiment corpus

The historical matching-May experiment selected 111 public exec definitions
with explicit postconditions before alias normalization:

- 34 free functions;
- 77 impl methods.

After alias normalization, the same snapshot contains 137 visible public exec
definitions with postconditions. The additional 26 targets have not yet been
run through the full determinism experiment.

Selection criteria are read from the matching snapshot's generated
`exec_functions.csv`:

```text
node_kind == definition
visibility == public
contract_status == post
context == free       # free-function run
context != free       # impl-method run
```

### Free-function run

Directory:

[public-free-2026-07-14](experiments/public-free-2026-07-14/)

Default-policy summary:

```text
21 complete
5 unknown
6 invalid trivial raw-pointer equalities
2 unsupported mutable-reference returns
```

The six pointer targets were rerun strictly:

[raw-pointer-strict-2026-07-14](experiments/raw-pointer-strict-2026-07-14/)

All six strict-pointer checks are complete, producing the effective free
result:

```text
27 complete
5 unknown
2 unsupported
```

### Impl-method run

Directory:

[impl-methods-2026-07-14](experiments/impl-methods-2026-07-14/)

Current result after view/equality fixes:

```text
60 complete
15 unknown
2 unsupported
```

All 77 targets now compile; there are no residual runner/verus errors in the
final summary.

### Combined automated result

```text
87 complete
20 unknown
4 unsupported
0 R0=sat
```

“0 R0=sat” does not mean there is no incompleteness. Manual semantic audit
identified intentional and genuine underconstraint among solver-unknown cases.

### No-post reward experiment

Directory:

[missing-spec-reward-2026-07-20](experiments/missing-spec-reward-2026-07-20/)

The alias-aware matching snapshot contains 44 exec definitions without a local
postcondition:

```text
26 already inherit a trait contract
10 are compiler/runtime plumbing
 8 use linear resources, hidden state, prophecy, or mode-incompatible APIs
```

Therefore the effective no-spec set is 18, not 44. The 26 trait impls must be
excluded from missing-spec generation.

The alias normalization added exactly one newly visible public no-post method:

```text
vstd::cell::invcell::InvCell::set
```

Five diagnostic candidates were checked:

```text
InvCell::set          ensures self.inv(val)
unreached             ensures false
runtime_assert        ensures b
IsThread::clone x2    ensures result@ == self@
```

Result:

```text
raw determinism reward: 3
guarded reward:         0
```

The three positive raw rewards are redundant or vacuous:

- `InvCell::set` mirrors its requires clause and returns unit;
- `unreached` has `requires false`;
- `runtime_assert` mirrors its requires clause and returns unit.

This demonstrates that determinism alone is not a valid reward for no-output,
false-precondition, hidden-state, or linear-resource functions. No generated
postcondition is recommended for upstream vstd from this batch.

### LLM generation + feedback batch

Directory:

[llm-missing-spec-gpt5mini-2026-07-20](experiments/llm-missing-spec-gpt5mini-2026-07-20/)

`gpt-5-mini` independently generated candidates for all 44 targets and received
one round of checker and anti-vacuity feedback:

```text
initial: 29 add_spec, 15 skip
final:    4 add_spec, 40 skip
raw reward:     0
guarded reward: 0
```

Feedback removed all 26 duplicate trait contracts. The four remaining
`add_spec` decisions were still unusable: private invariant, natural-language
non-Verus text, or tracked/exec-mode-incompatible Clone contracts. See
[LLM-MISSING-SPEC-FEEDBACK-2026-07-20.md](experiments/LLM-MISSING-SPEC-FEEDBACK-2026-07-20.md).

The same batch was rerun with `gpt-5.6-sol`:

```text
initial:  5 add_spec, 39 skip
final:    5 add_spec, 39 skip
raw reward:     0
guarded reward: 0
```

`gpt-5.6-sol` was much more conservative initially, but feedback caused five
correct skips to regress into redundant trait specs. See
[LLM-MISSING-SPEC-MODEL-COMPARISON-2026-07-20.md](experiments/LLM-MISSING-SPEC-MODEL-COMPARISON-2026-07-20.md).

## 9. Original 27-unknown audit

Primary document:

[UNKNOWN-AUDIT-2026-07-15.md](experiments/UNKNOWN-AUDIT-2026-07-15.md)

Original classification:

```text
14 A: should be complete; tool/equality/proof gap
 9 B: intentional/permitted nondeterminism
 4 C: genuine semantic underconstraint
 0 D: unresolved
```

Seven A-cases (`StringHashMap`) were fixed by the generic L3 view-bound change,
leaving the current 20 automated unknowns:

```text
 7 A
 9 B
 4 C
```

### Remaining A: should be complete

- `atomic::{fetch_and, fetch_xor, fetch_or}`
  - macro-generated permission types are invisible to source-level type/view
    discovery;
  - corrected equality over `.view()` fields verifies.
- `raw_ptr::ptr_ref2`
  - raw `SharedReference` equality is too strong;
  - equality over `value()`, address, and metadata verifies.
- `thread::thread_id`
  - requires tracked result components and `IsThread::agrees`.
- deprecated `InvCell::new`
  - equality should compare the exposed invariant predicate, not hidden cell
    identity.
- `RwLock::acquire_read`
  - `ReadHandle::lemma_readers_match` is the exact missing proof.

### B: intentional/permitted nondeterminism

- `float::float_cast`;
- `raw_ptr::allocate`;
- `thread::{spawn, join}`;
- deprecated `PCell::{empty, new}`;
- `simple_pptr::{empty, new}`;
- `RwLock::new`.

These should be classified as `incomplete_permitted` under concrete identity.
Fresh-handle cases can become complete only under an explicit quotient that
ignores identity and observes content/predicate.

### C: genuine semantic underconstraint

- deprecated `InvCell::{replace, get}`;
- `RwLock::{acquire_write, into_inner}`.

These contracts constrain a returned value only through an arbitrary invariant
predicate:

```text
inv(result)
```

An invariant predicate need not be functional. Two distinct values can satisfy
it, so the result is genuinely not uniquely specified. This may be an
intentional information-hiding design, but it is still semantic
underconstraint.

## 10. Artifact format

Each target directory under `experiments/*/artifacts/` contains:

```text
result.json
det_spec.json
harness.rs
verus_stdout.txt
verus_stderr.txt
verus_log/
```

`verus_log/` contains SMT/AIR/transcript artifacts, including the SMT2 consumed
by schema search.

Example:

```text
experiments/impl-methods-2026-07-14/
  artifacts/hash_map__insert__L106/
    result.json
    det_spec.json
    harness.rs
    verus_log/root.smt2
    ...
```

The artifacts are large. Current post-cleanup sizes are approximately:

```text
public-free experiment: approximately 64 MB
impl-method experiment: approximately 99 MB
strict-pointer experiment: approximately 11 MB
pilot experiment: approximately 28 MB
```

Do not commit all logs without deciding on a storage policy. The Markdown/JSON
summaries and selected failing/interesting harnesses are much smaller.

## 11. Reproduction commands

Run from:

```bash
cd /home/chentianyu/intent_formalization/spec-determinism
```

### Current-upstream inventory

```bash
python vstd-survey/scan_vstd.py \
  --vstd-root /home/chentianyu/verus/source/vstd \
  --commit cf3b5c3fb937b9effa9478d4735b49743d8646eb \
  --snapshot-date 2026-07-13 \
  --source verus-lang/verus:source/vstd \
  --out-dir vstd-survey/generated
```

This regenerates the alias-aware source-level inventory. Macro-expanded
concrete items remain outside the count.

### Matching May inventory for experiment selection

```bash
python vstd-survey/scan_vstd.py \
  --vstd-root /home/chentianyu/nanvix/toolchain/verus/vstd \
  --commit e479cce36490b8fa4b0fd7755aa742aec354372c \
  --snapshot-date 2026-05-17 \
  --source local-matching-vstd \
  --out-dir vstd-survey/experiments/public-free-2026-07-14/inventory \
  --no-report
```

### One target

```bash
python vstd-survey/run_determinism.py \
  --vstd-root /home/chentianyu/nanvix/toolchain/verus/vstd \
  --verus-root /home/chentianyu/nanvix/toolchain/verus \
  --out /tmp/vstd-one \
  --target hash_map:insert@106 \
  --timeout 240 \
  --rlimit 60
```

### All visible public free definitions with postconditions

```bash
python vstd-survey/run_determinism.py \
  --vstd-root /home/chentianyu/nanvix/toolchain/verus/vstd \
  --verus-root /home/chentianyu/nanvix/toolchain/verus \
  --out vstd-survey/experiments/public-free-2026-07-14 \
  --targets-csv vstd-survey/experiments/public-free-2026-07-14/inventory/exec_functions.csv \
  --public-free-post \
  --timeout 240 \
  --rlimit 60
```

### All visible public impl methods with postconditions

```bash
python vstd-survey/run_determinism.py \
  --vstd-root /home/chentianyu/nanvix/toolchain/verus/vstd \
  --verus-root /home/chentianyu/nanvix/toolchain/verus \
  --out vstd-survey/experiments/impl-methods-2026-07-14 \
  --targets-csv vstd-survey/experiments/public-free-2026-07-14/inventory/exec_functions.csv \
  --public-impl-post \
  --timeout 300 \
  --rlimit 60
```

### All no-post definitions and guarded reward

```bash
python vstd-survey/run_missing_spec_reward.py \
  --inventory-csv vstd-survey/experiments/public-free-2026-07-14/inventory/exec_functions.csv \
  --vstd-root /home/chentianyu/nanvix/toolchain/verus/vstd \
  --verus-root /home/chentianyu/nanvix/toolchain/verus \
  --out vstd-survey/experiments/missing-spec-reward-2026-07-20 \
  --timeout 240 \
  --rlimit 60
```

### LLM generation with one feedback round

```bash
python vstd-survey/run_llm_missing_spec_feedback.py \
  --manifest vstd-survey/experiments/missing-spec-reward-2026-07-20/effective_manifest.json \
  --vstd-root /home/chentianyu/nanvix/toolchain/verus/vstd \
  --verus-root /home/chentianyu/nanvix/toolchain/verus \
  --out vstd-survey/experiments/llm-missing-spec-gpt5mini-2026-07-20 \
  --model gpt-5-mini \
  --jobs 4 \
  --feedback-rounds 1 \
  --llm-timeout 600 \
  --det-timeout 240 \
  --rlimit 60
```

### Strict raw-pointer equality

Use `--compare-raw-pointers` for targets whose default equality is the trivial
raw-pointer opacity policy. The exact six-target command is reflected in:

[raw-pointer strict summary](experiments/raw-pointer-strict-2026-07-14/SUMMARY.md)

### Targeted self-tests

```bash
python -m spec_determinism.extract.extractor test
python -m spec_determinism.codegen.gen_det test
python -m py_compile vstd-survey/scan_vstd.py vstd-survey/run_determinism.py
```

## 12. Known limitations

### Coverage

1. macro-expanded functions/types/views are not enumerated.
2. the 111-target determinism experiment predates alias normalization; 26 newly
   visible public postcondition targets remain untested.
3. `assume_specification` is inventoried lexically but not determinism-tested
   by this runner.
4. signature-only external trait specs are not tested.
5. current-upstream source has not been tested with a matching compiled
   toolchain.

### Extraction/codegen

1. returned `&mut T` is unsupported because result substitution does not model
   `old(result)`/`final(result)`;
2. macro-generated atomic permission views are not discovered;
3. split-accessor abstractions such as `SharedReference::{value, ptr}` need a
   data-driven projection policy;
4. proof-lemma discovery is manual (`IsThread::agrees`,
   `ReadHandle::lemma_readers_match`);
5. the runner's permitted flag is currently always set to `False`; semantic
   audit labels are in the audit document, not in `summary.json`;
6. ViewRegistry emits many parse-error warnings and may be incomplete.

### Solver interpretation

1. `unknown` is never a determinism verdict;
2. many queries report `incomplete quantifiers` or incomplete arithmetic;
3. schema narrowing cannot repair an over-strong equal-fn;
4. no SMT `sat` was observed, but manual probes established intentional and
   genuine nondeterminism.

## 13. Recommended next work

Priority order for the next owner:

### P0 — run the newly visible targets

Use the alias-aware matching inventory to run the 26 public postcondition
targets that were absent from the historical 111-target corpus, especially:

```text
cell/invcell.rs
cell/pcell.rs
cell/pcell_maybe_uninit.rs
std_specs/cmp.rs
std_specs/core.rs
std_specs/iter.rs
std_specs/vec.rs
```

### P1 — build current upstream Verus/vstd

Build a matching toolchain for `~/verus@cf3b5c3` and stop mixing the July
inventory with the May experiment snapshot.

### P2 — automate the audited A-cases

1. synthesize views for macro-generated atomic permission types;
2. add a projection policy for `SharedReference`;
3. preserve/use tracked output components for `IsThread::agrees`;
4. add proof hints for `ReadHandle::lemma_readers_match`;
5. use invariant-predicate equality for `InvCell::new`.

### P3 — encode permitted nondeterminism

Add vstd-specific permitted rules for:

- allocator/fresh identity;
- float-cast relations;
- thread handles;
- PCell/PPtr/RwLock constructors.

Do not hide these by a global `equal == true`; record the quotient or permitted
reason explicitly.

### P4 — decide what to do with genuine C-cases

For:

- `InvCell::{replace, get}`;
- `RwLock::{acquire_write, into_inner}`;

choose one:

1. accept and document possible-value abstraction;
2. add a ghost exact-current-value accessor;
3. weaken/change the API so exact returned values are not promised.

The same review must be repeated for the non-deprecated
`cell::invcell::InvCell`, which has the same `inv(result)` contract shape.

### P5 — add structured audit annotations

Move the A/B/C labels from Markdown into machine-readable result metadata so
aggregators can report:

```text
complete
complete_tool_gap
incomplete_permitted
incomplete
unsupported
unknown
```

## 14. Takeover checklist

Before changing behavior:

1. preserve/commit the currently untracked `vstd-survey/` directory before any
   reset or cleanup;
2. read this document;
3. read `UNKNOWN-AUDIT-2026-07-15.md`;
4. confirm which source/toolchain snapshot is being used;
5. run extractor and gen_det self-tests;
6. run one known-complete target (`bytes:u16_from_le_bytes@79`);
7. run one line-disambiguated target (`hash_map:insert@106`);
8. inspect its `harness.rs` and `det_spec.json`;
9. verify the equal-fn is non-trivial and matches the contract's abstraction.

Before publishing numbers:

1. state that the scanner normalizes `verus_!`, but the 111-target experiment
   predates that fix and is not full coverage;
2. state whether strict pointer equality is included;
3. separate automatic R0 results from manual semantic audit;
4. do not call `unknown` incomplete or complete;
5. do not claim the 111-target set is the full vstd.
