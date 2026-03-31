#!/home/chentianyu/miniconda3/bin/python3
"""
Full pipeline v3 run on nanvix bitmap with AST-based spec extraction.
Steps: extract declarations → reuse brainstorm_v2 → formalize (new) → entailment → critic
"""
import json, os, sys, re, time
from pathlib import Path

BASE = Path.home() / "intent_formalization"
sys.path.insert(0, str(BASE / "src" / "utils"))
sys.path.insert(0, str(BASE / "src" / "pipeline"))

from verus_parser import verus_parser
from step1_extract import strip_body, extract_fn_name
from step3_formalize import assemble_proof_fn, _rewrite_declaration_to_proof_fn
from pipeline_common import parse_phi_blocks
from llm import LLMClient

VERUS_SO = str(BASE / "verus.so")
WORKSPACE = BASE / "nanvix" / "workspace" / "bitmap"
NANVIX_ROOT = Path.home() / "nanvix"
ORIGINAL_RS = WORKSPACE / "original.rs"
TEST_RS = NANVIX_ROOT / "src" / "libs" / "bitmap" / "src" / "lib.test.rs"
TEST_RS_BAK = TEST_RS.with_suffix(".rs.bak")

# Tags for this run
TAG = "v3"

def log(msg):
    ts = time.strftime("%H:%M:%S")
    print(f"[{ts}] {msg}", flush=True)

# ============================================================
# Step 1: Extract declarations from AST
# ============================================================
def step1_extract_declarations():
    log("=== STEP 1: Extract declarations ===")
    vp = verus_parser(VERUS_SO)
    source = ORIGINAL_RS.read_text()
    tree = vp.parser.parse(bytes(source, 'utf-8')).root_node
    exec_fns = vp.extract_exec_functions(tree, skip_external=True)

    declarations = {}
    exec_info = []
    for decl in exec_fns:
        name = extract_fn_name(decl)
        if name == 'main':
            continue
        decl_text = strip_body(decl)
        declarations[name] = decl_text
        exec_info.append({
            "name": name,
            "code": decl.text.decode(),
            "declaration": decl_text,
        })
        log(f"  {name}: {len(decl_text)} chars declaration")

    # Save exec_functions for step3
    (WORKSPACE / "exec_functions.json").write_text(json.dumps(exec_info, indent=2))
    log(f"  → {len(declarations)} exec functions extracted")
    return declarations

# ============================================================
# Step 2: Reuse existing brainstorm
# ============================================================
def step2_load_brainstorm():
    log("=== STEP 2: Load brainstorm (reusing v2) ===")
    brainstorm = json.loads((WORKSPACE / "brainstorm_v2.json").read_text())
    log(f"  → {len(brainstorm)} properties loaded")
    return brainstorm

