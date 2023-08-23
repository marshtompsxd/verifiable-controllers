// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use super::types::*;
use crate::external_api::spec::*;
use crate::kubernetes_api_objects::resource::*;
use crate::kubernetes_cluster::spec::{cluster::Cluster, message::*};
use crate::reconciler::spec::reconciler::Reconciler;
use crate::state_machine::action::*;
use crate::state_machine::state_machine::*;
use crate::temporal_logic::defs::*;
use vstd::{multiset::*, prelude::*};

verus! {

impl <K: ResourceView, E: ExternalAPI, R: Reconciler<K, E>> Cluster<K, E, R> {

pub open spec fn deliver() -> Action<NetworkState<E>, MessageOps<E>, ()> {
    Action {
        precondition: |msg_ops: MessageOps<E>, s: NetworkState<E>| {
            msg_ops.recv.is_Some() ==> s.in_flight.contains(msg_ops.recv.get_Some_0())
        },
        transition: |msg_ops: MessageOps<E>, s: NetworkState<E>| {
            if msg_ops.recv.is_Some() {
                let s_prime = NetworkState {
                    in_flight: s.in_flight.remove(msg_ops.recv.get_Some_0()).add(msg_ops.send)
                };
                (s_prime, ())
            } else {
                let s_prime = NetworkState {
                    in_flight: s.in_flight.add(msg_ops.send)
                };
                (s_prime, ())
            }
        },
    }
}

pub open spec fn network() -> NetworkStateMachine<NetworkState<E>, MessageOps<E>> {
    NetworkStateMachine {
        init: |s: NetworkState<E>| s.in_flight == Multiset::<Message<E>>::empty(),
        deliver: Self::deliver(),
    }
}

}

}
