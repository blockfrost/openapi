use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Network {
    #[serde(rename = "supply")]
    pub supply: Box<models::NetworkSupply>,
    #[serde(rename = "stake")]
    pub stake: Box<models::NetworkStake>,
}

impl Network {
    pub fn new(supply: models::NetworkSupply, stake: models::NetworkStake) -> Network {
        Network {
            supply: Box::new(supply),
            stake: Box::new(stake),
        }
    }
}

