Compare the specifications in bitmap_raw and bitmap_new.
You need to sperately compare the specfications in lib.spec.rs and lib.rs.
Then, write the comparison results in this folder.


based on set_theoretic_analysis.md, generate a powerpoint slides
use a light theme for this slide

add one outline slide:
    Improve the correctness and completeness of specifications via test generation
    Plans for VeruSage and Nanvix
    Step 1: Build test execution environment 
    Step 2: Generate doc string & documents from implementation
    Step 3: Generate specs and tests; then use tests to evaluate specs
    Optional: Rust library specification inference for Verus​
    Learn the abstract level during spec generation

Then add one slide about "Why LLM-generated tests can improve specs?"

* We can leverage different LLMs to generate specs and tests
1 If they are inconsistency, then at least one of them are incorrect (DeepTest)
2 We can adopt majority voting among different LLMs/translate the generated tests back to NL (and compared to the original NL description)

add one slides of challenges/plans in Learning the abstract level during spec generation:
 Challenge 1: the abstract level are highly domain-specific.
 Plan 1: can we start from bottom to up, i.e., generate a low-level spec (bitmap-raw), then iterate with LLMs to conduct abstraction
 Challenge 2: the abstraction are difficult to evalaute
 Plan 2: can we start from easy-to-evaluate module, e.g., data structure/algorithms (https://risemsr.github.io/blog/2026-03-06-autoclrs/)

