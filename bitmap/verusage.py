"""
Remove specifications (requires/ensures/decreases/recommends) from the target
function in every task of the VeruSAGE-Bench tasks.jsonl file.

Usage:
    python verusage.py \
        --input  /path/to/tasks.jsonl \
        --output /path/to/tasks_no_spec.jsonl \
        [--language_path /path/to/verus.so]
"""

import os
import sys
import json
import argparse

# Allow importing verus_parser from the static/ directory
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'static'))
from verus_parser import verus_parser, verus_editor, node_to_text


def remove_target_fn_spec(task_code: str, target_function: str,
                          language_path: str) -> str:
    """
    Parse *task_code* with tree-sitter, locate *target_function*, and strip
    its ``function_specifications`` node (requires / ensures / decreases /
    recommends clauses).  Returns the modified source code.

    If the target function is not found or has no specs, the original code is
    returned unchanged.
    """
    editor = verus_editor(task_code, language_path)
    functions = editor.vs_parser.extract_function(editor.current_ast.root_node)

    target_spec_nodes = []
    for fn in functions:
        name_node = fn.child_by_field_name('name')
        if name_node is None:
            continue
        if node_to_text(name_node) == target_function:
            spec_node = fn.child_by_field_name('specifications')
            if spec_node is not None:
                target_spec_nodes.append(spec_node)

    if target_spec_nodes:
        editor.replace_nodes(target_spec_nodes, target_str='')

    return editor.current_program


def process_tasks(input_path: str, output_path: str, language_path: str) -> None:
    """
    Read *input_path* (JSONL), strip specs from the target function in each
    task, and write the result to *output_path* (JSONL).

    Each JSON line is expected to have at least:
        - ``task``             : source code with proof body already removed
        - ``target_function``  : name of the function whose spec to remove
    """
    tasks = []
    with open(input_path, 'r') as f:
        for line in f:
            line = line.strip()
            if line:
                tasks.append(json.loads(line))

    print(f"Loaded {len(tasks)} tasks from {input_path}")

    processed = 0
    skipped = 0
    for idx, task in enumerate(tasks):
        task_code = task['task']
        target_fn = task['target_function']

        new_code = remove_target_fn_spec(task_code, target_fn, language_path)

        if new_code != task_code:
            processed += 1
        else:
            skipped += 1

        task['task'] = new_code

        if (idx + 1) % 100 == 0 or idx + 1 == len(tasks):
            print(f"  [{idx + 1}/{len(tasks)}] processed={processed}, "
                  f"unchanged={skipped}")

    os.makedirs(os.path.dirname(os.path.abspath(output_path)), exist_ok=True)
    with open(output_path, 'w') as f:
        for task in tasks:
            f.write(json.dumps(task, ensure_ascii=False) + '\n')

    print(f"Done. Wrote {len(tasks)} tasks to {output_path}")
    print(f"  Specs removed: {processed}, Already no spec / target not found: {skipped}")


def main():
    parser = argparse.ArgumentParser(
        description="Remove specifications from the target function in "
                    "VeruSAGE-Bench tasks.")
    parser.add_argument('--input', type=str,
                        default='/home/chentianyu/verus-proof-synthesis/'
                                'benchmarks/VeruSAGE-Bench/tasks.jsonl',
                        help='Path to the input tasks.jsonl')
    parser.add_argument('--output', type=str, default=None,
                        help='Path for the output JSONL. '
                             'Defaults to <input_dir>/tasks_no_spec.jsonl')
    parser.add_argument('--language_path', type=str,
                        default='/home/chentianyu/verus.so',
                        help='Path to the tree-sitter verus .so file')
    args = parser.parse_args()

    if args.output is None:
        input_dir = os.path.dirname(os.path.abspath(args.input))
        args.output = os.path.join(input_dir, 'tasks_no_spec.jsonl')

    process_tasks(args.input, args.output, args.language_path)


if __name__ == '__main__':
    main()
