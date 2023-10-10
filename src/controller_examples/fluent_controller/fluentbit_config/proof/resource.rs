// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use crate::external_api::exec::*;
use crate::fluent_controller::fluentbit_config::common::*;
use crate::fluent_controller::fluentbit_config::spec::resource::*;
use crate::fluent_controller::fluentbit_config::spec::types::{
    FluentBitConfigReconcileState, FluentBitConfigView,
};
use crate::kubernetes_api_objects::resource::ResourceWrapper;
use crate::kubernetes_api_objects::{
    container::*, label_selector::*, pod_template_spec::*, prelude::*, resource_requirements::*,
    volume::*,
};
use crate::reconciler::spec::resource_builder::*;
use crate::vstd_ext::string_map::StringMap;
use crate::vstd_ext::string_view::*;
use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::string::*;

verus! {

pub open spec fn get_request(sub_resource: SubResource, fbc: FluentBitConfigView) -> GetRequest {
    match sub_resource {
        SubResource::Secret => SecretBuilder::get_request(fbc),
    }
}

pub open spec fn make(sub_resource: SubResource, fbc: FluentBitConfigView, state: FluentBitConfigReconcileState) -> Result<DynamicObjectView, ()> {
    match sub_resource {
        SubResource::Secret => SecretBuilder::make(fbc, state),
    }
}

pub open spec fn update(sub_resource: SubResource, fbc: FluentBitConfigView, state: FluentBitConfigReconcileState, obj: DynamicObjectView) -> Result<DynamicObjectView, ()> {
    match sub_resource {
        SubResource::Secret => SecretBuilder::update(fbc, state, obj),
    }
}

pub open spec fn state_after_create_or_update(sub_resource: SubResource, obj: DynamicObjectView, state: FluentBitConfigReconcileState) -> (res: Result<FluentBitConfigReconcileState, ()>) {
    match sub_resource {
        SubResource::Secret => SecretBuilder::state_after_create_or_update(obj, state),
    }
}

pub open spec fn resource_state_matches(sub_resource: SubResource, fbc: FluentBitConfigView, resources: StoredState) -> bool {
    match sub_resource {
        SubResource::Secret => SecretBuilder::resource_state_matches(fbc, resources),
    }
}

pub open spec fn unchangeable(sub_resource: SubResource, object: DynamicObjectView, fbc: FluentBitConfigView) -> bool {
    match sub_resource {
        SubResource::Secret => SecretBuilder::unchangeable(object, fbc),
    }
}

}