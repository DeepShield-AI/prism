use aya_ebpf::{
	macros::perf_event, programs::PerfEventContext,
};
use aya_log_ebpf::info;
// use ebpf_common::{co_re::task_struct, core_read_kernel};

#[perf_event]
fn nr_csw(ctx: PerfEventContext) -> u32 {
	// let task = task_struct::current();
	// match core_read_kernel!(task, se, cfs_rq, rq, nr_switches) {
	// 	Ok(se) => debug!(&ctx, "here2: {}", se),
	// 	Err(_) => debug!(&ctx, "here2: error reading nr_switches"),
	// }
    info!(&ctx, "here");
	0
}
