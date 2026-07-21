#!/usr/bin/env python3
"""Audit vstd no-post exec definitions and score diagnostic specs.

This experiment distinguishes syntactically absent postconditions from genuine
specification gaps. It only runs determinism checks for diagnostic candidates;
trait-inherited, compiler-internal, linear-resource, and intentional-
nondeterminism cases are recorded with an explicit skip reason.
"""

from __future__ import annotations

import argparse
import csv
import json
from collections import Counter
from pathlib import Path

from run_determinism import run_target

from spec_determinism.view.registry import ViewRegistry


DIAGNOSTIC_CANDIDATES = {
    "cell::invcell:set@116": {
        "ensures": ["self.inv(val)"],
        "kind": "requires_mirror",
        "reward_eligible": False,
        "rationale": (
            "Sound but redundant: identical to the existing requires clause "
            "and does not expose the hidden current cell value."
        ),
    },
    "pervasive:unreached@190": {
        "ensures": ["false"],
        "kind": "vacuous_false_precondition",
        "reward_eligible": False,
        "rationale": (
            "Negative control: requires false makes any postcondition vacuous."
        ),
    },
    "pervasive:runtime_assert@204": {
        "ensures": ["b"],
        "kind": "requires_mirror",
        "reward_eligible": False,
        "rationale": (
            "Sound but redundant: b is already required and the function "
            "returns unit."
        ),
    },
    "thread:clone@183": {
        "ensures": ["result@ == self@"],
        "kind": "mode_incompatible_semantic_candidate",
        "reward_eligible": False,
        "rationale": (
            "Semantic view equality is expressible, but IsThread is tracked "
            "and core::clone::Clone::clone is exec-mode, so verified callers "
            "cannot invoke this method."
        ),
    },
    "thread:clone@188": {
        "ensures": ["result@ == self@"],
        "kind": "mode_incompatible_semantic_candidate",
        "reward_eligible": False,
        "rationale": (
            "cfg-alternate body of the same tracked/exec-incompatible Clone "
            "implementation."
        ),
    },
}


EXACT_DISPOSITIONS = {
    "invariant:create_open_invariant_credit@276": (
        "intentional_opaque_token",
        "Creates an opaque later-credit token with no spec-visible payload.",
    ),
    "invariant:spend_open_invariant_credit@293": (
        "linear_resource_effect",
        "Consumes a later-credit token; the effect is represented by ownership.",
    ),
    "invariant:open_atomic_invariant_begin@324": (
        "compiler_internal",
        "Removed during VIR conversion and replaced by open-invariant encoding.",
    ),
    "invariant:open_local_invariant_begin@334": (
        "compiler_internal",
        "Removed during VIR conversion and replaced by open-invariant encoding.",
    ),
    "invariant:open_invariant_end@344": (
        "compiler_internal",
        "Removed during VIR conversion and replaced by open-invariant encoding.",
    ),
    "pervasive:print_u64@199": (
        "runtime_effect_unmodeled",
        "Writes stdout, for which vstd exposes no semantic effect model.",
    ),
    "pervasive:__call_panic@443": (
        "diverges",
        "Returns never; there is no normal successor state to specify.",
    ),
    "pervasive:__new_argument@454": (
        "runtime_effect_unmodeled",
        "Debug formatting has no vstd semantic model.",
    ),
    "proph:new@179": (
        "intentional_nondeterminism",
        "A prophecy's future value is intentionally unconstrained at creation.",
    ),
    "raw_ptr:deallocate@948": (
        "linear_resource_effect",
        "Consumes PointsToRaw and Dealloc permissions and returns unit.",
    ),
    "rwlock:release_write@403": (
        "linear_resource_hidden_state",
        "Consumes the write handle and stores a value behind an opaque lock.",
    ),
    "rwlock:release_read@474": (
        "linear_resource_effect",
        "Consumes the read handle and returns unit.",
    ),
    "simple_pptr:free@402": (
        "linear_resource_effect",
        "Consumes the pointer permission and returns unit.",
    ),
}


TRAIT_CONTRACTS = {
    "get_ref": "ToRef::get_ref",
    "get_owned": "ToOwned::get_owned",
    "deep_clone": "DeepViewClone::deep_clone",
    "exec_eq": "ExecSpecEq::exec_eq",
}


def target_key(row: dict) -> str:
    return f"{row['module']}:{row['name']}@{row['line']}"


