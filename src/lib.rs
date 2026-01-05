#![doc(html_root_url = "https://docs.rs/bevy_minibuffer_inspector/0.3.0")]
#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]

mod inspector_plugins;
pub(crate) use inspector_plugins::*;
mod world_inspector;
pub use world_inspector::*;
mod resource_inspector;
pub use resource_inspector::*;
mod asset_inspector;
pub use asset_inspector::*;
mod state_inspector;
pub use state_inspector::*;
mod filter_query_inspector;
pub use filter_query_inspector::*;
pub(crate) mod utils;
