#!/home/chentianyu/miniconda3/bin/python3
"""
Spec Consistency Pipeline v2 — Full batch runner.

For each verified VeruSage file:
1. Copy source to workspace_v2/<task_name>/
2. Call LLM generator to produce candidate φ
3. Build entailment test files, run Verus on each
4. Call LLM critic on verified φ
5. Write summary.md

Usage:
  python3 run_full_pipeline.py [--concurrency N] [--limit N] [--offset N] [--file PATH]
"""

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

BASE = Path.home() / "intent_formalization"
VERUS = BASE / "verus" / "verus"
VERUSAGE = BASE / "verusage" / "source-projects"
WORKSPACE = BASE / "verusage" / "workspace_v3"
VERUS_SO = BASE / "verus.so"

# Initialize Verus parser for extracting exec functions
sys.path.insert(0, str(BASE / "src" / "utils"))
_vp = None
def get_verus_parser():
    global _vp
    if _vp is None:
        from verus_parser import verus_parser
        _vp = verus_parser(str(VERUS_SO))
    return _vp

def extract_exec_fn_info(source_text: str) -> list:
    """Extract exec function names and their code using verus_parser."""
    vp = get_verus_parser()
    tree = vp.parser.parse(bytes(source_text, 'utf-8')).root_node
    exec_fns = vp.extract_exec_functions(tree, skip_external=True)
    results = []
    for decl in exec_fns:
        for child in decl.children:
            if child.type == 'function_item':
                name_node = child.child_by_field_name('name')
                name = name_node.text.decode() if name_node else 'unknown'
                if name == 'main':
                    continue
                results.append({
                    'name': name,
                    'code': decl.text.decode(),
                })
    return results

GENERATOR_PROMPT = """You are a spec consistency query generator for Verus (Rust verification).

You are given:
1. A Verus source file (possibly summarized)
2. A list of EXECUTABLE functions extracted from the file (these are the functions with actual implementations, not spec or proof functions)

Your job: generate candidate "undesirable properties" ONLY for the provided executable functions' specifications (requires/ensures clauses). Do NOT generate tests for proof functions or spec functions.

For EACH candidate, output in this EXACT format (parseable):

===PHI_START===
NAME: <short_snake_case_name>
TARGET_FN: <name of the exec function being tested>
TYPE: behavioral | boundary | logical
SOURCE: spec_only
CODE:
```verus
proof fn phi_<n>_<snake_name>(<params>)
    requires
        <preconditions from the spec>,
    ensures
        <the undesirable property>,
{
    // proof body (can be empty if you expect Verus to figure it out)
}
```
REASON: <one line why this would be undesirable if entailed>
===PHI_END===

IMPORTANT:
- Generate EXACTLY 5 candidates, all targeting the executable functions listed
- Each proof fn must be SELF-CONTAINED — it will be appended after the original file's definitions
- Use types/functions/traits defined in the source file
- Do NOT add new `use` statements or `mod` declarations
- Do NOT wrap in verus!{} — the proof fn will be inserted inside an existing verus!{} block
- Keep proof bodies SHORT — rely on Verus's SMT solver
- If there are multiple exec functions, spread your tests across them
"""

BODY_AWARE_PROMPT = """You are a spec incompleteness detector for Verus (Rust verification).

You are given:
1. A Verus source file with type/spec definitions
2. EXECUTABLE functions with their full body AND their specification (requires/ensures)

Your job: find properties that the function body guarantees but the specification DOES NOT express.
These are spec *incompleteness* gaps — the function does something useful that callers can't rely on
because the spec doesn't promise it.

Strategy:
- Read the function body carefully. What does it actually compute/guarantee?
- Compare with the ensures clause. What's missing?
- Focus on behavioral intent: if a human reads the body, what would they expect the spec to say?
- Look for comments like "not strictly needed", TODO, FIXME near specs — these are developer-acknowledged gaps

Generate candidates in three dimensions:
- **Behavioral**: Body guarantees a semantic property (e.g., monotonicity, completeness, forward progress) not in ensures
- **Boundary**: Body handles edge cases (empty input, zero, overflow) but spec doesn't distinguish them
- **Logical**: Body maintains relationships between outputs (e.g., result.start + result.count <= bound) not stated

For EACH candidate, output in this EXACT format:

===PHI_START===
NAME: <short_snake_case_name>
TARGET_FN: <name of the exec function being tested>
TYPE: behavioral | boundary | logical
SOURCE: body_aware
CODE:
```verus
proof fn phi_<n>_<snake_name>(<params>)
    requires
        <preconditions from the spec>,
    ensures
        <the property that body guarantees but spec doesn't>,
{
}
```
REASON: <what the body does that the spec doesn't capture>
===PHI_END===

IMPORTANT:
- Generate EXACTLY 5 candidates
- Each proof fn must be SELF-CONTAINED
- Use types/functions/traits defined in the source file
- Do NOT add new `use` statements or `mod` declarations
- Do NOT wrap in verus!{} — will be inserted inside existing verus!{} block
- Focus on what's MISSING from the spec, not what's wrong with the body
"""

