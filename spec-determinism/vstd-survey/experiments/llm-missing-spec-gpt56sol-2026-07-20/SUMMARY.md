# LLM vstd missing-spec feedback experiment

- Model: `gpt-5.6-sol`
- Targets: 44
- Final add_spec decisions: 5
- Final skip decisions: 39
- Raw determinism reward: 0
- Guarded reward: 0
- LLM errors: 0

| Target | Decision | Ensures | Raw | Guarded | Issues |
|---|---|---|---:|---:|---|
| `cell::invcell:set@116` | skip | `` | 0 | 0 | no_candidate_postcondition, no_modeled_observable_output |
| `contrib::exec_spec::map:deep_clone@51` | add_spec | `res.len() <= self.len()` | 0 | 0 | already_specified_via_trait, checker_status:verus_error |
| `contrib::exec_spec::map:exec_eq@72` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::map:get_owned@36` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::map:get_ref@24` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::multiset:deep_clone@66` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::multiset:exec_eq@82` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::multiset:get_owned@52` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::multiset:get_ref@42` | add_spec | `res == self` | 0 | 0 | already_specified_via_trait, checker_status:verus_error |
| `contrib::exec_spec::option:deep_clone@24` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::option:exec_eq@36` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::option:get_owned@17` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::option:get_ref@10` | add_spec | `result == self` | 0 | 0 | already_specified_via_trait, checker_status:verus_error |
| `contrib::exec_spec::seq:deep_clone@33` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::seq:exec_eq@43` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::seq:exec_eq@55` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::seq:get_owned@24` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::seq:get_ref@15` | add_spec | `result@ == self@` | 0 | 0 | already_specified_via_trait, checker_status:verus_error |
| `contrib::exec_spec::set:deep_clone@40` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::set:exec_eq@56` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::set:get_owned@28` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::set:get_ref@18` | add_spec | `result == self` | 0 | 0 | already_specified_via_trait, checker_status:verus_error |
| `contrib::exec_spec::string:deep_clone@35` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::string:exec_eq@45` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::string:exec_eq@56` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::string:get_owned@28` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `contrib::exec_spec::string:get_ref@20` | skip | `` | 0 | 0 | already_specified_via_trait, no_candidate_postcondition |
| `invariant:create_open_invariant_credit@276` | skip | `` | 0 | 0 | no_candidate_postcondition, semantic_disposition:intentional_opaque_token |
| `invariant:open_atomic_invariant_begin@324` | skip | `` | 0 | 0 | no_candidate_postcondition, semantic_disposition:compiler_internal |
| `invariant:open_invariant_end@344` | skip | `` | 0 | 0 | no_candidate_postcondition, no_modeled_observable_output, semantic_disposition:compiler_internal |
| `invariant:open_local_invariant_begin@334` | skip | `` | 0 | 0 | no_candidate_postcondition, semantic_disposition:compiler_internal |
| `invariant:spend_open_invariant_credit@293` | skip | `` | 0 | 0 | no_candidate_postcondition, no_modeled_observable_output, semantic_disposition:linear_resource_effect |
| `pervasive:__call_panic@443` | skip | `` | 0 | 0 | no_candidate_postcondition, no_modeled_observable_output, semantic_disposition:diverges |
| `pervasive:__new_argument@454` | skip | `` | 0 | 0 | no_candidate_postcondition, semantic_disposition:runtime_effect_unmodeled |
| `pervasive:print_u64@199` | skip | `` | 0 | 0 | no_candidate_postcondition, no_modeled_observable_output, semantic_disposition:runtime_effect_unmodeled |
| `pervasive:runtime_assert@204` | skip | `` | 0 | 0 | no_candidate_postcondition, no_modeled_observable_output |
| `pervasive:unreached@190` | skip | `` | 0 | 0 | no_candidate_postcondition |
| `proph:new@179` | skip | `` | 0 | 0 | no_candidate_postcondition, semantic_disposition:intentional_nondeterminism |
| `raw_ptr:deallocate@948` | skip | `` | 0 | 0 | no_candidate_postcondition, no_modeled_observable_output, semantic_disposition:linear_resource_effect |
| `rwlock:release_read@474` | skip | `` | 0 | 0 | no_candidate_postcondition, no_modeled_observable_output, semantic_disposition:linear_resource_effect |
| `rwlock:release_write@403` | skip | `` | 0 | 0 | no_candidate_postcondition, no_modeled_observable_output, semantic_disposition:linear_resource_hidden_state |
| `simple_pptr:free@402` | skip | `` | 0 | 0 | no_candidate_postcondition, no_modeled_observable_output, semantic_disposition:linear_resource_effect |
| `thread:clone@183` | skip | `` | 0 | 0 | no_candidate_postcondition, tracked_value_exec_clone_is_uncallable |
| `thread:clone@188` | skip | `` | 0 | 0 | no_candidate_postcondition, tracked_value_exec_clone_is_uncallable |