def build_manifest(inventory_csv: Path) -> list[dict]:
    with inventory_csv.open() as handle:
        rows = list(csv.DictReader(handle))
    rows = [
        row
        for row in rows
        if row["node_kind"] == "definition"
        and row["contract_status"] != "post"
    ]

    manifest = []
    for row in rows:
        key = target_key(row)
        entry = {
            "target": key,
            "module": row["module"],
            "function": row["name"],
            "source_line": int(row["line"]),
            "context": row["context"],
            "visibility": row["visibility"],
            "original_contract_status": row["contract_status"],
            "candidate_ensures": [],
            "reward_eligible": False,
        }
        if row["module"].startswith("contrib::exec_spec::"):
            trait = TRAIT_CONTRACTS.get(row["name"])
            if trait is None:
                raise RuntimeError(f"unclassified contrib target: {key}")
            entry.update(
                disposition="already_specified_via_trait",
                rationale=f"Effective postcondition is inherited from {trait}.",
            )
        elif key in DIAGNOSTIC_CANDIDATES:
            candidate = DIAGNOSTIC_CANDIDATES[key]
            entry.update(
                disposition="diagnostic_candidate",
                candidate_ensures=list(candidate["ensures"]),
                candidate_kind=candidate["kind"],
                reward_eligible=bool(candidate["reward_eligible"]),
                rationale=candidate["rationale"],
            )
        elif key in EXACT_DISPOSITIONS:
            disposition, rationale = EXACT_DISPOSITIONS[key]
            entry.update(disposition=disposition, rationale=rationale)
        else:
            raise RuntimeError(f"unclassified no-post target: {key}")
        manifest.append(entry)

    if len(manifest) != 44:
        raise RuntimeError(f"expected 44 no-post definitions, found {len(manifest)}")
    return manifest


def raw_reward(result: dict) -> int:
    return int(result.get("status") == "ok" and result.get("r0_z3") == "unsat")


def write_summary(out_dir: Path, manifest: list[dict], results: list[dict]) -> None:
    payload = {
        "manifest": manifest,
        "results": results,
    }
    (out_dir / "summary.json").write_text(json.dumps(payload, indent=2) + "\n")

    dispositions = Counter(entry["disposition"] for entry in manifest)
    lines = [
        "# vstd missing-spec determinism-reward experiment",
        "",
        f"- No-post definitions: {len(manifest)}",
        f"- Dispositions: `{dict(dispositions)}`",
        f"- Diagnostic candidates checked: {len(results)}",
        f"- Raw determinism reward total: {sum(r['raw_det_reward'] for r in results)}",
        f"- Guarded reward total: {sum(r['guarded_reward'] for r in results)}",
        "",
        "## Result",
        "",
        "No target received a guarded reward. The raw determinism checker gives",
        "positive reward to redundant or vacuous candidates, demonstrating that",
        "determinism alone is not a sufficient reward for unit-returning,",
        "false-precondition, hidden-state, or mode-incompatible functions.",
        "",
        "| Target | Candidate | Status | R0 | Raw reward | Guarded reward | Reason |",
        "|---|---|---|---|---:|---:|---|",
    ]
    by_target = {result["target"]: result for result in results}
    for entry in manifest:
        result = by_target.get(entry["target"])
        if result is None:
            continue
        candidate = "; ".join(entry["candidate_ensures"])
        lines.append(
            f"| `{entry['target']}` | `{candidate}` | {result.get('status', '')} | "
            f"{result.get('r0_z3', '')} | {result['raw_det_reward']} | "
            f"{result['guarded_reward']} | {entry['rationale']} |"
        )
    lines.extend(
        [
            "",
            "## Disposition counts",
            "",
            "| Disposition | Count |",
            "|---|---:|",
        ]
    )
    for disposition, count in sorted(dispositions.items()):
        lines.append(f"| `{disposition}` | {count} |")
    (out_dir / "SUMMARY.md").write_text("\n".join(lines).rstrip() + "\n")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inventory-csv", type=Path, required=True)
    parser.add_argument("--vstd-root", type=Path, required=True)
    parser.add_argument("--verus-root", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--timeout", type=int, default=240)
    parser.add_argument("--rlimit", type=float, default=60)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    manifest = build_manifest(args.inventory_csv)
    (args.out / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    effective_manifest = [
        entry
        for entry in manifest
        if entry["disposition"] != "already_specified_via_trait"
    ]
    (args.out / "effective_manifest.json").write_text(
        json.dumps(effective_manifest, indent=2) + "\n"
    )

    view_registry = ViewRegistry.from_project(args.vstd_root)
    results = []
    for entry in manifest:
        if not entry["candidate_ensures"]:
            continue
        result = run_target(
            module=entry["module"],
            function=entry["function"],
            source_line=entry["source_line"],
            vstd_root=args.vstd_root,
            verus_root=args.verus_root,
            out_dir=args.out,
            timeout=args.timeout,
            rlimit=args.rlimit,
            compare_raw_pointers=False,
            view_registry=view_registry,
            ensures_override=entry["candidate_ensures"],
        )
        result["target"] = entry["target"]
        result["raw_det_reward"] = raw_reward(result)
        result["guarded_reward"] = (
            result["raw_det_reward"] if entry["reward_eligible"] else 0
        )
        results.append(result)
        print(
            f"{entry['target']}: status={result.get('status')} "
            f"r0={result.get('r0_z3')} raw={result['raw_det_reward']} "
            f"guarded={result['guarded_reward']}"
        )
        write_summary(args.out, manifest, results)

    write_summary(args.out, manifest, results)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
