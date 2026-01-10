pub mod shortcut_handler;
pub mod shortcut_mapper;
pub mod shortcut_register;

pub use shortcut_handler::handle_quick_picker_shortcut;
pub use shortcut_mapper::shortcut_from_config;
pub use shortcut_register::register_quick_picker_shortcut;
