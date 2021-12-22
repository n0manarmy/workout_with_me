use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Times {
    pub times: Vec<Times>,
}