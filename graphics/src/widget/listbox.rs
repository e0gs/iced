//! Allow your users to visually track the progress of a computation.
//!
//! A [`ProgressBar`] has a range of possible values and a current value,
//! as well as a length, height and style.
//!
//! [`ProgressBar`]: type.ProgressBar.html
use crate::{Backend, Point, Layout, Primitive, Renderer};
use iced_native::mouse;
use iced_native::listbox;
use iced_native::{Color, Rectangle};

pub use iced_style::listbox::{Style, StyleSheet};

/// A bar that displays progress.
///
/// This is an alias of an `iced_native` progress bar with an
/// `iced_wgpu::Renderer`.
pub type ListBox<'a, Message, Backend> = iced_native::ListBox<'a, Message, Renderer<Backend>>;

impl<B> listbox::Renderer for Renderer<B>
where
    B: Backend,
{
    const DEFAULT_PADDING: u16 = 5;

    type Style = Box<dyn StyleSheet>;

    const DEFAULT_HEIGHT: u16 = 30;

    fn draw(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        cursor_position: Point,
        is_disabled: bool,
        is_pressed: bool,
        style: &Self::Style,
        content: &Vec<String>,
        content_layout: Layout<'_>,
    ) -> Self::Output {
        let style = style_sheet.style();

        let background = Primitive::Group {
            primitives: vec![Primitive::Quad {
                bounds: Rectangle { ..bounds },
                background: style.background,
                border_radius: style.border_radius,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            }],
        };

        (
            background,
            mouse::Interaction::default(),
        )
    }
}
