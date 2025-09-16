use thiserror::Error;
#[derive(Error, Debug)]
pub enum AgentError {
	#[error("Metric error: {0}")]
	MetricError(#[from] prism_metric::MetricError),
	#[error("Send error: {0}")]
	SendError(#[from] prism_sender::SendError),
}