CRITIC_PROMPT = """You are a spec consistency critic for Verus.

You receive a Verus source file and a list of candidate properties (φ) that the spec has been
VERIFIED to entail (Verus confirmed the spec implies these). Each φ targets an EXECUTABLE function's
specification (requires/ensures). For each φ, decide:
- TRUE_POSITIVE: real spec issue on an exec function (too weak, too strong, missing axiom, unverified assumption, soundness gap)
- FALSE_POSITIVE: actually a desirable/expected property, not a spec gap, OR targets a proof/spec function instead of an exec function

BEFORE classifying each φ, apply these filters in order:

1. TAUTOLOGY CHECK: Does this φ hold purely from logic, types, or platform facts, independent of
   the function's spec? If removing all requires/ensures from the function would still make φ
   provable, it is a tautology — mark as FALSE_POSITIVE. Examples:
   - Pure logic: A == B ∧ B.contains(x) → A.contains(x)
   - Platform fact: usize <= u64::MAX on 64-bit
   - Type-level: a nat is always >= 0

2. GHOST/EXEC CHECK: Does the φ target a ghost struct, spec fn, or proof fn? In Verus, ghost code
   is proof-only and never executes. A ghost invariant that admits more values than strictly needed
   is a MORE GENERAL (stronger) proof, not a weaker spec. Only flag spec issues that affect
   executable function behavior. Examples of FALSE_POSITIVE:
   - Ghost struct Arch::inv() allows degenerate values — proof holds for wider domain
   - Spec fn admits inputs that can't occur at runtime

3. GENERALITY CHECK: Does the φ show the spec is "too permissive" by admitting more inputs/outputs?
   A spec that proves a property for ∀n≥0 is strictly stronger than one for ∀n≥4. More general
   specs are a feature, not a weakness — unless the generality causes the spec to FAIL TO EXPRESS
   intended behavior (e.g., missing completeness/liveness guarantees for exec functions).

Only after passing all three filters should you consider a φ as TRUE_POSITIVE.

Output EXACTLY this format for each:

===VERDICT_START===
PHI: <name>
VERDICT: TRUE_POSITIVE | FALSE_POSITIVE
CONFIDENCE: high | medium | low
REASONING: <2-3 sentences, mention which filter(s) you applied>
===VERDICT_END===

Then at the end output:
===SUMMARY===
<one paragraph summarizing the findings>
===END_SUMMARY===
"""


def task_name_for(source_path: Path) -> str:
    rel = source_path.relative_to(VERUSAGE)
    project = rel.parts[0]
    filename = source_path.stem
    return f"{project}__{filename}"


def run_verus(filepath: Path, timeout: int = 120) -> dict:
    try:
        result = subprocess.run(
            [str(VERUS), str(filepath)],
            capture_output=True, text=True, timeout=timeout
        )
        output = result.stdout + result.stderr
        success = "0 errors" in output and "verified" in output
        return {"success": success, "output": output, "timed_out": False}
    except subprocess.TimeoutExpired:
        return {"success": False, "output": "TIMEOUT", "timed_out": True}
    except Exception as e:
        return {"success": False, "output": str(e), "timed_out": False}


