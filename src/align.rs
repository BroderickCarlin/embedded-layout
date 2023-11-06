//! Alignment operations
//!
//! Alignment operations are used to arrange two [`View`]s relative to each other. A single [`align_*`]
//! call requires both a `horizontal` and a `vertical` alignment parameter.
//!
//! The list of currently supported alignments:
//!  - [`horizontal`]
//!    - `NoAlignment`, `Left`, `Center`, `Right`
//!    - `LeftToRight`
//!    - `RightToLeft`
//!  - [`vertical`]
//!    - `NoAlignment`, `Top`, `Center`, `Bottom`
//!    - `TopToBottom`
//!    - `BottomToTop`
//!
//! Alignment works by calling [`align_to`] or [`align_to_mut`] on an object that implements
//! the [`Align`] trait. The call needs a second [`View`] to align to, called the reference [`View`],
//! and two alignment parameters. The second [`View`] will not be translated by the alignment
//! operation.
//!
//! [`horizontal`]: crate::align::horizontal
//! [`vertical`]: crate::align::vertical
//! [`align_*`]: crate::align::Align
//! [`align_to`]: crate::align::Align::align_to
//! [`align_to_mut`]: crate::align::Align::align_to_mut
use crate::View;

use embedded_graphics::{prelude::Point, primitives::Rectangle};

/// This trait enables alignment operations for [`View`] objects
///
/// This trait is blanket-implemented for all objects that implement [`View`].
///
/// For more information, see the [module level documentation](crate::align)
pub trait Align {
    /// Return the object aligned to an other one using the alignment parameters as rules
    fn align_to<R>(self, reference: &R, alignment: &Alignment) -> Self
    where
        R: View;

    /// Return the object aligned to an other one using the alignment parameters as rules
    fn align_to_mut<R>(&mut self, reference: &R, alignment: &Alignment) -> &mut Self
    where
        R: View;
}

impl<T> Align for T
where
    T: View,
{
    #[inline]
    fn align_to<R>(mut self, reference: &R, alignment: &Alignment) -> Self
    where
        R: View,
    {
        self.align_to_mut(reference, alignment);
        self
    }

    #[inline]
    fn align_to_mut<R>(&mut self, reference: &R, alignment: &Alignment) -> &mut Self
    where
        R: View,
    {
        let self_bounds = self.bounding_box();
        let reference_bounds = reference.bounding_box();

        self.translate_mut(alignment.offset(self_bounds, reference_bounds));
        self
    }
}

/// TODO: Add docs
pub enum AlignmentPosition {
    /// In horizontal alignment, `Start` would be left aligned. In vertical alignment, `Start` would be top aligned
    Start,
    /// Aligns the center of two elements along an axis assumed through contextural usage
    Center,
    /// In horizontal alignment, `End` would be right aligned. In vertical alignment, `End` would be bottom aligned
    End,
    /// In horizontal alignment, `Before` would align the right edge of our
    Before,
    After,
}

pub struct Alignment {
    pub vertical: Option<AlignmentPosition>,
    pub horizontal: Option<AlignmentPosition>,
}

impl Alignment {
    #[inline]
    pub fn bidirectional(vertical: AlignmentPosition, horizontal: AlignmentPosition) -> Self {
        Self {
            vertical: Some(vertical),
            horizontal: Some(horizontal),
        }
    }

    #[inline]
    pub fn horizontal(horizontal: AlignmentPosition) -> Self {
        Self {
            vertical: None,
            horizontal: Some(horizontal),
        }
    }

    #[inline]
    pub fn vertical(vertical: AlignmentPosition) -> Self {
        Self {
            vertical: Some(vertical),
            horizontal: None,
        }
    }

    #[inline]
    pub fn center() -> Self {
        Self {
            vertical: Some(AlignmentPosition::Center),
            horizontal: Some(AlignmentPosition::Center),
        }
    }

