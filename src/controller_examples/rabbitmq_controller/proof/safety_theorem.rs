// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use crate::kubernetes_api_objects::prelude::*;
use crate::kubernetes_cluster::spec::{cluster::*, cluster_state_machine::Step, message::*};
use crate::rabbitmq_controller::{
    common::*,
    spec::{reconciler::*, resource::*, types::*},
};
use crate::temporal_logic::{defs::*, rules::*};
use crate::vstd_ext::string_view::int_to_string_view;
use vstd::prelude::*;

verus! {

pub open spec fn safety_theorem() -> bool {
    forall |rabbitmq: RabbitmqClusterView| #[trigger] cluster_spec_without_wf().entails(safety(rabbitmq))
}

pub open spec fn cluster_spec_without_wf() -> TempPred<RMQCluster> {
    lift_state(RMQCluster::init()).and(always(lift_action(RMQCluster::next())))
}

pub open spec fn safety(rabbitmq: RabbitmqClusterView) -> TempPred<RMQCluster> {
    always(lift_action(stateful_set_not_scaled_down(rabbitmq)))
}

/// To prove the safety property about stateful set, we need to first specify what the property is.
///
/// Previously, we planned to use Message to describe the possible update/deletion/creation actions, and also specify the
/// relevant properties. However, it is better not to include Message in the description the high-level safety property
/// because Message is just a tool and a detail of the system. For update action, one way to circumvent using Message is
/// to talk about the previous and current state: an object being updated means that it exists in both states but changes
/// in current state.
pub open spec fn stateful_set_not_scaled_down(rabbitmq: RabbitmqClusterView) -> ActionPred<RMQCluster> {
    |s: RMQCluster, s_prime: RMQCluster| {
        let sts_key = make_stateful_set_key(rabbitmq);
        s.resources().contains_key(sts_key)
        && s_prime.resources().contains_key(sts_key)
        ==> replicas_of_stateful_set(s_prime.resources()[sts_key]) >= replicas_of_stateful_set(s.resources()[sts_key])
    }
}

pub open spec fn replicas_of_stateful_set(obj: DynamicObjectView) -> int
    recommends
        obj.kind.is_StatefulSetKind(),
{
    StatefulSetView::unmarshal(obj).get_Ok_0().spec.get_Some_0().replicas.get_Some_0()
}

}
