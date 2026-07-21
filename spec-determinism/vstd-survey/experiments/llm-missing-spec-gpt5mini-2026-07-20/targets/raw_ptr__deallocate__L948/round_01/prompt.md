Revise a proposed Verus postcondition after determinism-checking feedback.

Target: raw_ptr:deallocate@948

Previous proposal:
```json
{
  "decision": "skip",
  "ensures": [],
  "useful": false,
  "rationale": "The function's safety is already captured by consuming the tracked PointsToRaw and Dealloc tokens in the signature. There is no additional, non\u2011vacuous pure/spec property expressible in the existing vstd public spec vocabulary that is both stronger and sound: any assertion about the absence of other tracked permissions, changing pointer provenance, or memory contents would either repeat the requires or be unsound given hidden/linear resources and intentional nondeterminism.",
  "risks": [
    "No extra postcondition means callers cannot rely on a pure-spec guarantee (e.g., memory zeroing or provenance mutation) beyond the linear-resource consumption enforced by the type system.",
    "Specifying an incorrect postcondition (e.g., about provenance or absence of any PointsToRaw) could be unsound because other tracked permissions or hidden state cannot be universally quantified over safely."
  ]
}
```

Checker result:
```json
{
  "status": "not_run"
}
```

Anti-vacuity/semantic issues:
```json
[
  "no_candidate_postcondition",
  "no_modeled_observable_output",
  "semantic_disposition:linear_resource_effect"
]
```

