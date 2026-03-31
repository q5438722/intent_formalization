// Test: assume-in-body entailment structure

verus! {

// φ1: new — valid input but Err (liveness gap)
proof fn phi_1_new_no_liveness(number_of_bits: usize, result: Result<Bitmap, Error>)
    requires
        // Spec requires: (new has no requires, but we add valid input as precondition)
        number_of_bits > 0,
        number_of_bits < u32::MAX as usize,
        number_of_bits % (u8::BITS as usize) == 0,
    ensures
        // Exact copy of spec ensures
        result matches Ok(bitmap) ==> {
            &&& bitmap.inv()
            &&& bitmap@.num_bits == number_of_bits as int
            &&& bitmap@.is_empty()
        },
        number_of_bits == 0 ==> result is Err,
        number_of_bits >= u32::MAX as usize ==> result is Err,
        number_of_bits % (u8::BITS as usize) != 0 ==> result is Err,
{
    // Bad scenario
    assume(result is Err);
}

// φ3: set Err + state mutated (frame condition)
proof fn phi_3_set_err_mutates(
    pre: Bitmap, post: Bitmap, index: usize, result: Result<(), Error>
)
    requires
        // Spec requires
        pre.inv(),
    ensures
        // Exact copy of spec ensures
        post.inv(),
        match result {
            Ok(()) => {
                &&& index < post@.num_bits
                &&& post@.is_bit_set(index as int)
                &&& !pre@.is_bit_set(index as int)
                &&& post@.num_bits == pre@.num_bits
                &&& forall|i: int|
                    0 <= i < post@.num_bits && i != (index as int) ==> post@.is_bit_set(i)
                        == pre@.is_bit_set(i)
                &&& post@.set_bits == pre@.set_bits.insert(index as int)
                &&& post@.usage() == pre@.usage() + 1
            },
            Err(_) => {
                &&& index >= pre@.num_bits || pre@.is_bit_set(index as int)
                &&& post == pre
            },
        },
{
    // Bad scenario: Err but state changed
    assume(result is Err);
    assume(post@ != pre@);
}

// φ5: alloc always returns 0 (fairness)
proof fn phi_5_alloc_always_zero(
    pre: Bitmap, post: Bitmap, result: Result<usize, Error>
)
    requires
        // Spec requires
        pre.inv(),
    ensures
        // Exact copy of spec ensures
        post.inv(),
        match result {
            Ok(index) => {
                &&& 0 <= index < post@.num_bits
                &&& post@.num_bits == pre@.num_bits
                &&& !pre@.is_bit_set(index as int)
                &&& post@.is_bit_set(index as int)
                &&& forall|i: int|
                    0 <= i < post@.num_bits && i != index ==> post@.is_bit_set(i) == pre@.is_bit_set(i)
                &&& post@.set_bits == pre@.set_bits.insert(index as int)
                &&& post@.usage() == pre@.usage() + 1
            },
            Err(_) => {
                &&& pre@.is_full()
                &&& post@ == pre@
            },
        },
{
    // Bad scenario: always returns 0 when bits 0 and 1 are both free
    assume(pre@.num_bits > 1);
    assume(!pre@.is_bit_set(0));
    assume(!pre@.is_bit_set(1));
    assume(result == Ok::<usize, Error>(0usize));
    assume(post@.is_bit_set(0));
    assume(post@.num_bits == pre@.num_bits);
    assume(forall|i: int| 0 < i < post@.num_bits ==> post@.is_bit_set(i) == pre@.is_bit_set(i));
    assume(post@.set_bits == pre@.set_bits.insert(0int));
    assume(post@.usage() == pre@.usage() + 1);
}

// φ6: alloc Err on non-full (liveness)
proof fn phi_6_alloc_err_nonfull(
    pre: Bitmap, post: Bitmap, result: Result<usize, Error>
)
    requires
        pre.inv(),
    ensures
        post.inv(),
        match result {
            Ok(index) => {
                &&& 0 <= index < post@.num_bits
                &&& post@.num_bits == pre@.num_bits
                &&& !pre@.is_bit_set(index as int)
                &&& post@.is_bit_set(index as int)
                &&& forall|i: int|
                    0 <= i < post@.num_bits && i != index ==> post@.is_bit_set(i) == pre@.is_bit_set(i)
                &&& post@.set_bits == pre@.set_bits.insert(index as int)
                &&& post@.usage() == pre@.usage() + 1
            },
            Err(_) => {
                &&& pre@.is_full()
                &&& post@ == pre@
            },
        },
{
    // Bad scenario: not full but Err
    assume(!pre@.is_full());
    assume(result is Err);
    assume(post@ == pre@);
}

} // verus!