def call_llm(system_prompt: str, user_prompt: str, session_id: str = None) -> str:
    """Call LLM via openclaw agent CLI using stdin pipe for large prompts."""
    import tempfile
    import uuid

    full_msg = f"[SYSTEM INSTRUCTIONS]\n{system_prompt}\n\n[USER REQUEST]\n{user_prompt}"

    # Write to temp file, pipe via stdin to avoid ARG_MAX
    with tempfile.NamedTemporaryFile(mode='w', suffix='.txt', delete=False) as f:
        f.write(full_msg)
        tmp_path = f.name

    # Use unique session ID to avoid locking main session
    sid = f"pipeline-{uuid.uuid4().hex[:12]}"

    try:
        cmd = ["openclaw", "agent", "--local", "--json", "--session-id", sid, "-m", "@" + tmp_path]

        # If @ syntax not supported, use a small wrapper
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=300)

        # If that fails, try reading file content but truncated
        if result.returncode != 0 and "unknown option" not in result.stderr:
            # Direct approach: pass message via python subprocess with the file
            msg = Path(tmp_path).read_text()
            # Truncate if too large (ARG_MAX is ~2MB on Linux, be safe at 128K)
            if len(msg.encode()) > 128000:
                msg = msg[:128000] + "\n\n[TRUNCATED — input too large]"
            result = subprocess.run(
                ["openclaw", "agent", "--local", "--json", "--session-id", sid, "-m", msg],
                capture_output=True, text=True, timeout=300
            )

        if result.returncode == 0:
            try:
                data = json.loads(result.stdout)
                if "payloads" in data:
                    texts = [p.get("text", "") for p in data["payloads"] if p.get("text")]
                    return "\n".join(texts)
                return data.get("reply", data.get("text", data.get("content", result.stdout)))
            except json.JSONDecodeError:
                return result.stdout
        else:
            return f"ERROR: {result.stderr}"
    except subprocess.TimeoutExpired:
        return "ERROR: LLM call timed out"
    except Exception as e:
        return f"ERROR: {e}"
    finally:
        os.unlink(tmp_path)


def extract_spec_portion(source_text: str, max_lines: int = 300) -> str:
    """For large files, extract the most spec-relevant portion."""
    lines = source_text.split('\n')
    if len(lines) <= max_lines:
        return source_text

    # Collect lines with spec keywords + surrounding context
    spec_keywords = ['requires', 'ensures', 'invariant', 'recommends', 'spec fn',
                     'proof fn', 'decreases', 'assert', 'open spec', 'closed spec',
                     'pub proof', 'pub open spec', 'pub closed spec']
    important_lines = set()
    for i, line in enumerate(lines):
        line_lower = line.lower().strip()
        if any(kw in line_lower for kw in spec_keywords):
            # Add context: 5 lines before, 10 lines after
            for j in range(max(0, i-5), min(len(lines), i+11)):
                important_lines.add(j)

    # Also include first 20 lines (imports/declarations) and last 5
    for i in range(min(20, len(lines))):
        important_lines.add(i)
    for i in range(max(0, len(lines)-5), len(lines)):
        important_lines.add(i)

    selected = sorted(important_lines)
    if len(selected) > max_lines:
        selected = selected[:max_lines]

    result_lines = []
    prev = -2
    for i in selected:
        if i > prev + 1:
            result_lines.append(f"// ... (lines {prev+2}-{i-1} omitted) ...")
        result_lines.append(lines[i])
        prev = i

    return '\n'.join(result_lines)


def parse_phi_blocks(text: str) -> list:
    blocks = []
    pattern = r'===PHI_START===(.*?)===PHI_END==='
    for match in re.finditer(pattern, text, re.DOTALL):
        block = match.group(1).strip()
        name_m = re.search(r'NAME:\s*(.+)', block)
        type_m = re.search(r'TYPE:\s*(.+)', block)
        source_m = re.search(r'SOURCE:\s*(.+)', block)
        code_m = re.search(r'```(?:verus|rust)?\s*\n(.*?)```', block, re.DOTALL)
        reason_m = re.search(r'REASON:\s*(.+)', block)

        if name_m and code_m:
            blocks.append({
                "name": name_m.group(1).strip(),
                "type": type_m.group(1).strip() if type_m else "unknown",
                "source": source_m.group(1).strip() if source_m else "spec_only",
                "code": code_m.group(1).strip(),
                "reason": reason_m.group(1).strip() if reason_m else "",
            })
    return blocks


def parse_verdicts(text: str) -> list:
    verdicts = []
    pattern = r'===VERDICT_START===(.*?)===VERDICT_END==='
    for match in re.finditer(pattern, text, re.DOTALL):
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


