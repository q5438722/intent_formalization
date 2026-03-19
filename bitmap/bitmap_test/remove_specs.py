import sys
import os

sys.path.insert(0, "/home/chentianyu/tree-sitter-verus/static")
from verus_parser import verus_editor

LANGUAGE_PATH = os.path.join(os.path.expanduser("~"), "verus.so")
INPUT_FILE = os.path.join(os.path.dirname(__file__), "lib.rs")
OUTPUT_FILE = os.path.join(os.path.dirname(__file__), "lib.no_spec.rs")

def main():
    with open(INPUT_FILE, "r") as f:
        raw_program = f.read()

    editor = verus_editor(raw_program, LANGUAGE_PATH)
    editor.remove_proof()
    editor.remove_specification()
    editor.clean_program()

    with open(OUTPUT_FILE, "w") as f:
        f.write(editor.current_program)

    print(f"Specs removed. Output written to {OUTPUT_FILE}")

if __name__ == "__main__":
    main()
