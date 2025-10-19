use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum MessageType {
    Warning,
    Failure,
    Message,
    Markdown,
}

#[derive(Debug, Clone, Serialize)]
pub struct DangerMessage {
    pub message_type: MessageType,
    pub message: String,
}

impl DangerMessage {
    pub fn new(message_type: MessageType, message: String) -> Self {
        Self {
            message_type,
            message,
        }
    }
}
