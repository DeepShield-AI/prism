use fake::{Dummy, Fake, Faker};

pub struct FakeDiskStats(pub Vec<FakeDiskStat>);

#[derive(Debug, Dummy, Clone)]
pub struct FakeDiskStat {
	/// major number
	pub major: u64,
	/// minor number
	pub minor: u64,
	/// device name
	pub name: String,
	/// reads completed successfully
	pub read_completed: u64,
	/// reads merged
	pub read_merged: u64,
	/// sectors read
	#[dummy(faker = "1..=1_000_000_000")]
	pub sectors_read: u64,
	/// time spent reading
	pub read_time: u64,
	/// writes completed
	pub write_completed: u64,
	/// writes merged
	pub write_merged: u64,
	/// sectors written
	#[dummy(faker = "1..=1_000_000_000")]
	pub sectors_written: u64,
	/// time spent writing
	pub writing_time: u64,
	/// I/Os currently in progress
	pub ios_in_progress: u64,
	/// time spent doing I/Os
	pub io_time: u64,
	/// weighted time spent doing I/Os
	pub weighted_io_time: u64,
	/// discards completed successfully (kernel 4.18+)
	pub discard_completed: u64,
	/// discards merged (kernel 4.18+)
	pub discard_merged: u64,
	/// sectors discarded (kernel 4.18+)
	#[dummy(faker = "1..=1_000_000_000")]
	pub sectors_discarded: u64,
	/// time spent discarding (kernel 4.18+)
	pub discarding_time: u64,
	/// flush requests completed successfully (kernel 5.5+)
	pub flush_completed: u64,
	/// time spent flushing (kernel 5.5+)
	pub flushing_time: u64,
}

impl FakeDiskStats {
	pub fn generate() -> Self {
		Self((Faker, 1..=10).fake())
	}
}

impl ToString for FakeDiskStats {
	fn to_string(&self) -> String {
		self.0
			.clone()
			.into_iter()
			.map(|f| {
				format!(
					"{:>4} {:>7} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
					f.major,
					f.minor,
					f.name,
					f.read_completed,
					f.read_merged,
					f.sectors_read,
					f.read_time,
					f.write_completed,
					f.write_merged,
					f.sectors_written,
					f.writing_time,
					f.ios_in_progress,
					f.io_time,
					f.weighted_io_time,
					f.discard_completed,
					f.discard_merged,
					f.sectors_discarded,
					f.discarding_time,
					f.flush_completed,
					f.flushing_time,
				)
			})
			.collect::<Vec<_>>()
			.join("\n")
	}
}
