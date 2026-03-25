#!/usr/bin/env python3
"""Step 4: Run critic on all workspace_v3 tasks with verified phis."""
import json
import os
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from run_full_pipeline import (
    call_llm, parse_verdicts, parse_summary, extract_spec_portion,
    write_summary_md, CRITIC_PROMPT
)

WORKSPACE = Path(os.path.expanduser("~/intent_formalization/verusage/workspace_v3"))
LOG = WORKSPACE / "step4_critic.log"

def run_critic(task_dir: Path):
    source_file = task_dir / "original.rs"
    ent_file = task_dir / "entailment_results.json"
    if not source_file.exists() or not ent_file.exists():
        return None

    phis = json.loads(ent_file.read_text())
    verified_phis = [p for p in phis if p.get("entailed")]
    if not verified_phis:
        return None

    task_name = task_dir.name
    source_text = source_file.read_text()
    spec_text = extract_spec_portion(source_text)

    phi_descriptions = ""
    for phi in verified_phis:
        phi_descriptions += f"\n### φ: {phi['name']}\n"
        phi_descriptions += f"Type: {phi['type']}\n"
        # Read code from phi file
        phi_file = list(task_dir.glob(f"phi_*_{phi['name']}.rs"))
        if phi_file:
            phi_code = phi_file[0].read_text()
        else:
            phi_code = "(code file not found)"
        phi_descriptions += f"```verus\n{phi_code}\n```\n"
        phi_descriptions += f"Reason flagged: {phi['reason']}\n"

    critic_response = call_llm(
        CRITIC_PROMPT,
        f"Source file:\n```rust\n{spec_text}\n```\n\nVerified candidate properties (spec entails all of these):\n{phi_descriptions}"
    )
    (task_dir / "critic_raw.txt").write_text(critic_response)

    verdicts = parse_verdicts(critic_response)
    llm_summary = parse_summary(critic_response)
    (task_dir / "verdicts.json").write_text(json.dumps(verdicts, indent=2))

    true_positives = [v for v in verdicts if v.get("verdict") == "TRUE_POSITIVE"]

    result = {
        "source": str(task_dir / "original.rs"),
        "timestamp": time.strftime("%Y-%m-%d %H:%M:%S"),
        "status": "COMPLETE",
        "phis_generated": len(phis),
        "phis_verified": len(verified_phis),
        "true_positives": len(true_positives),
        "false_positives": len(verdicts) - len(true_positives),
        "llm_summary": llm_summary,
    }
    write_summary_md(task_dir, result, phis, verdicts, llm_summary)
    return task_name, len(verified_phis), len(true_positives)


def main():
    tasks = []
    for d in sorted(WORKSPACE.iterdir()):
        if not d.is_dir():
            continue
        ent = d / "entailment_results.json"
        if not ent.exists():
            continue
        phis = json.loads(ent.read_text())
        if any(p.get("entailed") for p in phis):
            if not (d / "verdicts.json").exists():
                tasks.append(d)

    print(f"Step 4: Critic for {len(tasks)} tasks with verified phis\n")
    results = []
    for i, task_dir in enumerate(tasks):
        print(f"[{i+1}/{len(tasks)}] {task_dir.name}")
        try:
            r = run_critic(task_dir)
            if r:
                results.append(r)
                print(f"  -> {r[1]} verified, {r[2]} TP")
            else:
                print(f"  -> skipped")
        except Exception as e:
            print(f"  -> ERROR: {e}")
        time.sleep(1)  # rate limit

    print(f"\n=== Done: {len(results)} tasks critiqued ===")
    total_verified = sum(r[1] for r in results)
    total_tp = sum(r[2] for r in results)
    print(f"Total verified φ: {total_verified}, True positives: {total_tp}")

    # Write summary
    summary = {"tasks_critiqued": len(results), "total_verified": total_verified, "total_tp": total_tp, "details": []}
    for name, ver, tp in results:
        summary["details"].append({"task": name, "verified": ver, "true_positives": tp})
    (WORKSPACE / "step4_summary.json").write_text(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
