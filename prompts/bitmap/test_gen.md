You are an expert Verus programmer specializing in formal verification of Rust code.

Think step by step. Break down the problem, analyze each function, and reason about the correctness properties before writing specifications.

You need to write down your plan in the target directory.

Your target directory is `bitmap/bitmap_new`

You need to do as follows: 
1. remove specs on lib.rs (also remove proofs) based on `remove_specs.py`

[correctness]
2. generate correct test cases for this file (after removing specs)
3. execute and reflect on the test cases
3.1. when executing test cases, you need to copy it into `nanvix/src/libs/bitmap/`, include it into lib.rs, and then `cd nanvix && bash verify-bitmap.sh`
3.2. if one of these test cases fail, you need to determine whether the test cases is incorrect or the specs are incorrect.
3.3. if all test cases passed, you need to determine whether to add more complete test cases
4. Summary the results in the folder bitmap

[completeness]
2. generate incorrect test cases for this file (after removing specs) or inject errors to the preceding correct test cases
3. execute and reflect on the test cases
3.1. when executing test cases, you need to copy them one-by-one into `nanvix/src/libs/bitmap/`, include it into lib.rs, and then `cd nanvix && bash verify-bitmap.sh`
3.2. if one test case passed, you need to determine whether the test cases is correct or the specs are incomplete.
3.3. if one test case failed, you need to determine whether to add more incorrect test cases
4. Summary the results in the folder bitmap


[Note]
1. generate both parameterized and concrete tests. For concrete tests, you need to generate diverse test inputs to achieve a high code coverage.
2. For completeness, you need to iterately generate 5 rounds of incorrect test cases to evaluate the completeness of specs, you can refer to the specs to construct incomplete-trigger test cases.