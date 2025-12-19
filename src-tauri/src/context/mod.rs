pub mod app_info;
mod auto_tag;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(test)]
mod tests;

pub use auto_tag::generate_auto_tags;
