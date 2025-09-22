#![recursion_limit = "256"] // for async-stream
#![cfg_attr(docsrs, feature(doc_cfg), deny(rustdoc::broken_intra_doc_links))]

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
pub use agent::Agent;
pub use error::AgentError;
pub use prism_core::Module;

mod agent;
// #[cfg(feature = "ebpf")]
// pub(crate) mod ebpf;
mod error;
// pub(crate) mod utils;

static APP_NAME_SLUG: std::sync::OnceLock<String> = std::sync::OnceLock::new();

pub fn get_app_name() -> &'static str {
	option_env!("VECTOR_APP_NAME").unwrap_or("prism")
}

pub fn get_slugified_app_name() -> String {
	APP_NAME_SLUG
		.get_or_init(|| get_app_name().to_lowercase().replace(' ', "-"))
		.clone()
}
