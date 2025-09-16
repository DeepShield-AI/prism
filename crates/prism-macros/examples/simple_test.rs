use prism_macros::ProcParser;

#[derive(ProcParser, Debug)]
#[fmt = "kv"]
struct SimpleTest {
	mem_total: u64,
	mem_free: u64,
}

fn main() {
	let sample = r#"
MemTotal: 16384000 kB
MemFree: 8192000 kB
"#;

	match SimpleTest::parse(sample) {
		Ok(result) => {
			println!("Parsed successfully!");
			println!("MemTotal: {}", result.mem_total);
			println!("MemFree: {}", result.mem_free);
		},
		Err(e) => {
			eprintln!("Failed to parse: {}", e);
		},
	}
}
