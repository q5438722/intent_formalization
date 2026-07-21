Revise a proposed Verus postcondition after determinism-checking feedback.

Target: rwlock:release_read@474

Previous proposal:
```json
{
  "decision": "skip",
  "ensures": [],
  "useful": false,
  "rationale": "release_read consumes the tracked ReadHandle and performs an atomic, inherently concurrent token release. Any non\u2011vacuous postcondition would have to expose internal token/state (rc/inst) or quantify over concurrent interleavings. With the existing public vstd spec vocabulary and without adding new ghost resources or exposing internals, no sound, useful postcondition can be expressed.",
  "risks": [
    "Clients get no additional static guarantee from this function (only that the handle is consumed).",
    "If later verification needs stronger cross-thread invariants, the lock's internal spec must be made public or extra ghost tokens added."
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
 494:     }
 495: 
 496:     /// Indicates if the value `v` can be stored in the lock. Per the definition,
 497:     /// it depends on `[self.pred()]`, which is configured upon lock construction ([`RwLock::new`]).
 498:     pub open spec fn inv(&self, val: V) -> bool {
 499:         self.pred().inv(val)
 500:     }
 501: 
 502:     pub fn new(val: V, Ghost(pred): Ghost<Pred>) -> (s: Self)
 503:         requires
 504:             pred.inv(val),
 505:         ensures
 506:             s.pred() == pred,
 507:     {
 508:         let (cell, Tracked(perm)) = un::PCell::<V>::new(val);
 509: 
 510:         let tracked (Tracked(inst), Tracked(flag_exc), Tracked(flag_rc), _, _, _, _) =
 511:             RwLockToks::Instance::<
 512:             (Pred, CellId),
 513:             un::PointsTo<V>,
 514:             InternalPred<V, Pred>,
 515:         >::initialize_full((pred, cell.id()), perm, Option::Some(perm));
 516:         let inst = Tracked(inst);
 517: 
 518:         let exc = AtomicBool::new(Ghost(inst), false, Tracked(flag_exc));
 519:         let rc = AtomicUsize::new(Ghost(inst), 0, Tracked(flag_rc));
 520: 
 521:         RwLock { cell, exc, rc, inst, pred: Ghost(pred) }
 522:     }
 523: 
 524:     /// Acquires an exclusive write-lock. To release it, use [`WriteHandle::release_write`].
 525:     ///
 526:     /// **Warning:** The lock is _NOT_ released automatically when the handle
 527:     /// is dropped. You must call [`WriteHandle::release_write`].
 528:     /// Verus does not check that lock is released.
 529:     #[verifier::exec_allows_no_decreases_clause]
 530:     pub fn acquire_write(&self) -> (ret: (V, WriteHandle<'_, V, Pred>))
 531:         ensures
 532:             ({
 533:                 let val = ret.0;
 534:                 let write_handle = ret.1;
 535:                 &&& write_handle.rwlock() == *self
 536:                 &&& self.inv(val)
 537:             }),
 538:     {
 539:         proof {
 540:             use_type_invariant(self);
 541:         }
 542:         let mut done = false;
 543:         let tracked mut token: Option<
 544:             RwLockToks::pending_writer<(Pred, CellId), un::PointsTo<V>, InternalPred<V, Pred>>,
 545:         > = Option::None;
 546:         while !done
 547:             invariant
 548:                 done ==> token.is_some() && equal(token->0.instance_id(), self.inst@.id()),
 549:                 self.wf(),
 550:         {
 551:             let result =
 552:                 atomic_with_ghost!(
 553:                 &self.exc => compare_exchange(false, true);
 554:                 returning res;
 555:                 ghost g =>
 556:             {
 557:                 if res is Ok {
 558:                     token = Option::Some(self.inst.borrow().acquire_exc_start(&mut g));
 559:                 }
 560:             });
 561: 
 562:             done =
 563:             match result {
 564:                 Result::Ok(_) => true,
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
