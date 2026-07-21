#!/usr/bin/env python3
"""LLM generation + determinism-feedback loop for vstd no-post functions."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import time
from collections import Counter
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

from run_determinism import (
    module_file,
    normalize_verus_aliases,
    run_target,
)

from spec_determinism.extract.extractor import extract_spec
from spec_determinism.extract.types import TypeKind
from spec_determinism.view.registry import ViewRegistry


GENERATION_PROMPT = """\
You are proposing a Verus postcondition for one existing vstd exec function.

Target:
- id: {target}
- module/context: {context}
- visibility: {visibility}
- current contract status: {contract_status}

Relevant source:
```rust
{source_context}
```

Return JSON only:
{{
  "decision": "add_spec" | "skip",
  "ensures": ["Verus boolean expression", "..."],
  "useful": true | false,
  "rationale": "short explanation",
  "risks": ["..."]
}}

Rules:
- Do not edit files.
- Do not change the signature or requires clauses.
- Propose the strongest sound postcondition expressible with existing public
  vstd spec vocabulary.
- Do not use `true`, `false`, `arbitrary()`, `assume`, or a postcondition that
  merely repeats a requires clause.
- Account for trait-inherited contracts, tracked/linear resource consumption,
  compiler-erased functions, divergence, hidden interior state, and intentional
  nondeterminism.
- If no useful non-vacuous postcondition exists, choose `skip` and leave
  `ensures` empty.
"""


FEEDBACK_PROMPT = """\
Revise a proposed Verus postcondition after determinism-checking feedback.

Target: {target}

Previous proposal:
```json
{candidate}
```

Checker result:
```json
{checker}
```

Anti-vacuity/semantic issues:
```json
{issues}
```

Relevant source:
```rust
{source_context}
```

Return JSON only with the same schema:
{{
  "decision": "add_spec" | "skip",
  "ensures": ["..."],
  "useful": true | false,
  "rationale": "...",
  "risks": ["..."]
}}

