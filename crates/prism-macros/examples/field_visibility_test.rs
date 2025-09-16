use prism_macros::ProcParser;
use uom::si::f64::Information;

// 测试字段可见性支持
#[derive(ProcParser, Debug)]
#[fmt = "kv"]
pub struct FieldVisibilityTest {
	// 公共字段 - 生成公共getter方法
	pub mem_total: Information,

	// crate级别可见字段 - 生成crate级别getter方法
	pub(crate) mem_free: Information,

	// 私有字段 - 生成私有getter方法
	mem_available: Information,

	// 模块内可见字段 - 生成模块内可见getter方法
	pub(self) mem_cached: Information,
}

fn main() {
	let sample = r#"
MemTotal: 16384000 kB
MemFree: 8192000 kB
MemAvailable: 12288000 kB
MemCached: 4096000 kB
"#;

	let info = FieldVisibilityTest::parse(sample).unwrap();
	println!("字段可见性测试解析成功!");

	// 测试不同可见性级别的getter方法
	println!("MemTotal (pub): {} KB", info.get_mem_total().get::<uom::si::information::kilobyte>());
	println!(
		"MemFree (pub(crate)): {} KB",
		info.get_mem_free().get::<uom::si::information::kilobyte>()
	);
	println!(
		"MemAvailable (private): {} KB",
		info.get_mem_available().get::<uom::si::information::kilobyte>()
	);
	println!(
		"MemCached (pub(super)): {} KB",
		info.get_mem_cached().get::<uom::si::information::kilobyte>()
	);
}
