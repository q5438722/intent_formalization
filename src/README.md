# Spec Consistency Pipeline

Multi-agent pipeline for evaluating specification consistency via entailment-guided query generation.

## Architecture

```
Code + Spec + Tests
       │
       ▼
  Generator (Claude Opus 4.6)
  → Candidate adversarial queries φ
       │
       ▼
  Critic Ensemble (Claude Opus 4.6 + GPT-5.4 + Gemini 3.1 Pro)
  → Adversarial filtering via multi-turn dialogue
       │
       ▼
  Verus Entailment Check
  → S ⊢ φ ? → Inconsistency report
```

## Usage

```bash
# Set API key (GitHub Copilot)
export GITHUB_TOKEN=<your-token>

# Generate adversarial queries
python src/generate.py --spec path/to/spec.rs --output queries.json

# Run critic ensemble
python src/critic.py --queries queries.json --spec path/to/spec.rs --output filtered.json

# Verify via Verus
python src/verify.py --queries filtered.json --spec path/to/spec.rs --output report.json

# Or run full pipeline
python src/orchestrate.py --spec path/to/spec.rs --output results/
```

## Configuration

Edit `src/config.yaml` to adjust models, thresholds, and Verus paths.

## Requirements

```bash
pip install pyyaml
```

Requires `copilot` CLI in PATH (GitHub Copilot CLI).
