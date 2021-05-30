pub mod apis;
pub mod errors;
pub mod models;
pub mod utils;

use crate::models::VariableValueDto;
use std::collections::HashMap;

pub type ProcessVariablesMap = HashMap<String, VariableValueDto>;
