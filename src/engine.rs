// use super::config::Config;
// use super::models::{DecisionResultVariableDto, DecisionVariableDto};
// use super::utils;
// use std::collections::HashMap;
// use crate::client::
// use crate::REQWESTCLIENT;
// use camunda_client::apis::client::APIClient;
// use camunda_client::apis::Error;
// use camunda_client::models::{
//     CompleteExternalTaskDto, ExternalTaskBpmnError, ExternalTaskFailureDto,
//     FetchExternalTaskTopicDto, FetchExternalTasksDto, LockedExternalTaskDto,
//     ProcessInstanceWithVariablesDto, StartProcessInstanceDto, VariableValueDto,
// };

// use super::error::WorkflowError;
// // pub type Task = LockedExternalTaskDto;
// pub struct Engine {
//     _config: Config,
//     pub api_client: APIClient,
// }

// impl Engine {
//     pub fn new(config: &Config) -> Engine {
//         let configuration = utils::new_configuration(
//             &config.base_path,
//             &(
//                 config.camunda_username.to_owned(),
//                 Some(config.camunda_password.to_owned()),
//             ),
//         );
//         Engine {
//             _config: config.clone(),
//             api_client: APIClient::new(configuration),
//         }
//     }

//     pub fn new_process_instance(
//         &self,
//         key: &str,
//         start_process_instance_dto: StartProcessInstanceDto,
//     ) -> Result<ProcessInstanceWithVariablesDto, Error> {
//         self.api_client
//             .process_definition_api()
//             .start_process_instance_by_key(key, Some(start_process_instance_dto))
//     }

//     pub fn lock_task(
//         &self,
//         topic: &str,
//         worker_id: &str,
//         max_tasks: Option<i32>,
//         lock_duration: Option<i64>,
//     ) -> Result<Vec<LockedExternalTaskDto>, Error> {
//         // let now: String = Local::now().to_string();
//         // println!(
//         //     // "Worker {:#?} has started fetch & lock at {:#?}",
//         //     worker_id, now
//         // );
//         self.api_client
//             .external_task_api()
//             .fetch_and_lock(Some(FetchExternalTasksDto {
//                 worker_id: worker_id.to_owned(),
//                 max_tasks: max_tasks,
//                 use_priority: None,
//                 async_response_timeout: Some(30000i64),
//                 topics: Some(vec![FetchExternalTaskTopicDto::new(
//                     topic.to_owned(),
//                     lock_duration,
//                 )]),
//             }))
//     }

//     pub fn complete_task(
//         &self,
//         task: &LockedExternalTaskDto,
//         worker_id: &str,
//         variables: HashMap<String, VariableValueDto>,
//     ) -> Result<(), Error> {
//         let complete_external_task_dto = CompleteExternalTaskDto {
//             worker_id: Some(worker_id.to_owned()),
//             variables: Some(variables),
//             local_variables: None,
//         };
//         self.api_client
//             .external_task_api()
//             .complete_external_task_resource(
//                 //TODO eliminate unwrap()
//                 &task.id.as_ref().unwrap()[..],
//                 Some(complete_external_task_dto),
//             )
//     }

//     pub fn release_task(&self, task: &LockedExternalTaskDto) -> Result<(), Error> {
//         self.api_client
//             .external_task_api()
//             .unlock(&task.id.as_ref().unwrap())
//     }

//     pub fn bpmn_error(
//         &self,
//         task: &LockedExternalTaskDto,
//         error: Option<ExternalTaskBpmnError>,
//     ) -> Result<(), Error> {
//         self.api_client
//             .external_task_api()
//             .handle_external_task_bpmn_error(&task.id.as_ref().unwrap(), error)
//     }

//     pub fn process_failure(
//         &self,
//         task: &LockedExternalTaskDto,
//         failure: Option<ExternalTaskFailureDto>,
//     ) -> Result<(), Error> {
//         self.api_client
//             .external_task_api()
//             .handle_failure(&task.id.as_ref().unwrap(), failure)
//     }

//     pub async fn evaluate_decision(
//         &self,
//         key: &str,
//         variables: DecisionVariableDto,
//     ) -> Result<DecisionResultVariableDto, WorkflowError> {
//         let uri_str = format!(
//             "{}/decision-definition/key/{}/evaluate",
//             self._config.base_path, key
//         );
//         let client = REQWESTCLIENT.get().unwrap().clone();
//         let mut req_builder = client.post(uri_str.as_str());

//         // if let Some(ref user_agent) = self.api_client.user_agent {
//         //     req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
//         // }
//         req_builder = req_builder.json(&variables);
//         // send request
//         let req = req_builder.build()?;

//         let response = client.execute(req).await?.error_for_status()?;

//         // println!("RESPONSE JSON {:?}", response.json().await?);
//         Ok(response.json().await?)
//     }
// }
