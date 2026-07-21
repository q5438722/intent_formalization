Revise a proposed Verus postcondition after determinism-checking feedback.

Target: pervasive:unreached@190

Previous proposal:
```json
{
  "decision": "skip",
  "ensures": [],
  "useful": false,
  "rationale": "The `false` precondition makes every call unverifiable, so any postcondition would be vacuous and provide no useful information.",
  "risks": [
    "An arbitrary result property would misleadingly appear meaningful despite being justified only by the impossible precondition."
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
  "no_candidate_postcondition"
]
```

Relevant source:
```rust
 131: {
 132:     unimplemented!();
 133: }
 134: 
 135: #[cfg(verus_keep_ghost)]
 136: #[doc(hidden)]
 137: #[verifier::external_body]
 138: #[rustc_diagnostic_item = "verus::vstd::vstd::proof_nonstatic_call"]
 139: proof fn proof_nonstatic_call<Args: core::marker::Tuple, Output, F>(
 140:     tracked f: F,
 141:     tracked args: Args,
 142: ) -> (tracked output: Output) where F: FnOnce<Args, Output = Output>
 143:     requires
 144:         #![verifier::custom_err("Call to non-static function fails to satisfy `callee.requires(args)`")]
 145:         call_requires(f, args),
 146:     ensures
 147:         call_ensures(f, args, output),
 148: {
 149:     unimplemented!();
 150: }
 151: 
 152: /// A tool to check one's reasoning while writing complex spec functions.
 153: /// Not intended to be used as a mechanism for instantiating quantifiers, `spec_affirm` should
 154: /// be removed from spec functions once they are complete.
 155: ///
 156: /// ## Example
 157: ///
 158: /// ```rust
 159: /// #[spec(checked)] fn some_predicate(a: nat) -> bool {
 160: ///     recommends(a < 100);
 161: ///     if (a >= 50) {
 162: ///         let _ = spec_affirm(50 <= a && a < 100);
 163: ///         a >= 75
 164: ///     } else {
 165: ///         let _ = spec_affirm(a < 50);
 166: ///         // let _ = spec_affirm(a < 40); would raise a recommends note here
 167: ///         a < 25
 168: ///     }
 169: /// }
 170: /// ```
 171: pub closed spec fn spec_affirm(b: bool) -> bool
 172:     recommends
 173:         b,
 174: {
 175:     b
 176: }
 177: 
 178: /// In spec, all types are inhabited
 179: #[verifier::external_body]  /* vattr */
 180: #[allow(dead_code)]
 181: pub uninterp spec fn arbitrary<A>() -> A;
 182: 
 183: pub axiom fn proof_from_false<A>() -> (tracked a: A)
 184:     requires
 185:         false,
 186: ;
 187: 
 188: #[verifier::external_body]  /* vattr */
 189: #[allow(dead_code)]
 190: pub fn unreached<A>() -> A
 191:     requires
 192:         false,
 193: {
 194:     panic!("unreached_external")
 195: }
 196: 
 197: #[allow(unused_variables)]  // when built with cfg(not(feature = "std"))
 198: #[verifier::external_body]  /* vattr */
 199: pub fn print_u64(i: u64) {
 200:     println!("{}", i);
 201: }
 202: 
 203: #[verifier::external_body]
 204: pub fn runtime_assert(b: bool)
 205:     requires
 206:         b,
 207: {
 208:     runtime_assert_internal(b);
 209: }
 210: 
 211: } // verus!
 212: #[inline(always)]
 213: #[cfg_attr(verus_keep_ghost, verifier::external)]
 214: fn runtime_assert_internal(b: bool) {
 215:     assert!(b);
 216: }
 217: 
 218: /// Allows you to prove a boolean predicate by assuming its negation and proving
 219: /// a contradiction.
 220: ///
 221: /// `assert_by_contradiction!(b, { /* proof */ });`
 222: /// Equivalent to writing `if !b { /* proof */; assert(false); }`
 223: /// but is more concise and documents intent.
 224: ///
 225: /// ```rust
 226: /// assert_by_contradiction!(b, {
 227: ///     // assume !b here
 228: ///     // prove `false`
 229: /// });
 230: /// ```
 231: 
 232: #[macro_export]
 233: macro_rules! assert_by_contradiction {
 234:     ($($a:tt)*) => {
 235:         $crate::vstd::prelude::verus_proof_macro_exprs!($crate::assert_by_contradiction_internal!($($a)*))
 236:     }
 237: }
 238: 
 239: #[doc(hidden)]
 240: #[macro_export]
 241: macro_rules! assert_by_contradiction_internal {
 242:     ($predicate:expr, $bblock:block) => {
 243:         $crate::vstd::prelude::assert_by($predicate, {
 244:             if !$predicate {
 245:                 $bblock
 246:                 $crate::vstd::prelude::assert_(false);
 247:             }
 248:         });
 249:     };
 250: }
 251: 
 252: /// Macro to help set up boilerplate for specifying invariants when using
 253: /// invariant-based datatypes.
 254: ///
 255: /// This currently supports the `AtomicInvariant` and `LocalInvariant`
 256: /// types, as well as all the `atomic_ghost` types (e.g., `AtomicU64`, `AtomicBool`, and so on).
 257: /// It is important to first understand how these types work.
 258: /// In particular, `LocalInvariant` (for example) takes three type parameters,
 259: /// `K`, `V`, and `Pred: InvariantPredicate`.
 260: /// The `InvariantPredicate` trait lets the user specify an invariant at the static type
 261: /// level, while `K` allows the user to configure the invariant upon construction.
 262: /// `AtomicInvariant` uses the same system, and the `atomic_ghost` types are similar
 263: /// but use a different trait (`AtomicInvariantPredicate`).
 264: ///
 265: /// However, setting all this up in a typical application tends to involve a bit
 266: /// of boilerplate. That's where this macro comes in.
 267: ///
 268: /// # Usage
 269: ///
 270: /// The `struct_with_invariants!` macro is used at the item level, and it should contains
 271: /// a single struct declaration followed by a single declaration of a `spec` function
 272: /// returning `bool`. However, this spec function should not contain a boolean predicate
 273: /// as usual, but instead a series of _invariant declarations_.
 274: /// Each invariant declaration applies to a single field of the struct.
 275: ///
 276: /// ```rust
 277: /// struct_with_invariants!{
 278: ///     (pub)? struct $struct_name (<...>)? (where ...)? {
 279: ///         ( (pub)? $field_name: $type, )*
 280: ///     }
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
