use super::AgentError;
use prism_core::Module;
use prism_metric::MetricCollector;
use prism_runtime::handle;
use prism_sender::{Sender, file::FileSender};
use codec::encode::csv::CsvEncoderBuilder;
use log::{info, warn};
use tokio::{
	sync::{mpsc, watch},
	task::JoinHandle,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum State {
	Starting,
	Running,
	Terminating,
	Stopped,
	Failed,
}

pub struct Agent {
	state_tx: watch::Sender<State>,
	handle: Option<JoinHandle<Result<(), AgentError>>>,
}

impl Agent {
	pub fn new(_config_path: String) -> Result<Self, AgentError> {
		let _ = env_logger::builder().is_test(false).try_init();
		// #[cfg(feature = "ebpf")]
		// ebpf::prepare_ebpf();
		// init(config_path)?;

		let (state_tx, _rx) = watch::channel(State::Stopped);
		Ok(Self { state_tx, handle: None })
	}

	pub(crate) fn request_terminate(&self) {
		let _ = self.state_tx.send(State::Terminating);
	}

	pub(crate) fn current_state(&self) -> State {
		*self.state_tx.borrow()
	}

	pub(crate) fn subscribe_state(&self) -> watch::Receiver<State> {
		self.state_tx.subscribe()
	}
}

impl Module for Agent {
	type Config = ();
	type Error = AgentError;
	type Output = ();

	fn name(&self) -> &str {
		"Agent"
	}

	fn start(&mut self) -> Result<(), AgentError> {
		let _ = self.state_tx.send(State::Starting);

		let mut state_rx = self.state_tx.subscribe();
		let state_tx = self.state_tx.clone();

		self.handle = Some(handle().spawn(async move { run(state_tx, &mut state_rx).await }));
		info!("Starting agent");
		Ok(())
	}

	async fn stop(&mut self) -> Result<(), AgentError> {
		self.request_terminate();

		if let Some(handle) = self.handle.take() {
			if !handle.is_finished() {
				info!("Waiting for task to finish...");
			}

			match handle.await {
				Ok(Ok(())) => info!("Agent finished"),
				Ok(Err(e)) => {
					let _ = self.state_tx.send(State::Failed);
					warn!("Agent returned error: {:?}", e);
				},
				Err(e) => {
					let _ = self.state_tx.send(State::Failed);
					warn!("Agent join error: {:?}", e);
				},
			}
		}

		let _ = self.state_tx.send(State::Stopped);
		info!("Agent stopped");
		Ok(())
	}
}

async fn run(
	state_tx: watch::Sender<State>,
	state_rx: &mut watch::Receiver<State>,
) -> Result<(), AgentError> {
	let (metric_sender, metric_receiver) = mpsc::channel(1024);
	let mut metric_transport = Sender::new(
		"Metric transport",
		metric_receiver,
		FileSender::new("output/metrics.csv")?,
		CsvEncoderBuilder::new().build(),
	);
	metric_transport.start()?;

	let mut metric_collector = MetricCollector::new(metric_sender)?;
	metric_collector.start()?;

	let _ = state_tx.send(State::Running);

	loop {
		if state_rx.changed().await.is_err() {
			break;
		}
		if *state_rx.borrow() == State::Terminating {
			break;
		}
	}

	metric_collector.stop().await?;
	metric_transport.stop().await?;

	Ok(())
}
