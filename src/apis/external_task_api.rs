/*
 * Camunda BPM REST API
 *
 * OpenApi Spec for Camunda BPM REST API.
 *
 * The version of the OpenAPI document: 7.13.0
 *
 * Generated by: https://openapi-generator.tech
 */

use async_trait::async_trait;
use std::borrow::Borrow;
use std::option::Option;
use std::sync::Arc;

pub use reqwest;
pub use serde;
pub use serde_derive;
pub use serde_json;
pub use url;

use crate::{errors::errors::CamundaClientError, utils::url_encode};

use super::configuration;

pub struct ExternalTaskApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl ExternalTaskApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> ExternalTaskApiClient {
        ExternalTaskApiClient { configuration }
    }
}

#[async_trait]
pub trait ExternalTaskApi {
    async fn complete_external_task_resource(
        &self,
        id: &str,
        complete_external_task_dto: Option<crate::models::CompleteExternalTaskDto>,
    ) -> Result<(), CamundaClientError>;
    async fn extend_lock(
        &self,
        id: &str,
        extend_lock_on_external_task_dto: Option<crate::models::ExtendLockOnExternalTaskDto>,
    ) -> Result<(), CamundaClientError>;
    async fn fetch_and_lock(
        &self,
        fetch_external_tasks_dto: Option<crate::models::FetchExternalTasksDto>,
    ) -> Result<Vec<crate::models::LockedExternalTaskDto>, CamundaClientError>;
    async fn get_external_task(
        &self,
        id: &str,
    ) -> Result<crate::models::ExternalTaskDto, CamundaClientError>;
    async fn get_external_task_error_details(&self, id: &str)
        -> Result<String, CamundaClientError>;
    async fn get_external_tasks(
        &self,
        external_task_id: Option<&str>,
        external_task_id_in: Option<&str>,
        topic_name: Option<&str>,
        worker_id: Option<&str>,
        locked: Option<bool>,
        not_locked: Option<bool>,
        with_retries_left: Option<bool>,
        no_retries_left: Option<bool>,
        lock_expiration_after: Option<String>,
        lock_expiration_before: Option<String>,
        activity_id: Option<&str>,
        activity_id_in: Option<&str>,
        execution_id: Option<&str>,
        process_instance_id: Option<&str>,
        process_instance_id_in: Option<&str>,
        process_definition_id: Option<&str>,
        tenant_id_in: Option<&str>,
        active: Option<bool>,
        suspended: Option<bool>,
        priority_higher_than_or_equals: Option<i64>,
        priority_lower_than_or_equals: Option<i64>,
        sort_by: Option<&str>,
        sort_order: Option<&str>,
        first_result: Option<i32>,
        max_results: Option<i32>,
    ) -> Result<Vec<crate::models::ExternalTaskDto>, CamundaClientError>;
    async fn get_external_tasks_count(
        &self,
        external_task_id: Option<&str>,
        external_task_id_in: Option<&str>,
        topic_name: Option<&str>,
        worker_id: Option<&str>,
        locked: Option<bool>,
        not_locked: Option<bool>,
        with_retries_left: Option<bool>,
        no_retries_left: Option<bool>,
        lock_expiration_after: Option<String>,
        lock_expiration_before: Option<String>,
        activity_id: Option<&str>,
        activity_id_in: Option<&str>,
        execution_id: Option<&str>,
        process_instance_id: Option<&str>,
        process_instance_id_in: Option<&str>,
        process_definition_id: Option<&str>,
        tenant_id_in: Option<&str>,
        active: Option<bool>,
        suspended: Option<bool>,
        priority_higher_than_or_equals: Option<i64>,
        priority_lower_than_or_equals: Option<i64>,
    ) -> Result<crate::models::CountResultDto, CamundaClientError>;
    async fn get_topic_names(
        &self,
        with_locked_tasks: Option<bool>,
        with_unlocked_tasks: Option<bool>,
        with_retries_left: Option<bool>,
    ) -> Result<Vec<String>, CamundaClientError>;
    async fn handle_external_task_bpmn_error(
        &self,
        id: &str,
        external_task_bpmn_error: Option<crate::models::ExternalTaskBpmnError>,
    ) -> Result<(), CamundaClientError>;
    async fn handle_failure(
        &self,
        id: &str,
        external_task_failure_dto: Option<crate::models::ExternalTaskFailureDto>,
    ) -> Result<(), CamundaClientError>;
    async fn query_external_tasks(
        &self,
        first_result: Option<i32>,
        max_results: Option<i32>,
        external_task_query_dto: Option<crate::models::ExternalTaskQueryDto>,
    ) -> Result<Vec<crate::models::ExternalTaskDto>, CamundaClientError>;
    async fn query_external_tasks_count(
        &self,
        external_task_query_dto: Option<crate::models::ExternalTaskQueryDto>,
    ) -> Result<crate::models::CountResultDto, CamundaClientError>;
    async fn set_external_task_resource_priority(
        &self,
        id: &str,
        priority_dto: Option<crate::models::PriorityDto>,
    ) -> Result<(), CamundaClientError>;
    async fn set_external_task_resource_retries(
        &self,
        id: &str,
        retries_dto: Option<crate::models::RetriesDto>,
    ) -> Result<(), CamundaClientError>;
    async fn set_external_task_retries(
        &self,
        set_retries_for_external_tasks_dto: Option<crate::models::SetRetriesForExternalTasksDto>,
    ) -> Result<(), CamundaClientError>;
    async fn set_external_task_retries_async_operation(
        &self,
        set_retries_for_external_tasks_dto: Option<crate::models::SetRetriesForExternalTasksDto>,
    ) -> Result<crate::models::BatchDto, CamundaClientError>;
    async fn unlock(&self, id: &str) -> Result<(), CamundaClientError>;
}

