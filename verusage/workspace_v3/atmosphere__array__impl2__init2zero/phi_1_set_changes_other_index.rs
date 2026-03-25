use vstd::prelude::*;

fn main() {}

verus!{

// File: array.rs
pub struct Array<A, const N: usize>{
    pub seq: Ghost<Seq<A>>,
    pub ar: [A;N]
}

impl<A, const N: usize> Array<A, N> {

    #[verifier(inline)]
    pub open spec fn view(&self) -> Seq<A>{
        self.seq@
    }

    pub open spec fn wf(&self) -> bool{
        self.seq@.len() == N
    }

}


impl<A, const N: usize> Array<A, N> {

    #[verifier(external_body)]
    pub fn set(&mut self, i: usize, out: A)
        requires
            0 <= i < N,
            old(self).wf(),
        ensures
            self.seq@ =~= old(self).seq@.update(i as int, out),
            self.wf(),
	{
		unimplemented!()
	}

}


impl<const N: usize> Array<u8, N> {

    pub fn init2zero(&mut self)
        requires
            old(self).wf(),
            N <= usize::MAX,
        ensures
            forall|index:int| 0<= index < N ==> #[trigger] self@[index] == 0,
            self.wf(),
    {
        let mut i = 0;
        for i in 0..N
            invariant
                N <= usize::MAX,
                0<=i<=N,
                self.wf(),
                forall|j:int| #![auto] 0<=j<i ==> self@[j] == 0,
        {
            let tmp:Ghost<Seq<u8>> = Ghost(self@);
            assert(forall|j:int| #![auto] 0<=j<i ==> self@[j] == 0);
            self.set(i,0);
            assert(self@ =~= tmp@.update(i as int,0));
            assert(forall|j:int| #![auto] 0<=j<i ==> self@[j] == 0);
        }
    }

}



// === Entailment query ===
proof fn phi_1_set_changes_other_index(old_seq: Seq<u8>, new_seq: Seq<u8>, i: int, j: int, out: u8, n: int)
    requires
        old_seq.len() == n,
        0 <= i < n,
        new_seq =~= old_seq.update(i, out),
        new_seq.len() == n,
        0 <= j < n,
        j != i,
    ensures
        new_seq[j] != old_seq[j],
{
}

}
