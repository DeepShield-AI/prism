use prism_macros::ProcParser;
use uom::si::f64::Information;

// Test basic functionality - this should work
#[derive(Debug, Clone, ProcParser)]
#[fmt = "kv"]
pub struct TestBasicAllow {
	MemTotal: Information,
	MemFree: Information,
}

fn main() {
	println!("Meta attributes test example");
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_basic_functionality() {
		// Test that the structs can be instantiated and methods work
		let test_data = "MemTotal: 1000 kB\nMemFree: 500 kB\n";

		// This should compile without warnings about non_snake_case
		// because the #[allow(non_snake_case)] attribute is properly propagated
		let result = TestLintAttrs::parse(test_data);
		assert!(result.is_ok());

		let info = result.unwrap();
		// These getter methods should exist and work
		let _total = info.get_MemTotal();
		let _free = info.get_MemFree();
	}

	#[cfg(target_os = "linux")]
	#[test]
	fn test_cfg_conditional() {
		// This test should only compile on Linux due to #[cfg(target_os = "linux")]
		let test_data = "MemTotal: 1000 kB\nMemFree: 500 kB\n";
		let result = TestCfgAttr::parse(test_data);
		assert!(result.is_ok());
	}

	#[test]
	fn test_combined_attributes() {
		let test_data = "MemTotal: 1000 kB\nMemFree: 500 kB\nMemAvailable: 800 kB\n";

		#[allow(deprecated)]
		let result = TestCombinedAttrs::parse(test_data);
		assert!(result.is_ok());

		let info = result.unwrap();
		let _total = info.get_MemTotal();
		let _free = info.get_MemFree();
		let _available = info.get_MemAvailable();
	}
}
