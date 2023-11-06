use embedded_graphics::{
    prelude::{Dimensions, DrawTarget, Point},
    primitives::{
        ContainsPoint, OffsetOutline, PointsIter, Primitive, Rectangle, Styled, StyledDimensions,
        StyledDrawable,
    },
    transform::Transform,
};

fn rect_with_padding(rect: Rectangle, top: i32, right: i32, bottom: i32, left: i32) -> Rectangle {
    let rect_top = rect.top_left.y;
    let rect_right = rect.bottom_right().unwrap_or(rect.top_left).x;
    let rect_bottom = rect.bottom_right().unwrap_or(rect.top_left).y;
    let rect_left = rect.top_left.x;

    let final_top = rect_top.saturating_sub(top);
    let final_right = rect_right.saturating_add(right);
    let final_bottom = rect_bottom.saturating_add(bottom);
    let final_left = rect_left.saturating_sub(left);

    Rectangle::with_corners(
        Point::new(final_left, final_top),
        Point::new(final_right, final_bottom),
    )
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Padding<C> {
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
    child: C,
}

impl<C> Padding<C> {
    #[inline]
    pub const fn zero(child: C) -> Self {
        Self {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0,
            child,
        }
    }

    #[inline]
    pub const fn horizontal(padding: i32, child: C) -> Self {
        Self {
            top: 0,
            right: padding,
            bottom: 0,
            left: padding,
            child,
        }
    }

    #[inline]
    pub const fn vertical(padding: i32, child: C) -> Self {
        Self {
            top: padding,
            right: 0,
            bottom: padding,
            left: 0,
            child,
        }
    }

    #[inline]
    pub const fn vertical_and_horizontal(v: i32, h: i32, child: C) -> Self {
        Self {
            top: v,
            right: h,
            bottom: v,
            left: h,
            child,
        }
    }

    #[inline]
    pub const fn all(padding: i32, child: C) -> Self {
        Self {
            top: padding,
            right: padding,
            bottom: padding,
            left: padding,
            child,
        }
    }

    #[inline]
    pub const fn each(top: i32, right: i32, bottom: i32, left: i32, child: C) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
            child,
        }
    }
}

impl<C> Dimensions for Padding<C>
where
    C: Dimensions,
{
    #[inline]
    fn bounding_box(&self) -> Rectangle {
        rect_with_padding(
            self.child.bounding_box(),
            self.top,
            self.right,
            self.bottom,
            self.left,
        )
    }
}

impl<C> Transform for Padding<C>
where
    C: Transform,
{
    #[inline]
    fn translate(&self, by: Point) -> Self {
        Self {
            top: self.top,
            right: self.right,
            bottom: self.bottom,
            left: self.bottom,
            child: self.child.translate(by),
        }
    }

    #[inline]
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.child.translate_mut(by);
        self
    }
}

impl<C> ContainsPoint for Padding<C>
where
    C: ContainsPoint,
{
    #[inline]
    fn contains(&self, point: Point) -> bool {
        self.child.contains(point)
    }
}

impl<C> OffsetOutline for Padding<C>
where
    C: OffsetOutline,
{
    #[inline]
    fn offset(&self, offset: i32) -> Self {
        Self {
            top: self.top,
            right: self.right,
            bottom: self.bottom,
            left: self.bottom,
            child: self.child.offset(offset),
        }
    }
}

impl<C> PointsIter for Padding<C>
where
    C: PointsIter,
{
    type Iter = C::Iter;

    #[inline]
    fn points(&self) -> Self::Iter {
        self.child.points()
    }
}

impl<C> Primitive for Padding<C>
where
    C: Primitive,
{
    #[inline]
    fn into_styled<S>(self, style: S) -> Styled<Self, S>
    where
        Self: Sized,
    {
        Styled {
            primitive: self,
            style,
        }
    }
}

impl<C, S> StyledDimensions<S> for Padding<C>
where
    C: StyledDimensions<S>,
{
    #[inline]
    fn styled_bounding_box(&self, style: &S) -> Rectangle {
        rect_with_padding(
            self.child.styled_bounding_box(style),
            self.top,
            self.right,
            self.bottom,
            self.left,
        )
    }
}

impl<C, S> StyledDrawable<S> for Padding<C>
where
    C: StyledDrawable<S>,
{
    type Color = C::Color;
    type Output = C::Output;

    #[inline]
    fn draw_styled<D>(&self, style: &S, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.child.draw_styled(style, target)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use embedded_graphics::geometry::Size;

    #[test]
    fn test_zero_padding() {
        let test_rect = Rectangle::new(Point::new(10, 13), Size::new(5, 8));

        let padded_rect = Padding::zero(test_rect);

        assert_eq!(test_rect, padded_rect.bounding_box());
    }

    #[test]
    fn test_positive_padding() {
        let test_rect = Rectangle::new(Point::new(10, 13), Size::new(5, 8));

        let padded_rect = Padding::each(2, 12, 57, 9, test_rect);

        assert_eq!(
            Rectangle::with_corners(Point::new(1, 11), Point::new(26, 77)),
            padded_rect.bounding_box()
        );
    }

    #[test]
    fn test_negative_padding() {
        let test_rect = Rectangle::new(Point::new(10, 13), Size::new(5, 8));

        let padded_rect = Padding::all(-1, test_rect);

        assert_eq!(
            Rectangle::new(Point::new(11, 14), Size::new(3, 6)),
            padded_rect.bounding_box()
        );
    }
}
