/// Errors returned by the runtime program loop.
#[derive(Debug, thiserror::Error)]
pub enum ProgramError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}
