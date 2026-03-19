# Intent Formalization

Research project using LLMs to generate and evaluate formal specifications for the [Verus](https://github.com/verus-lang/verus) verified systems programming language.

## Case Studies

| Case | Description | Layout |
|------|-------------|--------|
| **Bitmap** | Bitmap allocator from the Nanvix OS | Multi-file: `lib.rs`, `lib.spec.rs`, `lib.proof.rs`, `lib.test.rs` |
| **VeruSAGE** | 849-task benchmark (single-file Verus programs) | JSONL, one `.rs` file per task |

## Repository Structure

```
├── bitmap/                     # Spec generation pipeline + Bitmap case study
│   ├── spec_gen.py             # LLM-based spec generation
│   ├── spec_proof_gen.py       # LLM-based spec + proof generation
│   ├── verusage.py             # Strip specs from tasks (tree-sitter)
│   ├── prompts/                # Prompt templates and YAML configs
│   ├── bitmap_new/             # Refactored Bitmap (attribute macros)
│   ├── bitmap_raw/             # Original Bitmap (verus-native)
│   ├── bitmap_test/            # Test harness
│   └── libs/                   # Verus library dependencies
│
├── framework/                  # Test generation & spec quality framework
│   ├── test_gen.py             # Generate verified test functions via LLM
│   ├── spec_checker.py         # Evaluate spec quality using tests
│   ├── verus_runner.py         # Verus verifier wrapper
│   ├── adapters/
│   │   ├── base.py             # CaseAdapter interface + data classes
│   │   ├── bitmap.py           # Bitmap multi-file adapter
│   │   └── verusage.py         # VeruSAGE single-file adapter
│   └── prompts/
│       └── test_gen_template.md
│
└── CLAUDE.md                   # Guidance for Claude Code
```

## Prerequisites

- **Python 3.12+**
- **Verus** — build from source (see `bitmap/build_verus.sh`) or use the Docker image
- **GitHub Copilot CLI** — used as the LLM backend for generation
- **tree-sitter Verus parser** (`verus.so`) — for AST-based spec manipulation
- **PyYAML** — for prompt config loading

### Environment Variables

| Variable | Description |
|----------|-------------|
| `VERUS_BIN` | Path to the Verus verifier binary |
| `VERUS_PATH` | Path to the Verus release directory |
| `COPILOT_BIN` | Path to the GitHub Copilot CLI binary |

## Test Generation & Spec Quality Framework

### Motivation

LLM-generated specs may be syntactically valid but semantically wrong. Verified test functions exercise `requires`/`ensures` clauses and can discriminate between correct and incorrect specs. Unlike runtime tests, Verus verified tests prove assertions hold for **all** inputs satisfying preconditions.

### Pipeline Overview

```
Source Code (Bitmap or VeruSAGE)
        │
        ▼
   CaseAdapter          ← normalises different file layouts
        │
        ▼
   test_gen.py           ← LLM generates verified test functions
   (Copilot CLI)
        │
        ▼
   spec_checker.py       ← evaluates spec quality
   ├─ Step 1: Validate tests against ground-truth spec (must verify)
   ├─ Step 2: Run valid tests against LLM-generated spec
   └─ Step 3: Quality score = passed / valid  (1.0 = perfect)
        │
        ▼
   spec_quality_report.jsonl
```

### Step 1: Generate Verified Tests

Generate test functions for the Bitmap case study:

```bash
python -m framework test_gen \
  --case bitmap \
  --project_dir ./bitmap \
  --variant bitmap_new \
  --output_dir ./output/test_gen/ \
  --models claude-opus-4.6 \
  --num_tests 5 \
  --max_workers 4 \
  --timeout 600
```

Generate test functions for VeruSAGE tasks:

```bash
python -m framework test_gen \
  --case verusage \
  --tasks_jsonl /path/to/tasks.jsonl \
  --output_dir ./output/test_gen/ \
  --language_path /path/to/verus.so \
  --models claude-opus-4.6 \
  --num_tests 3 \
  --max_workers 4
```

Output: `<output_dir>/<model>/test_gen_results.jsonl` — one line per task with `task_id`, `test_code`, etc.

### Step 2: Evaluate Spec Quality

Score LLM-generated specs against the generated tests:

```bash
python -m framework spec_checker \
  --case verusage \
  --tasks_jsonl /path/to/tasks.jsonl \
  --test_results ./output/test_gen/claude-opus-4.6/test_gen_results.jsonl \
  --spec_results ./output/spec_gen/claude-opus-4.6/results.jsonl \
  --output_dir ./output/eval/ \
  --timeout 300
```

For Bitmap:

```bash
python -m framework spec_checker \
  --case bitmap \
  --project_dir ./bitmap \
  --test_results ./output/test_gen/claude-opus-4.6/test_gen_results.jsonl \
  --output_dir ./output/eval/
```

Output: `<output_dir>/spec_quality_report.jsonl` — per-task quality scores.

### Adapter Pattern

The framework uses adapters to handle the two case studies uniformly:

| Adapter | `iter_tasks()` | `build_verifiable_source()` |
|---------|---------------|----------------------------|
| `BitmapAdapter` | Yields one task per public function in `lib.rs` | Injects tests into `lib.test.rs` before `} // verus!` |
| `VeruSAGEAdapter` | Reads JSONL, yields one task per line | Injects tests into the `verus! {}` block of the single `.rs` file |

Both adapters implement the `CaseAdapter` interface from `framework/adapters/base.py`.

### Key Data Classes

- **`Task`** — one unit of work: `task_id`, `target_function`, `source_code`, and optional `source_no_spec` / `generated_spec`
- **`FunctionInfo`** — metadata about a Verus function: name, signature, specs, body
- **`VerificationResult`** — parsed Verus output: `verified`, `errors`, `success`, `summary`
- **`SpecQualityReport`** — evaluation result: `valid_tests`, `passed_tests`, `quality_score`

## Spec Generation (existing pipeline)

Generate specs for VeruSAGE tasks:

```bash
python bitmap/spec_gen.py \
  --input /path/to/tasks.jsonl \
  --output_dir /path/to/output/ \
  --models claude-opus-4.6 gpt-5.3-codex \
  --max_workers 4 --timeout 300
```

Generate specs + proofs:

```bash
python bitmap/spec_proof_gen.py \
  --input /path/to/tasks.jsonl \
  --output_dir /path/to/output/ \
  --models claude-opus-4.6 gpt-5.3-codex \
  --max_workers 4 --timeout 600
```

Strip specs from tasks (prepare spec-less inputs):

```bash
python bitmap/verusage.py \
  --input /path/to/tasks.jsonl \
  --output /path/to/tasks_no_spec.jsonl \
  --language_path /path/to/verus.so
```
