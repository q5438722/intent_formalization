// GAP01: new(8) should succeed for valid input. Tests liveness guarantee of new().
// The spec uses `==>` (implication) not `&&`, so this may FAIL if spec is incomplete.
verus! {
fn test_gap01_new_liveness() {
    let result = Bitmap::new(8);
    // CORRECT: 8 > 0, 8 < MAX, 8 % 8 == 0, so new(8) should succeed.
    assert(result is Ok);
}
} // verus!
