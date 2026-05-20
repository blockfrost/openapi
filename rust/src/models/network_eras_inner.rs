use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkErasInner {
    #[serde(rename = "start")]
    pub start: Box<models::NetworkErasInnerStart>,
    #[serde(rename = "end")]
    pub end: Box<models::NetworkErasInnerEnd>,
    #[serde(rename = "parameters")]
    pub parameters: Box<models::NetworkErasInnerParameters>,
}

impl NetworkErasInner {
    pub fn new(start: models::NetworkErasInnerStart, end: models::NetworkErasInnerEnd, parameters: models::NetworkErasInnerParameters) -> NetworkErasInner {
        NetworkErasInner {
            start: Box::new(start),
            end: Box::new(end),
            parameters: Box::new(parameters),
        }
    }
}

