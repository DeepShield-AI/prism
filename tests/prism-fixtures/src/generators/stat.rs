use fake::{Dummy, Fake, Faker};

#[derive(Debug, Dummy, Clone)]
pub struct FakeCpuTime {
	/// Time spent in user mode
	pub user: u64,
	/// Time spent in user mode with low priority (nice)
	pub nice: u64,
	/// Time spent in system mode
	pub system: u64,
	/// Time spent in idle task
	pub idle: u64,
	/// Time spent waiting for I/O to complete
	pub iowait: u64,
	/// Time spent servicing hardware interrupts
	pub irq: u64,
	/// Time spent servicing software interrupts
	pub softirq: u64,
	/// Time stolen by other operating systems running in a virtual environment
	pub steal: u64,
	/// Time spent running a virtual CPU for guest operating systems
	pub guest: u64,
	/// Time spent running a niced guest
	pub guest_nice: u64,
}

pub struct FakeStat {
	/// Aggregate CPU statistics
	pub cpu_total: FakeCpuTime,
	/// Per-CPU statistics
	pub cpus: Vec<FakeCpuTime>,
	/// Total number of context switches
	pub context_switches: u64,
	/// Boot time in seconds since Unix epoch
	pub boot_time: u64,
	/// Total number of processes created
	pub processes: u64,
	/// Number of processes currently running
	pub procs_running: u64,
	/// Number of processes currently blocked
	pub procs_blocked: u64,
}

impl FakeStat {
	pub fn generate() -> Self {
		Self {
			cpu_total: Faker.fake(),
			cpus: (Faker, 1..16).fake(),
			context_switches: Faker.fake(),
			boot_time: Faker.fake(),
			processes: Faker.fake(),
			procs_running: Faker.fake(),
			procs_blocked: Faker.fake(),
		}
	}
}

impl ToString for FakeStat {
	fn to_string(&self) -> String {
		let mut result = String::new();
		result.push_str(&format!(
			"cpu  {} {} {} {} {} {} {} {} {} {}\n",
			self.cpu_total.user,
			self.cpu_total.nice,
			self.cpu_total.system,
			self.cpu_total.idle,
			self.cpu_total.iowait,
			self.cpu_total.irq,
			self.cpu_total.softirq,
			self.cpu_total.steal,
			self.cpu_total.guest,
			self.cpu_total.guest_nice
		));
		let cpus = self
			.cpus
			.clone()
			.into_iter()
			.enumerate()
			.map(|(i, cpu)| {
				format!(
					"cpu{}  {} {} {} {} {} {} {} {} {} {}",
					i,
					cpu.user,
					cpu.nice,
					cpu.system,
					cpu.idle,
					cpu.iowait,
					cpu.irq,
					cpu.softirq,
					cpu.steal,
					cpu.guest,
					cpu.guest_nice
				)
			})
			.collect::<Vec<_>>()
			.join("\n");
		result.push_str(&cpus);
		result.push_str("\nintr 5626640454 54 0 0 0 0 0 0 0 0 3 913 0 0 0 0 0 0 0 3337290 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 341 783056 837883 845154 873105 873631 890524 799746 857240 817852 864770 832325 867967 819385 911580 878128 949096 888977 879773 829688 791239 850308 855681 840074 828653 419310 0 0 0 0 209610 0 0 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 0 0 0 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 0 0 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 0 204094 1 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 0 0 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 0 0 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 0 157088808 204000321 5167367 5035222 204034507 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n");
		result.push_str(&format!("ctxt {}\n", self.context_switches));
		result.push_str(&format!("btime {}\n", self.boot_time));
		result.push_str(&format!("processes {}\n", self.processes));
		result.push_str(&format!("procs_running {}\n", self.procs_running));
		result.push_str(&format!("procs_blocked {}\n", self.procs_blocked));
		result.push_str(
			"softirq 4679053537 3 217228708 171684 2852529223 217514 0 1920121 752158546 1247 854826491",
		);
		result
	}
}
