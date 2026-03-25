"""
Spec extractor — strip proof bodies from Verus spec files.

Keeps: struct definitions, spec fn signatures, requires/ensures clauses.
Strips: proof bodies (replaces with { ... }).
"""

import re


def extract_spec_signatures(spec_code: str) -> str:
    """
    Extract spec signatures from Verus code, stripping proof bodies.
    
    Keeps:
    - struct/impl definitions
    - fn signatures with requires/ensures
    - spec fn bodies (they're part of the spec)
    - open spec fn bodies
    
    Strips:
    - proof fn bodies (replaced with { ... })
    - pub proof fn bodies
    """
    lines = spec_code.split("\n")
    result = []
    
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        # Detect proof fn start
        if re.match(r'\s*(pub\s+)?proof\s+fn\s+', stripped):
            # Collect the signature (fn name + params)
            sig_lines = [line]
            
            # Find requires/ensures block and opening brace
            brace_depth = 0
            in_spec_block = False
            found_body_start = False
            j = i
            
            # Count braces in current line
            brace_depth += stripped.count('{') - stripped.count('}')
            if '{' in stripped:
                found_body_start = True
            
            j = i + 1
            while j < len(lines) and not found_body_start:
                l = lines[j].strip()
                sig_lines.append(lines[j])
                brace_depth += l.count('{') - l.count('}')
                if '{' in l:
                    found_body_start = True
                j += 1
            
            # Now skip until brace_depth returns to 0
            while j < len(lines) and brace_depth > 0:
                l = lines[j].strip()
                brace_depth += l.count('{') - l.count('}')
                j += 1
            
            # Output signature lines but replace body
            # Find where requires/ensures end and body begins
            sig_text = "\n".join(sig_lines)
            result.append(sig_text)
            # Add closing with placeholder
            indent = re.match(r'(\s*)', lines[i]).group(1)
            result.append(f"{indent}    // ... proof body omitted ...")
            result.append(f"{indent}}}")
            result.append("")
            
            i = j
        else:
            result.append(line)
            i += 1
    
    return "\n".join(result)


def main():
    import sys
    if len(sys.argv) < 2:
        print("Usage: python -m src.utils.spec_extract <spec_file.rs>")
        sys.exit(1)
    
    with open(sys.argv[1]) as f:
        code = f.read()
    
    print(extract_spec_signatures(code))


if __name__ == "__main__":
    main()