def build_entailment_file(source_text: str, phi_code: str) -> str:
    """Insert phi into the source file before the final closing brace of verus!{}."""
    last_brace = source_text.rstrip().rfind('}')
    if last_brace == -1:
        raise ValueError("Cannot find closing brace")
    return source_text[:last_brace] + "\n\n// === Entailment query ===\n" + phi_code + "\n\n}\n"


def strip_spec(source_text: str) -> str:
    """Strip requires/ensures clauses from exec functions to test if φ is a tautology.

    Replaces requires/ensures blocks on exec functions with 'requires true, ensures true,'.
    This is conservative: if φ still verifies without the spec, it's spec-independent.
    """
    import re as _re
    lines = source_text.split('\n')
    result = []
    i = 0
    in_spec_block = False
    brace_depth = 0

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        # Detect start of requires/ensures on exec functions
        # We replace any requires/ensures clause content with 'true'
        if not in_spec_block and stripped in ('requires', 'ensures'):
            # Check if next non-empty lines are spec clauses (indented conditions)
            # Replace the clause block with 'true,'
            result.append(line)  # keep the keyword
            i += 1
            # Skip all indented condition lines until we hit another keyword or fn body
            added_true = False
            while i < len(lines):
                next_stripped = lines[i].strip()
                # Stop at: next keyword, opening brace, empty line followed by non-spec
                if next_stripped in ('requires', 'ensures', 'recommends', 'decreases', '{', '}', ''):
                    break
                if next_stripped.startswith('//'):
                    result.append(lines[i])  # keep comments
                    i += 1
                    continue
                if not added_true:
                    # Replace first condition line with 'true,'
                    indent = len(lines[i]) - len(lines[i].lstrip())
                    result.append(' ' * indent + 'true,')
                    added_true = True
                # Skip remaining condition lines
                i += 1
            continue

        result.append(line)
        i += 1

    return '\n'.join(result)


