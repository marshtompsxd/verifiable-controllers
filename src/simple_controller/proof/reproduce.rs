// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use crate::kubernetes_api_objects::{common::*, custom_resource::*, object::*};
use crate::kubernetes_cluster::{
    proof::{
        controller_runtime_liveness, controller_runtime_safety, kubernetes_api_liveness,
        kubernetes_api_safety,
    },
    spec::{
        controller::common::{controller_req_msg, ControllerActionInput},
        controller::controller_runtime::{continue_reconcile, end_reconcile},
        controller::state_machine::controller,
        distributed_system::*,
        message::*,
    },
};
use crate::pervasive::*;
use crate::pervasive::{option::*, result::*};
use crate::simple_controller::proof::safety;
use crate::simple_controller::proof::shared::*;
use crate::simple_controller::spec::{
    simple_reconciler,
    simple_reconciler::{simple_reconciler, SimpleReconcileState},
};
use crate::temporal_logic::{defs::*, rules::*};
use builtin::*;
use builtin_macros::*;

verus! {

proof fn lemma_after_create_cm_pc_leads_to_cm_always_exists(cr: CustomResourceView) // this proof will pass
    ensures
        sm_spec(simple_reconciler()).entails(
            lift_state(reconciler_at_after_create_cm_pc(cr))
                .leads_to(always(lift_state(cm_exists(cr))))
        ),
{
    assert forall |ex| #[trigger] sm_spec(simple_reconciler()).satisfied_by(ex) implies
    lift_state(reconciler_at_after_create_cm_pc(cr)).leads_to(lift_state(cm_exists(cr))).satisfied_by(ex) by {

        safety::lemma_always_reconcile_create_cm_done_implies_pending_create_cm_req_in_flight_or_cm_exists(cr);

        // The following assertion will fail if the steps after it are deleted
        assert(sm_spec(simple_reconciler()).implies(always(lift_state(safety::reconcile_create_cm_done_implies_pending_create_cm_req_in_flight_or_cm_exists(cr)))).satisfied_by(ex));

        // Comment everything in the assertion-forall block after this line
        assert forall |i| #[trigger] lift_state(reconciler_at_after_create_cm_pc(cr)).satisfied_by(ex.suffix(i))
        implies eventually(lift_state(cm_exists(cr))).satisfied_by(ex.suffix(i)) by {
            assert(lift_state(safety::reconcile_create_cm_done_implies_pending_create_cm_req_in_flight_or_cm_exists(cr)).satisfied_by(ex.suffix(i)));
            let s = ex.suffix(i).head();
            let req_msg = choose |m: Message| {
                #[trigger] is_controller_create_cm_request_msg(m, cr)
                && s.reconcile_state_of(cr.object_ref()).pending_req_msg == Option::Some(m)
                && (s.message_in_flight(m) || s.resource_key_exists(simple_reconciler::subresource_configmap(cr.object_ref()).object_ref()))
            };
            assert(is_controller_create_cm_request_msg(req_msg, cr)
                && s.reconcile_state_of(cr.object_ref()).pending_req_msg == Option::Some(req_msg)
                && (s.message_in_flight(req_msg) || s.resource_key_exists(simple_reconciler::subresource_configmap(cr.object_ref()).object_ref()))
            );

            if (s.resource_key_exists(simple_reconciler::subresource_configmap(cr.object_ref()).object_ref())) {
                assert(lift_state(cm_exists(cr)).satisfied_by(ex.suffix(i).suffix(0)));
            } else {
                let cm = KubernetesObject::ConfigMap(simple_reconciler::subresource_configmap(cr.object_ref()));
                let pre = |s: State<SimpleReconcileState>| {
                    &&& s.message_in_flight(req_msg) &&& req_msg.dst == HostId::KubernetesAPI &&& req_msg.content.is_create_request()
                    &&& req_msg.content.get_create_request().obj == cm
                };
                kubernetes_api_liveness::lemma_create_req_leads_to_res_exists::<SimpleReconcileState>(simple_reconciler(), req_msg, cm);
                instantiate_entailed_leads_to::<State<SimpleReconcileState>>(ex, i, sm_spec(simple_reconciler()), lift_state(pre), lift_state(cm_exists(cr)));
            }
        };
    };

    // And also comment the following line
    lemma_p_leads_to_cm_always_exists(cr, lift_state(reconciler_at_after_create_cm_pc(cr)));

    // Then Verus will report Line 40 and Line 45 assertion failure and the proof post-condition fails like the following:

    /*
        error: assertion failed
      --> simple_controller/proof/reproduce.rs:40:5
       |
    40 |     lift_state(reconciler_at_after_create_cm_pc(cr)).leads_to(lift_state(cm_exists(cr))).satisfied_by(ex) by {
       |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ assertion failed

    error: assertion failed
      --> simple_controller/proof/reproduce.rs:45:16
       |
    45 |         assert(sm_spec(simple_reconciler()).implies(always(lift_state(safety::reconcile_create_cm_done_implies_pending_create_cm_req_in_flight_or_cm_exists(cr)))).satisfied_by(ex));
       |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ assertion failed

    error: postcondition not satisfied
      --> simple_controller/proof/reproduce.rs:38:1
       |
    34 | /         sm_spec(simple_reconciler()).entails(
    35 | |             lift_state(reconciler_at_after_create_cm_pc(cr))
    36 | |                 .leads_to(always(lift_state(cm_exists(cr))))
    37 | |         ),
       | |_________- failed this postcondition
    38 | / {
    39 | |     assert forall |ex| #[trigger] sm_spec(simple_reconciler()).satisfied_by(ex) implies
    40 | |     lift_state(reconciler_at_after_create_cm_pc(cr)).leads_to(lift_state(cm_exists(cr))).satisfied_by(ex) by {
    41 | |
    ...  |
    80 | |     // but the assertion at 45 should be satisfied, and Verus should report the postcondition fails.
    81 | | }
       | |_^ at the end of the function body
     */

    // However, the assertion at 45 should be satisfied, otherwise there is no reason the original proof can pass
}

proof fn lemma_p_leads_to_cm_always_exists(cr: CustomResourceView, p: TempPred<State<SimpleReconcileState>>)
    requires
        sm_spec(simple_reconciler()).entails(
            p.leads_to(lift_state(cm_exists(cr)))
        ),
    ensures
        sm_spec(simple_reconciler()).entails(
            p.leads_to(always(lift_state(cm_exists(cr))))
        ),
{
    let next_and_invariant = |s: State<SimpleReconcileState>, s_prime: State<SimpleReconcileState>| {
        &&& next(simple_reconciler())(s, s_prime)
        &&& safety::delete_cm_req_msg_not_in_flight(cr)(s)
    };

    safety::lemma_delete_cm_req_msg_never_in_flight(cr);

    strengthen_next::<State<SimpleReconcileState>>(sm_spec(simple_reconciler()), next(simple_reconciler()), safety::delete_cm_req_msg_not_in_flight(cr), next_and_invariant);
    leads_to_stable_temp::<State<SimpleReconcileState>>(sm_spec(simple_reconciler()), lift_action(next_and_invariant), p, lift_state(cm_exists(cr)));
}

}
