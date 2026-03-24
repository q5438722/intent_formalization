#!/usr/bin/env python3
"""
Count VeruSAGE tasks that:
  1. Contain NO #[verifier::external_body] or #[verifier(external_body)] tags
  2. Have at least one executable (exec) function — i.e., a function that is NOT
     annotated as `proof`, `spec`, `open spec`, `closed spec`, or `uninterp`.

An "exec" function in Verus is any `fn` declaration that lacks proof/spec/uninterp
mode keywords.  `fn main()` is excluded since it's just boilerplate in these tasks.

Usage:
    python script/count_executable_tasks.py [--source_dir DIR] [--list] [--verbose]
"""

import argparse
import os
import re
import sys


# ---------------------------------------------------------------------------
# Patterns
# ---------------------------------------------------------------------------

# Matches both annotation styles:
#   #[verifier::external_body]
#   #[verifier(external_body)]
EXTERNAL_BODY_RE = re.compile(
    r"#\[verifier(?:::|\()external_body\)?\]"
)

# Matches function declarations.  Captures the optional mode keyword and
# the function name.
#
# The pattern handles declarations like:
#   fn foo(...)
#   pub fn foo(...)
#   pub exec fn foo(...)
#   pub open spec fn foo(...)
#   pub closed spec fn foo(...)
#   proof fn foo(...)
#   pub uninterp fn foo(...)
#
# Group 1: mode keyword (spec|proof|exec|uninterp) or None
# Group 2: function name
FN_DECL_RE = re.compile(
    r"""
    (?:^|[\s{;])                       # start of line or after whitespace/brace/semicolon
    (?:pub\s+)?                        # optional pub
    (?:(?:open|closed)\s+)?            # optional open/closed (for spec fns)
    (?:(spec(?:\(checked\))?|proof|exec|uninterp)\s+)? # optional mode keyword (incl. spec(checked))
    fn\s+                              # fn keyword
    (\w+)                              # function name
    """,
    re.VERBOSE | re.MULTILINE,
)

# Lines that are comments (to skip fn mentions in comments)
COMMENT_LINE_RE = re.compile(r"^\s*//")


def _strip_comments(source: str) -> str:
    """Remove single-line comments and block comments from source."""
    # Remove block comments (non-greedy, handles nested poorly but sufficient)
    source = re.sub(r"/\*.*?\*/", "", source, flags=re.DOTALL)
    # Remove single-line comments
    source = re.sub(r"//[^\n]*", "", source)
    return source


def has_external_body(source: str) -> bool:
    """Return True if the source contains any external_body annotation."""
    return bool(EXTERNAL_BODY_RE.search(source))


def has_exec_function(source: str) -> list[str]:
    """
    Return a list of exec function names found in the source.

    An exec function is any `fn` declaration whose mode is NOT
    spec/proof/uninterp.  `fn main` is excluded.
    """
    cleaned = _strip_comments(source)
    exec_fns = []
    for match in FN_DECL_RE.finditer(cleaned):
        mode = match.group(1)
        name = match.group(2)
        # Skip non-exec modes
        if mode is not None and mode.startswith(("spec", "proof", "uninterp")):
            continue
        # Skip main() boilerplate
        if name == "main":
            continue
        exec_fns.append(name)
    return exec_fns


def discover_tasks(source_dir: str) -> list[dict]:
    """Walk verified/ subtrees and return task metadata."""
    tasks = []
    for project in sorted(os.listdir(source_dir)):
        verified_dir = os.path.join(source_dir, project, "verified")
        if not os.path.isdir(verified_dir):
            continue
        for root, _dirs, files in os.walk(verified_dir):
            for fname in sorted(files):
                if not fname.endswith(".rs"):
                    continue
                fpath = os.path.join(root, fname)
                rel_category = os.path.relpath(root, verified_dir)
                tasks.append({
                    "project": project,
                    "category": rel_category if rel_category != "." else "",
                    "file": fname,
                    "path": fpath,
                })
    return tasks


def main():
    parser = argparse.ArgumentParser(
        description="Count VeruSAGE tasks with no external_body and at least one exec function."
    )
    parser.add_argument(
        "--source_dir",
        default=os.path.join(
            os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
            "verusage", "source-projects",
        ),
        help="Root of source-projects directory",
    )
    parser.add_argument(
        "--list", action="store_true",
        help="List qualifying task file paths",
    )
    parser.add_argument(
        "--verbose", action="store_true",
        help="Show exec function names for each qualifying task",
    )
    parser.add_argument(
        "--output", type=str, default=None,
        help="Write qualifying task paths to this file (one per line)",
    )
    args = parser.parse_args()

    tasks = discover_tasks(args.source_dir)
    total = len(tasks)

    # Four buckets: (has_external_body, has_exec_fn)
    buckets: dict[tuple[bool, bool], list[tuple[dict, list[str]]]] = {
        (True, True): [],
        (True, False): [],
        (False, True): [],
        (False, False): [],
    }

    for task in tasks:
        with open(task["path"]) as f:
            source = f.read()

        ext = has_external_body(source)
        exec_fns = has_exec_function(source)
        buckets[(ext, bool(exec_fns))].append((task, exec_fns))

    # --- Summary table ---
    label = {
        (True, True):   "external_body=YES, exec_fn=YES",
        (True, False):  "external_body=YES, exec_fn=NO ",
        (False, True):  "external_body=NO , exec_fn=YES",
        (False, False): "external_body=NO , exec_fn=NO ",
    }
    print(f"Total tasks (verified/): {total}")
    print()
    print(f"  {'Category':<40s} {'Count':>5s}")
    print(f"  {'-'*40} {'-'*5}")
    for key in [(True, True), (True, False), (False, True), (False, False)]:
        print(f"  {label[key]:<40s} {len(buckets[key]):>5d}")
    print()

    # --- Per-project breakdown ---
    projects: dict[str, dict[tuple[bool, bool], int]] = {}
    for key, items in buckets.items():
        for task, _ in items:
            proj = task["project"]
            projects.setdefault(proj, {k: 0 for k in buckets})
            projects[proj][key] += 1

    header = "ext+exec  ext-only  exec-only proof/spec"
    print(f"  {'Project':<25s} {header}   Total")
    print(f"  {'-'*25} {'-'*len(header)}   -----")
    for proj in sorted(projects):
        p = projects[proj]
        proj_total = sum(p.values())
        print(
            f"  {proj:<25s} "
            f"{p[(True,True)]:>8d}  {p[(True,False)]:>8d}  "
            f"{p[(False,True)]:>9d}  {p[(False,False)]:>9d}   "
            f"{proj_total:>5d}"
        )
    print()

    # --- Detailed listing per bucket (when --list or --verbose) ---
    if args.list or args.verbose:
        for key in [(False, True), (True, True), (False, False), (True, False)]:
            items = buckets[key]
            if not items:
                continue
            print(f"--- {label[key]} ({len(items)}) ---")
            for task, exec_fns in items:
                if args.verbose and exec_fns:
                    print(f"  {task['path']}")
                    print(f"    exec fns: {', '.join(exec_fns)}")
                else:
                    print(f"  {task['path']}")
            print()

    if args.output:
        with open(args.output, "w") as f:
            for task, _ in buckets[(False, True)]:
                f.write(task["path"] + "\n")
        print(f"Wrote {len(buckets[(False, True)])} paths (no external_body + has exec fn) to {args.output}")


if __name__ == "__main__":
    main()
