use {
    AddBevel,
    AddBorder,
    AddRound,
};
use internal::{
    CanRectangle,
    HasRectangle,
    Line,
    Radius,
    Rectangle,
    SourceRectangle,
};

pub struct RectangleVariant(pub Rectangle);
pub struct EllipseVariant(pub Rectangle);
pub struct LineVariant(pub Line);
pub struct ImageVariant<'a, I> {
    pub image: &'a I, 
    pub src_rect: SourceRectangle
}

pub struct BevelCorner(pub Radius);
pub struct RoundCorner(pub Radius);

pub type EllipseShape = Shape<EllipseVariant, (), ()>;
pub type EllipseBorderShape = Shape<EllipseVariant, Radius, ()>;
pub type BevelRectangleShape = Shape<RectangleVariant, (), BevelCorner>;
pub type BevelRectangleBorderShape = Shape<RectangleVariant, Radius, BevelCorner>;
pub type ImageShape<'a, I> = Shape<ImageVariant<'a, I>, (), ()>;
pub type LineShape = Shape<LineVariant, (), ()>;
pub type RectangleShape = Shape<RectangleVariant, (), ()>;
pub type RectangleBorderShape = Shape<RectangleVariant, Radius, ()>;
pub type RoundRectangleShape = Shape<RectangleVariant, (), RoundCorner>;
pub type RoundRectangleBorderShape = Shape<RectangleVariant, Radius, RoundCorner>;

/// A rectangle shape.
#[deriving(Copy)]
pub struct Shape<
    TVariant, 
    TBorderRadius,
    TCorner
> {
    /// Rectangle.
    pub variant: TVariant,
    /// Border radius.
    pub border_radius: TBorderRadius,
    /// Round radius.
    pub corner: TCorner,
}


impl
AddRound<RoundRectangleShape> 
for RectangleShape {
    #[inline(always)]
    fn round(
        &self, 
        radius: Radius
    ) -> RoundRectangleShape {
        Shape {
            variant: self.variant,
            corner: RoundCorner(radius),
            border_radius: self.border_radius,
        }
    }
}

impl
AddRound<RoundRectangleBorderShape> 
for RectangleBorderShape {
    #[inline(always)]
    fn round(&self, radius: Radius) -> RoundRectangleBorderShape {
        Shape {
            variant: self.variant,
            border_radius: self.border_radius,
            corner: RoundCorner(radius),
        }
    }
}

impl
AddBevel<BevelRectangleShape> 
for RectangleShape {
    #[inline(always)]
    fn bevel(
        &self, 
        radius: Radius
    ) -> BevelRectangleShape {
        Shape {
            variant: self.variant,
            corner: BevelCorner(radius),
            border_radius: self.border_radius,
        }
    }
}


impl
AddBorder<EllipseBorderShape> 
for EllipseShape {
    #[inline(always)]
    fn border_radius(
        &self, 
        radius: f64
    ) -> EllipseBorderShape {
        Shape {
            border_radius: radius,
            variant: self.variant,
            corner: self.corner,
        }
    }
}

impl
AddBorder<RectangleBorderShape> 
for RectangleShape {
    #[inline(always)]
    fn border_radius(
        &self, 
        radius: f64
    ) -> RectangleBorderShape {
        Shape {
            border_radius: radius,
            variant: self.variant,
            corner: self.corner,
        }
    }
}

impl
AddBevel<BevelRectangleBorderShape> 
for RectangleBorderShape {
    #[inline(always)]
    fn bevel(
        &self, 
        radius: Radius
    ) -> BevelRectangleBorderShape {
        Shape {
            corner: BevelCorner(radius),
            variant: self.variant,
            border_radius: self.border_radius,     
        }
    }
}

/// Shapes containing a rectangle can change rectangle.
///
/// Can not contain an ellipse.
impl<B: Copy, C: Copy>
CanRectangle<Shape<RectangleVariant, B, C>, Rectangle> 
for Shape<RectangleVariant, B, C> {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> Shape<RectangleVariant, B, C> {
        Shape { 
            variant: RectangleVariant(rect), 
            ..*self 
        }
    }
}

/// Shapes containing an ellipse can change rectangle.
///
/// Can not contain a rectangle.
impl<B: Copy, C: Copy>
CanRectangle<Shape<EllipseVariant, B, C>, Rectangle> 
for Shape<EllipseVariant, B, C> {
    #[inline(always)]
    fn rectangle(&self, rect: Rectangle) -> Shape<EllipseVariant, B, C> {
        Shape { 
            variant: EllipseVariant(rect), 
            ..*self 
        }
    }
}


/// Gets rectangle of rectangle shape.
impl<B, C>
HasRectangle<Rectangle>
for Shape<RectangleVariant, B, C> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        let RectangleVariant(res) = self.variant;
        res
    }
}

/// Gets ellipse of ellipse shape.
impl<B, C>
HasRectangle<Rectangle>
for Shape<EllipseVariant, B, C> {
    #[inline(always)]
    fn get_rectangle(&self) -> Rectangle {
        let EllipseVariant(res) = self.variant;
        res
    }
}