    #[inline]
    pub fn offset(&self, target: Rectangle, reference: Rectangle) -> Point {
        let x = match self.horizontal {
            Some(AlignmentPosition::Start) => reference.top_left.x - target.top_left.x,
            Some(AlignmentPosition::Center) => {
                (reference.top_left.x + (reference.size.width as i32 / 2))
                    - (target.top_left.x + (target.size.width as i32 / 2))
            }
            Some(AlignmentPosition::End) => {
                (reference.top_left.x + reference.size.width as i32)
                    - (target.top_left.x + target.size.width as i32)
            }
            Some(AlignmentPosition::Before) => {
                reference.top_left.x - (target.top_left.x + target.size.width as i32)
            }
            Some(AlignmentPosition::After) => {
                (reference.top_left.x + reference.size.width as i32) - target.top_left.x
            }
            None => 0,
        };

        let y = match self.vertical {
            Some(AlignmentPosition::Start) => reference.top_left.y - target.top_left.y,
            Some(AlignmentPosition::Center) => {
                (reference.top_left.y + (reference.size.height as i32 / 2))
                    - (target.top_left.y + (target.size.height as i32 / 2))
            }
            Some(AlignmentPosition::End) => {
                (reference.top_left.y + reference.size.height as i32)
                    - (target.top_left.y + target.size.height as i32)
            }
            Some(AlignmentPosition::Before) => {
                reference.top_left.y - (target.top_left.y + target.size.height as i32)
            }
            Some(AlignmentPosition::After) => {
                (reference.top_left.y + reference.size.height as i32) - target.top_left.y
            }
            None => 0,
        };

        Point::new(x, y)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use embedded_graphics::{geometry::AnchorPoint, prelude::Size};

    #[test]
    fn test_horizontal_center() {
        fn check_center_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            let center_of_reference = reference.top_left + reference.size / 2;
            let center_of_result = result.top_left + result.size / 2;

            // The size hasn't changed
            assert_eq!(result.size, source.size);

            // Horizontal coordinate matches reference
            assert_eq!(center_of_result.x, center_of_reference.x);

            // Vertical coordinate is unchanged
            assert_eq!(result.top_left.y, source.top_left.y);
        }

        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let alignment = Alignment::horizontal(AlignmentPosition::Center);

        let result = rect1.align_to(&rect2, &alignment);
        check_center_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, &alignment);
        check_center_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_left() {
        fn check_left_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            // The size hasn't changed
            assert_eq!(result.size, source.size);

            // Horizontal coordinate matches reference
            assert_eq!(result.top_left.x, reference.top_left.x);

            // Vertical coordinate is unchanged
            assert_eq!(result.top_left.y, source.top_left.y);
        }

        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let alignment = Alignment::horizontal(AlignmentPosition::Start);

