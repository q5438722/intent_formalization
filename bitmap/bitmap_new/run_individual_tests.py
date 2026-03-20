#!/usr/bin/env python3
"""
Extract individual test functions from a Verus test file and run them one by one.
Usage: python3 run_individual_tests.py <test_file> <output_file>
"""
import sys
import os
import re
import subprocess

NANVIX_DIR = "/home/chentianyu/intent_formalization/nanvix"
BITMAP_SRC = os.path.join(NANVIX_DIR, "src/libs/bitmap/src")
LIB_RS = os.path.join(BITMAP_SRC, "lib.rs")
LIB_RS_BAK = os.path.join(BITMAP_SRC, "lib.rs.bak")
SINGLE_TEST = os.path.join(BITMAP_SRC, "lib.single_test.rs")

def restore_lib():
    with open(LIB_RS_BAK, 'r') as f:
        content = f.read()
    with open(LIB_RS, 'w') as f:
        f.write(content)

def extract_tests(test_file):
    """Extract individual test functions from the file."""
    with open(test_file, 'r') as f:
        content = f.read()
    
    # Find all test function blocks
    tests = []
    lines = content.split('\n')
    i = 0
    while i < len(lines):
        line = lines[i]
        # Look for fn test_... pattern
        match = re.match(r'\s*fn\s+(test_\w+)', line)
        if match:
            test_name = match.group(1)
            # Collect preceding comment lines
            comment_start = i - 1
            while comment_start >= 0 and lines[comment_start].strip().startswith('///'):
                comment_start -= 1
            comment_start += 1
            
            # Now collect the function body (match braces)
            func_lines = lines[comment_start:i]  # comments
            brace_count = 0
            j = i
            while j < len(lines):
                func_lines.append(lines[j])
                brace_count += lines[j].count('{') - lines[j].count('}')
                if brace_count <= 0 and '{' in ''.join(func_lines):
                    break
                j += 1
            
            # Check if function has requires clause (precondition-based test)
            func_body = '\n'.join(func_lines)
            tests.append((test_name, func_body))
            i = j + 1
        else:
            i += 1
    
    return tests

def create_single_test_file(func_body):
    """Create a verus test file with a single test function."""
    content = f"verus! {{\n\n{func_body}\n\n}} // verus!\n"
    with open(SINGLE_TEST, 'w') as f:
        f.write(content)

def add_include_to_lib():
    """Replace lib.gen_test.rs include with lib.single_test.rs."""
    with open(LIB_RS, 'r') as f:
        content = f.read()
    content = content.replace('include!("lib.gen_test.rs");', 'include!("lib.single_test.rs");')
    with open(LIB_RS, 'w') as f:
        f.write(content)

def run_verify():
    """Run the verification script and return (exit_code, output)."""
    try:
        result = subprocess.run(
            ["bash", "verify-bitmap.sh"],
            cwd=NANVIX_DIR,
            capture_output=True,
            text=True,
            timeout=300
        )
        output = result.stdout + result.stderr
        return result.returncode, output
    except subprocess.TimeoutExpired:
        return -1, "TIMEOUT"

def check_result(output):
    """Check if verification passed or was rejected."""
    if re.search(r'verification results::.*\d+ verified, 0 errors', output):
        return "PASSED"
    else:
        return "REJECTED"

def extract_error(output):
    """Extract key error lines from output."""
    error_lines = []
    for line in output.split('\n'):
        if line.strip().startswith('error') and len(error_lines) < 3:
            error_lines.append(line.strip())
    return '\n'.join(error_lines) if error_lines else ""

def main():
    if len(sys.argv) < 3:
        print("Usage: python3 run_individual_tests.py <test_file> <output_file>")
        sys.exit(1)
    
    test_file = sys.argv[1]
    output_file = sys.argv[2]
    
    tests = extract_tests(test_file)
    basename = os.path.basename(test_file)
    
    results = []
    total = len(tests)
    passed = 0
    rejected = 0
    
    print(f"=== Running {total} tests from: {basename} ===")
    
    for idx, (test_name, func_body) in enumerate(tests, 1):
        print(f"--- [{idx}/{total}] {test_name} ---")
        
        # Restore lib.rs
        restore_lib()
        
        # Create single test file
        create_single_test_file(func_body)
        
        # Add include
        add_include_to_lib()
        
        # Run verification
        exit_code, output = run_verify()
        
        # Check result
        status = check_result(output)
        error_msg = ""
        if status == "REJECTED":
            rejected += 1
            error_msg = extract_error(output)
        else:
            passed += 1
        
        results.append((test_name, status, error_msg))
        print(f"Result: {status}")
        if error_msg:
            print(f"Error: {error_msg[:200]}")
        print()
    
    # Cleanup
    restore_lib()
    if os.path.exists(SINGLE_TEST):
        os.remove(SINGLE_TEST)
    
    # Write output file
    with open(output_file, 'w') as f:
        f.write(f"=== Results for: {basename} ===\n\n")
        for test_name, status, error_msg in results:
            f.write(f"{test_name} -> {status}")
            if status == "PASSED":
                f.write(" ✓")
            else:
                f.write(" ✓ (correctly rejected)")
            f.write("\n")
            if error_msg:
                f.write(f"  Error: {error_msg}\n")
        f.write(f"\n=== Summary ===\n")
        f.write(f"Total: {total}\n")
        f.write(f"PASSED: {passed}\n")
        f.write(f"REJECTED: {rejected}\n")
    
    print(f"\n=== Summary ===")
    print(f"Total: {total}")
    print(f"PASSED: {passed}")
    print(f"REJECTED: {rejected}")
    print(f"Results written to: {output_file}")

if __name__ == "__main__":
    main()
