use crate::{DangerContext, DangerMessage, MessageType};

pub struct Danger {
    context: DangerContext,
    messages: Vec<DangerMessage>,
}

impl Danger {
    pub fn new(context: DangerContext) -> Self {
        Self {
            context,
            messages: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let dangerfile_path = self.find_dangerfile()?;
        println!("Dangerfile path: {}", dangerfile_path);
        Ok(())
    }

    pub fn warn(&mut self, message: &str) {
        self.messages.push(DangerMessage::new(
            MessageType::Warning,
            message.to_string(),
        ));
    }

    pub fn fail(&mut self, message: &str) {
        self.messages.push(DangerMessage::new(
            MessageType::Failure,
            message.to_string(),
        ));
    }

    pub fn message(&mut self, message: &str) {
        self.messages.push(DangerMessage::new(
            MessageType::Message,
            message.to_string(),
        ));
    }

    pub fn markdown(&mut self, message: &str) {
        self.messages.push(DangerMessage::new(
            MessageType::Markdown,
            message.to_string(),
        ));
    }

    pub fn git_diff(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok("git diff here".to_string())
    }

    fn find_dangerfile(&self) -> Result<String, Box<dyn std::error::Error>> {
        let possible_paths = ["dangerfile.rs", "./danger/dangerfile.rs"];

        for path in possible_paths {
            if std::path::Path::new(path).exists() {
                return Ok(path.to_string());
            }
        }

        Err("No Dangerfile.rs found".into())
    }

    pub fn get_messages(&self) -> &[DangerMessage] {
        &self.messages
    }
}
