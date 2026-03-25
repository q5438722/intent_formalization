#!/home/chentianyu/miniconda3/bin/python3
"""
Step 4: Critic — filter verified φ into true/false positives + write summary.

Reads:  workspace_v3/<task_name>/entailment_results.json + original.rs
Writes: workspace_v3/<task_name>/critic_raw.txt
        workspace_v3/<task_name>/verdicts.json
        workspace_v3/<task_name>/summary.md

Usage:
  python3 step4_critic.py [--limit N] [--offset N] [--model MODEL]
"""

import argparse
import json
import re
import sys
import time
from pathlib import Path

BASE = Path.home() / "intent_formalization"
WORKSPACE = BASE / "verusage" / "workspace_v3"

sys.path.insert(0, str(BASE / "src" / "utils"))
from llm import LLMClient

CRITIC_PROMPT = """You are a spec consistency critic for Verus.

You receive a Verus source file and verified candidate properties (φ) that the spec ENTAILS.
Each φ targets an EXECUTABLE function's spec. For each φ, decide:
- TRUE_POSITIVE: real spec issue (too weak/strong, missing axiom, unverified assumption, soundness gap)
- FALSE_POSITIVE: actually expected/desirable, or targets proof/spec fn instead of exec fn

Output EXACTLY this format for each:

===VERDICT_START===
PHI: <name>
VERDICT: TRUE_POSITIVE | FALSE_POSITIVE
CONFIDENCE: high | medium | low
REASONING: <2-3 sentences>
===VERDICT_END===

Then at the end:
===SUMMARY===
<one paragraph summarizing findings>
===END_SUMMARY===
"""


def parse_verdicts(text: str) -> list:
    verdicts = []
    for match in re.finditer(r'===VERDICT_START===(.*?)===VERDICT_END===', text, re.DOTALL):
        block = match.group(1).strip()
        phi_m = re.search(r'PHI:\s*(.+)', block)
        verdict_m = re.search(r'VERDICT:\s*(.+)', block)
        conf_m = re.search(r'CONFIDENCE:\s*(.+)', block)
        reason_m = re.search(r'REASONING:\s*(.+)', block, re.DOTALL)

        if phi_m and verdict_m:
            reason = reason_m.group(1).strip() if reason_m else ""
            reason = reason.split("===")[0].strip()
            verdicts.append({
                "phi": phi_m.group(1).strip(),
                "verdict": verdict_m.group(1).strip(),
                "confidence": conf_m.group(1).strip() if conf_m else "unknown",
                "reasoning": reason,
            })
    return verdicts


def parse_summary(text: str) -> str:
    m = re.search(r'===SUMMARY===(.*?)===END_SUMMARY===', text, re.DOTALL)
    return m.group(1).strip() if m else ""


def extract_spec_portion(source_text: str, max_lines: int = 400) -> str:
    lines = source_text.split('\n')
    if len(lines) <= max_lines:
        return source_text
    spec_keywords = ['requires', 'ensures', 'invariant', 'recommends', 'spec fn',
                     'proof fn', 'pub fn', 'fn ', 'struct ', 'impl ', 'trait ']
    important = set()
    for i, line in enumerate(lines):
        low = line.lower().strip()
        if any(kw in low for kw in spec_keywords):
            for j in range(max(0, i - 3), min(len(lines), i + 8)):
                important.add(j)
    for i in range(min(30, len(lines))):
        important.add(i)
    for i in range(max(0, len(lines) - 5), len(lines)):
        important.add(i)
    selected = sorted(important)[:max_lines]
    result = []
    prev = -2
    for i in selected:
        if i > prev + 1:
            result.append(f"// ... (lines {prev+2}-{i-1} omitted) ...")
        result.append(lines[i])
        prev = i
    return '\n'.join(result)


