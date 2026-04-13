/// Represents all errors that can occur during project generation
#[derive(Debug, thiserror::Error)]
pub enum SpmError {
	#[error("I/O error: {0}")]
	Io(#[from] std::io::Error),
	#[error("failed to write file: {0}")]
	Write(String),
	#[error("operation interrupted by user")]
	Interrupted,
}
