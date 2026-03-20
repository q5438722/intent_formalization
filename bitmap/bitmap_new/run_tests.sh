#!/bin/bash
# Run individual Verus tests from inconsistency test files.
# Usage: ./run_tests.sh <test_file> <output_file>

set -uo pipefail

NANVIX_DIR="/home/chentianyu/intent_formalization/nanvix"
BITMAP_SRC="$NANVIX_DIR/src/libs/bitmap/src"
LIB_RS_BAK="$BITMAP_SRC/lib.rs.bak"
LIB_RS="$BITMAP_SRC/lib.rs"
TEST_FILE="$1"
OUTPUT_FILE="$2"

# Restore lib.rs to original state
restore_lib() {
    cp "$LIB_RS_BAK" "$LIB_RS"
}

# Extract test function names from the test file
extract_test_names() {
    grep -oP 'fn\s+\K(test_\w+)' "$TEST_FILE"
}

# Create a single-test file containing only one test function
create_single_test() {
    local test_name="$1"
    local out_file="$2"
    
    # Use Python to extract the single test function
    python3 -c "
import re
import sys

with open('$TEST_FILE', 'r') as f:
    content = f.read()

# Find the function and extract it with its comment/doc lines
pattern = r'((?://[^\n]*\n)*fn\s+${test_name}\s*\([^)]*\)(?:\s*\n\s*requires[^{]*)?)\s*\{' 
# More robust: find fn start title, then match balanced braces
lines = content.split('\n')
in_func = False
brace_count = 0
func_lines = []
comment_lines = []

for i, line in enumerate(lines):
    if not in_func:
        stripped = line.strip()
        if stripped.startswith('///') or stripped.startswith('//'):
            comment_lines.append(line)
        elif 'fn $test_name' in line:
            in_func = True
            func_lines = comment_lines + [line]
            brace_count += line.count('{') - line.count('}')
            comment_lines = []
        else:
            comment_lines = []
    else:
        func_lines.append(line)
        brace_count += line.count('{') - line.count('}')
        if brace_count <= 0:
            break

func_body = '\n'.join(func_lines)
# Wrap in verus! block
result = 'verus! {\n\n' + func_body + '\n\n} // verus!'
with open('$out_file', 'w') as f:
    f.write(result)
"
}

echo "=== Running tests from: $(basename $TEST_FILE) ===" | tee "$OUTPUT_FILE"
echo "" | tee -a "$OUTPUT_FILE"

# Get all test names
TEST_NAMES=$(extract_test_names)
TOTAL=$(echo "$TEST_NAMES" | wc -l)
PASS=0
REJECT=0
IDX=0

for test_name in $TEST_NAMES; do
    IDX=$((IDX + 1))
    echo "--- [$IDX/$TOTAL] $test_name ---" | tee -a "$OUTPUT_FILE"
    
    # Restore lib.rs
    restore_lib
    
    # Create single test file
    SINGLE_TEST="$BITMAP_SRC/lib.single_test.rs"
    create_single_test "$test_name" "$SINGLE_TEST"
    
    # Add include to lib.rs (replace gen_test include or add after test include)
    # Replace gen_test line with our test
    sed -i 's|include!("lib.gen_test.rs");|include!("lib.single_test.rs");|' "$LIB_RS"
    
    # Run verification
    cd "$NANVIX_DIR"
    OUTPUT=$(bash verify-bitmap.sh 2>&1) || true
    EXIT_CODE=$?
    
    # Check result
    if echo "$OUTPUT" | grep -q "verification results:: .* verified, 0 errors"; then
        echo "Result: PASSED" | tee -a "$OUTPUT_FILE"
        PASS=$((PASS + 1))
    else
        echo "Result: REJECTED" | tee -a "$OUTPUT_FILE"
        REJECT=$((REJECT + 1))
        # Extract key error line
        ERROR_LINE=$(echo "$OUTPUT" | grep -E "^error" | head -3)
        if [ -n "$ERROR_LINE" ]; then
            echo "Error: $ERROR_LINE" | tee -a "$OUTPUT_FILE"
        fi
    fi
    echo "" | tee -a "$OUTPUT_FILE"
done

# Cleanup
restore_lib
rm -f "$BITMAP_SRC/lib.single_test.rs"

echo "=== Summary ===" | tee -a "$OUTPUT_FILE"
echo "Total: $TOTAL" | tee -a "$OUTPUT_FILE"
echo "PASSED: $PASS" | tee -a "$OUTPUT_FILE"
echo "REJECTED: $REJECT" | tee -a "$OUTPUT_FILE"
