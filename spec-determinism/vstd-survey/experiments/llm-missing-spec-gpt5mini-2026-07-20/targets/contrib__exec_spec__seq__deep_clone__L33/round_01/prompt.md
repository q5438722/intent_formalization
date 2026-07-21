Revise a proposed Verus postcondition after determinism-checking feedback.

Target: contrib::exec_spec::seq:deep_clone@33

Previous proposal:
```json
{
  "decision": "add_spec",
  "ensures": [
    "res.deep_view() == self.deep_view()"
  ],
  "useful": true,
  "rationale": "Matches the DeepViewClone trait guarantee: the cloned Vec must have the same deep view as the original. This is the strongest, sound, and general specification expressible with existing vstd vocabulary (it implies per-element deep_view equality and length equality).",
  "risks": [
    "None substantive \u2014 relies on T: DeepViewClone to ensure per-element preservation of deep_view; implementation is external_body so the body must actually satisfy this contract.",
    "If T's deep_view has hidden/internal state or nondeterminism, the spec only constrains observable deep_view equality and not internal/runtime aliasing concerns."
  ]
}
```

Checker result:
```json
{
  "status": "verus_error",
  "requires": [],
  "ensures": [
    "res.deep_view() == self.deep_view()"
  ],
  "equal_fn_trivial": false,
  "stderr_tail": "error[E0405]: cannot find trait `DeepViewClone` in this scope\n  --> vstd-survey/experiments/llm-missing-spec-gpt5mini-2026-07-20/targets/contrib__exec_spec__seq__deep_clone__L33/round_00/artifacts/contrib__exec_spec__seq__deep_clone__L33/harness.rs:10:33\n   |\n10 | spec fn det_deep_clone_equal<T: DeepViewClone>(r1: Vec<T>, r2: Vec<T>) -> bool {\n   |                                 ^^^^^^^^^^^^^ not found in this scope\n   |\nhelp: consider importing this trait\n   |\n 3 + use vstd::contrib::exec_spec::DeepViewClone;\n   |\n\nerror[E0405]: cannot find trait `DeepViewClone` in this scope\n  --> vstd-survey/experiments/llm-missing-spec-gpt5mini-2026-07-20/targets/contrib__exec_spec__seq__deep_clone__L33/round_00/artifacts/contrib__exec_spec__seq__deep_clone__L33/harness.rs:14:28\n   |\n14 | proof fn det_deep_clone<T: DeepViewClone>(g_neq_tuple: bool, self_: &Vec<T>, r1: Vec<T>, r2: Vec<T>)\n   |                            ^^^^^^^^^^^^^ not found in this scope\n   |\nhelp: consider importing this trait\n   |\n 3 + use vstd::contrib::exec_spec::DeepViewClone;\n   |\n\nerror[E0425]: cannot find value `res` in this scope\n  --> vstd-survey/experiments/llm-missing-spec-gpt5mini-2026-07-20/targets/contrib__exec_spec__seq__deep_clone__L33/round_00/artifacts/contrib__exec_spec__seq__deep_clone__L33/harness.rs:17:18\n   |\n17 |             &&& (res.deep_view() == self_.deep_view())\n   |                  ^^^ not found in this scope\n\nerror[E0425]: cannot find value `res` in this scope\n  --> vstd-survey/experiments/llm-missing-spec-gpt5mini-2026-07-20/targets/contrib__exec_spec__seq__deep_clone__L33/round_00/artifacts/contrib__exec_spec__seq__deep_clone__L33/harness.rs:18:18\n   |\n18 |             &&& (res.deep_view() == self_.deep_view())\n   |                  ^^^ not found in this scope\n\nerror[E0603]: module `seq` is private\n --> vstd-survey/experiments/llm-missing-spec-gpt5mini-2026-07-20/targets/contrib__exec_spec__seq__deep_clone__L33/round_00/artifacts/contrib__exec_spec__seq__deep_clone__L33/harness.rs:4:31\n  |\n4 | use vstd::contrib::exec_spec::seq::*;\n  |                               ^^^ private module\n  |\nnote: the module `seq` is defined here\n --> vstd/contrib/exec_spec/mod.rs:19:0\n\nerror: aborting due to 5 previous errors\n\nSome errors have detailed explanations: E0405, E0425, E0603.\nFor more information about an error, try `rustc --explain E0405`.\n"
}
```

Anti-vacuity/semantic issues:
```json
[
  "already_specified_via_trait",
  "checker_status:verus_error"
]
```