# ============================================================
# Step 3: Formalize with AST-based spec extraction
# ============================================================
FORMALIZE_PROMPT = """You are a Verus (Rust verification) code generator.

You are given:
1. A Verus source file with type definitions, spec functions, and executable functions
2. A list of negative properties described in natural language
3. For each target function, its EXACT declaration (signature + requires + ensures with empty body)

Your job: for EACH property, write ONLY the body (assume statements) for a Verus proof fn.

## How it works

The function declaration already has the EXACT spec from the AST. You do NOT copy or modify the spec.
You ONLY provide the body contents (assume statements) and the free variable parameters.

## CRITICAL RULES
1. You do NOT write requires or ensures. They are mechanically extracted from the AST.
2. You ONLY write assume() statements for the body.
3. Use FREE VARIABLES for the exec function's state and return value:
   - `pre` for `old(self)`, `post` for `self`, `result` for the return value
   - For functions without &mut self (like `new`), just use the relevant params + result
   - You may call spec fns and ghost methods (e.g., `pre@.is_full()`)
4. Common bad scenarios:
   - Liveness: `assume(result is Err)` — valid input but Err
   - Frame condition: `assume(post@.is_bit_set(j) != pre@.is_bit_set(j))` — unrelated state changed
   - Fairness: `assume(result == Ok::<_, Error>(0usize))` — degenerate output
5. Separate STATE ASSUMPTIONS from BAD SCENARIO with comments.
6. For Result types, use `Ok::<RetType, Error>(value)` for type annotations if needed.
7. For `exists` quantifiers with `<` comparisons, wrap in parens: `(a < b)` to avoid parser issues.

## State scenarios
If a property includes `state_scenarios`, generate ONE body per scenario:
- First: NO state assumptions (universal)
- Each subsequent: add state assumptions for that scenario
- Name suffix: `<name>`, `<name>_empty`, `<name>_almost_full`, etc.

For EACH property, output in this EXACT format:

===PHI_START===
NAME: <short_snake_case_name>
TARGET_FN: <name of the exec function being tested>
TYPE: behavioral | boundary | logical
SOURCE: <source from the property>
PROPERTY: <the natural language property>
PARAMS: <free variable list, e.g.: pre: Bitmap, post: Bitmap, result: Result<usize, Error>>
BODY:
```rust
// --- State assumptions ---
assume(post.inv());

// --- Bad scenario ---
assume(result is Err);
```
REASON: <one line why this would be undesirable>
===PHI_END===
"""

def step3_formalize(declarations, properties):
    log("=== STEP 3: Formalize (AST-based) ===")
    llm = LLMClient(timeout=600)
    source_path = str(ORIGINAL_RS.resolve())

    BATCH_SIZE = 5
    all_candidates = []
    all_raw = []

    for batch_start in range(0, len(properties), BATCH_SIZE):
        batch = properties[batch_start:batch_start + BATCH_SIZE]
        batch_num = batch_start // BATCH_SIZE + 1
        total_batches = (len(properties) + BATCH_SIZE - 1) // BATCH_SIZE

        # Build prompt with declarations
        prop_text = ""
        for i, p in enumerate(batch):
            target_fn = p.get('target_fn', '?')
            prop_text += f"\n### Property {i+1}: {p.get('id', f'prop_{i+1}')}\n"
            prop_text += f"- **Target:** `{target_fn}`\n"
            prop_text += f"- **Category:** {p.get('category', '?')}\n"
            prop_text += f"- **Source:** {p.get('source', '?')}\n"
            prop_text += f"- **Property:** {p.get('property', '?')}\n"
            if p.get('body_evidence'):
                prop_text += f"- **Body evidence:** {p['body_evidence']}\n"
            prop_text += f"- **Reasoning:** {p.get('reasoning', '?')}\n"
            if p.get('state_scenarios'):
                prop_text += f"- **State scenarios:** {json.dumps(p['state_scenarios'])}\n"
            if target_fn in declarations:
                prop_text += f"- **Function declaration (exact spec — do NOT modify):**\n```rust\n{declarations[target_fn]}\n```\n"

        user_prompt = (
            f"Read the Verus source file at: {source_path}\n"
            f"\n## Properties to formalize:\n{prop_text}\n"
            f"\nFor each property, provide ONLY the body (assume statements) and PARAMS."
        )

        log(f"  batch {batch_num}/{total_batches} ({len(batch)} properties)...")
        t0 = time.time()
        try:
            resp = llm.chat(FORMALIZE_PROMPT, user_prompt, model="claude-opus-4.6")
            raw = resp.content
        except Exception as e:
            raw = f"ERROR: {e}"
            log(f"  ERROR: {e}")

        elapsed = time.time() - t0
        candidates = parse_phi_blocks(raw)

        # Assemble full proof fns for body-only candidates
        for c in candidates:
            if "body" in c:
                c["code"] = assemble_proof_fn(c, declarations)

        all_raw.append(f"=== BATCH {batch_num} ===\n{raw}")
        all_candidates.extend(candidates)
        log(f"  → {len(candidates)} φ in {elapsed:.0f}s")

    (WORKSPACE / f"formalize_{TAG}_raw.txt").write_text("\n\n".join(all_raw))
    (WORKSPACE / f"candidates_{TAG}.json").write_text(json.dumps(all_candidates, indent=2))
    log(f"  → Total: {len(all_candidates)} candidates")
    return all_candidates

