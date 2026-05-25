use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NotificationMethod {
    Standard(StandardNotificationMethod),
    Other(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StandardNotificationMethod {
    #[serde(rename = "notifications/initialized")]
    Initialized,
}
