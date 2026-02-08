#![forbid(unsafe_code)]

//! Runtime loop for terminal applications built with `nodecode-terminal-kit`.
//! The API is conceptually aligned with Bubble Tea's model/message/command shape.

mod command;
mod config;
mod error;
mod model;
mod program;

pub use command::Command;
pub use config::{ExitKeys, ProgramConfig};
pub use error::ProgramError;
pub use model::Model;
pub use program::Program;

pub mod prelude {
    pub use crate::{Command, ExitKeys, Model, Program, ProgramConfig, ProgramError};
}
