use std::collections::HashMap;

use models::VariableValueDto;

#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod models;

pub type ProcessVariablesMap = HashMap<String, VariableValueDto>;
