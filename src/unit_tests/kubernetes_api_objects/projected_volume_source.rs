// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::kubernetes_api_objects::container::*;
use crate::kubernetes_api_objects::object_meta::*;
use crate::kubernetes_api_objects::pod::*;
use crate::kubernetes_api_objects::resource::*;
use crate::kubernetes_api_objects::volume::*;
use crate::vstd_ext::string_map::*;
use vstd::prelude::*;
use vstd::string::*;

verus! {
// Tests for projected volume source
#[test]
#[verifier(external)]
pub fn test_default() {
    let projected_volume_source = ProjectedVolumeSource::default();
    assert_eq!(projected_volume_source.into_kube(), deps_hack::k8s_openapi::api::core::v1::ProjectedVolumeSource::default());
}

#[test]
#[verifier(external)]
pub fn test_set_sources() {
    let mut projected_volume_source = ProjectedVolumeSource::default();
    let volume_projections_gen = || {
        let mut volume_projection_1 = VolumeProjection::default();
        let mut volume_projection_2 = VolumeProjection::default();
        let mut volume_projections = Vec::new();
        volume_projection_1.set_config_map(ConfigMapProjection::default());
        volume_projection_2.set_secret(SecretProjection::default());
        volume_projections.push(volume_projection_1);
        volume_projections.push(volume_projection_2);
        volume_projections
    };
    projected_volume_source.set_sources(volume_projections_gen());
    assert_eq!(
        volume_projections_gen()
            .into_iter()
            .map(|s: VolumeProjection| s.into_kube())
            .collect::<Vec<_>>(),
        projected_volume_source.into_kube().sources.unwrap()
    );
}

#[test]
#[verifier(external)]
pub fn test_clone(){
    let mut projected_volume_source = ProjectedVolumeSource::default();
    let volume_projections_gen = || {
        let mut volume_projection_1 = VolumeProjection::default();
        let mut volume_projection_2 = VolumeProjection::default();
        let mut volume_projections = Vec::new();
        volume_projection_1.set_config_map(ConfigMapProjection::default());
        volume_projection_2.set_secret(SecretProjection::default());
        volume_projections.push(volume_projection_1);
        volume_projections.push(volume_projection_2);
        volume_projections
    };
    projected_volume_source.set_sources(volume_projections_gen());
    let projected_volume_source_clone = projected_volume_source.clone();
    assert_eq!(projected_volume_source.into_kube(), projected_volume_source_clone.into_kube());
}

#[test]
#[verifier(external)]
pub fn test_kube() {
    let projected_volume_source = ProjectedVolumeSource::from_kube(deps_hack::k8s_openapi::api::core::v1::ProjectedVolumeSource{
        sources: Some(
            vec![
                deps_hack::k8s_openapi::api::core::v1::VolumeProjection{
                    config_map: Some(deps_hack::k8s_openapi::api::core::v1::ConfigMapProjection::default()),
                    ..Default::default()
                },
                deps_hack::k8s_openapi::api::core::v1::VolumeProjection{
                    secret: Some(deps_hack::k8s_openapi::api::core::v1::SecretProjection::default()),
                    ..Default::default()
                }
            ]
        ),
       ..Default::default()
    });

    assert_eq!(
        projected_volume_source.into_kube(),
        deps_hack::k8s_openapi::api::core::v1::ProjectedVolumeSource{
            sources: Some(
                vec![
                    deps_hack::k8s_openapi::api::core::v1::VolumeProjection{
                        config_map: Some(deps_hack::k8s_openapi::api::core::v1::ConfigMapProjection::default()),
                        ..Default::default()
                    },
                    deps_hack::k8s_openapi::api::core::v1::VolumeProjection{
                        secret: Some(deps_hack::k8s_openapi::api::core::v1::SecretProjection::default()),
                        ..Default::default()
                    }
                ]
            ),
           ..Default::default()
        }
    );
}
}
