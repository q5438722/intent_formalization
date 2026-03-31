"""
Shared utilities for the spec consistency pipeline.

- extract_spec_portion: extract spec-relevant lines from large files
- build_entailment_file: insert φ into source for Verus check
- strip_spec: remove requires/ensures content for tautology testing
- parse_phi_blocks: parse ===PHI_START===...===PHI_END=== blocks
- parse_verdicts: parse ===VERDICT_START===...===VERDICT_END=== blocks
"""

import re
from pathlib import Path


def extract_spec_portion(source_text: str, max_lines: int = 400) -> str:
    """For large files, extract spec-relevant portions with context."""
    lines = source_text.split('\n')
    if len(lines) <= max_lines:
        return source_text

    spec_keywords = ['requires', 'ensures', 'invariant', 'recommends', 'spec fn',
                     'proof fn', 'pub fn', 'fn ', 'struct ', 'impl ', 'trait ',
                     'decreases', 'open spec', 'closed spec',
                     'pub proof', 'pub open spec', 'pub closed spec', 'enum ']
    important = set()
    for i, line in enumerate(lines):
        low = line.lower().strip()
        if any(kw in low for kw in spec_keywords):
            for j in range(max(0, i - 3), min(len(lines), i + 8)):
                important.add(j)

    # Always include first 30 and last 5 lines
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


def build_entailment_file(source_text: str, phi_code: str) -> str:
    """Insert φ proof fn before the final closing brace of verus!{} block."""
    last_brace = source_text.rstrip().rfind('}')
    if last_brace == -1:
        raise ValueError("Cannot find closing brace in source file")
    return (source_text[:last_brace] +
            "\n\n// === Entailment query ===\n" +
            phi_code + "\n\n}\n")


def strip_spec(source_text: str) -> str:
    """Strip requires/ensures clause content, replacing with 'true,'.

    Used for tautology detection: if φ still verifies after stripping,
    it's spec-independent (a tautology).
    """
    lines = source_text.split('\n')
    result = []
    i = 0

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        if stripped in ('requires', 'ensures'):
            result.append(line)  # keep keyword
            i += 1
            added_true = False
            while i < len(lines):
                next_stripped = lines[i].strip()
                # Stop at: next keyword, opening brace, empty line, closing brace
                if next_stripped in ('requires', 'ensures', 'recommends', 'decreases', '{', '}', ''):
                    break
                if next_stripped.startswith('//'):
                    result.append(lines[i])  # keep comments
                    i += 1
                    continue
                if not added_true:
                    indent = len(lines[i]) - len(lines[i].lstrip())
                    result.append(' ' * indent + 'true,')
                    added_true = True
                # Skip remaining condition lines
                i += 1
            continue

        result.append(line)
        i += 1

    return '\n'.join(result)


def parse_phi_blocks(text: str) -> list:
    """Parse ===PHI_START===...===PHI_END=== blocks from generator output.
    
    Supports two formats:
    - Legacy: CODE block with full proof fn
    - New: BODY block with only assume statements + PARAMS for free variables
    """
    blocks = []
    pattern = r'===PHI_START===(.*?)===PHI_END==='
    for match in re.finditer(pattern, text, re.DOTALL):
        block = match.group(1).strip()
        name_m = re.search(r'NAME:\s*(.+)', block)
        target_m = re.search(r'TARGET_FN:\s*(.+)', block)
        type_m = re.search(r'TYPE:\s*(.+)', block)
        source_m = re.search(r'SOURCE:\s*(.+)', block)
        property_m = re.search(r'PROPERTY:\s*(.+)', block)
        params_m = re.search(r'PARAMS:\s*(.+)', block)
        reason_m = re.search(r'REASON:\s*(.+)', block)

        # Try BODY first (new format), fall back to CODE (legacy)
        # For BODY, look for the code block after BODY:
        body_m = re.search(r'BODY:\s*\n```(?:verus|rust)?\s*\n(.*?)```', block, re.DOTALL)
        code_m = re.search(r'CODE:\s*\n```(?:verus|rust)?\s*\n(.*?)```', block, re.DOTALL)
        if code_m is None:
            code_m = re.search(r'```(?:verus|rust)?\s*\n(.*?)```', block, re.DOTALL)

        if name_m and (body_m or code_m):
            entry = {
                "name": name_m.group(1).strip(),
                "target_fn": target_m.group(1).strip() if target_m else "",
                "type": type_m.group(1).strip() if type_m else "unknown",
                "source": source_m.group(1).strip() if source_m else "spec_only",
                "property": property_m.group(1).strip() if property_m else "",
                "reason": reason_m.group(1).strip() if reason_m else "",
            }
            if body_m:
                entry["body"] = body_m.group(1).strip()
                entry["params"] = params_m.group(1).strip() if params_m else ""
            if code_m:
                entry["code"] = code_m.group(1).strip()
            blocks.append(entry)
    return blocks


def parse_verdicts(text: str) -> list:
    """Parse ===VERDICT_START===...===VERDICT_END=== blocks from critic output."""
    verdicts = []
    for match in re.finditer(r'===VERDICT_START===(.*?)===VERDICT_END===', text, re.DOTALL):
        block = match.group(1).strip()
        phi_m = re.search(r'PHI:\s*(.+)', block)
        verdict_m = re.search(r'VERDICT:\s*(.+)', block)
        conf_m = re.search(r'CONFIDENCE:\s*(.+)', block)
        filter_m = re.search(r'FILTER_APPLIED:\s*(.+)', block)
        reason_m = re.search(r'REASONING:\s*(.+)', block, re.DOTALL)

        if phi_m and verdict_m:
            reason = reason_m.group(1).strip() if reason_m else ""
            reason = reason.split("===")[0].strip()
            verdicts.append({
                "phi": phi_m.group(1).strip(),
                "verdict": verdict_m.group(1).strip(),
                "confidence": conf_m.group(1).strip() if conf_m else "unknown",
                "filter_applied": filter_m.group(1).strip() if filter_m else "none",
                "reasoning": reason,
            })
    return verdicts


def parse_summary(text: str) -> str:
    """Parse ===SUMMARY===...===END_SUMMARY=== block."""
    m = re.search(r'===SUMMARY===(.*?)===END_SUMMARY===', text, re.DOTALL)
    return m.group(1).strip() if m else ""
