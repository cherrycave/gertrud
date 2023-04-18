use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionUpdate {
    pub subscriptions: Vec<SubscriptionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscriptionType {
    PlayerSend,
}
