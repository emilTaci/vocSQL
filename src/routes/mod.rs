mod health_check;
mod subscriptions;

pub use health_check::*;
pub use subscriptions::*;

#[derive(serde::Serialize)]
pub struct Response {
    pub status: String,
    pub message: String,
}

pub fn create_response(status: &str, message: &str) -> Response {
    Response {
        status: status.to_string(),
        message: message.to_string(),
    }
}