Relevant source:
```rust
   1: //! This module contains [`Seq`]-specific method implementations.
   2: use crate::contrib::exec_spec::*;
   3: use crate::prelude::*;
   4: 
   5: verus! {
   6: 
   7: // Note: the exec translations which use iterators are unverified.
   8: broadcast use crate::group_vstd_default;
   9: 
  10: /// Impls for shared traits
  11: /// NOTE: can't implement [`ExecSpecType`] for [`Seq<T>`]
  12: /// since it conflicts with [`SpecString`] (i.e., [`Seq<char>`]).
  13: impl<'a, T: DeepView> ToRef<&'a [T]> for &'a Vec<T> {
  14:     #[inline(always)]
  15:     fn get_ref(self) -> &'a [T] {
  16:         self.as_slice()
  17:     }
  18: }
  19: 
  20: impl<'a, T: DeepView + DeepViewClone> ToOwned<Vec<T>> for &'a [T] {
  21:     /// TODO: verify this
  22:     #[verifier::external_body]
  23:     #[inline(always)]
  24:     fn get_owned(self) -> Vec<T> {
  25:         self.iter().map(|x| x.deep_clone()).collect()
  26:     }
  27: }
  28: 
  29: impl<T: DeepViewClone> DeepViewClone for Vec<T> {
  30:     /// TODO: verify this
  31:     #[verifier::external_body]
  32:     #[inline(always)]
  33:     fn deep_clone(&self) -> Self {
  34:         self.iter().map(|x| x.deep_clone()).collect()
  35:     }
  36: }
  37: 
  38: impl<'a, T: DeepView> ExecSpecEq<'a> for &'a [T] where &'a T: ExecSpecEq<'a, Other = &'a T> {
  39:     type Other = &'a [T];
  40: 
  41:     #[verifier::external_body]
  42:     #[inline(always)]
  43:     fn exec_eq(this: Self, other: Self::Other) -> bool {
  44:         this.len() == other.len() && this.iter().zip(other.iter()).all(
  45:             |(a, b)| <&'a T>::exec_eq(a, b),
  46:         )
  47:     }
  48: }
  49: 
  50: impl<'a, T: DeepView> ExecSpecEq<'a> for &'a Vec<T> where &'a T: ExecSpecEq<'a, Other = &'a T> {
  51:     type Other = &'a Vec<T>;
  52: 
  53:     #[verifier::external_body]
  54:     #[inline(always)]
  55:     fn exec_eq(this: Self, other: Self::Other) -> bool {
  56:         this.len() == other.len() && this.iter().zip(other.iter()).all(
  57:             |(a, b)| <&'a T>::exec_eq(a, b),
  58:         )
  59:     }
  60: }
  61: 
  62: impl<'a, T: DeepView> ExecSpecLen for &'a [T] {
  63:     #[inline(always)]
  64:     fn exec_len(self) -> (res: usize)
  65:         ensures
  66:             res == self.deep_view().len(),
  67:     {
  68:         self.len()
  69:     }
  70: }
  71: 
  72: impl<'a, T: DeepView> ExecSpecIndex<'a> for &'a [T] {
  73:     type Elem = &'a T;
  74: 
  75:     #[inline(always)]
  76:     fn exec_index(self, index: usize) -> (res: Self::Elem)
  77:         ensures
  78:             res.deep_view() == self.deep_view()[index as int],
  79:     {
  80:         self.get(index).unwrap()
  81:     }
  82: }
  83: 
  84: //
  85: // Trait definitions for methods
  86: //
  87: /// Spec for executable version of [`Seq::add`].
  88: pub trait ExecSpecSeqAdd<'a, Out: Sized + DeepView>: Sized + DeepView + ToOwned<Out> {
  89:     fn exec_add(self, rhs: Self) -> Out;
  90: }
  91: 
  92: /// Spec for executable version of [`Seq::push`].
  93: pub trait ExecSpecSeqPush<'a, Out: Sized + DeepView>: Sized + DeepView + ToOwned<Out> {
  94:     type Elem: DeepView + DeepViewClone;
  95: 
  96:     fn exec_push(self, a: Self::Elem) -> Out;
  97: }
  98: 
  99: /// Spec for executable version of [`Seq::update`].
 100: pub trait ExecSpecSeqUpdate<'a, Out: Sized + DeepView>: Sized + DeepView + ToOwned<Out> {
 101:     type Elem: DeepView + DeepViewClone;
 102: 
 103:     fn exec_update(self, i: usize, a: Self::Elem) -> Out;
 104: }
 105: 
 106: /// Spec for executable version of [`Seq::subrange`].
 107: pub trait ExecSpecSeqSubrange<'a>: Sized + DeepView<V = Seq<<Self::Elem as DeepView>::V>> {
 108:     type Elem: DeepView;
 109: 
 110:     fn exec_subrange(self, start_inclusive: usize, end_exclusive: usize) -> Self
 111:         requires
 112:             0 <= start_inclusive <= end_exclusive <= self.deep_view().len(),
 113:     ;
 114: }
 115: 
 116: /// Spec for executable version of [`Seq::empty`].
 117: pub trait ExecSpecSeqEmpty: Sized {
 118:     fn exec_empty() -> Self;
 119: }
 120: 
 121: /// Spec for executable version of [`Seq::to_multiset`].
 122: pub trait ExecSpecSeqToMultiset<'a>: Sized {
 123:     type Elem: DeepView + DeepViewClone + std::hash::Hash + std::cmp::Eq;

// Shared trait declarations:
T001: //! This module provides runtime utilities for the compiled
T002: //! executable code of [`verus_builtin_macros::exec_spec_verified`]
T003: //! and [`verus_builtin_macros::exec_spec_unverified`].
T004: #![cfg(all(feature = "alloc", feature = "std"))]
T005: 
T006: use crate::multiset::*;
T007: use crate::prelude::*;
T008: use std::collections::HashMap;
T009: use std::collections::HashSet;
T010: pub use verus_builtin_macros::exec_spec_unverified;
T011: pub use verus_builtin_macros::exec_spec_verified;
T012: 
T013: mod map;
T014: pub use map::*;
T015: mod multiset;
T016: pub use multiset::*;
T017: mod option;
T018: pub use option::*;
T019: mod seq;
T020: pub use seq::*;
T021: mod set;
T022: pub use set::*;
T023: mod string;
T024: pub use string::*;
T025: 
T026: verus! {
T027: 
T028: /// [`ToRef`] and [`ToOwned`] are almost the same trait
T029: /// but separated to avoid type inference ambiguities.
T030: pub trait ToRef<T: Sized + DeepView>: Sized + DeepView<V = T::V> {
T031:     fn get_ref(self) -> (res: T)
T032:         ensures
T033:             res.deep_view() == self.deep_view(),
T034:     ;
T035: }
T036: 
T037: pub trait ToOwned<T: Sized + DeepView>: Sized + DeepView<V = T::V> {
T038:     fn get_owned(self) -> (res: T)
T039:         ensures
T040:             res.deep_view() == self.deep_view(),
T041:     ;
T042: }
T043: 
T044: /// Cloned objects have the same deep view
T045: pub trait DeepViewClone: Sized + DeepView {
T046:     fn deep_clone(&self) -> (res: Self)
T047:         ensures
T048:             res.deep_view() == self.deep_view(),
T049:     ;
T050: }
T051: 
T052: /// Any spec types used in [`exec_spec_verified`] or [`exec_spec_unverified`] macros
T053: /// must implement this trait to indicate
T054: /// the corresponding exec type (owned and borrowed versions).
T055: pub trait ExecSpecType where
T056:     for <'a>&'a Self::ExecOwnedType: ToRef<Self::ExecRefType<'a>>,
T057:     for <'a>Self::ExecRefType<'a>: ToOwned<Self::ExecOwnedType>,
T058:  {
T059:     /// Owned version of the exec type.
T060:     type ExecOwnedType: DeepView<V = Self>;
T061: 
T062:     /// Reference version of the exec type.
T063:     type ExecRefType<'a>: DeepView<V = Self>;
T064: }
T065: 
T066: /// Spec for the executable version of equality.
T067: pub trait ExecSpecEq<'a>: DeepView + Sized {
T068:     type Other: DeepView<V = Self::V>;
T069: 
T070:     fn exec_eq(this: Self, other: Self::Other) -> (res: bool)
T071:         ensures
T072:             res == (this.deep_view() =~~= other.deep_view()),
T073:     ;
T074: }
T075: 
T076: /// Spec for executable version of [`Seq`] and [`str`] indexing.
T077: pub trait ExecSpecIndex<'a>: Sized + DeepView<V = Seq<<Self::Elem as DeepView>::V>> {
T078:     type Elem: DeepView;
T079: 
T080:     fn exec_index(self, index: usize) -> Self::Elem
T081:         requires
T082:             0 <= index < self.deep_view().len(),
T083:     ;
T084: }
T085: 
T086: /// Spec for executable version of `len`.
T087: pub trait ExecSpecLen {
T088:     fn exec_len(self) -> usize;
T089: }
T090: 
T091: /// A macro to implement various traits for primitive arithmetic types.
T092: macro_rules! impl_primitives {
T093:     ($(,)?) => {};
T094:     ($t:ty $(,$rest:ty)* $(,)?) => {
T095:         verus! {
T096:             impl ExecSpecType for $t {
T097:                 type ExecOwnedType = $t;
T098:                 type ExecRefType<'a> = $t;
T099:             }
T100: 
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
