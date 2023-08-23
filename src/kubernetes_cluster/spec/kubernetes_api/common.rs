// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use crate::external_api::spec::*;
use crate::kubernetes_api_objects::{api_method::*, common::*, dynamic::*};
use crate::kubernetes_cluster::spec::message::*;
use crate::state_machine::action::*;
use crate::state_machine::state_machine::*;

use crate::temporal_logic::defs::*;
use vstd::{multiset::*, prelude::*};

verus! {

pub type StoredState = Map<ObjectRef, DynamicObjectView>;

pub type Uid = nat;

pub type ResourceVersion = nat;

pub struct KubernetesAPIState {
    pub resources: StoredState,
    pub uid_counter: Uid,
    pub resource_version_counter: ResourceVersion,
}

pub enum KubernetesAPIStep {
    HandleRequest,
}

pub struct KubernetesAPIActionInput<E: ExternalAPI> {
    pub recv: Option<Message<E>>,
    pub rest_id_allocator: RestIdAllocator,
}

pub type KubernetesAPIActionOutput<E> = (Multiset<Message<E>>, RestIdAllocator);

pub type KubernetesAPIStateMachine<E> = StateMachine<KubernetesAPIState, KubernetesAPIActionInput<E>, KubernetesAPIActionInput<E>, KubernetesAPIActionOutput<E>, KubernetesAPIStep>;

pub type KubernetesAPIAction<E> = Action<KubernetesAPIState, KubernetesAPIActionInput<E>, KubernetesAPIActionOutput<E>>;

}
