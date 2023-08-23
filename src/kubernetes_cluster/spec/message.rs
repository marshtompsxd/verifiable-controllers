// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
#![allow(unused_imports)]
use crate::external_api::spec::*;
use crate::kubernetes_api_objects::{api_method::*, common::*, dynamic::*, error::*};
use crate::pervasive_ext::string_view::*;
use vstd::{multiset::*, prelude::*};

verus! {

pub struct MessageOps<E: ExternalAPI> {
    pub recv: Option<Message<E>>,
    pub send: Multiset<Message<E>>,
}

pub struct Message<E: ExternalAPI> {
    pub src: HostId,
    pub dst: HostId,
    pub content: MessageContent<E>,
}

#[is_variant]
pub enum HostId {
    KubernetesAPI,
    BuiltinController,
    CustomController,
    External,
    Client,
}

pub type RestId = nat;

// RestIdAllocator allocates unique RestId for each request sent by the controller.
// The Kubernetes API state machine ensures the response will carry the same RestId.
pub struct RestIdAllocator {
    pub rest_id_counter: RestId,
}

impl RestIdAllocator {
    // Allocate a RestId which is the current rest_id_counter
    // and also returns a new RestIdAllocator with a different rest_id_counter.
    //
    // An important assumption of RestIdAllocator is that the user (i.e., state machine)
    // after allocating one RestId, will use the returned new RestIdAllocator
    // to allocate the next RestId.
    // With this assumption, the user of RestIdAllocator always gets a RestId
    // which differs from all the RestIds allocated before because the
    // returned RestIdAllocator has a incremented rest_id_counter.
    //
    // Besides the uniqueness, the allocated RestId can also serve as a timestamp
    // if we need to say some Rest messages are sent before the others.
    pub open spec fn allocate(self) -> (Self, RestId) {
        (RestIdAllocator {
            rest_id_counter: self.rest_id_counter + 1,
        }, self.rest_id_counter)
    }
}

// Each MessageContent is a request/response and a rest id
#[is_variant]
pub enum MessageContent<E: ExternalAPI> {
    APIRequest(APIRequest, RestId),
    APIResponse(APIResponse, RestId),
    ExternalAPIRequest(E::Input, RestId),
    ExternalAPIResponse(E::Output, RestId),
}

// Some handy methods for pattern matching and retrieving information from MessageContent
impl<E: ExternalAPI> MessageContent<E> {
    pub open spec fn is_get_request(self) -> bool {
        &&& self.is_APIRequest()
        &&& self.get_APIRequest_0().is_GetRequest()
    }

    pub open spec fn get_get_request(self) -> GetRequest
        recommends
            self.is_get_request()
    {
        self.get_APIRequest_0().get_GetRequest_0()
    }

    pub open spec fn is_list_request(self) -> bool {
        &&& self.is_APIRequest()
        &&& self.get_APIRequest_0().is_ListRequest()
    }

    pub open spec fn get_list_request(self) -> ListRequest
        recommends
            self.is_list_request()
    {
        self.get_APIRequest_0().get_ListRequest_0()
    }

    pub open spec fn is_create_request(self) -> bool {
        &&& self.is_APIRequest()
        &&& self.get_APIRequest_0().is_CreateRequest()
    }

    pub open spec fn get_create_request(self) -> CreateRequest
        recommends
            self.is_create_request()
    {
        self.get_APIRequest_0().get_CreateRequest_0()
    }

    pub open spec fn is_delete_request(self) -> bool {
        &&& self.is_APIRequest()
        &&& self.get_APIRequest_0().is_DeleteRequest()
    }

    pub open spec fn is_delete_request_with_key(self, key: ObjectRef) -> bool {
        &&& self.is_APIRequest()
        &&& self.get_APIRequest_0().is_DeleteRequest()
        &&& self.get_APIRequest_0().get_DeleteRequest_0().key == key
    }

    pub open spec fn get_delete_request(self) -> DeleteRequest
        recommends
            self.is_delete_request()
    {
        self.get_APIRequest_0().get_DeleteRequest_0()
    }

    pub open spec fn is_update_request(self) -> bool {
        &&& self.is_APIRequest()
        &&& self.get_APIRequest_0().is_UpdateRequest()
    }

    pub open spec fn is_update_request_with_key(self, key: ObjectRef) -> bool {
        &&& self.is_APIRequest()
        &&& self.get_APIRequest_0().is_UpdateRequest()
        &&& self.get_APIRequest_0().get_UpdateRequest_0().key == key
    }

    pub open spec fn get_update_request(self) -> UpdateRequest
        recommends
            self.is_update_request()
    {
        self.get_APIRequest_0().get_UpdateRequest_0()
    }

    pub open spec fn get_req_id(self) -> RestId
        recommends
            self.is_APIRequest()
    {
        self.get_APIRequest_1()
    }

    pub open spec fn is_get_response(self) -> bool {
        &&& self.is_APIResponse()
        &&& self.get_APIResponse_0().is_GetResponse()
    }

    pub open spec fn get_get_response(self) -> GetResponse
        recommends
            self.is_get_response()
    {
        self.get_APIResponse_0().get_GetResponse_0()
    }

    pub open spec fn is_create_response(self) -> bool {
        &&& self.is_APIResponse()
        &&& self.get_APIResponse_0().is_CreateResponse()
    }

    pub open spec fn get_create_response(self) -> CreateResponse
        recommends
            self.is_create_response()
    {
        self.get_APIResponse_0().get_CreateResponse_0()
    }

    pub open spec fn is_delete_response(self) -> bool {
        &&& self.is_APIResponse()
        &&& self.get_APIResponse_0().is_DeleteResponse()
    }

    pub open spec fn get_delete_response(self) -> DeleteResponse
        recommends
            self.is_delete_response()
    {
        self.get_APIResponse_0().get_DeleteResponse_0()
    }

    pub open spec fn is_list_response(self) -> bool {
        &&& self.is_APIResponse()
        &&& self.get_APIResponse_0().is_ListResponse()
    }

    pub open spec fn get_list_response(self) -> ListResponse
        recommends
            self.is_list_response()
    {
        self.get_APIResponse_0().get_ListResponse_0()
    }

    pub open spec fn get_resp_id(self) -> RestId
        recommends
            self.is_APIResponse()
    {
        self.get_APIResponse_1()
    }

    pub open spec fn get_rest_id(self) -> RestId
    {
        match self {
            MessageContent::APIRequest(_, id) => id,
            MessageContent::APIResponse(_, id) => id,
            MessageContent::ExternalAPIRequest(_, id) => id,
            MessageContent::ExternalAPIResponse(_, id) => id,
        }
    }
}

pub open spec fn is_ok_resp(resp: APIResponse) -> bool {
    match resp {
        APIResponse::GetResponse(get_resp) => get_resp.res.is_Ok(),
        APIResponse::ListResponse(list_resp) => list_resp.res.is_Ok(),
        APIResponse::CreateResponse(create_resp) => create_resp.res.is_Ok(),
        APIResponse::DeleteResponse(delete_resp) => delete_resp.res.is_Ok(),
        APIResponse::UpdateResponse(update_resp) => update_resp.res.is_Ok(),
    }
}

impl<E: ExternalAPI> Message<E> {

pub open spec fn controller_req_msg(req: APIRequest, req_id: RestId) -> Message<E> {
    Message::form_msg(HostId::CustomController, HostId::KubernetesAPI, MessageContent::APIRequest(req, req_id))
}

pub open spec fn controller_external_req_msg(req: E::Input, req_id: RestId) -> Message<E> {
    Message::form_msg(HostId::CustomController, HostId::KubernetesAPI, MessageContent::ExternalAPIRequest(req, req_id))
}

pub open spec fn built_in_controller_req_msg(msg_content: MessageContent<E>) -> Message<E> {
    Message::form_msg(HostId::BuiltinController, HostId::KubernetesAPI, msg_content)
}

pub open spec fn client_req_msg(msg_content: MessageContent<E>) -> Message<E> {
    Message::form_msg(HostId::Client, HostId::KubernetesAPI, msg_content)
}

// TODO: consider the external request/response messages
pub open spec fn resp_msg_matches_req_msg(resp_msg: Message<E>, req_msg: Message<E>) -> bool {
    &&& resp_msg.content.is_APIResponse()
    &&& req_msg.content.is_APIRequest()
    &&& resp_msg.dst == req_msg.src
    &&& resp_msg.src == req_msg.dst
    &&& resp_msg.content.get_APIResponse_1() == req_msg.content.get_APIRequest_1()
    &&& match resp_msg.content.get_APIResponse_0() {
        APIResponse::GetResponse(_) => req_msg.content.get_APIRequest_0().is_GetRequest(),
        APIResponse::ListResponse(_) => req_msg.content.get_APIRequest_0().is_ListRequest(),
        APIResponse::CreateResponse(_) => req_msg.content.get_APIRequest_0().is_CreateRequest(),
        APIResponse::DeleteResponse(_) => req_msg.content.get_APIRequest_0().is_DeleteRequest(),
        APIResponse::UpdateResponse(_) => req_msg.content.get_APIRequest_0().is_UpdateRequest(),
    }
}

// TODO: handle list request
pub open spec fn form_matched_resp_msg(req_msg: Message<E>, result: Result<DynamicObjectView, APIError>) -> Message<E>
    recommends req_msg.content.is_APIRequest(),
{
    match req_msg.content.get_APIRequest_0() {
        APIRequest::GetRequest(_) => Self::form_get_resp_msg(req_msg, result),
        APIRequest::ListRequest(_) => Self::form_list_resp_msg(req_msg, Err(APIError::Invalid)),
        APIRequest::CreateRequest(_) => Self::form_create_resp_msg(req_msg, result),
        APIRequest::DeleteRequest(_) => Self::form_delete_resp_msg(req_msg, result),
        APIRequest::UpdateRequest(_) => Self::form_update_resp_msg(req_msg, result),
    }
}

pub open spec fn form_msg(src: HostId, dst: HostId, msg_content: MessageContent<E>) -> Message<E> {
    Message {
        src: src,
        dst: dst,
        content: msg_content,
    }
}

pub open spec fn form_get_resp_msg(req_msg: Message<E>, result: Result<DynamicObjectView, APIError>) -> Message<E>
    recommends req_msg.content.is_get_request(),
{
    Self::form_msg(req_msg.dst, req_msg.src, Self::get_resp_msg_content(result, req_msg.content.get_req_id()))
}

pub open spec fn form_list_resp_msg(req_msg: Message<E>, result: Result<Seq<DynamicObjectView>, APIError>) -> Message<E>
    recommends req_msg.content.is_list_request(),
{
    Self::form_msg(req_msg.dst, req_msg.src, Self::list_resp_msg_content(result, req_msg.content.get_req_id()))
}

pub open spec fn form_create_resp_msg(req_msg: Message<E>, result: Result<DynamicObjectView, APIError>) -> Message<E>
    recommends req_msg.content.is_create_request(),
{
    Self::form_msg(req_msg.dst, req_msg.src, Self::create_resp_msg_content(result, req_msg.content.get_req_id()))
}

pub open spec fn form_delete_resp_msg(req_msg: Message<E>, result: Result<DynamicObjectView, APIError>) -> Message<E>
    recommends req_msg.content.is_delete_request(),
{
    Self::form_msg(req_msg.dst, req_msg.src, Self::delete_resp_msg_content(result, req_msg.content.get_req_id()))
}

pub open spec fn form_update_resp_msg(req_msg: Message<E>, result: Result<DynamicObjectView, APIError>) -> Message<E>
    recommends req_msg.content.is_update_request(),
{
    Self::form_msg(req_msg.dst, req_msg.src, Self::update_resp_msg_content(result, req_msg.content.get_req_id()))
}

pub open spec fn get_req_msg_content(key: ObjectRef, req_id: RestId) -> MessageContent<E> {
    MessageContent::APIRequest(APIRequest::GetRequest(GetRequest{
        key: key,
    }), req_id)
}

pub open spec fn list_req_msg_content(kind: Kind, namespace: StringView, req_id: RestId) -> MessageContent<E> {
    MessageContent::APIRequest(APIRequest::ListRequest(ListRequest{
        kind: kind,
        namespace: namespace,
    }), req_id)
}

pub open spec fn create_req_msg_content(namespace: StringView, obj: DynamicObjectView, req_id: RestId) -> MessageContent<E> {
    MessageContent::APIRequest(APIRequest::CreateRequest(CreateRequest{
        namespace: namespace,
        obj: obj,
    }), req_id)
}

pub open spec fn delete_req_msg_content(key: ObjectRef, req_id: RestId) -> MessageContent<E> {
    MessageContent::APIRequest(APIRequest::DeleteRequest(DeleteRequest{
        key: key,
    }), req_id)
}

pub open spec fn update_req_msg_content(key: ObjectRef, obj: DynamicObjectView, req_id: RestId) -> MessageContent<E> {
    MessageContent::APIRequest(APIRequest::UpdateRequest(UpdateRequest{
        key: key,
        obj: obj,
    }), req_id)
}

pub open spec fn get_resp_msg_content(res: Result<DynamicObjectView, APIError>, resp_id: RestId) -> MessageContent<E> {
    MessageContent::APIResponse(APIResponse::GetResponse(GetResponse{
        res: res,
    }), resp_id)
}

pub open spec fn list_resp_msg_content(res: Result<Seq<DynamicObjectView>, APIError>, resp_id: RestId) -> MessageContent<E> {
    MessageContent::APIResponse(APIResponse::ListResponse(ListResponse{
        res: res,
    }), resp_id)
}

pub open spec fn create_resp_msg_content(res: Result<DynamicObjectView, APIError>, resp_id: RestId) -> MessageContent<E> {
    MessageContent::APIResponse(APIResponse::CreateResponse(CreateResponse{
        res: res,
    }), resp_id)
}

pub open spec fn delete_resp_msg_content(res: Result<DynamicObjectView, APIError>, resp_id: RestId) -> MessageContent<E> {
    MessageContent::APIResponse(APIResponse::DeleteResponse(DeleteResponse{
        res: res,
    }), resp_id)
}

pub open spec fn update_resp_msg_content(res: Result<DynamicObjectView, APIError>, resp_id: RestId) -> MessageContent<E> {
    MessageContent::APIResponse(APIResponse::UpdateResponse(UpdateResponse{
        res: res,
    }), resp_id)
}

}

pub open spec fn api_request_msg_before<E: ExternalAPI>(rest_id: RestId) -> FnSpec(Message<E>) -> bool {
    |msg: Message<E>|
        msg.dst.is_KubernetesAPI()
        && msg.content.is_APIRequest()
        && msg.content.get_rest_id() < rest_id
}

pub open spec fn received_msg_destined_for<E: ExternalAPI>(recv: Option<Message<E>>, host_id: HostId) -> bool {
    if recv.is_Some() {
        recv.get_Some_0().dst == host_id
    } else {
        true
    }
}

}
