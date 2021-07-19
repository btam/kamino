#![cfg_attr(not(debug_assertions), deny(warnings))] // Warnings-as-errors mode for release builds

mod command_error;
pub use command_error::*;

mod macros;
pub use self::macros::*;
