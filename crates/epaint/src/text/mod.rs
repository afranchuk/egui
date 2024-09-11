//! Everything related to text, fonts, text layout, cursors etc.

pub mod cursor;
mod font;
mod fonts;
mod string_manager;
mod text_layout;
mod text_layout_types;

/// One `\t` character is this many spaces wide.
pub const TAB_SIZE: usize = 4;

pub use {
    fonts::{FontData, FontDefinitions, FontFamily, FontId, FontTweak, Fonts, FontsImpl},
    string_manager::*,
    text_layout::layout,
    text_layout_types::*,
};

/// Suggested character to use to replace those in password text fields.
pub const PASSWORD_REPLACEMENT_CHAR: char = '•';
