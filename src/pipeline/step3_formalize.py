#!/home/chentianyu/miniconda3/bin/python3
"""
Step 3: Formalize natural-language negative properties into Verus proof functions.

Takes brainstormed properties (natural language) and generates corresponding
Verus proof fn code for entailment checking.

Reads:  workspace/<task_name>/brainstorm.json + original.rs
Writes: workspace/<task_name>/candidates.json

Usage:
  python3 step3_formalize.py [--limit N] [--offset N] [--model MODEL] [--workspace DIR]
"""

import argparse
import json
import sys
import time
from pathlib import Path

BASE = Path.home() / "intent_formalization"

sys.path.insert(0, str(BASE / "src" / "utils"))
from llm import LLMClient
from pipeline_common import extract_spec_portion, parse_phi_blocks


# ---------------------------------------------------------------------------
# Prompt
# ---------------------------------------------------------------------------

FORMALIZE_PROMPT = """You are a Verus (Rust verification) code generator.

You are given:
1. A Verus source file with type definitions, spec functions, and executable functions
2. A list of negative properties described in natural language
3. For each target function, its EXACT declaration (signature + requires + ensures with empty body)

Your job: for EACH property, write ONLY the body (assume statements) for a Verus `proof fn`
that formalizes it as an ENTAILMENT CHECK for spec completeness.

## How it works

You will receive a function declaration like:
```rust
#[verus_spec(result =>
    requires
        old(self).inv(),
    ensures
        self.inv(),
        match result { Ok(v) => { ... }, Err(_) => { ... } },
)]
pub fn alloc(&mut self) -> Result<usize, Error> { }
```

The declaration already has the EXACT spec. You do NOT copy or modify the spec.
You ONLY provide the body contents (assume statements).

## Interpretation
- If Verus VERIFIES the proof fn: the spec allows the bad behavior → spec is INCOMPLETE
- If Verus REJECTS it: the spec excludes the bad behavior → spec is complete

## CRITICAL RULES
1. You do NOT write requires or ensures. They are mechanically extracted from the AST.
2. You ONLY write assume() statements for the body.
3. Use FREE VARIABLES for the exec function's state and return value:
   - `pre` for `old(self)`, `post` for `self`, `result` for the return value
   - You may call spec fns and ghost methods on these (e.g., `pre@.is_full()`)
4. Common bad scenarios to encode as assume():
   - Liveness: `assume(result is Err)` — valid input but Err
   - Frame condition: `assume(post@.is_bit_set(j) != pre@.is_bit_set(j))` — unrelated state changed
   - Fairness: `assume(result == Ok(0))` — degenerate output
   - Precision: `assume(result is Ok)` + output violates expected property
5. Use comments to clearly separate STATE ASSUMPTIONS from BAD SCENARIO:
   ```rust
   // --- State assumptions ---
   assume(pre@.num_bits > 1);
   assume(!pre@.is_bit_set(0));

   // --- Bad scenario ---
   assume(result is Err);
   ```

## State scenarios

If a property includes `state_scenarios`, generate ONE body per scenario:
- The first should have NO state assumptions (universal — tests all states)
- Each subsequent one adds state assumptions for that specific scenario
- Name them with a suffix: `<name>`, `<name>_empty`, `<name>_almost_full`, etc.

For EACH property, output in this EXACT format:

===PHI_START===
NAME: <short_snake_case_name>
TARGET_FN: <name of the exec function being tested>
TYPE: behavioral | boundary | logical
SOURCE: <source from the property: spec_only or body_aware>
PROPERTY: <the natural language property being formalized>
PARAMS: <free variable list, e.g.: pre: Bitmap, post: Bitmap, result: Result<usize, Error>>
BODY:
```rust
// --- State assumptions ---
assume(post.inv());
assume(pre@.num_bits > 1);

// --- Bad scenario ---
assume(result is Err);
```
REASON: <one line why this would be undesirable if entailed>
===PHI_END===

RULES:
- If the property has `state_scenarios`, generate ONE entry per scenario (universal + each specific state)
- Otherwise, generate ONE entry per property
- Do NOT write requires/ensures — they come from the AST
- Only output assume() statements for the body
- Use types/functions/traits from the source file
- If a property is too vague to formalize, do your best and note it in REASON
"""


