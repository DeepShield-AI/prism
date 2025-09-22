use bytes::BytesMut;
use prism_event::metric::Metric;

/// A abstraction for sending data and serialize data
pub trait Sendable: Send + 'static {
	// Encode data to bytes stream and wait for sender to send
	fn encode(&self, _: &mut BytesMut) -> Result<(), std::io::Error> {
		Ok(())
	}
}

pub trait Sender<S: Sendable>: Send + 'static {
	type Error;
	/// Sends a message. Cache the message to encoder buffer.
	fn send(&mut self, message: BytesMut) -> impl Future<Output = Result<(), Self::Error>> + Send;
	/// Flushes the encoder buffer and sends the data.
	/// This is usually called when the buffer is full or when the transport is stopped.
	async fn flush(&mut self) -> Result<(), Self::Error>;
}

impl Sendable for Metric {
	fn encode(&self, o: &mut BytesMut) -> Result<(), std::io::Error> {
		self.encode(o)
	}
}

// TODO: change metric send logic to
impl Sendable for Vec<Metric> {}
