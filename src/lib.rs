//! Enable simple layout operations in [`embedded-graphics`]
//!
//! This crate extends [`embedded-graphics`] with tools that ease positioning of drawable objects.
//!
//! `embedded-layout` consists of three main parts:
//! - [alignments] that can be used to position two objects relative to one another
//!   * `horizontal`
//!     * `NoAlignment`, `Left`, `Right`, `Center`
//!     * `LeftToRight`, `RightToLeft`
//!   * `vertical`
//!     * `NoAlignment`, `Top`, `Bottom`, `Center`
//!     * `TopToBottom`, `BottomToTop`
//! - [layouts] that can be used to arrange multiple views
//!   * `LinearLayout`
//! - [view groups] which are collections of view objects
//!   * `Chain` to create ad-hoc collections (can hold views of different types)
//!   * `Views` to create view groups from arrays and slices (can only hold views of a single type)
//!   * `derive(ViewGroup)` to turn any plain old Rust struct into a view group
//!
//! # Views
//!
//! The term "view" refers to anything `embedded-layout` can work with. Basically, a view is an
//! object that can be displayed. [`View`] is the most basic trait in `embedded-layout`. Views
//! implement the [`View`] trait to enable translation and alignment operations on them, and also to
//! allow them to be used with layouts.
//!
//! [`View`] is implemented for [`embedded-graphics`] display objects. There's also an example about
//! how you can implement custom [`View`] objects.
//!
//! ## Examples
//!
//! The examples are based on [the `embedded-graphics` simulator]. The simulator is built on top of
//! `SDL2`. See the [simulator README] for more information.
//!
//! ### Draw some text to the center of the display
//!
//! ``` no_verify
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
//! #
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_6X9, MonoTextStyle},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     text::Text,
//! };
//! use embedded_layout::prelude::*;
//!
//! // Create a Rectangle from the display's dimensions
//! let display_area = display.bounding_box();
//!
//! let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
//!
//! Text::new("Hello!", Point::zero(), text_style)
//!     // align text to the center of the display
//!     .align_to(&display_area, horizontal::Center, vertical::Center)
//!     .draw(&mut display)
//!     .unwrap();
//! ```
//!
//! ### Use [`LinearLayout`] to arrange multiple objects
//!
//! ``` ignore
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
//! #
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_6X9, MonoTextStyle},
//!     pixelcolor::BinaryColor,
//!     prelude::*,
//!     text::Text,
//! };
//! use embedded_layout::{layout::linear::LinearLayout, prelude::*};
//!
//! let display_area = display.bounding_box();
//!
//! let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
//!
//! LinearLayout::vertical(
//!     Chain::new(Text::new("Vertical", Point::zero(), text_style))
//!         .append(Text::new("Linear", Point::zero(), text_style))
//!         .append(Text::new("Layout", Point::zero(), text_style))
//! )
//! .with_alignment(horizontal::Center)
//! .arrange()
//! .align_to(&display_area, horizontal::Center, vertical::Center)
//! .draw(&mut display)
//! .unwrap();
//! ```
//!
//! [`embedded-graphics`]: https://crates.io/crates/embedded-graphics/0.6.2
//! [the `embedded-graphics` simulator]: https://crates.io/crates/embedded-graphics-simulator/0.2.1
//! [fully qualified syntax]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
//! [`View`]: crate::View
//! [layouts]: crate::layout
//! [`LinearLayout`]: crate::layout::linear::LinearLayout
//! [simulator README]: https://github.com/jamwaffles/embedded-graphics/tree/v0.6/simulator#usage-without-sdl2
//! [alignments]: crate::align
//! [view groups]: crate::view_group

#![cfg_attr(not(test), no_std)]
// #![deny(missing_docs)]
#![deny(clippy::missing_inline_in_public_items)]
#![warn(clippy::all)]

use embedded_graphics::{prelude::Dimensions, transform::Transform};

pub mod align;
pub mod component;
pub mod layout;
pub mod padding;

/// The essentials.
pub mod prelude {
    pub use crate::{
        align::{Align, Alignment, AlignmentPosition},
        component::Component,
        padding::Padding,
        View,
    };
}

/// A `View` is a marker trait used to refer to items that can be manipulated with `embedded-layout` operations.
pub trait View: Transform + Dimensions {}

impl<T> View for T where T: Transform + Dimensions {}
