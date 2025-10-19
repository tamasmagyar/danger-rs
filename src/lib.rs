pub mod danger;
pub mod message;
pub mod runner;

pub use danger::Danger;
pub use message::{DangerMessage, MessageType};
pub use runner::{CiPlatform, DangerContext};

pub fn run_danger() -> Result<(), Box<dyn std::error::Error>> {
    let context = runner::DangerContext::detect()?;
    let mut danger = Danger::new(context);
    danger.run()
}
