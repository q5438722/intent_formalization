Revise a proposed Verus postcondition after determinism-checking feedback.

Target: rwlock:release_write@403

Previous proposal:
```json
{
  "decision": "skip",
  "ensures": [],
  "useful": false,
  "rationale": "No safe, non\u2011vacuous postcondition can be expressed using only the visible public vstd spec vocabulary in this snippet. The implementation updates runtime state (cell.put and clearing the exclusive flag) and performs ghost updates via instance methods, but the precise ghost accessors and public spec functions needed to state these facts (e.g., a canonical accessor for the lock's exclusive flag or cell value on RwLock) are not visible. Proposing any concrete ensures would risk being unsound or not expressible with the shown public API.",
  "risks": [
    "Stating a concrete postcondition (e.g., that the lock's exclusive flag is false or that the cell value equals new_val) might rely on non-public/unknown spec accessors and therefore be unsound.",
    "A too-weak postcondition (repeating the requires) would be unhelpful; a too-strong one might be unverifiable given trait/ghost details not shown."
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
  "semantic_disposition:linear_resource_hidden_state"
]
```

Relevant source:
```rust
 344:         pred: Ghost<Pred>,
 345:     }
 346: 
 347:     #[verifier::type_invariant]
 348:     spec fn wf(&self) -> bool {
 349:         invariant on exc with (inst) is (v: bool, g: RwLockToks::flag_exc<(Pred, CellId), un::PointsTo<V>, InternalPred<V, Pred>>) {
 350:             g.instance_id() == inst@.id()
 351:                 && g.value() == v
 352:         }
 353: 
 354:         invariant on rc with (inst) is (v: usize, g: RwLockToks::flag_rc<(Pred, CellId), un::PointsTo<V>, InternalPred<V, Pred>>) {
 355:             g.instance_id() == inst@.id()
 356:                 && g.value() == v
 357:         }
 358: 
 359:         predicate {
 360:             self.inst@.k() == (self.pred@, self.cell.id())
 361:         }
 362:     }
 363: }
 364: 
 365: /// Handle obtained for an exclusive write-lock from an [`RwLock`].
 366: ///
 367: /// Note that this handle does not contain a reference to the lock-protected object;
 368: /// ownership of the object is obtained separately from [`RwLock::acquire_write`].
 369: /// This may be changed in the future.
 370: ///
 371: /// **Warning:** The lock is _NOT_ released automatically when the handle
 372: /// is dropped. You must call [`release_write`](WriteHandle::release_write).
 373: /// Verus does not check that lock is released.
 374: pub struct WriteHandle<'a, V, Pred: RwLockPredicate<V>> {
 375:     handle: Tracked<RwLockToks::writer<(Pred, CellId), un::PointsTo<V>, InternalPred<V, Pred>>>,
 376:     perm: Tracked<un::PointsTo<V>>,
 377:     rwlock: &'a RwLock<V, Pred>,
 378: }
 379: 
 380: /// Handle obtained for a shared read-lock from an [`RwLock`].
 381: ///
 382: /// **Warning:** The lock is _NOT_ released automatically when the handle
 383: /// is dropped. You must call [`release_read`](ReadHandle::release_read).
 384: /// Verus does not check that lock is released.
 385: pub struct ReadHandle<'a, V, Pred: RwLockPredicate<V>> {
 386:     handle: Tracked<RwLockToks::reader<(Pred, CellId), un::PointsTo<V>, InternalPred<V, Pred>>>,
 387:     rwlock: &'a RwLock<V, Pred>,
 388: }
 389: 
 390: impl<'a, V, Pred: RwLockPredicate<V>> WriteHandle<'a, V, Pred> {
 391:     #[verifier::type_invariant]
 392:     spec fn wf_write_handle(self) -> bool {
 393:         equal(self.perm@.id(), self.rwlock.cell.id()) && self.perm@.is_uninit() && equal(
 394:             self.handle@.instance_id(),
 395:             self.rwlock.inst@.id(),
 396:         ) && self.rwlock.wf()
 397:     }
 398: 
 399:     pub closed spec fn rwlock(self) -> RwLock<V, Pred> {
 400:         *self.rwlock
 401:     }
 402: 
 403:     pub fn release_write(self, new_val: V)
 404:         requires
 405:             self.rwlock().inv(new_val),
 406:     {
 407:         proof {
 408:             use_type_invariant(&self);
 409:         }
 410:         let WriteHandle { handle: Tracked(handle), perm: Tracked(mut perm), rwlock } = self;
 411:         self.rwlock.cell.put(Tracked(&mut perm), new_val);
 412: 
 413:         atomic_with_ghost!(
 414:             &rwlock.exc => store(false);
 415:             ghost g =>
 416:         {
 417:             self.rwlock.inst.borrow().release_exc(perm, &mut g, perm, handle);
 418:         });
 419:     }
 420: }
 421: 
 422: impl<'a, V, Pred: RwLockPredicate<V>> ReadHandle<'a, V, Pred> {
 423:     #[verifier::type_invariant]
 424:     spec fn wf_read_handle(self) -> bool {
 425:         equal(self.handle@.instance_id(), self.rwlock.inst@.id())
 426:             && self.handle@.element().is_init() && equal(
 427:             self.handle@.element().id(),
 428:             self.rwlock.cell.id(),
 429:         ) && self.rwlock.wf()
 430:     }
 431: 
 432:     pub closed spec fn view(self) -> V {
 433:         self.handle@.element().value()
 434:     }
 435: 
 436:     pub closed spec fn rwlock(self) -> RwLock<V, Pred> {
 437:         *self.rwlock
 438:     }
 439: 
 440:     /// Obtain a shared reference to the object contained in the lock.
 441:     pub fn borrow<'b>(&'b self) -> (val: &'b V)
 442:         ensures
 443:             val == self.view(),
 444:     {
 445:         proof {
 446:             use_type_invariant(self);
 447:         }
 448:         let tracked perm = self.rwlock.inst.borrow().read_guard(
 449:             self.handle@.element(),
 450:             self.handle.borrow(),
 451:         );
 452:         self.rwlock.cell.borrow(Tracked(&perm))
 453:     }
 454: 
 455:     pub proof fn lemma_readers_match(
 456:         tracked read_handle1: &ReadHandle<V, Pred>,
 457:         tracked read_handle2: &ReadHandle<V, Pred>,
 458:     )
 459:         requires
 460:             read_handle1.rwlock() == read_handle2.rwlock(),
 461:         ensures
 462:             (equal(read_handle1.view(), read_handle2.view())),
 463:     {
 464:         use_type_invariant(read_handle1);
 465:         use_type_invariant(read_handle2);
 466:         read_handle1.rwlock.inst.borrow().read_match(
 467:             read_handle1.handle@.element(),
 468:             read_handle2.handle@.element(),
 469:             &read_handle1.handle.borrow(),
 470:             &read_handle2.handle.borrow(),
 471:         );
 472:     }
 473: 
 474:     pub fn release_read(self) {
 475:         proof {
 476:             use_type_invariant(&self);
 477:         }
 478:         let ReadHandle { handle: Tracked(handle), rwlock } = self;
 479: 
 480:         let _ =
 481:             atomic_with_ghost!(
 482:             &rwlock.rc => fetch_sub(1);
 483:             ghost g =>
 484:         {
 485:             rwlock.inst.borrow().release_shared(handle.element(), &mut g, handle);
 486:         });
 487:     }
 488: }
 489: 
 490: impl<V, Pred: RwLockPredicate<V>> RwLock<V, Pred> {
 491:     /// Predicate configured for this lock instance.
 492:     pub closed spec fn pred(&self) -> Pred {
 493:         self.pred@
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
