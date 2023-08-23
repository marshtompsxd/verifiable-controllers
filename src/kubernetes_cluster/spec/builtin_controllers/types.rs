// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use crate::kubernetes_api_objects::{api_method::*, common::*, dynamic::*};
use crate::kubernetes_cluster::spec::message::*;
use crate::state_machine::action::*;
use crate::state_machine::state_machine::*;
use crate::temporal_logic::defs::*;
use vstd::{multiset::*, prelude::*};

verus! {

pub type StoredState = Map<ObjectRef, DynamicObjectView>;

pub struct BuiltinControllersState {}

#[is_variant]
pub enum BuiltinControllersStep {
    RunGarbageCollector,
}

#[is_variant]
pub enum BuiltinControllerChoice {
    GarbageCollector,
}

pub struct BuiltinControllersActionInput {
    pub choice: BuiltinControllerChoice,
    pub key: ObjectRef,
    pub resources: StoredState,
    pub rest_id_allocator: RestIdAllocator,
}

pub type BuiltinControllersActionOutput<E> = (Multiset<Message<E>>, RestIdAllocator);

pub type BuiltinControllersStateMachine<E> = StateMachine<BuiltinControllersState,
                                            BuiltinControllersActionInput,
                                            BuiltinControllersActionInput,
                                            BuiltinControllersActionOutput<E>,
                                            BuiltinControllersStep>;

pub type BuiltinControllersAction<E> = Action<BuiltinControllersState,
                                        BuiltinControllersActionInput,
                                        BuiltinControllersActionOutput<E>>;

}
