//! Allow your users to visually track the progress of a computation.
//!
//! A [`ListBox`] has a range of possible values and a current value,
//! as well as a length, height and style.
//!
//! [`ListBox`]: type.ListBox.html
use crate::Renderer;

pub use iced_graphics::listbox::{Style, StyleSheet};

/// A bar that displays progress.
///
/// This is an alias of an `iced_native` progress bar with an
/// `iced_wgpu::Renderer`.
pub type ListBox = iced_native::ListBox<Renderer>;