Relevant source:
```rust
 889: 
 890:     /// Alignment of the allocation you are allowed to deallocate.
 891:     #[verifier::inline]
 892:     pub open spec fn align(self) -> nat {
 893:         self.view().align
 894:     }
 895: 
 896:     /// Provenance of the allocation you are allowed to deallocate.
 897:     #[verifier::inline]
 898:     pub open spec fn provenance(self) -> Provenance {
 899:         self.view().provenance
 900:     }
 901: }
 902: 
 903: /// Allocate with the global allocator.
 904: /// The precondition should be consistent with the [documented safety conditions on `alloc`](https://doc.rust-lang.org/alloc/alloc/trait.GlobalAlloc.html#tymethod.alloc).
 905: /// Returns a pointer with a corresponding [`PointsToRaw`] and [`Dealloc`] permissions.
 906: #[cfg(feature = "std")]
 907: #[verifier::external_body]
 908: pub fn allocate(size: usize, align: usize) -> (pt: (
 909:     *mut u8,
 910:     Tracked<PointsToRaw>,
 911:     Tracked<Dealloc>,
 912: ))
 913:     requires
 914:         valid_layout(size, align),
 915:         size != 0,
 916:     ensures
 917:         pt.1@.is_range(pt.0.addr() as int, size as int),
 918:         pt.0.addr() + size <= usize::MAX + 1,
 919:         pt.2@@ == (DeallocData {
 920:             addr: pt.0.addr(),
 921:             size: size as nat,
 922:             align: align as nat,
 923:             provenance: pt.1@.provenance(),
 924:         }),
 925:         pt.0.addr() as int % align as int == 0,
 926:         pt.0@.provenance == pt.1@.provenance(),
 927:     opens_invariants none
 928: {
 929:     // SAFETY: valid_layout is a precondition
 930:     let layout = unsafe { alloc::alloc::Layout::from_size_align_unchecked(size, align) };
 931:     // SAFETY: size != 0
 932:     let p = unsafe { ::alloc::alloc::alloc(layout) };
 933:     if p == core::ptr::null_mut() {
 934:         std::process::abort();
 935:     }
 936:     (p, Tracked::assume_new(), Tracked::assume_new())
 937: }
 938: 
 939: /// Deallocate with the global allocator.
 940: ///
 941: /// The [`Dealloc`] permission ensures that the
 942: /// [documented safety conditions on `dealloc`](https://doc.rust-lang.org/1.82.0/core/alloc/trait.GlobalAlloc.html#tymethod.dealloc)
 943: /// are satisfied; by also giving up permission of the [`PointsToRaw`] permission,
 944: /// we ensure there can be no use-after-free bug as a result of this deallocation.
 945: /// In order to do so, the parameters of the [`PointsToRaw`] and [`Dealloc`] permissions must match the parameters of the deallocation.
 946: #[cfg(feature = "alloc")]
 947: #[verifier::external_body]
 948: pub fn deallocate(
 949:     p: *mut u8,
 950:     size: usize,
 951:     align: usize,
 952:     Tracked(pt): Tracked<PointsToRaw>,
 953:     Tracked(dealloc): Tracked<Dealloc>,
 954: )
 955:     requires
 956:         dealloc.addr() == p.addr(),
 957:         dealloc.size() == size,
 958:         dealloc.align() == align,
 959:         dealloc.provenance() == pt.provenance(),
 960:         pt.is_range(dealloc.addr() as int, dealloc.size() as int),
 961:         p@.provenance == dealloc.provenance(),
 962:     opens_invariants none
 963: {
 964:     // SAFETY: ensured by dealloc token
 965:     let layout = unsafe { alloc::alloc::Layout::from_size_align_unchecked(size, align) };
 966:     unsafe {
 967:         ::alloc::alloc::dealloc(p, layout);
 968:     }
 969: }
 970: 
 971: /// This is meant to be a replacement for `&'a T` that allows Verus to keep track of
 972: /// not just the `T` value but the pointer as well.
 973: /// It would be better to get rid of this and use normal reference types `&'a T`,
 974: /// but there are a lot of unsolved implementation questions.
 975: /// The existence of `SharedReference<'a, T>` is a stop-gap.
 976: #[verifier::external_body]
 977: #[verifier::accept_recursive_types(T)]
 978: pub struct SharedReference<'a, T>(&'a T);
 979: 
 980: impl<'a, T> Clone for SharedReference<'a, T> {
 981:     #[verifier::external_body]
 982:     fn clone(&self) -> (ret: Self)
 983:         ensures
 984:             ret == *self,
 985:     {
 986:         SharedReference(self.0)
 987:     }
 988: }
 989: 
 990: impl<'a, T> Copy for SharedReference<'a, T> {
 991: 
 992: }
 993: 
 994: impl<'a, T> SharedReference<'a, T> {
 995:     pub uninterp spec fn value(self) -> T;
 996: 
 997:     pub uninterp spec fn ptr(self) -> *const T;
 998: 
 999:     #[verifier::external_body]
1000:     fn new(t: &'a T) -> (s: Self)
1001:         ensures
1002:             s.value() == t,
1003:     {
1004:         SharedReference(t)
1005:     }
1006: 
1007:     #[verifier::external_body]
1008:     fn as_ref(self) -> (t: &'a T)
1009:         ensures
1010:             t == self.value(),
1011:     {
1012:         self.0
1013:     }
1014: 
1015:     #[verifier::external_body]
1016:     fn as_ptr(self) -> (ptr: *const T)
1017:         ensures
1018:             ptr == self.ptr(),
1019:     {
1020:         &*self.0
1021:     }
1022: 
1023:     pub axiom fn points_to(tracked self) -> (tracked pt: &'a PointsTo<T>)
1024:         ensures
1025:             pt.ptr() == self.ptr(),
1026:             pt.is_init(),
1027:             pt.value() == self.value(),
1028:     ;
1029: }
1030: 
1031: /// Like [`ptr_ref`] but returns a `SharedReference` so it keeps track of the relationship
1032: /// between the pointers.
1033: /// Note the resulting reference's pointers does NOT have the same provenance.
1034: /// This is because in Rust models like Stacked Borrows / Tree Borrows, the pointer
1035: /// gets a new tag.
1036: #[inline(always)]
1037: #[verifier::external_body]
1038: pub fn ptr_ref2<'a, T>(ptr: *const T, Tracked(perm): Tracked<&PointsTo<T>>) -> (v: SharedReference<
```

Return JSON only with the same schema:
{
  "decision": "add_spec" | "skip",
  "ensures": ["..."],
  "useful": true | false,
  "rationale": "...",
  "risks": ["..."]
}

Do not optimize for `R0 = unsat` by returning a redundant, vacuous, false-domain,
unit-output, or semantically unusable specification. Choose `skip` if no useful
postcondition can be expressed.