def process_one_file(source_path: Path) -> dict:
    """Run the full pipeline for one file."""
    task_name = task_name_for(source_path)
    task_dir = WORKSPACE / task_name
    task_dir.mkdir(parents=True, exist_ok=True)

    result = {
        "task": task_name,
        "source": str(source_path),
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
    }

    # Step 0: Copy source
    source_text = source_path.read_text()
    shutil.copy2(source_path, task_dir / "original.rs")

    # Step 0.5: Extract exec functions using verus_parser
    try:
        exec_fns = extract_exec_fn_info(source_text)
    except Exception as e:
        exec_fns = []
        print(f"  [warn] Parser failed for {task_name}: {e}")

    if not exec_fns:
        result["status"] = "NO_EXEC_FUNCTIONS"
        write_summary_md(task_dir, result, [], [])
        print(f"  [skip] {task_name} — no exec functions found")
        return result

    # Save exec function info
    (task_dir / "exec_functions.json").write_text(json.dumps(
        [{"name": f["name"], "code_len": len(f["code"])} for f in exec_fns], indent=2))

    # Step 1a: Spec-only generator
    print(f"  [gen-1a] {task_name} ({len(exec_fns)} exec fns) — spec-only")
    spec_text = extract_spec_portion(source_text)
    exec_fn_section = "\n\n## Executable Functions to Test:\n\n"
    for fn in exec_fns:
        exec_fn_section += f"### `{fn['name']}`\n```verus\n{fn['code']}\n```\n\n"

    gen_response = call_llm(
        GENERATOR_PROMPT,
        f"Source file (spec-relevant portions):\n\n```rust\n{spec_text}\n```\n\n{exec_fn_section}\n\nGenerate 5 candidate undesirable properties targeting ONLY the exec functions listed above."
    )
    (task_dir / "generator_1a_raw.txt").write_text(gen_response)
    phis_1a = parse_phi_blocks(gen_response)

    # Step 1b: Body-aware generator
    print(f"  [gen-1b] {task_name} — body-aware")
    body_section = "\n\n## Executable Functions (with body):\n\n"
    for fn in exec_fns:
        body_section += f"### `{fn['name']}`\n```verus\n{fn['code']}\n```\n\n"

    gen_response_body = call_llm(
        BODY_AWARE_PROMPT,
        f"Source file (spec-relevant portions):\n\n```rust\n{spec_text}\n```\n\n{body_section}\n\nGenerate 5 candidate properties that the body guarantees but the spec does NOT express."
    )
    (task_dir / "generator_1b_raw.txt").write_text(gen_response_body)
    phis_1b = parse_phi_blocks(gen_response_body)

    # Merge both sets
    phis = phis_1a + phis_1b
    result["phis_1a"] = len(phis_1a)
    result["phis_1b"] = len(phis_1b)

    if not phis:
        result["status"] = "NO_PHIS_GENERATED"
        result["generator_error"] = gen_response[:500]
        write_summary_md(task_dir, result, [], [])
        print(f"  [skip] {task_name} — no φ generated")
        return result

    result["phis_generated"] = len(phis)
    # Backward compat: also save combined raw
    (task_dir / "generator_raw.txt").write_text(
        f"=== Step 1a (spec-only): {len(phis_1a)} φ ===\n" +
        (task_dir / "generator_1a_raw.txt").read_text() +
        f"\n\n=== Step 1b (body-aware): {len(phis_1b)} φ ===\n" +
        (task_dir / "generator_1b_raw.txt").read_text()
    )

    # Step 2: Entailment check for each phi
    print(f"  [ent] {task_name} — {len(phis)} candidates")
    verified_phis = []
    for i, phi in enumerate(phis):
        try:
            test_code = build_entailment_file(source_text, phi["code"])
            test_file = task_dir / f"phi_{i+1}_{phi['name']}.rs"
            test_file.write_text(test_code)

            check = run_verus(test_file, timeout=120)
            phi["entailed"] = check["success"]
            phi["verus_output"] = check["output"][-500:] if check["output"] else ""
            phi["timed_out"] = check["timed_out"]

            if check["success"]:
                verified_phis.append(phi)
        except Exception as e:
            phi["entailed"] = False
            phi["error"] = str(e)

    result["phis_verified"] = len(verified_phis)
    (task_dir / "entailment_results.json").write_text(json.dumps(phis, indent=2))

    if not verified_phis:
        result["status"] = "NO_PHIS_VERIFIED"
        write_summary_md(task_dir, result, phis, [])
        print(f"  [skip] {task_name} — no φ verified")
        return result

    # Step 3a: Tautology check — remove spec and re-verify
    print(f"  [taut] {task_name} — checking {len(verified_phis)} verified φ")
    non_tautological = []
    for phi in verified_phis:
        try:
            stripped_source = strip_spec(source_text)
            test_code = build_entailment_file(stripped_source, phi["code"])
            test_file = task_dir / f"taut_{phi['name']}.rs"
            test_file.write_text(test_code)
            check = run_verus(test_file, timeout=120)
            phi["tautology"] = check["success"]
            if check["success"]:
                print(f"    [taut] {phi['name']} — TAUTOLOGY (FP)")
            else:
                non_tautological.append(phi)
        except Exception as e:
            # If stripping fails, keep the phi (conservative)
            phi["tautology"] = False
            phi["tautology_error"] = str(e)
            non_tautological.append(phi)

    result["tautologies_filtered"] = len(verified_phis) - len(non_tautological)

    if not non_tautological:
        result["status"] = "ALL_TAUTOLOGICAL"
        result["true_positives"] = 0
        result["false_positives"] = 0
        write_summary_md(task_dir, result, phis, [])
        print(f"  [skip] {task_name} — all φ tautological")
        return result

    # Step 3b: LLM Critic (ghost/exec check + spec relevance + verdict)
    print(f"  [crit] {task_name} — {len(non_tautological)} non-tautological")
    phi_descriptions = ""
    for i, phi in enumerate(non_tautological):
        phi_descriptions += f"\n### φ: {phi['name']}\n"
        phi_descriptions += f"Type: {phi['type']} | Source: {phi.get('source', 'spec_only')}\n"
        phi_descriptions += f"```verus\n{phi['code']}\n```\n"
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
    result["true_positives"] = len(true_positives)
    result["false_positives"] = len(verdicts) - len(true_positives)
    result["status"] = "COMPLETE"
    result["llm_summary"] = llm_summary

    # Step 4: Write summary
    write_summary_md(task_dir, result, phis, verdicts, llm_summary)
    print(f"  [done] {task_name} — {len(true_positives)} TP / {len(verified_phis)} verified / {len(phis)} generated")
    return result