# ---------------------------------------------------------------------------
# Assembly: combine AST declaration + LLM body
# ---------------------------------------------------------------------------

def _rewrite_declaration_to_proof_fn(declaration: str, phi_name: str, params: str) -> str:
    """Transform an exec fn declaration into a proof fn declaration.
    
    Replaces:
    - pub fn name(...) -> RetType  →  proof fn phi_name(free_vars)
    - #[verus_spec(result => requires ... ensures ...)]  →  requires ... ensures ...
    - { }  →  will be filled with body
    
    For nanvix-style #[verus_spec(...)], extract requires/ensures from the attribute.
    For inline-spec style, extract from fn body.
    """
    import re
    
    # Extract requires and ensures from #[verus_spec(...)]
    spec_m = re.search(
        r'#\[verus_spec\((\w+)\s*=>\s*(.*?)\)\]\s*(?:pub\s+)?fn',
        declaration, re.DOTALL
    )
    
    if spec_m:
        result_var = spec_m.group(1)  # e.g., "result"
        spec_body = spec_m.group(2).strip()
        
        # Split into requires and ensures
        requires_m = re.search(r'requires\s+(.*?)(?=\bensures\b)', spec_body, re.DOTALL)
        ensures_m = re.search(r'ensures\s+(.*?)$', spec_body, re.DOTALL)
        
        requires_text = requires_m.group(1).strip().rstrip(',') if requires_m else ""
        ensures_text = ensures_m.group(1).strip().rstrip(',') if ensures_m else spec_body.strip().rstrip(',')
        
        # Build proof fn
        # Replace old(self)/self references: old(self) -> pre, self -> post
        # Handle multi-line old(self,) patterns first
        requires_text = re.sub(r'old\(\s*self\s*,?\s*\)', 'pre', requires_text)
        ensures_text = re.sub(r'old\(\s*self\s*,?\s*\)', 'pre', ensures_text)
        requires_text = requires_text.replace('self', 'post')
        ensures_text = ensures_text.replace('self', 'post')
        # In proof fns, pre/post are values not references, so *pre -> pre
        requires_text = re.sub(r'\*pre\b', 'pre', requires_text)
        requires_text = re.sub(r'\*post\b', 'post', requires_text)
        ensures_text = re.sub(r'\*pre\b', 'pre', ensures_text)
        ensures_text = re.sub(r'\*post\b', 'post', ensures_text)
        
        parts = [f"proof fn {phi_name}({params})"]
        if requires_text:
            parts.append(f"    requires\n        {requires_text},")
        parts.append(f"    ensures\n        {ensures_text},")
        
        return "\n".join(parts)
    
    # Fallback: inline spec style (requires/ensures directly in fn signature)
    # Just replace fn name and mode
    result = declaration
    result = re.sub(r'(?:pub\s+)?fn\s+\w+', f'proof fn {phi_name}', result, count=1)
    # Remove return type (proof fns don't have one in our pattern)
    result = re.sub(r'\)\s*->\s*[^{]+\{', ') {', result)
    return result


def assemble_proof_fn(candidate: dict, declarations: dict) -> str:
    """Assemble a full proof fn from AST declaration + LLM-generated body."""
    target_fn = candidate.get("target_fn", "")
    name = candidate.get("name", "unknown")
    body = candidate.get("body", "")
    params = candidate.get("params", "")
    
    if target_fn not in declarations:
        # Fallback: can't assemble without declaration
        return f"// ERROR: no declaration found for {target_fn}\n// body: {body}"
    
    declaration = declarations[target_fn]
    phi_name = f"phi_{name}" if not name.startswith("phi_") else name
    
    header = _rewrite_declaration_to_proof_fn(declaration, phi_name, params)
    
    return f"{header}\n{{\n    {body}\n}}"


# ---------------------------------------------------------------------------
# Processing
# ---------------------------------------------------------------------------

