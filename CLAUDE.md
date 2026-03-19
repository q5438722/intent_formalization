# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Research project using LLMs to generate formal specifications and proofs for the **Verus** verified systems programming language. Two case studies:
- **Bitmap Allocator** — multi-file Verus project with split spec/proof/test files
- **VeruSAGE** — benchmark of 849 single-file Verus tasks (JSONL format)

The tooling strips specifications from Verus source files, uses GitHub Copilot CLI to regenerate them, and evaluates quality via verified test generation.

## Key Commands

### Building Verus from source
```bash
# See bitmap/build_verus.sh
cd verus/source/ && bash ./tools/get-z3.sh && source ../tools/activate && vargo build --release
# Set VERUS_PATH to verus/source/target-verus/release
```

### Running spec generation
```bash
python bitmap/spec_gen.py \
  --input /path/to/tasks.jsonl \
  --output_dir /path/to/output/ \
  --models claude-opus-4.6 gpt-5.3-codex \
  --max_workers 4 --timeout 300
```

### Running spec + proof generation
```bash
python bitmap/spec_proof_gen.py \
  --input /path/to/tasks.jsonl \
  --output_dir /path/to/output/ \
  --models claude-opus-4.6 gpt-5.3-codex \
  --max_workers 4 --timeout 600
```

### Stripping specs from tasks
```bash
python bitmap/verusage.py \
  --input /path/to/tasks.jsonl \
  --output /path/to/tasks_no_spec.jsonl \
  --language_path /path/to/verus.so
```

### Constructing prompts from YAML configs
```bash
python bitmap/prompts/construct.py <config.yml> [-o output_file]
```

### Test generation (framework)
```bash
# Generate verified test functions for Bitmap:
python -m framework test_gen \
  --case bitmap --project_dir ./bitmap --variant bitmap_new \
  --output_dir /path/to/output/ --num_tests 5

# Generate verified test functions for VeruSAGE:
python -m framework test_gen \
  --case verusage --tasks_jsonl /path/to/tasks.jsonl \
  --output_dir /path/to/output/ --num_tests 3

# Evaluate spec quality using generated tests:
python -m framework spec_checker \
  --case verusage --tasks_jsonl /path/to/tasks.jsonl \
  --test_results /path/to/test_gen_results.jsonl \
  --spec_results /path/to/spec_gen_results.jsonl \
  --output_dir /path/to/eval/
```

## Environment Variables

- `COPILOT_BIN` — path to the GitHub Copilot CLI binary (defaults to `~/.vscode-server/.../copilotCli/copilot`)
- `VERUS_BIN` — path to the Verus verifier binary
- `VERUS_PATH` — path to the Verus release binary directory

## Architecture

### Core Pipeline
1. **`bitmap/verusage.py`** — Strips `requires`/`ensures`/`decreases`/`recommends` clauses from target functions using tree-sitter with a Verus grammar (`verus.so`)
2. **`bitmap/spec_gen.py`** — Orchestrates LLM-based spec generation via Copilot CLI with parallel workers; outputs JSONL results per model
3. **`bitmap/spec_proof_gen.py`** — Same as above but generates both specs AND proof bodies/assertions/loop invariants
4. **`bitmap/prompts/construct.py`** — Assembles prompts from YAML config files + markdown template components (`{{placeholder}}` interpolation)

### Bitmap Allocator Implementations
Three variants under `bitmap/`:
- **`bitmap_raw/`** — Original Verus-native implementation
- **`bitmap_new/`** — Refactored version using attribute macros with improved abstraction (specs moved from concrete `Bitmap` to abstract `BitmapView`)
- **`bitmap_test/`** — Test harness for evaluation

Each implementation follows a split-file pattern:
- `lib.rs` (implementation), `lib.spec.rs` (specifications), `lib.proof.rs` (proofs), `lib.test.rs` (tests)

### Dependencies
- **`bitmap/libs/`** — Verus library modules (bitmap, nanvix-*, sys, etc.) used by the allocator implementations
- **tree-sitter Verus parser** — imported from a sibling `static/` directory as `verus_parser`
- **PyYAML** — for prompt config loading

### Prompt System
- `bitmap/prompts/components/` — Reusable prompt fragments (system role, reasoning style, etc.)
- `bitmap/prompts/tests/` — Test prompt configurations
- Templates use `{{placeholder}}` markers filled from YAML-defined components

### Test Generation & Spec Quality Framework (`framework/`)
The framework evaluates LLM-generated specs by generating verified test functions and checking them against both ground-truth and generated specs.

**Pipeline:** Source → (adapter) → LLM generates tests → inject into source → Verus verifies → quality score

- **`framework/adapters/base.py`** — `CaseAdapter` abstract interface: `iter_tasks()`, `build_verifiable_source()`, `write_test_file()`
- **`framework/adapters/bitmap.py`** — Adapter for multi-file Bitmap project (injects tests into `lib.test.rs`)
- **`framework/adapters/verusage.py`** — Adapter for single-file VeruSAGE tasks (injects tests into `verus! {}` block)
- **`framework/test_gen.py`** — LLM-based test generation orchestrator (parallel, resumable, JSONL output)
- **`framework/spec_checker.py`** — Spec quality evaluator: validates tests against ground-truth, then scores generated specs
- **`framework/verus_runner.py`** — Verus binary wrapper with result parsing

**Spec quality scoring:** A test is "valid" if it verifies with the ground-truth spec. Quality score = (valid tests passing with generated spec) / (total valid tests). Score of 1.0 means the generated spec is indistinguishable from ground truth (by these tests).

## Verification

Success is measured by Verus compiler output: `"verification results:: N verified, 0 errors"`. The generation scripts parse this output to determine whether generated specs/proofs are correct.
