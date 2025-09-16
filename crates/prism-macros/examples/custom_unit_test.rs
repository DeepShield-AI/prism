use prism_macros::ProcParser;
use uom::si::{
	f64::Information,
	information::{byte, kilobyte, megabyte},
};

#[derive(ProcParser, Debug)]
#[fmt = "kv"]
struct CustomUnitTest {
	mem_total: Information,
	#[unit = "uom::si::information::kilobyte"]
	mem_free: Information,
	#[unit = "uom::si::information::megabyte"]
	mem_available: Information,
}

fn main() {
	let sample = r#"
MemTotal: 16384000 kB
MemFree: 8192000 kB
MemAvailable: 12288 MB
"#;

	match CustomUnitTest::parse(sample) {
		Ok(result) => {
			println!("解析成功!");
			println!("MemTotal: {} bytes", result.mem_total.get::<byte>());
			println!("MemFree: {} KB", result.mem_free.get::<kilobyte>());
			println!("MemAvailable: {} MB", result.mem_available.get::<megabyte>());

			println!("\n单位转换验证:");
			println!("MemTotal: {} KB", result.mem_total.get::<kilobyte>());
			println!("MemFree: {} bytes", result.mem_free.get::<byte>());
			println!("MemAvailable: {} bytes", result.mem_available.get::<byte>());
		},
		Err(e) => {
			eprintln!("解析失败: {}", e);
		},
	}
}