def formalize_batch(llm: LLMClient, model: str, source_path: str, properties: list, declarations: dict) -> tuple[str, list]:
    """Formalize a batch of natural-language properties into Verus code.
    
    declarations: dict mapping fn_name -> declaration text (fn shell with empty body)
    """
    prop_text = ""
    for i, p in enumerate(properties):
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
        # Include the function declaration if available
        if target_fn in declarations:
            prop_text += f"- **Function declaration (exact spec — do NOT modify):**\n```rust\n{declarations[target_fn]}\n```\n"

    user_prompt = (
        f"Read the Verus source file at: {source_path}\n"
        f"\n## Properties to formalize:\n{prop_text}\n"
        f"\nFor each property, provide ONLY the body (assume statements). "
        f"Do NOT write requires/ensures — they are mechanically extracted from the AST."
    )

    try:
        resp = llm.chat(FORMALIZE_PROMPT, user_prompt, model=model)
        raw = resp.content
    except Exception as e:
        raw = f"ERROR: {e}"

    candidates = parse_phi_blocks(raw)
    return raw, candidates


def process_one(task_dir: Path, llm: LLMClient, model: str) -> dict:
    """Formalize brainstormed properties for one task."""
    brainstorm_file = task_dir / "brainstorm.json"
    original_file = task_dir / "original.rs"

    if not brainstorm_file.exists() or not original_file.exists():
        return {"task": task_dir.name, "status": "missing_files"}

    properties = json.loads(brainstorm_file.read_text())
    if not properties:
        return {"task": task_dir.name, "status": "no_properties", "candidates": 0}

    source_path = str(original_file.resolve())

    # Load declarations if available (from step1 exec_functions.json or task-level)
    declarations = {}
    exec_fns_file = task_dir / "exec_functions.json"
    if exec_fns_file.exists():
        for fn_info in json.loads(exec_fns_file.read_text()):
            if "declaration" in fn_info:
                declarations[fn_info["name"]] = fn_info["declaration"]

    # Batch properties in groups of 5 to avoid timeout on large files
    BATCH_SIZE = 5
    all_candidates = []
    all_raw = []

    for batch_start in range(0, len(properties), BATCH_SIZE):
        batch = properties[batch_start:batch_start + BATCH_SIZE]
        batch_num = batch_start // BATCH_SIZE + 1
        total_batches = (len(properties) + BATCH_SIZE - 1) // BATCH_SIZE
        print(f"  [form] {task_dir.name} — batch {batch_num}/{total_batches} ({len(batch)} properties)")

        raw, candidates = formalize_batch(llm, model, source_path, batch, declarations)
        all_raw.append(f"=== BATCH {batch_num} ===\n{raw}")
        all_candidates.extend(candidates)

    # If we have declarations and body-only candidates, assemble full proof fns
    if declarations:
        for c in all_candidates:
            if "body" in c and "code" not in c:
                c["code"] = assemble_proof_fn(c, declarations)

    (task_dir / "formalize_raw.txt").write_text("\n\n".join(all_raw))
    (task_dir / "candidates.json").write_text(json.dumps(all_candidates, indent=2))

    return {
        "task": task_dir.name,
        "properties": len(properties),
        "candidates": len(all_candidates),
        "status": "ok" if all_candidates else "no_candidates",
    }


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="Step 3: Formalize properties into Verus code")
    parser.add_argument("--limit", type=int, default=None)
    parser.add_argument("--offset", type=int, default=0)
    parser.add_argument("--model", type=str, default="claude-opus-4.6")
    parser.add_argument("--workspace", type=str, default=str(BASE / "verusage" / "workspace_v4"))
    args = parser.parse_args()

    workspace = Path(args.workspace)
    task_dirs = sorted([
        d for d in workspace.iterdir()
        if d.is_dir()
        and (d / "brainstorm.json").exists()
        and not (d / "candidates.json").exists()
    ])

    task_dirs = task_dirs[args.offset:]
    if args.limit:
        task_dirs = task_dirs[:args.limit]

    print(f"Step 3: Formalizing for {len(task_dirs)} tasks (model={args.model})")
    llm = LLMClient(timeout=600)

    total_candidates = 0
    for i, td in enumerate(task_dirs):
        print(f"\n[{i+1}/{len(task_dirs)}]")
        try:
            r = process_one(td, llm, args.model)
            total_candidates += r.get("candidates", 0)
            print(f"  → {r['status']} ({r.get('candidates', 0)} candidates)")
        except Exception as e:
            print(f"  [error] {td.name}: {e}")

    print(f"\n=== Done: {total_candidates} candidates formalized ===")


if __name__ == "__main__":
    main()
