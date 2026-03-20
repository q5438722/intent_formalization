You are an expert Verus programmer specializing in formal verification of Rust code.

Think step by step. Break down the problem, analyze each function, and reason about the correctness properties before writing specifications.

You need to write down your plan in the target directory.

Your target directory is `workspace`, and generate a result folder for the target file.

You need to do as follows: 
[correctness]
1. generate correct test cases for this file (after removing specs)
2. execute and reflect on the test cases
2.1. when executing test cases, you need to execute `verus /path/to/file.rs` and include the test case file into the target file.
2.2. if one of these test cases fail, you need to determine whether the test cases is incorrect or the specs are incorrect.
2.3. if all test cases passed, you need to determine whether to add more complete test cases
3. Summary the results in the folder result folder

[completeness]
1. generate incorrect test cases for this file (after removing specs) or inject errors to the preceding correct test cases
2. execute and reflect on the test cases
2.1. when executing test cases, you need to execute `verus /path/to/file.rs` and include the test case file into the target file.
2.2. if one test case passed, you need to determine whether the test cases is correct or the specs are incomplete.
2.3. if one test case failed, you need to determine whether to add more incorrect test cases
3. Summary the results in the folder result folder


[Note]
1. generate both parameterized and concrete tests. For concrete tests, you need to generate diverse test inputs to achieve a high code coverage.
2. For completeness, you need to iterately generate 5 rounds of incorrect test cases to evaluate the completeness of specs, you can refer to the specs to construct incomplete-trigger test cases.