use serde::{Deserialize, Serialize};

mod request;
mod error;
mod success;
mod notification;

pub use request::JsonRequest;
pub use error::JsonError;
pub use success::JsonSuccess;
pub use notification::JsonNotification;

pub fn default_jsonrpc() -> String {
	"2.0".into()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RequestId {
    String(String),
    Number(i64),
}

impl Default for RequestId {
	fn default() -> Self {
		Self::Number(0)
	}
}