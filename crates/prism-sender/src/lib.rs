use prism_core::{Module, sender::Sendable};
use prism_runtime::handle;
use bytes::BytesMut;
use codec::encode::Encoder;
pub use error::SendError;
use log::{debug, info, warn};
use std::sync::{
	Arc,
	atomic::{AtomicBool, Ordering},
};
use tokio::{sync::mpsc::Receiver, task::JoinHandle};
mod error;
pub mod file;

pub struct Sender<T, S, E>
where
	T: Sendable,
	S: prism_core::sender::Sender<T>,
	E: Encoder<T>,
{
	name: &'static str,
	running: Arc<AtomicBool>,
	handle: Option<JoinHandle<Result<(), SendError>>>,
	receiver: Option<Receiver<T>>,
	sender: Option<S>,
	encoder: Option<E>,
}

impl<T, S, E> Sender<T, S, E>
where
	T: Sendable,
	S: prism_core::sender::Sender<T>,
	E: Encoder<T>,
{
	pub fn new(name: &'static str, receiver: Receiver<T>, sender: S, encoder: E) -> Self {
		Self {
			name,
			running: Arc::new(AtomicBool::new(false)),
			// config: (),
			receiver: Some(receiver),
			sender: Some(sender),
			encoder: Some(encoder),
			handle: None,
		}
	}
}

impl<T, S, E> Module for Sender<T, S, E>
where
	T: Sendable,
	S: prism_core::sender::Sender<T>,
	E: Encoder<T>,
	SendError: From<<E as Encoder<T>>::Error> + From<<S as prism_core::sender::Sender<T>>::Error>,
{
	type Config = ();
	type Error = SendError;
	type Output = ();

	fn name(&self) -> &str {
		self.name
	}

	fn start(&mut self) -> Result<Self::Output, Self::Error> {
		if self.running.swap(true, Ordering::Relaxed) {
			warn!("{} sender is already running.", self.name);
			return Ok(());
		}
		let running = Arc::clone(&self.running);
		let mut receiver = self.receiver.take().unwrap();
		let mut sender = self.sender.take().unwrap();
		let mut encoder = self.encoder.take().unwrap();
		self.handle = Some(handle().spawn(async move {
			while running.load(Ordering::Relaxed) {
				match receiver.recv().await {
					Some(message) => {
						debug!("Sending message");
						let mut encoded = BytesMut::new();
						encoder.encode(message, &mut encoded)?;
						debug!("Encoded message: {:?}", encoded);
						sender.send(encoded).await?;
					},
					None => {
						warn!("Receiver disconnected, stopping transport.");
						break;
					},
				}
			}
			Ok(())
		}));
		info!("{} sender started.", self.name);
		Ok(())
	}

	async fn stop(&mut self) -> Result<Self::Output, Self::Error> {
		if !self.running.swap(false, Ordering::Relaxed) {
			warn!("{} sender is already stopped.", self.name);
			return Ok(());
		}

		if let Some(thread) = self.handle.take() {
			thread
				.await
				.unwrap_or_else(|_| panic!("Failed to join {} sender thread", self.name))?;
		}
		info!("{} sender stopped.", self.name);
		Ok(())
	}
}
