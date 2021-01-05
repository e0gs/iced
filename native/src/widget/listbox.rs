//! Provide progress feedback to your users.
use crate::event::{self, Event};
use crate::layout;
use crate::mouse;
use crate::text;
use crate::row;
use crate::scrollable;
use crate::{
    Clipboard, Element, Hasher, Layout, Length, Point, Rectangle, Widget, Scrollable, Row
};
use std::hash::Hash;

/// A bar that displays progress.
///
/// # Example
/// ```
/// # use iced_native::renderer::Null;
/// #
/// # pub type ProgressBar = iced_native::ProgressBar<Null>;
/// let value = 50.0;
///
/// ProgressBar::new(0.0..=100.0, value);
/// ```
///
/// ![Progress bar drawn with `iced_wgpu`](https://user-images.githubusercontent.com/18618951/71662391-a316c200-2d51-11ea-9cef-52758cab85e3.png)
#[allow(missing_debug_implementations)]
pub struct ListBox<'a, Message, Renderer: self::Renderer> {
    state: &'a mut State,
    on_press: Option<Message>,
	width: Length,
    height: Length,
    min_width: u32,
    min_height: u32,
    padding: u16,
    scrollable_state: scrollable::State,
    content: Vec<String>,
	style: Renderer::Style,
}

impl<'a, Message, Renderer: self::Renderer> ListBox<'a, Message, Renderer> {
    /// Creates a new [`ListBox`].
    ///
    /// It expects:
    ///   * an inclusive range of possible values
    ///   * the current value of the [`ListBox`]
    ///
    /// [`ListBox`]: struct.ListBox.html
    pub fn new(state: &'a mut State, content: Vec<String>) -> Self {
        ListBox {
            state,
            on_press: None,
            width: Length::Shrink,
            height: Length::Shrink,
            min_width: 0,
            min_height: 0,
            padding: Renderer::DEFAULT_PADDING,
            scrollable_state: scrollable::State::default(),
            content,
            style: Renderer::Style::default(),
        }
    }

    /// Sets the width of the [`ListBox`].
    ///
    /// [`ListBox`]: struct.ListBox.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`ListBox`].
    ///
    /// [`ListBox`]: struct.ListBox.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the style of the [`ListBox`].
    ///
    /// [`ListBox`]: struct.ListBox.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

/// The local state of a [`Button`].
///
/// [`Button`]: struct.Button.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    is_pressed: bool,
    active: bool,
}

impl State {
    /// Creates a new [`State`].
    ///
    /// [`State`]: struct.State.html
    pub fn new() -> State {
        State::default()
    }
}

impl<'a, Message, Renderer: self::Renderer> Widget<Message, Renderer> for ListBox<'a, Message, Renderer>
where
    Message: Clone,
    Renderer: self::Renderer + text::Renderer + row::Renderer + scrollable::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        /*
        let padding = f32::from(self.padding);
        let limits = limits
            .min_width(self.min_width)
            .min_height(self.min_height)
            .width(self.width)
            .height(self.height)
            .pad(padding);

        let mut content = self.content.layout(renderer, &limits);
        content.move_to(Point::new(padding, padding));

        let size = limits.resolve(content.size()).pad(padding);

        layout::Node::with_children(size, vec![content])
        */

        let t = Row::<(), Renderer>::new();
        
        Scrollable::<(), Renderer>::new(&mut scrollable::State::default())
            .layout(renderer, limits)
        
        /*            
        Row::<(), Renderer>::new()
            .layout(renderer, limits)
        */
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if self.on_press.is_some() {
                    let bounds = layout.bounds();

                    if bounds.contains(cursor_position) {
                        self.state.is_pressed = true;

                        return event::Status::Captured;
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if let Some(on_press) = self.on_press.clone() {
                    let bounds = layout.bounds();

                    if self.state.is_pressed {
                        self.state.is_pressed = false;

                        if bounds.contains(cursor_position) {
                            messages.push(on_press);
                        }

                        return event::Status::Captured;
                    }
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) -> Renderer::Output {
        self::Renderer::draw(
            renderer,
            defaults,
            layout.bounds(),
            cursor_position,
            self.on_press.is_none(),
            self.state.active,
            &self.style,
            &self.content,
            layout.children().next().unwrap(),
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.height.hash(state);
    }
}

/// The renderer of a [`ListBox`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`ListBox`] in your user interface.
///
/// [`ListBox`]: struct.ListBox.html
/// [renderer]: ../../renderer/index.html
pub trait Renderer: crate::Renderer {
    /// The default padding of a [`ListBox`].
    const DEFAULT_PADDING: u16;

    /// The style supported by this renderer.
    type Style: Default;

    /// The default height of a [`ListBox`].
    ///
    /// [`ListBox`]: struct.ListBox.html
    const DEFAULT_HEIGHT: u16;

    /// Draws a [`ListBox`].
    ///
    /// It receives:
    ///   * the bounds of the [`ListBox`]
    ///   * the range of values of the [`ListBox`]
    ///   * the current value of the [`ListBox`]
    ///   * maybe a specific background of the [`ListBox`]
    ///   * maybe a specific active color of the [`ListBox`]
    ///
    /// [`ListBox`]: struct.ListBox.html
    /*
    fn draw(
        &self,
        bounds: Rectangle,
        values: &[String],
        style: &Self::Style,
    ) -> Self::Output;*/

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
    ) -> Self::Output;
}

impl<'a, Message, Renderer: self::Renderer> From<ListBox<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + self::Renderer + text::Renderer + row::Renderer + scrollable::Renderer,
{
    fn from(
        list_box: ListBox<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(list_box)
    }
}