# ============================================================
# Step 4: Entailment — inject φ into source and run Verus
# ============================================================
def step4_entailment(candidates):
    log("=== STEP 4: Entailment ===")

    # Read original test file
    original_test = TEST_RS_BAK.read_text() if TEST_RS_BAK.exists() else TEST_RS.read_text()

    # Find insertion point (before closing `}` of verus!{} block)
    insert_marker = "\n} // verus!"
    if insert_marker not in original_test:
        # Try alternative
        lines = original_test.rstrip().rsplit("}", 1)
        if len(lines) == 2:
            base = lines[0]
            suffix = "}"
        else:
            log("  ERROR: cannot find insertion point")
            return []
    else:
        idx = original_test.index(insert_marker)
        base = original_test[:idx]
        suffix = original_test[idx:]

    # Build φ block
    phi_code = "\n\n// ===== GENERATED PHI TESTS (v3 — AST-based) =====\n\n"
    written = 0
    for c in candidates:
        code = c.get("code", "")
        if code and not code.startswith("// ERROR"):
            phi_code += f"{code}\n\n"
            written += 1

    full_test = base + phi_code + suffix
    TEST_RS.write_text(full_test)
    log(f"  Written {len(full_test)} chars, {written} proof fns")

    # Run Verus
    log("  Running Verus verification...")
    import subprocess
    t0 = time.time()
    result = subprocess.run(
        ["bash", "scripts/verify-bitmap.sh"],
        capture_output=True, text=True, timeout=600,
        cwd=str(NANVIX_ROOT)
    )
    elapsed = time.time() - t0
    log(f"  Verus completed in {elapsed:.0f}s")

    stderr = result.stderr

    # Parse: extract fn names that appear in error context
    failed_fns = set()
    for m in re.finditer(r'proof fn (phi_\S+)\(', stderr):
        failed_fns.add(m.group(1))

    # Parse verification summary
    summary_m = re.search(r'(\d+) verified, (\d+) errors', stderr)
    if summary_m:
        log(f"  Results: {summary_m.group(1)} verified, {summary_m.group(2)} errors")

    # Map candidates to results
    for c in candidates:
        code = c.get("code", "")
        fn_m = re.search(r'proof fn (\S+)\(', code)
        if fn_m:
            code_fn = fn_m.group(1)
            c["code_fn"] = code_fn
            c["entailed"] = code_fn not in failed_fns
            c["verified"] = code_fn not in failed_fns
        else:
            c["code_fn"] = ""
            c["entailed"] = False
            c["verified"] = False

    verified = [c for c in candidates if c.get("entailed")]
    failed = [c for c in candidates if not c.get("entailed")]
    log(f"  → {len(verified)} verified (incomplete), {len(failed)} failed (complete)")

    (WORKSPACE / f"entailment_{TAG}.json").write_text(json.dumps(candidates, indent=2))

    # Save stderr for debugging
    (WORKSPACE / f"verus_{TAG}_stderr.txt").write_text(stderr)

    return candidates

