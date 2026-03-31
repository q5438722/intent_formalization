#!/home/chentianyu/miniconda3/bin/python3
"""
Standalone script to run step3 formalize on bitmap brainstorm_v2.
Run with: nohup python3 scripts/run_formalize_v2.py > /tmp/formalize_v2.log 2>&1 &
"""
import json, sys, re, ast, time, os

os.environ.setdefault("LLM_TIMEOUT", "600")
os.environ.setdefault("LLM_STALL_TIMEOUT", "90")

BASE = os.path.expanduser("~/intent_formalization")
sys.path.insert(0, os.path.join(BASE, "src/utils"))

from llm import LLMClient
from pipeline_common import parse_phi_blocks

# Load FORMALIZE_PROMPT from step3
with open(os.path.join(BASE, "src/pipeline/step3_formalize.py")) as f:
    content = f.read()
tree = ast.parse(content)
for node in ast.walk(tree):
    if isinstance(node, ast.Assign):
        for t in node.targets:
            if isinstance(t, ast.Name) and t.id == "FORMALIZE_PROMPT":
                FORMALIZE_PROMPT = ast.literal_eval(node.value)

ws = os.path.join(BASE, "nanvix/workspace/bitmap")
properties = json.loads(open(os.path.join(ws, "brainstorm_v2.json")).read())
source_path = os.path.join(ws, "original.rs")

llm = LLMClient()
all_candidates = []
all_raw = []
errors = 0

print(f"Starting formalize v2: {len(properties)} properties, timeout={os.environ['LLM_TIMEOUT']}s, stall={os.environ['LLM_STALL_TIMEOUT']}s", flush=True)

for idx, p in enumerate(properties):
    print(f"\n[{idx+1}/{len(properties)}] {p['id']}: {p['target_fn']} — {p['property'][:60]}...", flush=True)
    t0 = time.time()

    prop_text = f"""
### Property: {p['id']}
- **Target:** `{p['target_fn']}`
- **Category:** {p['category']}
- **Source:** {p['source']}
- **Property:** {p['property']}
- **Reasoning:** {p['reasoning']}
- **State scenarios:** {json.dumps(p.get('state_scenarios', []))}
"""
    user_prompt = (
        f"Read the Verus source file at: {source_path}\n"
        f"\n## Property to formalize:\n{prop_text}\n"
        f"\nFormalize into Verus proof fn(s)."
    )

    try:
        resp = llm.chat(FORMALIZE_PROMPT, user_prompt, model="claude-opus-4.6")
        raw = resp.content
        candidates = parse_phi_blocks(raw)
        print(f"  → {len(candidates)} φ in {time.time()-t0:.0f}s", flush=True)
        for c in candidates:
            print(f"    {c['name']}", flush=True)
    except Exception as e:
        raw = f"ERROR: {e}"
        candidates = []
        errors += 1
        print(f"  → ERROR in {time.time()-t0:.0f}s: {e}", flush=True)

    all_raw.append(f"=== {p['id']} ===\n{raw}")
    all_candidates.extend(candidates)

    # Save incrementally
    with open(os.path.join(ws, "formalize_v2_raw.txt"), "w") as f:
        f.write("\n\n".join(all_raw))
    with open(os.path.join(ws, "candidates_v2.json"), "w") as f:
        f.write(json.dumps(all_candidates, indent=2))

print(f"\n{'='*60}", flush=True)
print(f"DONE: {len(all_candidates)} candidates from {len(properties)} properties ({errors} errors)", flush=True)
print(f"Files: {ws}/candidates_v2.json, {ws}/formalize_v2_raw.txt", flush=True)