        let result = rect1.align_to(&rect2, &alignment);
        check_left_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, &alignment);
        check_left_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_right() {
        fn check_right_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            // The size hasn't changed
            assert_eq!(result.size, source.size);

            // Horizontal coordinate matches reference
            assert_eq!(
                result.anchor_point(AnchorPoint::BottomRight).x,
                reference.anchor_point(AnchorPoint::BottomRight).x
            );

            // Vertical coordinate is unchanged
            assert_eq!(
                result.anchor_point(AnchorPoint::BottomRight).y,
                source.anchor_point(AnchorPoint::BottomRight).y
            );
        }

        let alignment = Alignment::horizontal(AlignmentPosition::End);

        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, &alignment);
        check_right_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, &alignment);
        check_right_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_left_to_right() {
        fn check_left_to_right_alignment(
            source: Rectangle,
            reference: Rectangle,
            result: Rectangle,
        ) {
            // The size hasn't changed
            assert_eq!(result.size, source.size);

            // Left is at right + 1
            assert_eq!(
                result.top_left.x,
                reference.anchor_point(AnchorPoint::BottomRight).x + 1
            );

            // Vertical coordinate is unchanged
            assert_eq!(
                result.anchor_point(AnchorPoint::BottomRight).y,
                source.anchor_point(AnchorPoint::BottomRight).y
            );
        }

        let alignment = Alignment::horizontal(AlignmentPosition::After);

        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let result = rect1.align_to(&rect2, &alignment);
        check_left_to_right_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, &alignment);
        check_left_to_right_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_left_to_right_empty() {
        let rect1 = Rectangle::new(Point::new(0, 0), Size::zero());
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let alignment = Alignment::horizontal(AlignmentPosition::After);

        let result = rect1.align_to(&rect2, &alignment);
        // The size hasn't changed
        assert_eq!(result.size, rect1.size);

        // Left is at right
        assert_eq!(
            result.top_left.x,
            rect2.anchor_point(AnchorPoint::BottomRight).x + 1
        );

        // Vertical coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).y,
            rect1.anchor_point(AnchorPoint::BottomRight).y
        );

        // Test the other direction
        let result = rect2.align_to(&rect1, &alignment);

        // The size hasn't changed
        assert_eq!(result.size, rect2.size);

        // Left is at right
        assert_eq!(
            result.top_left.x,
            rect1.anchor_point(AnchorPoint::BottomRight).x
        );

        // Vertical coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).y,
            rect2.anchor_point(AnchorPoint::BottomRight).y
        );
    }

    #[test]
    fn test_right_to_left() {
        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let alignment = Alignment::horizontal(AlignmentPosition::Before);

        let result = rect1.align_to(&rect2, &alignment);
        // The size hasn't changed
        assert_eq!(result.size, rect1.size);

        // Left is at right - 1
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect2.top_left.x - 1
        );

        // Vertical coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).y,
            rect1.anchor_point(AnchorPoint::BottomRight).y
        );

        // Test the other direction
        let result = rect2.align_to(&rect1, &alignment);
        // The size hasn't changed
        assert_eq!(result.size, rect2.size);

        // Left is at right + 1
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect1.top_left.x - 1
        );

        // Vertical coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).y,
            rect2.anchor_point(AnchorPoint::BottomRight).y
        );
    }

    #[test]
    fn test_right_to_left_empty() {
        let rect1 = Rectangle::new(Point::new(0, 0), Size::zero());
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let alignment = Alignment::horizontal(AlignmentPosition::Before);

        let result = rect1.align_to(&rect2, &alignment);
        // The size hasn't changed
        assert_eq!(result.size, rect1.size);

        // Left is at right
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect2.top_left.x
        );

        // Vertical coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).y,
            rect1.anchor_point(AnchorPoint::BottomRight).y
        );

        // Test the other direction
        let result = rect2.align_to(&rect1, &alignment);
        // The size hasn't changed
        assert_eq!(result.size, rect2.size);

        // Left is at right + 1
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect1.top_left.x - 1
        );

        // Vertical coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).y,
            rect2.anchor_point(AnchorPoint::BottomRight).y
        );
    }

    #[test]
    fn test_vertical_center() {
        fn check_center_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            let center_of_reference = reference.top_left + reference.size / 2;
            let center_of_result = result.top_left + result.size / 2;

            // The size hasn't changed
            assert_eq!(result.size, source.size);

            // Vertical coordinate matches reference
            assert_eq!(center_of_result.y, center_of_reference.y);

            // Horizontal coordinate is unchanged
            assert_eq!(result.top_left.x, source.top_left.x);
        }

        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let alignment = Alignment::vertical(AlignmentPosition::Center);

        let result = rect1.align_to(&rect2, &alignment);
        check_center_alignment(rect1, rect2, result);

        // Test the other direction

        let result = rect2.align_to(&rect1, &alignment);
        check_center_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_top() {
        fn check_top_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            // The size hasn't changed
            assert_eq!(result.size, source.size);

            // Vertical coordinate matches reference
            assert_eq!(result.top_left.y, reference.top_left.y);

            // Horizontal coordinate is unchanged
            assert_eq!(result.top_left.x, source.top_left.x);
        }

        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let alignment = Alignment::vertical(AlignmentPosition::Start);

        let result = rect1.align_to(&rect2, &alignment);
        check_top_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, &alignment);
        check_top_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_bottom() {
        fn check_bottom_alignment(source: Rectangle, reference: Rectangle, result: Rectangle) {
            // The size hasn't changed
            assert_eq!(result.size, source.size);

            // Vertical coordinate matches reference
            assert_eq!(
                result.anchor_point(AnchorPoint::BottomRight).y,
                reference.anchor_point(AnchorPoint::BottomRight).y
            );

            // Horizontal coordinate is unchanged
            assert_eq!(
                result.anchor_point(AnchorPoint::BottomRight).x,
                source.anchor_point(AnchorPoint::BottomRight).x
            );
        }

        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let alignment = Alignment::vertical(AlignmentPosition::End);

        let result = rect1.align_to(&rect2, &alignment);
        check_bottom_alignment(rect1, rect2, result);

        // Test the other direction
        let result = rect2.align_to(&rect1, &alignment);
        check_bottom_alignment(rect2, rect1, result);
    }

    #[test]
    fn test_top_to_bottom() {
        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let alignment = Alignment::vertical(AlignmentPosition::After);

        let result = rect1.align_to(&rect2, &alignment);
        // The size hasn't changed
        assert_eq!(result.size, rect1.size);

        // Top is at bottom + 1
        assert_eq!(
            result.top_left.y,
            rect2.anchor_point(AnchorPoint::BottomRight).y + 1
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect1.anchor_point(AnchorPoint::BottomRight).x
        );

        // Test the other direction
        let result = rect2.align_to(&rect1, &alignment);
        // The size hasn't changed
        assert_eq!(result.size, rect2.size);

        // Top is at bottom + 1
        assert_eq!(
            result.top_left.y,
            rect1.anchor_point(AnchorPoint::BottomRight).y + 1
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect2.anchor_point(AnchorPoint::BottomRight).x
        );
    }

    #[test]
    fn test_top_to_bottom_empty() {
        let rect1 = Rectangle::new(Point::new(0, 0), Size::zero());
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let alignment = Alignment::vertical(AlignmentPosition::After);

        let result = rect1.align_to(&rect2, &alignment);
        // The size hasn't changed
        assert_eq!(result.size, rect1.size);

        // Top is at bottom
        assert_eq!(
            result.top_left.y,
            rect2.anchor_point(AnchorPoint::BottomRight).y + 1
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect1.anchor_point(AnchorPoint::BottomRight).x
        );

        // Test the other direction
        let result = rect2.align_to(&rect1, &alignment);
        // The size hasn't changed
        assert_eq!(result.size, rect2.size);

        // Top is at bottom + 1
        assert_eq!(
            result.top_left.y,
            rect1.anchor_point(AnchorPoint::BottomRight).y
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect2.anchor_point(AnchorPoint::BottomRight).x
        );
    }

    #[test]
    fn test_bottom_to_top() {
        let rect1 = Rectangle::with_corners(Point::new(0, 0), Point::new(10, 10));
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let alignment = Alignment::vertical(AlignmentPosition::Before);

        let result = rect1.align_to(&rect2, &alignment);
        // The size hasn't changed
        assert_eq!(result.size, rect1.size);

        // Bottom is at top - 1
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).y,
            rect2.top_left.y - 1
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect1.anchor_point(AnchorPoint::BottomRight).x
        );

        // Test the other direction
        let result = rect2.align_to(&rect1, &alignment);
        // The size hasn't changed
        assert_eq!(result.size, rect2.size);

        // Bottom is at top - 1
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).y,
            rect1.top_left.y - 1
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect2.anchor_point(AnchorPoint::BottomRight).x
        );
    }

    #[test]
    fn test_bottom_to_top_empty() {
        let rect1 = Rectangle::new(Point::new(0, 0), Size::zero());
        let rect2 = Rectangle::with_corners(Point::new(30, 20), Point::new(40, 50));

        let alignment = Alignment::vertical(AlignmentPosition::Before);

        let result = rect1.align_to(&rect2, &alignment);
        // The size hasn't changed
        assert_eq!(result.size, rect1.size);

        // Bottom is at top
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).y,
            rect2.top_left.y
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect1.anchor_point(AnchorPoint::BottomRight).x
        );

        // Test the other direction
        let result = rect2.align_to(&rect1, &alignment);
        // The size hasn't changed
        assert_eq!(result.size, rect2.size);

        // Bottom is at top - 1
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).y,
            rect1.top_left.y - 1
        );

        // Horizontal coordinate is unchanged
        assert_eq!(
            result.anchor_point(AnchorPoint::BottomRight).x,
            rect2.anchor_point(AnchorPoint::BottomRight).x
        );
    }
}
