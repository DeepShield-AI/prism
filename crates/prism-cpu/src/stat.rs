use prism_macros::ProcParser;
use prism_metric_common::procfs_root;
use prism_metric_utils::read_to_string;
use uom::si::f64::Time;

use crate::{CpuMetricError, utils::clock_ticks_to_seconds};

/// CPU statistics from /proc/stat
/// Each CPU line contains: user nice system idle iowait irq softirq steal guest guest_nice
#[derive(ProcParser)]
#[fmt = "table"]
pub struct CpuTime {
    /// Time spent in user mode
    #[arg(with = clock_ticks_to_seconds)]
    user: Time,
    /// Time spent in user mode with low priority (nice)
    #[arg(with = clock_ticks_to_seconds)]
    nice: Time,
    /// Time spent in system mode
    #[arg(with = clock_ticks_to_seconds)]
    system: Time,
    /// Time spent in idle task
    #[arg(with = clock_ticks_to_seconds)]
    idle: Time,
    /// Time spent waiting for I/O to complete
    #[arg(with = clock_ticks_to_seconds)]
    iowait: Time,
    /// Time spent servicing hardware interrupts
    #[arg(with = clock_ticks_to_seconds)]
    irq: Time,
    /// Time spent servicing software interrupts
    #[arg(with = clock_ticks_to_seconds)]
    softirq: Time,
    /// Time stolen by other operating systems running in a virtual environment
    #[arg(with = clock_ticks_to_seconds)]
    steal: Time,
    /// Time spent running a virtual CPU for guest operating systems
    #[arg(with = clock_ticks_to_seconds)]
    guest: Time,
    /// Time spent running a niced guest
    #[arg(with = clock_ticks_to_seconds)]
    guest_nice: Time,
}

/// Overall system statistics from /proc/stat
pub struct Stat {
    /// Aggregate CPU statistics
    cpu_total: CpuTime,
    /// Per-CPU statistics
    cpus: Vec<CpuTime>,
    /// Total number of context switches
    context_switches: u64,
    /// Boot time in seconds since Unix epoch
    boot_time: u64,
    /// Total number of processes created
    processes: u64,
    /// Number of processes currently running
    procs_running: u64,
    /// Number of processes currently blocked
    procs_blocked: u64,
}

impl Stat {
    pub fn context_switches(&self) -> u64 {
        self.context_switches
    }

    pub fn boot_time(&self) -> u64 {
        self.boot_time
    }

    pub fn processes(&self) -> u64 {
        self.processes
    }

    pub fn procs_running(&self) -> u64 {
        self.procs_running
    }

    pub fn procs_blocked(&self) -> u64 {
        self.procs_blocked
    }

    pub fn cpu_times(&self) -> impl IntoIterator<Item = (usize, &CpuTime)> {
        self.cpus.iter().enumerate()
    }

    fn parse(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
        use nom::{
            bytes::complete::take_while1,
            character::complete::{digit1, multispace1},
            combinator::map_res,
            sequence::preceded,
            IResult, Parser,
        };

        let mut cpu_total = None;
        let mut cpus = Vec::new();
        let mut context_switches = 0;
        let mut boot_time = 0;
        let mut processes = 0;
        let mut procs_running = 0;
        let mut procs_blocked = 0;

        fn parse_key_value(input: &str) -> IResult<&str, (&str, u64)> {
            (
                take_while1(|c: char| c.is_alphanumeric() || c == '_'),
                preceded(
                    multispace1,
                    map_res(digit1, |s: &str| s.parse::<u64>())
                )
            ).parse(input)
        }

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with("cpu") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 11 {
                    let cpu_data = parts[1..].join(" ");
                    if parts[0] == "cpu" {
                        if let Ok(mut parsed_cpus) = CpuTime::parse_all(&cpu_data) {
                            if let Some(cpu) = parsed_cpus.pop() {
                                cpu_total = Some(cpu);
                            }
                        }
                    } else if parts[0].len() > 3 && parts[0].starts_with("cpu") {
                        if let Ok(mut parsed_cpus) = CpuTime::parse_all(&cpu_data) {
                            if let Some(cpu) = parsed_cpus.pop() {
                                cpus.push(cpu);
                            }
                        }
                    }
                }
            } else {
                if let Ok((_, (key, value))) = parse_key_value(line) {
                    match key {
                        "ctxt" => context_switches = value,
                        "btime" => boot_time = value,
                        "processes" => processes = value,
                        "procs_running" => procs_running = value,
                        "procs_blocked" => procs_blocked = value,
                        _ => {}
                    }
                }
            }
        }

        let cpu_total = cpu_total.ok_or("Missing aggregate CPU statistics")?;

        Ok(Stat {
            cpu_total,
            cpus,
            context_switches,
            boot_time,
            processes,
            procs_running,
            procs_blocked,
        })
    }
}

pub async fn stat() -> Result<Stat, CpuMetricError> {
    let content = read_to_string(procfs_root().join("stat")).await?;
    Stat::parse(&content).map_err(Into::into)
}