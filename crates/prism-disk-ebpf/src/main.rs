#![cfg_attr(not(test), no_std, no_main)]

pub use prism_disk_ebpf;

#[allow(dead_code)]
#[cfg_attr(not(test), panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	loop {}
}