#[async_trait]
impl ExternalTaskApi for ExternalTaskApiClient {
    async fn complete_external_task_resource(
        &self,
        id: &str,
        complete_external_task_dto: Option<crate::models::CompleteExternalTaskDto>,
    ) -> Result<(), CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/external-task/{id}/complete",
            configuration.base_path,
            id = url_encode::url_encode(id)
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&complete_external_task_dto);

        // send request
        let resp = req_builder.send().await?;

        resp.error_for_status()?.json().await?;
        Ok(())
    }

    async fn extend_lock(
        &self,
        id: &str,
        extend_lock_on_external_task_dto: Option<crate::models::ExtendLockOnExternalTaskDto>,
    ) -> Result<(), CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/external-task/{id}/extendLock",
            configuration.base_path,
            id = url_encode::url_encode(id)
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&extend_lock_on_external_task_dto);

        // send request
        let resp = req_builder.send().await?;

        resp.error_for_status()?.json().await?;
        Ok(())
    }

    async fn fetch_and_lock(
        &self,
        fetch_external_tasks_dto: Option<crate::models::FetchExternalTasksDto>,
    ) -> Result<Vec<crate::models::LockedExternalTaskDto>, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/external-task/fetchAndLock", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&fetch_external_tasks_dto);

        // send request
        let resp = req_builder.send().await?;

        Ok(resp.error_for_status()?.json().await?)
    }

    async fn get_external_task(
        &self,
        id: &str,
    ) -> Result<crate::models::ExternalTaskDto, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/external-task/{id}",
            configuration.base_path,
            id = url_encode::url_encode(id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let resp = req_builder.send().await?;

        Ok(resp.error_for_status()?.json().await?)
    }

    async fn get_external_task_error_details(
        &self,
        id: &str,
    ) -> Result<String, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/external-task/{id}/errorDetails",
            configuration.base_path,
            id = url_encode::url_encode(id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let resp = req_builder.send().await?;

        Ok(resp.error_for_status()?.json().await?)
    }

    async fn get_external_tasks(
        &self,
        external_task_id: Option<&str>,
        external_task_id_in: Option<&str>,
        topic_name: Option<&str>,
        worker_id: Option<&str>,
        locked: Option<bool>,
        not_locked: Option<bool>,
        with_retries_left: Option<bool>,
        no_retries_left: Option<bool>,
        lock_expiration_after: Option<String>,
        lock_expiration_before: Option<String>,
        activity_id: Option<&str>,
        activity_id_in: Option<&str>,
        execution_id: Option<&str>,
        process_instance_id: Option<&str>,
        process_instance_id_in: Option<&str>,
        process_definition_id: Option<&str>,
        tenant_id_in: Option<&str>,
        active: Option<bool>,
        suspended: Option<bool>,
        priority_higher_than_or_equals: Option<i64>,
        priority_lower_than_or_equals: Option<i64>,
        sort_by: Option<&str>,
        sort_order: Option<&str>,
        first_result: Option<i32>,
        max_results: Option<i32>,
    ) -> Result<Vec<crate::models::ExternalTaskDto>, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/external-task", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = external_task_id {
            req_builder = req_builder.query(&[("externalTaskId", &s.to_string())]);
        }
        if let Some(ref s) = external_task_id_in {
            req_builder = req_builder.query(&[("externalTaskIdIn", &s.to_string())]);
        }
        if let Some(ref s) = topic_name {
            req_builder = req_builder.query(&[("topicName", &s.to_string())]);
        }
        if let Some(ref s) = worker_id {
            req_builder = req_builder.query(&[("workerId", &s.to_string())]);
        }
        if let Some(ref s) = locked {
            req_builder = req_builder.query(&[("locked", &s.to_string())]);
        }
        if let Some(ref s) = not_locked {
            req_builder = req_builder.query(&[("notLocked", &s.to_string())]);
        }
        if let Some(ref s) = with_retries_left {
            req_builder = req_builder.query(&[("withRetriesLeft", &s.to_string())]);
        }
        if let Some(ref s) = no_retries_left {
            req_builder = req_builder.query(&[("noRetriesLeft", &s.to_string())]);
        }
        if let Some(ref s) = lock_expiration_after {
            req_builder = req_builder.query(&[("lockExpirationAfter", &s.to_string())]);
        }
        if let Some(ref s) = lock_expiration_before {
            req_builder = req_builder.query(&[("lockExpirationBefore", &s.to_string())]);
        }
        if let Some(ref s) = activity_id {
            req_builder = req_builder.query(&[("activityId", &s.to_string())]);
        }
        if let Some(ref s) = activity_id_in {
            req_builder = req_builder.query(&[("activityIdIn", &s.to_string())]);
        }
        if let Some(ref s) = execution_id {
            req_builder = req_builder.query(&[("executionId", &s.to_string())]);
        }
        if let Some(ref s) = process_instance_id {
            req_builder = req_builder.query(&[("processInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = process_instance_id_in {
            req_builder = req_builder.query(&[("processInstanceIdIn", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_id {
            req_builder = req_builder.query(&[("processDefinitionId", &s.to_string())]);
        }
        if let Some(ref s) = tenant_id_in {
            req_builder = req_builder.query(&[("tenantIdIn", &s.to_string())]);
        }
        if let Some(ref s) = active {
            req_builder = req_builder.query(&[("active", &s.to_string())]);
        }
        if let Some(ref s) = suspended {
            req_builder = req_builder.query(&[("suspended", &s.to_string())]);
        }
        if let Some(ref s) = priority_higher_than_or_equals {
            req_builder = req_builder.query(&[("priorityHigherThanOrEquals", &s.to_string())]);
        }
        if let Some(ref s) = priority_lower_than_or_equals {
            req_builder = req_builder.query(&[("priorityLowerThanOrEquals", &s.to_string())]);
        }
        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sortBy", &s.to_string())]);
        }
        if let Some(ref s) = sort_order {
            req_builder = req_builder.query(&[("sortOrder", &s.to_string())]);
        }
        if let Some(ref s) = first_result {
            req_builder = req_builder.query(&[("firstResult", &s.to_string())]);
        }
        if let Some(ref s) = max_results {
            req_builder = req_builder.query(&[("maxResults", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let resp = req_builder.send().await?;

        Ok(resp.error_for_status()?.json().await?)
    }

    async fn get_external_tasks_count(
        &self,
        external_task_id: Option<&str>,
        external_task_id_in: Option<&str>,
        topic_name: Option<&str>,
        worker_id: Option<&str>,
        locked: Option<bool>,
        not_locked: Option<bool>,
        with_retries_left: Option<bool>,
        no_retries_left: Option<bool>,
        lock_expiration_after: Option<String>,
        lock_expiration_before: Option<String>,
        activity_id: Option<&str>,
        activity_id_in: Option<&str>,
        execution_id: Option<&str>,
        process_instance_id: Option<&str>,
        process_instance_id_in: Option<&str>,
        process_definition_id: Option<&str>,
        tenant_id_in: Option<&str>,
        active: Option<bool>,
        suspended: Option<bool>,
        priority_higher_than_or_equals: Option<i64>,
        priority_lower_than_or_equals: Option<i64>,
    ) -> Result<crate::models::CountResultDto, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/external-task/count", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = external_task_id {
            req_builder = req_builder.query(&[("externalTaskId", &s.to_string())]);
        }
        if let Some(ref s) = external_task_id_in {
            req_builder = req_builder.query(&[("externalTaskIdIn", &s.to_string())]);
        }
        if let Some(ref s) = topic_name {
            req_builder = req_builder.query(&[("topicName", &s.to_string())]);
        }
        if let Some(ref s) = worker_id {
            req_builder = req_builder.query(&[("workerId", &s.to_string())]);
        }
        if let Some(ref s) = locked {
            req_builder = req_builder.query(&[("locked", &s.to_string())]);
        }
        if let Some(ref s) = not_locked {
            req_builder = req_builder.query(&[("notLocked", &s.to_string())]);
        }
        if let Some(ref s) = with_retries_left {
            req_builder = req_builder.query(&[("withRetriesLeft", &s.to_string())]);
        }
        if let Some(ref s) = no_retries_left {
            req_builder = req_builder.query(&[("noRetriesLeft", &s.to_string())]);
        }
        if let Some(ref s) = lock_expiration_after {
            req_builder = req_builder.query(&[("lockExpirationAfter", &s.to_string())]);
        }
        if let Some(ref s) = lock_expiration_before {
            req_builder = req_builder.query(&[("lockExpirationBefore", &s.to_string())]);
        }
        if let Some(ref s) = activity_id {
            req_builder = req_builder.query(&[("activityId", &s.to_string())]);
        }
        if let Some(ref s) = activity_id_in {
            req_builder = req_builder.query(&[("activityIdIn", &s.to_string())]);
        }
        if let Some(ref s) = execution_id {
            req_builder = req_builder.query(&[("executionId", &s.to_string())]);
        }
        if let Some(ref s) = process_instance_id {
            req_builder = req_builder.query(&[("processInstanceId", &s.to_string())]);
        }
        if let Some(ref s) = process_instance_id_in {
            req_builder = req_builder.query(&[("processInstanceIdIn", &s.to_string())]);
        }
        if let Some(ref s) = process_definition_id {
            req_builder = req_builder.query(&[("processDefinitionId", &s.to_string())]);
        }
        if let Some(ref s) = tenant_id_in {
            req_builder = req_builder.query(&[("tenantIdIn", &s.to_string())]);
        }
        if let Some(ref s) = active {
            req_builder = req_builder.query(&[("active", &s.to_string())]);
        }
        if let Some(ref s) = suspended {
            req_builder = req_builder.query(&[("suspended", &s.to_string())]);
        }
        if let Some(ref s) = priority_higher_than_or_equals {
            req_builder = req_builder.query(&[("priorityHigherThanOrEquals", &s.to_string())]);
        }
        if let Some(ref s) = priority_lower_than_or_equals {
            req_builder = req_builder.query(&[("priorityLowerThanOrEquals", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let resp = req_builder.send().await?;

        Ok(resp.error_for_status()?.json().await?)
    }

    async fn get_topic_names(
        &self,
        with_locked_tasks: Option<bool>,
        with_unlocked_tasks: Option<bool>,
        with_retries_left: Option<bool>,
    ) -> Result<Vec<String>, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/external-task/topic-names", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = with_locked_tasks {
            req_builder = req_builder.query(&[("withLockedTasks", &s.to_string())]);
        }
        if let Some(ref s) = with_unlocked_tasks {
            req_builder = req_builder.query(&[("withUnlockedTasks", &s.to_string())]);
        }
        if let Some(ref s) = with_retries_left {
            req_builder = req_builder.query(&[("withRetriesLeft", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let resp = req_builder.send().await?;

        Ok(resp.error_for_status()?.json().await?)
    }

    async fn handle_external_task_bpmn_error(
        &self,
        id: &str,
        external_task_bpmn_error: Option<crate::models::ExternalTaskBpmnError>,
    ) -> Result<(), CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/external-task/{id}/bpmnError",
            configuration.base_path,
            id = url_encode::url_encode(id)
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&external_task_bpmn_error);

        // send request
        let resp = req_builder.send().await?;

        resp.error_for_status()?.json().await?;
        Ok(())
    }

    async fn handle_failure(
        &self,
        id: &str,
        external_task_failure_dto: Option<crate::models::ExternalTaskFailureDto>,
    ) -> Result<(), CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/external-task/{id}/failure",
            configuration.base_path,
            id = url_encode::url_encode(id)
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&external_task_failure_dto);

        // send request
        let resp = req_builder.send().await?;

        resp.error_for_status()?.json().await?;
        Ok(())
    }

    async fn query_external_tasks(
        &self,
        first_result: Option<i32>,
        max_results: Option<i32>,
        external_task_query_dto: Option<crate::models::ExternalTaskQueryDto>,
    ) -> Result<Vec<crate::models::ExternalTaskDto>, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/external-task", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref s) = first_result {
            req_builder = req_builder.query(&[("firstResult", &s.to_string())]);
        }
        if let Some(ref s) = max_results {
            req_builder = req_builder.query(&[("maxResults", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&external_task_query_dto);

        // send request
        let resp = req_builder.send().await?;

        Ok(resp.error_for_status()?.json().await?)
    }

    async fn query_external_tasks_count(
        &self,
        external_task_query_dto: Option<crate::models::ExternalTaskQueryDto>,
    ) -> Result<crate::models::CountResultDto, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/external-task/count", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&external_task_query_dto);

        // send request
        let resp = req_builder.send().await?;

        Ok(resp.error_for_status()?.json().await?)
    }

    async fn set_external_task_resource_priority(
        &self,
        id: &str,
        priority_dto: Option<crate::models::PriorityDto>,
    ) -> Result<(), CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/external-task/{id}/priority",
            configuration.base_path,
            id = url_encode::url_encode(id)
        );
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&priority_dto);

        // send request
        let resp = req_builder.send().await?;

        resp.error_for_status()?.json().await?;
        Ok(())
    }

    async fn set_external_task_resource_retries(
        &self,
        id: &str,
        retries_dto: Option<crate::models::RetriesDto>,
    ) -> Result<(), CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/external-task/{id}/retries",
            configuration.base_path,
            id = url_encode::url_encode(id)
        );
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&retries_dto);

        // send request
        let resp = req_builder.send().await?;

        resp.error_for_status()?.json().await?;
        Ok(())
    }

    async fn set_external_task_retries(
        &self,
        set_retries_for_external_tasks_dto: Option<crate::models::SetRetriesForExternalTasksDto>,
    ) -> Result<(), CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/external-task/retries", configuration.base_path);
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&set_retries_for_external_tasks_dto);

        // send request
        let resp = req_builder.send().await?;

        resp.error_for_status()?.json().await?;
        Ok(())
    }

    async fn set_external_task_retries_async_operation(
        &self,
        set_retries_for_external_tasks_dto: Option<crate::models::SetRetriesForExternalTasksDto>,
    ) -> Result<crate::models::BatchDto, CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/external-task/retries-async", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&set_retries_for_external_tasks_dto);

        // send request
        let resp = req_builder.send().await?;

        Ok(resp.error_for_status()?.json().await?)
    }

    async fn unlock(&self, id: &str) -> Result<(), CamundaClientError> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/external-task/{id}/unlock",
            configuration.base_path,
            id = url_encode::url_encode(id)
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let resp = req_builder.send().await?;

        resp.error_for_status()?.json().await?;
        Ok(())
    }
}
