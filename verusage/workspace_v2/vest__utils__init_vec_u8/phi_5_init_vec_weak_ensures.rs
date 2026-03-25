use vstd::prelude::*;

fn main() {}

verus!{

// File: src/utils.rs
pub exec fn init_vec_u8(n: usize) -> (res: Vec<u8>)
    ensures
        res@.len() == n,
{
    let mut i: usize = 0;
    let mut ret: Vec<u8> = Vec::new();
    while i < n
        invariant
            0 <= i <= n,
            ret@.len() == i,
        decreases n - i,
    {
        ret.push(0);
        assert(ret@[i as int] == 0);
        i = i + 1
    }
    ret
}




// === Entailment query ===
exec fn phi_5_init_vec_weak_ensures()
{
    let v = init_vec_u8(5);
    assert(v@.len() == 5);
    // The spec is too weak: we know the implementation pushes 0,
    // but the ensures doesn't capture element values.
    // A conforming implementation could push any u8 value.
}

}