def write_summary_md(task_dir: Path, result: dict, phis: list, verdicts: list, llm_summary: str = ""):
    md = f"# Spec Consistency Report\n\n"
    md += f"**Source:** `{result.get('source', '?')}`\n"
    md += f"**Date:** {result.get('timestamp', '?')}\n"
    md += f"**Status:** {result.get('status', '?')}\n\n"

    gen_count = len(phis)
    ver_count = sum(1 for p in phis if p.get("entailed"))
    tp_count = result.get("true_positives", 0)
    fp_count = result.get("false_positives", 0)

    md += f"## Stats\n\n"
    md += f"- Candidates generated: {gen_count}\n"
    md += f"- Entailed (verified): {ver_count}\n"
    md += f"- True positives: {tp_count}\n"
    md += f"- False positives: {fp_count}\n\n"

    if llm_summary:
        md += f"## Summary\n\n{llm_summary}\n\n"

    tp_verdicts = [v for v in verdicts if v.get("verdict") == "TRUE_POSITIVE"]
    if tp_verdicts:
        md += f"## True Positives (Spec Issues)\n\n"
        for v in tp_verdicts:
            md += f"### {v['phi']}\n"
            md += f"- **Confidence:** {v.get('confidence', '?')}\n"
            md += f"- **Reasoning:** {v.get('reasoning', '?')}\n\n"

    md += f"## All Candidates\n\n"
    for i, p in enumerate(phis):
        md += f"### φ{i+1}: {p.get('name', 'unnamed')}\n"
        md += f"- **Type:** {p.get('type', '?')}\n"
        md += f"- **Entailed:** {'✅' if p.get('entailed') else '❌'}"
        if p.get('timed_out'):
            md += " (timeout)"
        md += "\n"
        if p.get('reason'):
            md += f"- **Why flagged:** {p['reason']}\n"

        for v in verdicts:
            if v.get("phi") == p.get("name"):
                md += f"- **Verdict:** {v['verdict']} ({v.get('confidence', '?')})\n"
                md += f"- **Reasoning:** {v.get('reasoning', '?')}\n"
                break
        md += "\n"

    (task_dir / "summary.md").write_text(md)


def main():
    parser = argparse.ArgumentParser(description="Spec Consistency Pipeline v2 — Full")
    parser.add_argument("--concurrency", type=int, default=1,
                        help="Concurrent file processing (default: 1, sequential)")
    parser.add_argument("--limit", type=int, default=None)
    parser.add_argument("--offset", type=int, default=0)
    parser.add_argument("--file", type=str, default=None)
    args = parser.parse_args()

    if args.file:
        files = [Path(args.file)]
    else:
        valid_list = WORKSPACE / "baseline_valid.json"
        if not valid_list.exists():
            print("Error: baseline_valid.json not found. Run baseline check first.")
            sys.exit(1)
        files = [Path(f) for f in json.loads(valid_list.read_text())]
        files = files[args.offset:]
        if args.limit:
            files = files[:args.limit]

    print(f"Pipeline: {len(files)} files, concurrency={args.concurrency}")
    all_results = []

    if args.concurrency <= 1:
        for i, f in enumerate(files):
            print(f"\n[{i+1}/{len(files)}] {f.name}")
            try:
                r = process_one_file(f)
                all_results.append(r)
            except Exception as e:
                print(f"  [error] {f.name}: {e}")
                all_results.append({"task": task_name_for(f), "status": "ERROR", "error": str(e)})
    else:
        with ThreadPoolExecutor(max_workers=args.concurrency) as pool:
            future_map = {pool.submit(process_one_file, f): f for f in files}
            done = 0
            for fut in as_completed(future_map):
                done += 1
                f = future_map[fut]
                try:
                    r = fut.result()
                    all_results.append(r)
                except Exception as e:
                    print(f"  [error] {f.name}: {e}")
                    all_results.append({"task": task_name_for(f), "status": "ERROR", "error": str(e)})

    # Global summary
    summary_path = WORKSPACE / "pipeline_results.json"
    summary_path.write_text(json.dumps(all_results, indent=2))

    complete = sum(1 for r in all_results if r.get("status") == "COMPLETE")
    total_tp = sum(r.get("true_positives", 0) for r in all_results)
    print(f"\n=== DONE ===")
    print(f"Processed: {len(all_results)}")
    print(f"Complete: {complete}")
    print(f"Total true positives: {total_tp}")
    print(f"Results: {summary_path}")


if __name__ == "__main__":
    main()