# ============================================================
# Step 5: Critic
# ============================================================
CRITIC_PROMPT = """You are reviewing Verus spec completeness test results.

Each φ was a `proof fn` with:
- requires/ensures = EXACT COPY of the target function's spec (mechanically extracted from AST)
- body = assume() statements encoding a "bad scenario"

If Verus VERIFIED the φ, it means the spec does NOT exclude that bad behavior → potential spec gap.

Your job: for each verified φ, determine if it's a TRUE POSITIVE (real spec gap) or FALSE POSITIVE.

## False positive reasons:
1. **Intentional generality**: the spec deliberately abstracts over this (e.g., non-deterministic allocation order)
2. **Ghost/exec confusion**: testing ghost-level properties that don't apply to exec behavior
3. **Tautological**: the bad scenario is vacuously true or logically trivial
4. **Duplicate**: semantically identical to another φ (note which one)

## True positive criteria:
- The spec genuinely fails to exclude a behavior that callers would reasonably expect to be excluded
- The gap has practical consequences (callers can't rely on expected behavior)

For each φ, output:

===VERDICT_START===
PHI: <phi_name>
VERDICT: TRUE_POSITIVE | FALSE_POSITIVE
CONFIDENCE: high | medium | low
FILTER_APPLIED: <which filter caught it, or "incompleteness" for TP>
REASONING: <brief explanation>
===VERDICT_END===

End with:
===SUMMARY===
<overall summary: how many TP/FP, what distinct gaps were found>
===END_SUMMARY===
"""

def step5_critic(candidates):
    log("=== STEP 5: Critic ===")
    verified = [c for c in candidates if c.get("entailed")]
    if not verified:
        log("  No verified φ to critique")
        return

    log(f"  {len(verified)} verified φ to critique")

    phi_text = ""
    for i, r in enumerate(verified):
        phi_text += f"\n### φ{i+1}: {r.get('code_fn', r.get('name', '?'))}\n"
        phi_text += f"- Target: `{r.get('target_fn', '?')}`\n"
        phi_text += f"- Property: {r.get('property', '?')}\n"
        phi_text += f"- Code:\n```rust\n{r.get('code', '')}\n```\n"

    user_prompt = (
        f"Read the Verus source file at: {ORIGINAL_RS.resolve()}\n\n"
        f"## Verified (entailed) φ candidates:\n{phi_text}\n\n"
        f"Evaluate each φ and output verdicts."
    )

    llm = LLMClient(timeout=300)
    t0 = time.time()
    resp = llm.chat(CRITIC_PROMPT, user_prompt, model="claude-opus-4.6")
    raw = resp.content
    elapsed = time.time() - t0

    (WORKSPACE / f"critic_{TAG}_raw.txt").write_text(raw)

    tp = raw.lower().count('true_positive')
    fp = raw.lower().count('false_positive')
    log(f"  → {tp} TP, {fp} FP in {elapsed:.0f}s")

    # Parse verdicts
    from pipeline_common import parse_verdicts
    verdicts = parse_verdicts(raw)
    (WORKSPACE / f"verdicts_{TAG}.json").write_text(json.dumps(verdicts, indent=2))

    return verdicts

# ============================================================
# Main
# ============================================================
def main():
    log(f"Pipeline {TAG} — Full run on nanvix bitmap")
    log(f"Workspace: {WORKSPACE}")

    # Step 1
    declarations = step1_extract_declarations()

    # Step 2
    properties = step2_load_brainstorm()

    # Step 3
    candidates = step3_formalize(declarations, properties)

    if not candidates:
        log("ERROR: no candidates generated")
        return

    # Step 4
    candidates = step4_entailment(candidates)

    # Step 5
    verdicts = step5_critic(candidates)

    # Summary
    verified = sum(1 for c in candidates if c.get("entailed"))
    failed = sum(1 for c in candidates if not c.get("entailed"))
    log(f"\n{'='*60}")
    log(f"PIPELINE {TAG} COMPLETE")
    log(f"  Candidates: {len(candidates)}")
    log(f"  Entailment: {verified} verified / {failed} failed")
    if verdicts:
        tp = sum(1 for v in verdicts if v.get("verdict", "").upper() == "TRUE_POSITIVE")
        fp = sum(1 for v in verdicts if v.get("verdict", "").upper() == "FALSE_POSITIVE")
        log(f"  Critic: {tp} TP / {fp} FP")
    log(f"{'='*60}")


if __name__ == "__main__":
    main()
