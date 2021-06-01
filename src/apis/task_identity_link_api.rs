/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.13.0
 *
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use reqwest;

use super::configuration;

pub struct TaskIdentityLinkApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl TaskIdentityLinkApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> TaskIdentityLinkApiClient {
        TaskIdentityLinkApiClient { configuration }
    }
}

pub trait TaskIdentityLinkApi {
    fn add_identity_link(
        &self,
        id: &str,
        identity_link_dto: Option<crate::models::IdentityLinkDto>,
    ) -> Result<(), crate::apis::CamundaClientError>;
    fn delete_identity_link(
        &self,
        id: &str,
        identity_link_dto: Option<crate::models::IdentityLinkDto>,
    ) -> Result<(), crate::apis::CamundaClientError>;
    fn get_identity_links(
        &self,
        id: &str,
        _type: Option<&str>,
    ) -> Result<Vec<crate::models::IdentityLinkDto>, crate::apis::CamundaClientError>;
}

impl TaskIdentityLinkApi for TaskIdentityLinkApiClient {
    fn add_identity_link(
        &self,
        id: &str,
        identity_link_dto: Option<crate::models::IdentityLinkDto>,
    ) -> Result<(), crate::apis::CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/task/{id}/identity-links",
            configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&identity_link_dto);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn delete_identity_link(
        &self,
        id: &str,
        identity_link_dto: Option<crate::models::IdentityLinkDto>,
    ) -> Result<(), crate::apis::CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/task/{id}/identity-links/delete",
            configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&identity_link_dto);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn get_identity_links(
        &self,
        id: &str,
        _type: Option<&str>,
    ) -> Result<Vec<crate::models::IdentityLinkDto>, crate::apis::CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/task/{id}/identity-links",
            configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = _type {
            req_builder = req_builder.query(&[("type", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