Do not optimize for `R0 = unsat` by returning a redundant, vacuous, false-domain,
unit-output, or semantically unusable specification. Choose `skip` if no useful
postcondition can be expressed.
"""


def call_copilot(
    *,
    prompt: str,
    model: str,
    copilot_bin: str,
    timeout: int,
    cwd: Path,
) -> str:
    cmd = [
        copilot_bin,
        "--model",
        model,
        "-s",
        "--no-auto-update",
        "--allow-all-tools",
        "--allow-all-paths",
        "-p",
        prompt,
    ]
    proc = subprocess.run(
        cmd,
        cwd=cwd,
        capture_output=True,
        text=True,
        timeout=timeout,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip() or f"copilot exited {proc.returncode}")
    return proc.stdout


def parse_json_response(text: str) -> dict:
    blocks = re.findall(r"```(?:json)?\s*(.*?)```", text, flags=re.DOTALL)
    candidates = list(reversed(blocks)) + [text]
    decoder = json.JSONDecoder()
    for candidate in candidates:
        candidate = candidate.strip()
        try:
            value = json.loads(candidate)
            if isinstance(value, dict):
                return value
        except json.JSONDecodeError:
            pass
        for match in re.finditer(r"\{", candidate):
            try:
                value, _ = decoder.raw_decode(candidate[match.start() :])
                if isinstance(value, dict):
                    return value
            except json.JSONDecodeError:
                continue
    raise ValueError("Copilot response did not contain valid JSON")


def source_context(vstd_root: Path, entry: dict) -> str:
    path = module_file(vstd_root, entry["module"])
    lines = path.read_text(errors="replace").splitlines()
    line = entry["source_line"]
    start = max(0, line - 60)
    end = min(len(lines), line + 90)
    numbered = [
        f"{idx + 1:4d}: {lines[idx]}"
        for idx in range(start, end)
    ]
    if entry["module"].startswith("contrib::exec_spec::"):
        trait_path = vstd_root / "contrib" / "exec_spec" / "mod.rs"
        trait_lines = trait_path.read_text(errors="replace").splitlines()[:100]
        numbered.extend(
            ["", "// Shared trait declarations:"]
            + [f"T{idx + 1:03d}: {text}" for idx, text in enumerate(trait_lines)]
        )
    return "\n".join(numbered)


def normalize_expr(expr: str) -> str:
    return re.sub(r"\s+", "", expr).strip("()")


def observable_output(vstd_root: Path, entry: dict) -> bool:
    path = module_file(vstd_root, entry["module"])
    source = normalize_verus_aliases(path.read_text(errors="replace"))
    spec = extract_spec(
        source,
        entry["function"],
        type_sources=[source],
        source_line=entry["source_line"],
    )
    for _, output_type in spec.output_vars():
        if output_type.kind != TypeKind.UNIT and output_type.name != "!":
            return True
    return False


def anti_vacuity_issues(
    *,
    entry: dict,
    candidate: dict,
    checker: dict | None,
    has_observable_output: bool,
) -> list[str]:
    issues = []
    ensures = [
        str(expr).strip()
        for expr in candidate.get("ensures") or []
        if str(expr).strip()
    ]
    normalized = {normalize_expr(expr) for expr in ensures}
    requires = {normalize_expr(expr) for expr in checker.get("requires", [])} if checker else set()
    if not ensures:
        issues.append("no_candidate_postcondition")
    if normalized & {"true", "false"}:
        issues.append("constant_postcondition")
    if normalized and normalized <= requires:
        issues.append("postcondition_implied_by_requires")
    if not has_observable_output:
        issues.append("no_modeled_observable_output")
    disposition = entry["disposition"]
    if disposition == "already_specified_via_trait":
        issues.append("already_specified_via_trait")
    if disposition in {
        "compiler_internal",
        "runtime_effect_unmodeled",
        "diverges",
        "intentional_nondeterminism",
        "intentional_opaque_token",
        "linear_resource_effect",
        "linear_resource_hidden_state",
    }:
        issues.append(f"semantic_disposition:{disposition}")
    if entry["target"].startswith("thread:clone@"):
        issues.append("tracked_value_exec_clone_is_uncallable")
    if checker:
        if checker.get("status") != "ok":
            issues.append(f"checker_status:{checker.get('status')}")
        elif checker.get("r0_z3") != "unsat":
            issues.append(f"determinism_not_proved:{checker.get('r0_z3')}")
        if checker.get("equal_fn_trivial"):
            issues.append("trivial_equal_fn")
    return sorted(set(issues))


def checker_summary(result: dict | None) -> dict:
    if result is None:
        return {"status": "not_run"}
    keys = (
        "status",
        "r0_z3",
        "classification",
        "requires",
        "ensures",
        "equal_fn_trivial",
        "stderr_tail",
        "error",
    )
    return {key: result[key] for key in keys if key in result}


def run_one(
    *,
    entry: dict,
    vstd_root: Path,
    verus_root: Path,
    out_root: Path,
    view_registry: ViewRegistry,
    model: str,
    copilot_bin: str,
    llm_timeout: int,
    det_timeout: int,
    rlimit: float,
    feedback_rounds: int,
) -> dict:
    target_dir = out_root / "targets" / entry["target"].replace("::", "__").replace(":", "__").replace("@", "__L")
    target_dir.mkdir(parents=True, exist_ok=True)
    context = source_context(vstd_root, entry)
    (target_dir / "source_context.rs.txt").write_text(context + "\n")

    history = []
    prompt = GENERATION_PROMPT.format(
        target=entry["target"],
        context=entry["context"],
        visibility=entry["visibility"],
        contract_status=entry["original_contract_status"],
        source_context=context,
    )

    for round_index in range(feedback_rounds + 1):
        round_dir = target_dir / f"round_{round_index:02d}"
        round_dir.mkdir()
        (round_dir / "prompt.md").write_text(prompt)
        started = time.monotonic()
        try:
            response = call_copilot(
                prompt=prompt,
                model=model,
                copilot_bin=copilot_bin,
                timeout=llm_timeout,
                cwd=round_dir,
            )
            llm_ms = int((time.monotonic() - started) * 1000)
            (round_dir / "response.txt").write_text(response)
            candidate = parse_json_response(response)
            (round_dir / "candidate.json").write_text(
                json.dumps(candidate, indent=2) + "\n"
            )
        except Exception as exc:
            record = {
                "round": round_index,
                "status": "llm_error",
                "error": f"{type(exc).__name__}: {exc}",
            }
            history.append(record)
            break

        ensures = [
            str(expr).strip()
            for expr in candidate.get("ensures") or []
            if str(expr).strip()
        ]
        checker = None
        if candidate.get("decision") == "add_spec" and ensures:
            checker = run_target(
                module=entry["module"],
                function=entry["function"],
                source_line=entry["source_line"],
                vstd_root=vstd_root,
                verus_root=verus_root,
                out_dir=round_dir,
                timeout=det_timeout,
                rlimit=rlimit,
                compare_raw_pointers=False,
                view_registry=view_registry,
                ensures_override=ensures,
            )

        has_output = observable_output(vstd_root, entry)
        issues = anti_vacuity_issues(
            entry=entry,
            candidate=candidate,
            checker=checker,
            has_observable_output=has_output,
        )
        raw_reward = int(
            checker is not None
            and checker.get("status") == "ok"
            and checker.get("r0_z3") == "unsat"
        )
        guarded_reward = int(raw_reward == 1 and not issues)
        record = {
            "round": round_index,
            "llm_ms": llm_ms,
            "candidate": candidate,
            "checker": checker_summary(checker),
            "anti_vacuity_issues": issues,
            "raw_det_reward": raw_reward,
            "guarded_reward": guarded_reward,
        }
        history.append(record)
        (round_dir / "round_result.json").write_text(
            json.dumps(record, indent=2) + "\n"
        )

        if guarded_reward == 1 or round_index >= feedback_rounds:
            break
        prompt = FEEDBACK_PROMPT.format(
            target=entry["target"],
            candidate=json.dumps(candidate, indent=2),
            checker=json.dumps(checker_summary(checker), indent=2),
            issues=json.dumps(issues, indent=2),
            source_context=context,
        )

    final = history[-1] if history else {"status": "no_round"}
    result = {
        "target": entry["target"],
        "semantic_disposition": entry["disposition"],
        "history": history,
        "final": final,
    }
    (target_dir / "summary.json").write_text(json.dumps(result, indent=2) + "\n")
    return result


def write_batch_summary(out_dir: Path, results: list[dict], metadata: dict) -> None:
    final_candidates = []
    for result in results:
        final = result.get("final") or {}
        candidate = final.get("candidate") or {}
        final_candidates.append(
            {
                "target": result["target"],
                "semantic_disposition": result["semantic_disposition"],
                "decision": candidate.get("decision", ""),
                "useful_claim": candidate.get("useful"),
                "ensures": candidate.get("ensures") or [],
                "raw_det_reward": final.get("raw_det_reward", 0),
                "guarded_reward": final.get("guarded_reward", 0),
                "anti_vacuity_issues": final.get("anti_vacuity_issues") or [],
                "checker": final.get("checker") or {},
            }
        )
    counts = {
        "targets": len(results),
        "llm_add_spec": sum(item["decision"] == "add_spec" for item in final_candidates),
        "llm_skip": sum(item["decision"] == "skip" for item in final_candidates),
        "raw_reward": sum(item["raw_det_reward"] for item in final_candidates),
        "guarded_reward": sum(item["guarded_reward"] for item in final_candidates),
        "llm_errors": sum((result.get("final") or {}).get("status") == "llm_error" for result in results),
    }
    payload = {
        "metadata": metadata,
        "counts": counts,
        "results": results,
        "final_candidates": final_candidates,
    }
    (out_dir / "batch_summary.json").write_text(json.dumps(payload, indent=2) + "\n")

    lines = [
        "# LLM vstd missing-spec feedback experiment",
        "",
        f"- Model: `{metadata['model']}`",
        f"- Targets: {counts['targets']}",
        f"- Final add_spec decisions: {counts['llm_add_spec']}",
        f"- Final skip decisions: {counts['llm_skip']}",
        f"- Raw determinism reward: {counts['raw_reward']}",
        f"- Guarded reward: {counts['guarded_reward']}",
        f"- LLM errors: {counts['llm_errors']}",
        "",
        "| Target | Decision | Ensures | Raw | Guarded | Issues |",
        "|---|---|---|---:|---:|---|",
    ]
    for item in sorted(final_candidates, key=lambda value: value["target"]):
        ensures = "; ".join(item["ensures"])
        issues = ", ".join(item["anti_vacuity_issues"])
        lines.append(
            f"| `{item['target']}` | {item['decision']} | `{ensures}` | "
            f"{item['raw_det_reward']} | {item['guarded_reward']} | {issues} |"
        )
    (out_dir / "SUMMARY.md").write_text("\n".join(lines).rstrip() + "\n")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--vstd-root", type=Path, required=True)
    parser.add_argument("--verus-root", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--model", default="gpt-5-mini")
    parser.add_argument("--copilot-bin", default="copilot")
    parser.add_argument("--jobs", type=int, default=4)
    parser.add_argument("--limit", type=int)
    parser.add_argument(
        "--effective-only",
        action="store_true",
        help="exclude definitions whose postcondition is inherited from a trait",
    )
    parser.add_argument("--feedback-rounds", type=int, default=1)
    parser.add_argument("--llm-timeout", type=int, default=600)
    parser.add_argument("--det-timeout", type=int, default=240)
    parser.add_argument("--rlimit", type=float, default=60)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    manifest = json.loads(args.manifest.read_text())
    if args.effective_only:
        manifest = [
            entry
            for entry in manifest
            if entry["disposition"] != "already_specified_via_trait"
        ]
    if args.limit is not None:
        manifest = manifest[: args.limit]
    args.out.mkdir(parents=True, exist_ok=True)

    view_registry = ViewRegistry.from_project(args.vstd_root)
    metadata = {
        "model": args.model,
        "vstd_root": str(args.vstd_root.resolve()),
        "verus_root": str(args.verus_root.resolve()),
        "feedback_rounds": args.feedback_rounds,
    }

    results = []
    with ThreadPoolExecutor(max_workers=args.jobs) as pool:
        futures = {
            pool.submit(
                run_one,
                entry=entry,
                vstd_root=args.vstd_root,
                verus_root=args.verus_root,
                out_root=args.out,
                view_registry=view_registry,
                model=args.model,
                copilot_bin=args.copilot_bin,
                llm_timeout=args.llm_timeout,
                det_timeout=args.det_timeout,
                rlimit=args.rlimit,
                feedback_rounds=args.feedback_rounds,
            ): entry
            for entry in manifest
        }
        done = 0
        for future in as_completed(futures):
            done += 1
            entry = futures[future]
            try:
                result = future.result()
            except Exception as exc:
                result = {
                    "target": entry["target"],
                    "semantic_disposition": entry["disposition"],
                    "history": [],
                    "final": {
                        "status": "exception",
                        "error": f"{type(exc).__name__}: {exc}",
                    },
                }
            results.append(result)
            final = result.get("final") or {}
            print(
                f"[{done}/{len(manifest)}] {entry['target']} "
                f"decision={(final.get('candidate') or {}).get('decision')} "
                f"raw={final.get('raw_det_reward', 0)} "
                f"guarded={final.get('guarded_reward', 0)}",
                flush=True,
            )
            write_batch_summary(args.out, results, metadata)

    write_batch_summary(args.out, results, metadata)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
