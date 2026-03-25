use vstd::prelude::*;

fn main() {}

verus!{

// File: spec_t/mmu/pt_mem.rs
pub struct PTMem {
    pub mem: Map<usize, usize>,
    pub pml4: usize,
}

impl PTMem {

    pub open spec fn write(self, addr: usize, value: usize) -> PTMem {
        PTMem {
            mem: self.mem.insert(addr, value),
            pml4: self.pml4,
        }
    }

    pub open spec fn read(self, addr: usize) -> usize {
        self.mem[addr]
    }

    pub open spec fn write_seq(self, writes: Seq<(usize, usize)>) -> Self {
        writes.fold_left(self, |acc: PTMem, wr: (_, _)| acc.write(wr.0, wr.1))
    }

    pub broadcast proof fn lemma_write_seq_idle(self, writes: Seq<(usize, usize)>, addr: usize)
        requires forall|i| 0 <= i < writes.len() ==> (#[trigger] writes[i]).0 != addr
        ensures #[trigger] self.write_seq(writes).read(addr) == self.read(addr)
        decreases writes.len()
    {
        if writes.len() == 0 {
        } else {
            broadcast use PTMem::lemma_write_seq;
            self.lemma_write_seq_idle(writes.drop_last(), addr)
        }
    }

	#[verifier::external_body]
    pub broadcast proof fn lemma_write_seq(self, writes: Seq<(usize, usize)>)
        ensures #![trigger self.write_seq(writes)]
            self.write_seq(writes).pml4 == self.pml4,
            self.mem.dom().subset_of(self.write_seq(writes).mem.dom()),
        decreases writes.len()
	{
		unimplemented!()
	}

}





// === Entailment query ===
proof fn phi_1_lemma_write_seq_external_body(pt: PTMem, writes: Seq<(usize, usize)>)
    requires
        true,
    ensures
        pt.write_seq(writes).pml4 == pt.pml4,
        pt.mem.dom().subset_of(pt.write_seq(writes).mem.dom()),
{
    pt.lemma_write_seq(writes);
}

}
