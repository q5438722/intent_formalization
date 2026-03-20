// ERR09: After alloc on a 64-bit bitmap, claim num_bits changed.
verus! {
fn test_err09_num_bits_changes() {
    let result = Bitmap::new(64);
    if let Ok(mut bitmap) = result {
        let a = bitmap.alloc();
        if let Ok(_) = a {
            // WRONG: num_bits should remain 64.
            assert(bitmap@.num_bits != 64);
        }
    }
}
} // verus!
