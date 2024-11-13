mod health_check;
mod subscriptions;

pub use health_check::*;
pub use subscriptions::*;

#[derive(serde::Serialize)]
pub struct Response {
    pub status: String,
    pub message: String,
}

pub fn create_response<S: Into<String>, M: Into<String>>(status: S, message: M) -> Response {
    Response {
        status: status.into(),
        message: message.into(),
    }
}
