use prism_macros::ProcParser;
use uom::si::{f64::Information, information::byte};

#[derive(ProcParser, Debug)]
#[fmt = "kv"]
struct UomTest {
	mem_total: Information,
}

fn main() {
	let sample = r#"
MemTotal: 16384000 kB
"#;

	match UomTest::parse(sample) {
		Ok(result) => {
			println!("Parsed successfully!");
			println!("MemTotal: {} bytes", result.mem_total.get::<byte>());
		},
		Err(e) => {
			eprintln!("Failed to parse: {}", e);
		},
	}
}
