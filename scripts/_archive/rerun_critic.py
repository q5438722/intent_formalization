#!/usr/bin/env python3
"""Re-run critic stage only for tasks where it failed with ARG_MAX error."""
import json
import os
import re
import sys
import time
from pathlib import Path

# Reuse functions from the main pipeline
sys.path.insert(0, str(Path(__file__).parent))
from run_full_pipeline import (
    call_llm, parse_verdicts, parse_summary, extract_spec_portion,
    write_summary_md, CRITIC_PROMPT, WORKSPACE, VERUSAGE
)

def rerun_critic(task_dir: Path):
    critic_file = task_dir / "critic_raw.txt"
    if not critic_file.exists():
        return
    content = critic_file.read_text()
    if "Argument list too long" not in content:
        return

    source_file = task_dir / "original.rs"
    if not source_file.exists():
        return

    source_text = source_file.read_text()
    spec_text = extract_spec_portion(source_text)

    # Load entailment results to find verified phis
    ent_file = task_dir / "entailment_results.json"
    if not ent_file.exists():
        return
    phis = json.loads(ent_file.read_text())
    verified_phis = [p for p in phis if p.get("entailed")]
    if not verified_phis:
        return

    task_name = task_dir.name
    print(f"  [re-crit] {task_name} — {len(verified_phis)} verified")

    phi_descriptions = ""
    for phi in verified_phis:
        phi_descriptions += f"\n### φ: {phi['name']}\n"
        phi_descriptions += f"Type: {phi['type']}\n"
        phi_descriptions += f"```verus\n{phi['code']}\n```\n"
        phi_descriptions += f"Reason flagged: {phi['reason']}\n"

    critic_response = call_llm(
        CRITIC_PROMPT,
        f"Source file:\n```rust\n{spec_text}\n```\n\nVerified candidate properties (spec entails all of these):\n{phi_descriptions}"
    )
    critic_file.write_text(critic_response)

    verdicts = parse_verdicts(critic_response)
    llm_summary = parse_summary(critic_response)
    (task_dir / "verdicts.json").write_text(json.dumps(verdicts, indent=2))

    true_positives = [v for v in verdicts if v.get("verdict") == "TRUE_POSITIVE"]

    result = {
        "task": task_name,
        "source": str(source_file),
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "phis_generated": len(phis),
        "phis_verified": len(verified_phis),
        "true_positives": len(true_positives),
        "false_positives": len(verdicts) - len(true_positives),
        "status": "COMPLETE",
        "llm_summary": llm_summary,
    }
    write_summary_md(task_dir, result, phis, verdicts, llm_summary)
    print(f"  [done] {task_name} — {len(true_positives)} TP / {len(verified_phis)} verified")


if __name__ == "__main__":
    broken = []
    for d in sorted(WORKSPACE.iterdir()):
        if d.is_dir() and (d / "critic_raw.txt").exists():
            if "Argument list too long" in (d / "critic_raw.txt").read_text():
                broken.append(d)

    print(f"Re-running critic for {len(broken)} tasks")
    for i, d in enumerate(broken):
        print(f"\n[{i+1}/{len(broken)}]")
        try:
            rerun_critic(d)
        except Exception as e:
            print(f"  [error] {d.name}: {e}")
