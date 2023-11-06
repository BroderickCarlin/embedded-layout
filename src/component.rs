use embedded_graphics::{
    prelude::{Dimensions, DrawTarget, Point},
    primitives::{
        ContainsPoint, OffsetOutline, PointsIter, Rectangle, StyledDimensions, StyledDrawable,
    },
    transform::Transform,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Component<A, B> {
    child_a: A,
    child_b: B,
}

impl<A, B> Component<A, B> {
    #[inline]
    pub const fn new(child_a: A, child_b: B) -> Self {
        Self { child_a, child_b }
    }
}

impl<A, B> Dimensions for Component<A, B>
where
    A: Dimensions,
    B: Dimensions,
{
    #[inline]
    fn bounding_box(&self) -> Rectangle {
        let bb_a = self.child_a.bounding_box();
        let bb_b = self.child_b.bounding_box();

        let top_left = bb_a.top_left.component_min(bb_b.top_left);
        let bottom_right = bb_a
            .bottom_right()
            .unwrap_or(bb_a.top_left)
            .component_max(bb_b.bottom_right().unwrap_or(bb_b.top_left));

        Rectangle::with_corners(top_left, bottom_right)
    }
}

impl<A, B> Transform for Component<A, B>
where
    A: Transform,
    B: Transform,
{
    #[inline]
    fn translate(&self, by: Point) -> Self {
        Self {
            child_a: self.child_a.translate(by),
            child_b: self.child_b.translate(by),
        }
    }

    #[inline]
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.child_a.translate_mut(by);
        self.child_b.translate_mut(by);
        self
    }
}

impl<A, B> ContainsPoint for Component<A, B>
where
    A: ContainsPoint,
    B: ContainsPoint,
{
    #[inline]
    fn contains(&self, point: Point) -> bool {
        self.child_a.contains(point) || self.child_b.contains(point)
    }
}

impl<A, B> OffsetOutline for Component<A, B>
where
    A: OffsetOutline,
    B: OffsetOutline,
{
    #[inline]
    fn offset(&self, offset: i32) -> Self {
        Self {
            child_a: self.child_a.offset(offset),
            child_b: self.child_b.offset(offset),
        }
    }
}

impl<A, B> PointsIter for Component<A, B>
where
    A: PointsIter,
    B: PointsIter,
{
    type Iter = core::iter::Chain<A::Iter, B::Iter>;

    #[inline]
    fn points(&self) -> Self::Iter {
        self.child_a.points().chain(self.child_b.points())
    }
}

impl<A, B, S> StyledDimensions<S> for Component<A, B>
where
    A: StyledDimensions<S>,
    B: StyledDimensions<S>,
{
    #[inline]
    fn styled_bounding_box(&self, style: &S) -> Rectangle {
        let a = self.child_a.styled_bounding_box(style);
        let b = self.child_b.styled_bounding_box(style);

        let top_left = a.top_left.component_min(b.top_left);
        let bottom_right = a
            .bottom_right()
            .unwrap_or(a.top_left)
            .component_max(b.bottom_right().unwrap_or(b.top_left));

        Rectangle::with_corners(top_left, bottom_right)
    }
}

impl<A, B, S> StyledDrawable<S> for Component<A, B>
where
    A: StyledDrawable<S>,
    B: StyledDrawable<S, Color = A::Color>,
{
    type Color = A::Color;
    type Output = (A::Output, B::Output);

    #[inline]
    fn draw_styled<D>(&self, style: &S, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let a = self.child_a.draw_styled(style, target)?;
        let b = self.child_b.draw_styled(style, target)?;

        Ok((a, b))
    }
}
