use prism_macros::ProcParser;
use uom::si::f64::Information;

#[derive(ProcParser, Debug)]
#[fmt = "kv"]
pub(crate) struct VisibilityTest {
	mem_total: Information,
	mem_free: Information,
	mem_available: Information,
}

fn main() {
	let sample = r#"
MemTotal: 16384000 kB
MemFree: 8192000 kB
MemAvailable: 12288000 kB
"#;

	match VisibilityTest::parse(sample) {
		Ok(result) => {
			println!("解析成功!");
			println!(
				"MemTotal: {} KB",
				result.get_mem_total().get::<uom::si::information::kilobyte>()
			);
			println!(
				"MemFree: {} KB",
				result.get_mem_free().get::<uom::si::information::kilobyte>()
			);
			println!(
				"MemAvailable: {} KB",
				result.get_mem_available().get::<uom::si::information::kilobyte>()
			);
		},
		Err(e) => {
			eprintln!("解析失败: {}", e);
		},
	}
}
