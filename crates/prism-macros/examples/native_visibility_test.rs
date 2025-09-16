use prism_macros::ProcParser;
use uom::si::f64::Information;

// 私有结构体 - 生成的方法也是私有的
#[derive(ProcParser, Debug)]
#[fmt = "kv"]
struct PrivateMemInfo {
	mem_total: Information,
	mem_free: Information,
}

// 公共结构体 - 生成的方法是公共的
#[derive(ProcParser, Debug)]
#[fmt = "kv"]
pub struct PublicMemInfo {
	mem_total: Information,
	mem_free: Information,
}

// crate级别可见的结构体 - 生成的方法也是crate级别可见
#[derive(ProcParser, Debug)]
#[fmt = "kv"]
pub(crate) struct CrateMemInfo {
	mem_total: Information,
	mem_free: Information,
}

// 模块内可见的结构体 - 生成的方法也是模块内可见
#[derive(ProcParser, Debug)]
#[fmt = "kv"]
pub(self) struct SelfMemInfo {
	mem_total: Information,
	mem_free: Information,
}

fn main() {
	let sample = r#"
MemTotal: 16384000 kB
MemFree: 8192000 kB
"#;

	// 测试私有结构体
	let private_info = PrivateMemInfo::parse(sample).unwrap();
	println!("私有结构体解析成功!");
	println!(
		"MemTotal: {} KB",
		private_info.get_mem_total().get::<uom::si::information::kilobyte>()
	);

	// 测试公共结构体
	let public_info = PublicMemInfo::parse(sample).unwrap();
	println!("公共结构体解析成功!");
	println!(
		"MemTotal: {} KB",
		public_info.get_mem_total().get::<uom::si::information::kilobyte>()
	);

	// 测试crate级别结构体
	let crate_info = CrateMemInfo::parse(sample).unwrap();
	println!("Crate级别结构体解析成功!");
	println!("MemTotal: {} KB", crate_info.get_mem_total().get::<uom::si::information::kilobyte>());

	// 测试self级别结构体
	let self_info = SelfMemInfo::parse(sample).unwrap();
	println!("Self级别结构体解析成功!");
	println!("MemTotal: {} KB", self_info.get_mem_total().get::<uom::si::information::kilobyte>());
}