def write_summary_md(task_dir: Path, source_path: str, phis: list, verdicts: list, llm_summary: str):
    tp = [v for v in verdicts if v.get("verdict") == "TRUE_POSITIVE"]
    fp = [v for v in verdicts if v.get("verdict") == "FALSE_POSITIVE"]
    verified = [p for p in phis if p.get("entailed")]

    md = f"# Spec Consistency Report\n\n"
    md += f"**Source:** `{source_path}`\n"
    md += f"**Date:** {time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())}\n\n"
    md += f"## Stats\n\n"
    md += f"- Candidates generated: {len(phis)}\n"
    md += f"- Entailed (verified): {len(verified)}\n"
    md += f"- True positives: {len(tp)}\n"
    md += f"- False positives: {len(fp)}\n\n"

    if llm_summary:
        md += f"## Summary\n\n{llm_summary}\n\n"

    if tp:
        md += f"## True Positives\n\n"
        for v in tp:
            md += f"### {v['phi']}\n"
            md += f"- **Confidence:** {v.get('confidence', '?')}\n"
            md += f"- **Reasoning:** {v.get('reasoning', '?')}\n\n"

    md += f"## All Candidates\n\n"
    for i, p in enumerate(phis):
        md += f"### φ{i+1}: {p.get('name', '?')}"
        if p.get('target_fn'):
            md += f" → `{p['target_fn']}`"
        md += "\n"
        md += f"- **Type:** {p.get('type', '?')}\n"
        md += f"- **Entailed:** {'✅' if p.get('entailed') else '❌'}\n"
        if p.get('reason'):
            md += f"- **Why flagged:** {p['reason']}\n"
        for v in verdicts:
            if v.get("phi") == p.get("name"):
                md += f"- **Verdict:** {v['verdict']} ({v.get('confidence', '?')})\n"
                break
        md += "\n"

    (task_dir / "summary.md").write_text(md)


def process_one(task_dir: Path, llm: LLMClient, model: str) -> dict:
    ent_file = task_dir / "entailment_results.json"
    orig_file = task_dir / "original.rs"

    if not ent_file.exists():
        return {"task": task_dir.name, "status": "no_entailment"}

    phis = json.loads(ent_file.read_text())
    verified = [p for p in phis if p.get("entailed")]

    if not verified:
        # Still write summary for no-verified case
        write_summary_md(task_dir, str(orig_file), phis, [], "")
        return {"task": task_dir.name, "status": "no_verified", "verified": 0}

    source_text = orig_file.read_text()
    spec_text = extract_spec_portion(source_text)

    phi_desc = ""
    for p in verified:
        phi_desc += f"\n### φ: {p['name']}"
        if p.get('target_fn'):
            phi_desc += f" (targets `{p['target_fn']}`)"
        phi_desc += f"\nType: {p.get('type', '?')}\n"
        # Find the code from candidates.json
        candidates = json.loads((task_dir / "candidates.json").read_text())
        for c in candidates:
            if c["name"] == p["name"]:
                phi_desc += f"```verus\n{c['code']}\n```\n"
                break
        phi_desc += f"Reason flagged: {p.get('reason', '?')}\n"

    print(f"  [crit] {task_dir.name} — {len(verified)} verified")
    try:
        resp = llm.chat(
            CRITIC_PROMPT,
            f"Source file:\n```rust\n{spec_text}\n```\n\n"
            f"Verified candidate properties:\n{phi_desc}",
            model=model,
        )
        raw = resp.content
    except Exception as e:
        raw = f"ERROR: {e}"

    (task_dir / "critic_raw.txt").write_text(raw)

    verdicts = parse_verdicts(raw)
    llm_summary = parse_summary(raw)
    (task_dir / "verdicts.json").write_text(json.dumps(verdicts, indent=2))

    tp = [v for v in verdicts if v.get("verdict") == "TRUE_POSITIVE"]
    write_summary_md(task_dir, str(orig_file), phis, verdicts, llm_summary)

    print(f"  [done] {task_dir.name} — {len(tp)} TP / {len(verified)} verified")
    return {
        "task": task_dir.name,
        "status": "complete",
        "verified": len(verified),
        "true_positives": len(tp),
        "false_positives": len(verdicts) - len(tp),
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--limit", type=int, default=None)
    parser.add_argument("--offset", type=int, default=0)
    parser.add_argument("--model", type=str, default="claude-opus-4.6")
    args = parser.parse_args()

    # Find tasks with entailment results but no verdicts yet
    task_dirs = sorted([
        d for d in WORKSPACE.iterdir()
        if d.is_dir()
        and (d / "entailment_results.json").exists()
        and not (d / "verdicts.json").exists()
    ])

    task_dirs = task_dirs[args.offset:]
    if args.limit:
        task_dirs = task_dirs[:args.limit]

    print(f"Step 4: Critic for {len(task_dirs)} tasks (model={args.model})")

    llm = LLMClient(timeout=300)
    total_tp = 0

    for i, td in enumerate(task_dirs):
        print(f"\n[{i+1}/{len(task_dirs)}]")
        try:
            r = process_one(td, llm, args.model)
            total_tp += r.get("true_positives", 0)
        except Exception as e:
            print(f"  [error] {td.name}: {e}")

    print(f"\n=== Done: {total_tp} total true positives ===")


if __name__ == "__main__":
    main()
