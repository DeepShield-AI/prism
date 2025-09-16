use prism_macros::ProcParser;
use uom::si::f64::Information;

#[derive(ProcParser)]
#[fmt = "kv"]
struct MemInfo {
	#[key = "MemTotal"]
	mem_total: Option<Information>,

	#[key = "MemFree"]
	mem_free: Option<Information>,

	#[key = "MemAvailable"]
	mem_available: Option<Information>,

	#[key = "Buffers"]
	buffers: Option<Information>,

	#[key = "Cached"]
	cached: Option<Information>,

	// Handle complex field names with parentheses
	inactive: Option<Information>,

	// This will automatically map to "Active(anon)" in /proc/meminfo
	active_anon: Option<Information>,

	// This will automatically map to "Inactive(anon)" in /proc/meminfo
	inactive_anon: Option<Information>,

	// This will automatically map to "Active(file)" in /proc/meminfo
	active_file: Option<Information>,

	// This will automatically map to "Inactive(file)" in /proc/meminfo
	inactive_file: Option<Information>,
}

fn main() {
	let meminfo_content = r#"
MemTotal:       16384000 kB
MemFree:         8192000 kB
MemAvailable:   12288000 kB
Buffers:          512000 kB
Cached:          2048000 kB
Inactive:       21792828 kB
Active(anon):   15117428 kB
Inactive(anon):    31868 kB
Active(file):    8191712 kB
Inactive(file): 21760960 kB
"#;

	let meminfo = MemInfo::parse(meminfo_content).unwrap();

	println!("Memory Information:");
	if let Some(total) = meminfo.get_mem_total() {
		println!("Total: {} bytes", total.get::<uom::si::information::byte>());
	}
	if let Some(free) = meminfo.get_mem_free() {
		println!("Free: {} bytes", free.get::<uom::si::information::byte>());
	}
	if let Some(available) = meminfo.get_mem_available() {
		println!("Available: {} bytes", available.get::<uom::si::information::byte>());
	}
	if let Some(active_anon) = meminfo.get_active_anon() {
		println!("Active(anon): {} bytes", active_anon.get::<uom::si::information::byte>());
	}
	if let Some(inactive_anon) = meminfo.get_inactive_anon() {
		println!("Inactive(anon): {} bytes", inactive_anon.get::<uom::si::information::byte>());
	}
	if let Some(active_file) = meminfo.get_active_file() {
		println!("Active(file): {} bytes", active_file.get::<uom::si::information::byte>());
	}
	if let Some(inactive_file) = meminfo.get_inactive_file() {
		println!("Inactive(file): {} bytes", inactive_file.get::<uom::si::information::byte>());
	}
}
